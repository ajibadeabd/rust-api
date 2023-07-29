use std::time::Duration;

use mongodb::{
    sync::{Client, ClientSession},
    results::InsertOneResult, bson::{oid::ObjectId, Document, Bson, doc}, options::{FindOneOptions, UpdateModifications, self, TransactionOptions, Acknowledgment, WriteConcern, ReadConcern}};
use rocket::{State, serde::json::Json, http::Status};

use crate::{database::Database, modules::{response_handler::CustomError, provider::payment::{PaymentProviderHashMap, PaystackApi, paystack::{TransactionDTO, DepositResponseDataDetails, TransferPAymentPayload}}}, app::{user::user_model::User, account::account_type::{TransactionStatus, TransactionType}}};

use super::{account_type::{AccountData, DepositAccountData, WithdrawAccountData}, account_model::Account, transaction_model::Transaction};







pub fn create_transaction(db: &State<Database>,new_transaction:Transaction)
-> Result<InsertOneResult,mongodb::error::Error>
{
     db.transaction().save(&new_transaction)
}
pub fn get_provider_name(amount: f64, currency: &str)->Option<std::string::String>{
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
    let provider_name = get_provider_name(deposit_data.amount,&deposit_data.currency);
             
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
    let created_transaction = create_transaction(db,new_transaction).unwrap();
            let provider_instance = get_provider_instance (provider_name.unwrap().as_str()).unwrap();
            println!("here {:?}",created_transaction.inserted_id.to_owned().clone());
            let transaction_id   =  match created_transaction.inserted_id {
                Bson::ObjectId(object_id) => object_id.to_string(),
                _ => panic!("Expected ObjectId type for inserted_id."),
            };

            let dto = TransactionDTO {
                transactionReference: transaction_id,
                currency:deposit_data.currency.clone(),
                amount: deposit_data.amount.clone(),
                userEmail: email,
                callbackUrl: "http://localhost:8000/callback".to_string()
            };  
        provider_instance.initialize_transaction(dto).await
    
}





pub async fn lock_amount(db: &State<Database>,user_account_id:String, amount:f64,session : &mut ClientSession){
       let update_doc = UpdateModifications::Document(
        doc!{ "$inc": { "amount": -amount, "locked_balance": amount }}
    );
      
       let new_tran =  db.transaction().update_one(doc!{
        "_id":user_account_id,
        }, update_doc, None, Some(session));
    //    .save_with_session(&new_transaction, &mut session).unwrap();

}
pub async  fn initialize_withdrawal(db: &State<Database>,withdraw_data:Json<WithdrawAccountData >,user_account_id:Option<ObjectId>)
// ->Result<DepositResponseDataDetails,CustomError>{
    ->Result<(),CustomError>{
    let provider_name = get_provider_name(withdraw_data.amount,&withdraw_data.currency);
             
    if let None  = &provider_name {
        return Err(CustomError::BadRequest("Unable to process payment. Currency not supported.".to_string()));
    };
    let new_transaction= Transaction::new(
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
             session.start_transaction(    options );

       let new_tran =  db.transaction().save_with_session(&new_transaction, &mut session).unwrap();
    let _ = lock_amount(
        db,
        user_account_id.unwrap().to_string(),
        withdraw_data.amount.clone(),
       &mut  session
    ).await;

    let provider_instance = get_provider_instance(provider_name.unwrap().as_str()).unwrap();

    let transfer_payment_payload =  TransferPAymentPayload {
                      account_type:"nuban".to_string(),
                      account_number:withdraw_data.bank_account_number.to_string(),
                      name:withdraw_data.bank_account_name.clone(),
                      bank_code:withdraw_data.bank_code.to_string(),
                      currency:withdraw_data.currency.clone()
    };
    let _= provider_instance.initialize_transfer(transfer_payment_payload).await;
 session.cluster_time();
    Ok(())
 
    
}



