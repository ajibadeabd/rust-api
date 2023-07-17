use rocket::{State, serde::json::Json};

use crate::{database::Database, modules::generic_type::{GenericResponse, ResponseType}};

use super::{
    account_type::AccountData,
    account_service
};







pub fn create_account(db: &State<Database>,account_data: Json<AccountData>)
-> ResponseType<Option<String>>
{

    let new_account = account_service::create_new_account(db, account_data);

    let response = GenericResponse {
        message:"Account created successfully".to_owned(),
        status:"success".to_owned(),
        data:None
    };
    Ok(Json(response))
}
