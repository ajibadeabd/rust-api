

use mongodb::{results::{InsertOneResult, UpdateResult}, bson::{ Document, doc, oid::ObjectId }, options::{FindOneOptions, UpdateModifications, UpdateOptions, FindOptions}, sync::ClientSession};
use rocket::{State, serde::json::Json};


use crate::{database::Database, modules::{response_handler::CustomError, provider::payment::{PaymentProviderHashMap, paystack::PaymentEvent}}, app::account::transaction_service};

use super::{account_model::Account, account_type::{PaymentEventRequestBody, TransactionType}, transaction_model::Transaction};


// use hmac::{Hmac, NewMac};
// use sha2::Sha512;




use hmac::{Mac};






pub fn create_new_account(db: &State<Database>,new_account_data: Account)
-> Result<InsertOneResult,CustomError>
{

    let new_account = db.account().save(&new_account_data);

    match new_account {
        Ok(account)=>Ok(account),
        Err(_error)=>Err(CustomError::NotFound("Unable to create an account".to_string()))
    }
}


pub fn get_account(db: &State<Database>,find_by:Document,filer_by:Option<FindOneOptions>)
-> Result<Option<Account>,mongodb::error::Error>
{
    db.account().find_one(find_by,filer_by)
}

pub fn accounts(db: &State<Database>,_currency:Option<String>,user_id:&Option<ObjectId>)
->  Vec<Account> 

{
    db.account().find(Some(doc!{ "user_id":user_id}),None).unwrap()
}

 
pub fn accounts_with_transaction(db: &State<Database>,_currency:Option<String>,user_id:&Option<ObjectId>)
// ->  Vec<Account> 
->Vec<Account>

{
    // FindOptions::Documents;
    let aggregation_pipeline: Vec<Document> = vec![
        doc! {
        "$match":{
            "user_id":user_id
               },
        },
        doc! {
        "$lookup": {
            "from": "Transaction",
            "localField": "transactions",
            "foreignField": "_id",
            "as": "transactions"
         },

    }
    ];

     db.account().accounts_with_transactions(aggregation_pipeline,None).unwrap()
    // .unwrap()
}

 

pub fn update_account_transaction(db: &State<Database>,filter_by:&Document,update_doc:&UpdateModifications,update_option:Option<UpdateOptions>,session:Option<&mut ClientSession>)-> Result<UpdateResult,mongodb::error::Error>{
  db.account().update_one(filter_by,update_doc,update_option,session)
}

 
pub fn webhook(db: &State<Database>,signature:String,provider_name:String,payload:Json<PaymentEventRequestBody>)
// -> Result<(),Value>
// -> Option<String>
// ->(bool, Option<String>)
->Option<bool> 
{
    let provider = PaymentProviderHashMap::new();
   let provider_instance =  provider.get_provider_instance(Some(provider_name.as_str())).unwrap();
    // if verify_hash_key(&payload) {
    //     return Err(CustomError::BadRequest("wrong signature".to_string()));
    // }
   let eventData: PaymentEvent =  provider_instance.verify_webhook_payload(&signature, &payload).unwrap();
     let transaction = get_transaction_by_reference (db,&eventData);
    //  if  transaction.is_none() {
    //     // return None;
    //     return Err(CustomError::BadRequest("Transaction record not found".to_string()));
    //  }
     match transaction {
        None=>{
            println!("Transaction record not found");
        return None;
        },
        Some(transaction)=>{
            if &eventData.transaction_type != &transaction.transaction_type {
                return None;
            }
            if eventData.transaction_type ==TransactionType::DEPOSIT{
                
            }else if eventData.transaction_type ==TransactionType::WITHDRAWAL {

            }

        }
     }
     return Some(true)
  

    // Ok(())
    // Ok(None)

    // db.account().update_one(filter_by,update_doc,update_option,session)
}

fn get_transaction_by_reference (db: &State<Database>,eventData:&PaymentEvent)->Option<Transaction>{
    let find_by = doc!{
        "provider_reference": eventData.provider_reference.to_string(),
        "_id": eventData.transaction_id.to_string(),
    };
   transaction_service::get_transaction(db, find_by)

}
  
// pub fn verify_hash_key(payload:&Json<PaymentEventRequestBody>){

// type HmacSha256 = Hmac<Sha256>;
// let payload_string = serde_json::to_string(&payload).unwrap();

//     let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
//     .expect("HMAC can take key of any size");
//     mac.update(b"payload_string");
// }
 

// let transactions = account_service::webhook(x_paystack_signature,provider);
 

// let transactions = account_service::accounts(db,currency,auth_user.id);
// Ok(generic_response ("Transfer transaction successfully done.",Some(transactions),Some(Status::Created.code)))
