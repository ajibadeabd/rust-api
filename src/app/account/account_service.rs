use mongodb::results::InsertOneResult;
use rocket::{State, serde::json::Json, http::Status};

use crate::{database::Database, modules::response_handler::CustomError};

use super::account_type::AccountData;







pub fn create_new_account(db: &State<Database>,account_data: Json<AccountData>)
-> Result<InsertOneResult,CustomError>
{
    let new_account = db.account().save(&account_data);

    match new_account {
        Ok(account)=>Ok(account),
        Err(error)=>Err(CustomError::NotFound("Unable to create an account".to_string()))
    }


}
