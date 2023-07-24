
use rocket::{ 
    
      serde::json::Json,State};

use crate::{
    modules::{ response_handler::{ CustomError, CustomResult
    }
},
    database::Database, app::{user::user_model::User, account::account_type::DepositAccountData}};
use super::{
    account_type::AccountData,
    account_controller
};

 
 
#[post("/", data = "<account_data>")]
pub async fn account_creation(
    db: &State<Database>,
    account_data: Json<AccountData>,
     auth_user: User
    )

-> Result<CustomResult, CustomError>

     {
       account_controller::create_account(db, account_data,auth_user)
}

#[post("/deposit", data = "<deposit_data>")]
pub async fn deposit(
    db: &State<Database>,
    deposit_data: Json<DepositAccountData>,
     auth_user: User
    // )-> Result<CustomResult, CustomError> {
    // )->(){
    )->Result<(), CustomError>{
       account_controller::initialize_deposit(db, deposit_data,auth_user)
}

