use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, Document}, options::FindOneOptions};
use rocket::{State, serde::json::Json, http::Status};

use crate::{database::Database, modules::response_handler::CustomError, app::user::user_model::User};

use super::{account_type::{AccountData, DepositAccountData}, account_model::Account, transaction_model::Transaction};







pub fn create_transaction(db: &State<Database>,new_transaction:Transaction)
-> Result<InsertOneResult,mongodb::error::Error>
{
     db.transaction().save(&new_transaction)

    // match new_account {
    //     Ok(account)=>Ok(account),
    //     Err(error)=>Err(CustomError::NotFound("Unable to create an account".to_string()))
    // }
}

 