use mongodb::{results::{InsertOneResult, UpdateResult}, bson::{ Document, doc, oid::ObjectId }, options::{FindOneOptions, UpdateModifications, UpdateOptions}, sync::ClientSession};
use rocket::{State};

use crate::{database::Database, modules::response_handler::CustomError};

use super::{account_model::Account};







pub fn create_new_account(db: &State<Database>,new_account_data: Account)
-> Result<InsertOneResult,CustomError>
{

    let new_account = db.account().save(&new_account_data);

    match new_account {
        Ok(account)=>Ok(account),
        Err(error)=>Err(CustomError::NotFound("Unable to create an account".to_string()))
    }
}


pub fn get_account(db: &State<Database>,find_by:Document,filer_by:Option<FindOneOptions>)
-> Result<Option<Account>,mongodb::error::Error>
{
    db.account().find_one(find_by,filer_by)
}

pub fn accounts(db: &State<Database>,currency:Option<String>,user_id:Option<ObjectId>)
->  Vec<Account> 

{
    db.account().find(Some(doc!{ "user_id":user_id})).unwrap()
}

 
pub fn update_account_transaction(db: &State<Database>,filter_by:Document,update_doc:UpdateModifications,update_option:Option<UpdateOptions>,session:Option<&mut ClientSession>)-> Result<UpdateResult,mongodb::error::Error>{
  db.account().update_one(filter_by,update_doc,update_option,session)
}

 

// let transactions = account_service::accounts(db,currency,auth_user.id);
// Ok(generic_response ("Transfer transaction successfully done.",Some(transactions),Some(Status::Created.code)))
