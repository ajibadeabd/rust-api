use rocket::{State, serde::json::Json, http::Status};
use serde_json::Value;

use crate::{database::Database, modules::{generic_type::{GenericResponse, ResponseType}, response_handler::{CustomError, CustomResult, generic_response}}};

use super::{
    account_type::AccountData,
    account_service
};







pub fn create_account(db: &State<Database>,account_data: Json<AccountData>)
-> Result<CustomResult, CustomError>
{

    let new_account = account_service::create_new_account(db, account_data);
    if let Err(error)= new_account {
        return Err(CustomError::BadRequest("Error occur while creating an account".to_string()))
    }
    let response:GenericResponse<Option<String>> = GenericResponse {
        message:"Account created successfully".to_owned(),
        status:"success".to_owned(),
        data:None
    };
    let json_response = serde_json::to_string(&response).unwrap();
    Ok(generic_response("has successfully logged in.",Some(json_response),None))
}
