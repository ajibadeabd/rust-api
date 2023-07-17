use mongodb::results::InsertOneResult;
use rocket::{State, serde::json::Json, http::Status};

use crate::database::Database;

use super::account_type::AccountData;







pub fn create_new_account(db: &State<Database>,account_data: Json<AccountData>)
-> Result<InsertOneResult,Status>
{
    let new_account = db.account().save(&account_data);

    match new_account {
        Ok(account)=>Ok(account),
        Err(error)=>Err(Status::FailedDependency)
    }


}
