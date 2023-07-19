
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};

use crate::{
    modules::{generic_type::ResponseType, response_handler::{CustomError, CustomResult}},
    database::Database};
use super::{
    account_type::AccountData,
    account_controller
};
 
#[post("/", data = "<account_data>")]
pub async fn account_creation(
    db: &State<Database>,
    account_data: Json<AccountData>) 
-> Result<CustomResult, CustomError>

    // ->rocket::response::status::Custom<ResponseType<Option<String>>>
     {
       account_controller::create_account(db, account_data)
    //    rocket::response::status::Custom(Status::Created, response)
}

