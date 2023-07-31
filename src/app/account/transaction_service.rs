use std::time::Duration;

use mongodb::{
    sync::{Client, ClientSession},
    results::InsertOneResult, bson::{oid::ObjectId, Document, Bson, doc}, options::{FindOneOptions, UpdateModifications, self, TransactionOptions, Acknowledgment, WriteConcern, ReadConcern}};
use rocket::{State, serde::json::Json, http::Status};

use crate::{database::Database, modules::{response_handler::CustomError, provider::payment::{PaymentProviderHashMap, PaystackApi, paystack::{TransactionDTO, DepositResponseDataDetails, TransferPAymentPayload}}}, app::{user::user_model::User, account::account_type::{TransactionStatus, TransactionType}}};

use super::{account_type::{AccountData, DepositAccountData, WithdrawAccountData, TransferPaymentData, TransactionsQueryData}, account_model::Account, transaction_model::Transaction, account_service::{get_account, update_account_transaction}};







pub fn create_transaction(db: &State<Database>,new_transaction:&Transaction)
-> Result<InsertOneResult,mongodb::error::Error>
{
     db.transaction().save(new_transaction)
}
pub fn get_provider_name(amount: &f64, currency: &str)->Option<std::string::String>{
    let provider = PaymentProviderHashMap::new();
    provider.get_provider_name(amount, currency)
}
pub fn get_provider_instance(provider_name:&str)->Option<PaystackApi>{
    let provider = PaymentProviderHashMap::new();
    Some(provider.get_provider_instance(Some(provider_name)).unwrap())
}


pub async  fn initialize_deposit(db: &State<Database>,deposit_data:Json<DepositAccountData>,user_account_id:Option<ObjectId>,email: String)
->Result<DepositResponseDataDetails,CustomError>
{
    let provider_name = get_provider_name(&deposit_data.amount,&deposit_data.currency);
             
    if let None  = &provider_name {
        return Err(CustomError::BadRequest("Unable to process payment. Currency not supported.".to_string()));
    };
    let new_transaction= Transaction::new(
        deposit_data.amount.clone(), 
        deposit_data.currency.clone(),
        0.0, 
        user_account_id.unwrap().to_string(),
        "INTERNAL".to_string(),
        provider_name.clone().unwrap(),
        TransactionType::DEPOSIT,
        TransactionStatus::PENDING,
    );
    let created_transaction = create_transaction(db,&new_transaction).unwrap();
    let transaction_id   = extract_object_id(&created_transaction.inserted_id);
    let update_doc = UpdateModifications::Document(
        doc!{ "$push": { "transactions": transaction_id} }
    );
    let filter_data = doc!{"_id":&user_account_id.unwrap()};
    let _ = update_account_transaction(db,filter_data,update_doc,None,None);

            let provider_instance = get_provider_instance (provider_name.unwrap().as_str()).unwrap();
            let dto = TransactionDTO {
                transactionReference: transaction_id.to_string(),
                currency:deposit_data.currency.clone(),
                amount: deposit_data.amount.clone(),
                userEmail: email,
                callbackUrl: "http://localhost:8000/callback".to_string()
            };  
        provider_instance.initialize_transaction(dto).await
    
}





pub async fn lock_amount(db: &State<Database>,user_account_id:  &ObjectId, amount: &f64, session : Option<&mut ClientSession>){
       let update_doc = UpdateModifications::Document(
        doc!{ "$inc": { "balance": -amount, "locked_balance": amount }}
    );
       let new_tran =  db.account().update_one(doc!{
        "_id":user_account_id,
        }, update_doc, None,  session);

}


pub async fn withdraw_fund(db: &State<Database>,user_account_id:  &ObjectId, amount: &f64, session : Option<&mut ClientSession>){
    let update_doc = UpdateModifications::Document(
     doc!{ "$inc": { "locked_balance": -amount }}
 );
   
    let new_tran =  db.account().update_one(doc!{
     "_id":user_account_id,
     }, update_doc, None, session);


}


pub async fn deposit_fund(db: &State<Database>,user_account_id:  &ObjectId, amount: &f64, session : Option<&mut ClientSession>){
    let update_doc = UpdateModifications::Document(
     doc!{ "$inc": { "balance": amount }}
 );
    let new_tran =  db.account().update_one(doc!{
     "_id":user_account_id,
     }, update_doc, None, session);
}



pub async  fn initialize_withdrawal(db: &State<Database>,withdraw_data:Json<WithdrawAccountData >,user_account_id:Option<ObjectId>)->Result<Transaction,CustomError>{
    let provider_name = get_provider_name(&withdraw_data.amount,&withdraw_data.currency);
             
    if let None  = &provider_name {
        return Err(CustomError::BadRequest("Unable to process payment. Currency not supported.".to_string()));
    };
    let mut new_transaction= Transaction::new(
        withdraw_data.amount.clone(), 
        withdraw_data.currency.clone(),
        0.0, 
        "INTERNAL".to_string(),
        user_account_id.unwrap().to_string(), 
        provider_name.clone().unwrap(),
        TransactionType::WITHDRAWAL,
        TransactionStatus::PENDING,
    );  
    // ClientSession;
    let  mut session  = Client::start_session(&db.client(),None).unwrap();
    let options = TransactionOptions::builder()
    .read_concern(ReadConcern::majority())
    .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
    .build();
    session.start_transaction(options);

       let new_transaction_result =  db.transaction().create(&new_transaction, Some(&mut session)).unwrap();
    let _ = lock_amount(
        db,
        &user_account_id.unwrap(),
        &withdraw_data.amount,
       Some(&mut  session)
    ).await;
    let transaction_id   = extract_object_id(&new_transaction_result.inserted_id);

    let update_doc = UpdateModifications::Document(
        doc!{ "$push": { "transactions": transaction_id} }
    );
    let filter_data = doc!{"_id":&user_account_id.unwrap()};
    let _ = update_account_transaction(db,filter_data,update_doc,None,None);


    let provider_instance = get_provider_instance(provider_name.unwrap().as_str()).unwrap();

    let account_name = provider_instance.get_account_name(
        &withdraw_data.bank_account_number,
        &withdraw_data.bank_code,
    ).await?;

    let transfer_payment_payload =  TransferPAymentPayload {
        account_type:"nuban".to_string(),
        account_number:withdraw_data.bank_account_number.to_string(),
        name:account_name,
        bank_code:withdraw_data.bank_code.clone(),
        currency:withdraw_data.currency.clone()
    };

   
    let transfer_response = provider_instance.initialize_transfer(transfer_payment_payload).await?;
    
    let transaction_id   =  extract_object_id(&new_transaction_result.inserted_id) ;
    let update_doc = UpdateModifications::Document(
         doc! { "$set":{"provider_fee":0, "provider_reference":&transfer_response.recipient_code} });

    let _ =  db.transaction().update_one(
        doc!{
            "_id":transaction_id,
        },update_doc, None,
        Some(&mut session));
    new_transaction.provider_reference = Some(transfer_response.recipient_code);
    new_transaction.provider_fee = Some(0.0);
    new_transaction.id = Some(transaction_id);
    session.commit_transaction();
    Ok(new_transaction)
 
    
}


pub async fn transfer_fund(db: &State<Database>,transfer_data:Json<TransferPaymentData>,auth_user:User)
-> Result< Transaction,CustomError>
{
    let user_account  = get_account(db,doc!{
        "user_id":auth_user.id,
        "currency":&transfer_data.currency
    },None);
    
    let user_account= match user_account {
        Ok(Some(user))=>user,
        _=>return Err(CustomError::BadRequest(format!("User has no account in {}.",transfer_data.currency)))
    };
    let receiver_object_id = &ObjectId::parse_str(&transfer_data.receiver_id).unwrap();
    
    if &auth_user.id.unwrap() == receiver_object_id {
       return Err(CustomError::BadRequest(format!("You cant transfer money to your self")));
    }
 
    let receiver_account  = get_account(db,doc!{
        "user_id":&ObjectId::parse_str(&transfer_data.receiver_id).unwrap(),
        "currency":&transfer_data.currency
    },None);
    let receiver_account= match receiver_account {
        Ok(Some(receiver))=>receiver,
        _=>return Err(CustomError::BadRequest(format!("Receiver has no account in {}.",transfer_data.currency)))
    };
    if &user_account.balance < &transfer_data.amount {
        return Err(CustomError::BadRequest(format!("Insufficient  balance")));
     }

    let _= lock_amount(db,&user_account.id.unwrap(),&transfer_data
    .amount,None).await;


    let _= deposit_fund(db, &receiver_account.id.unwrap(),&transfer_data
    .amount,None).await;

    let _= withdraw_fund(db,&user_account.id.unwrap(),&transfer_data
    .amount,None).await;


    let mut new_transaction= Transaction::new(
        transfer_data.amount.clone(), 
        transfer_data.currency.clone(),
        0.0, 
        receiver_account.id.unwrap().to_string(),
        user_account.id.unwrap().to_string(),
        "INTERNAL".to_string(),
        TransactionType::TRANSFER,
        TransactionStatus::SUCCESS,
    );
    let new_transaction_response = create_transaction(db, &new_transaction);
    let transaction_id   = extract_object_id(&new_transaction_response.unwrap().inserted_id);
    new_transaction.id=Some(transaction_id);
     
    
    let update_doc = UpdateModifications::Document(
        doc!{ "$push": { "transactions": transaction_id} }
    );
    let filter_data = doc!{"_id":&user_account.id.unwrap()};
    let _ = update_account_transaction(db,filter_data,update_doc,None,None);

    Ok(new_transaction)
}




pub fn transactions(
    db: &State<Database>,
    transaction_data:TransactionsQueryData,
    auth_user:User
)->Vec<Transaction>{
     let user_accounts: Vec<String> = auth_user.accounts.unwrap().iter().map(|id| id.to_hex()).collect();

    let mut filter_by = doc! {
        "$or": [
            {"receiver_id": {"$in": &user_accounts}},
            {"giver_id": {"$in": &user_accounts}},
        ]
    };
    if let Some(account_id) = &transaction_data.account_id {
        filter_by = doc! {
            "$or": [
                {"receiver_id":  account_id.trim()},
                {"giver_id":  account_id.trim() },
            ]
        };
        }
    
    if let Some(currency) = &transaction_data.currency {
        filter_by.insert("currency", currency);
    }
    if let Some(transaction_id ) = &transaction_data.transaction_id {
        filter_by.insert("_id", ObjectId::parse_str(transaction_id.trim()).unwrap());
    }
    if let Some(limit) = &transaction_data.limit {
        filter_by.insert("limit", limit);
    }
    if let Some(page) = &transaction_data.page {
        filter_by.insert("page", page);
    }
    
let transactions: Vec<Transaction> = db.transaction().find_all(Some(filter_by)).unwrap();
  transactions
}

fn extract_object_id (object_id: &Bson)->ObjectId{
    match object_id {
        Bson::ObjectId(object_id) => *object_id,
        _ => panic!("Expected ObjectId type for inserted_id."),
    }
}


