use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, Document}, options::FindOneOptions};
use rocket::{State, serde::json::Json, http::Status};

use crate::{database::Database, modules::response_handler::CustomError, app::user::user_model::User};

use super::{account_type::AccountData, account_model::Account};







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
