
use rocket::{ serde::json::Json,State, http::uri::Query };
use serde::{Deserialize, Serialize};

use crate::{ modules::{ response_handler::{ CustomError, CustomResult }
}, database::Database, app::{user::user_model::User, account::account_type::{DepositAccountData, WithdrawAccountData, TransferPaymentData}}};
use super::{
    account_type::{AccountData, TransactionsQueryData},
    account_controller
};

use std::collections::HashMap;
 
#[post("/", data = "<account_data>")]
pub async fn account_creation(
    db: &State<Database>,
    account_data: Json<AccountData>,
     auth_user: User
    )-> Result<CustomResult, CustomError> {
       account_controller::create_account(db, account_data,auth_user)
}

#[post("/deposit", data = "<deposit_data>")]
pub async fn deposit(
    db: &State<Database>,
    deposit_data: Json<DepositAccountData>,
     auth_user: User
    )-> Result<CustomResult, CustomError> {
       account_controller::initialize_deposit(db, deposit_data,auth_user).await 
}

#[post("/withdraw", data = "<withdraw_data>")]
pub async fn withdraw(
    db: &State<Database>,
    withdraw_data: Json<WithdrawAccountData>,
     auth_user: User
    )-> Result<CustomResult, CustomError> {
       account_controller::initialize_withdrawal(db, withdraw_data,auth_user.id).await 
}
#[post("/transfer_funds", data = "<transfer_data>")]
pub async fn transfer_funds(
    db: &State<Database>,
    transfer_data: Json<TransferPaymentData>,
     auth_user: User
    )-> Result<CustomResult, CustomError> {
       account_controller::transfer_funds(db, transfer_data , auth_user).await 
}

// #[post("/transactions", data = "<transfer_data>")]
// pub async fn transactions(
//     db: &State<Database>,
//     transfer_data: Json<TransferPaymentData>,
//      auth_user: User
//     )-> Result<CustomResult, CustomError> {
//        account_controller::transfer_funds(db, transfer_data , auth_user).await 
// }


// transactionId?: string;
//   accountId?: string;
//   currency?: string;


// Route handler for /transactions (matches all POST requests to /transactions with query parameters)
#[post("/transactions?<currency>&<transaction_id>&<account_id>")]
pub async fn transactions(
    db: &State<Database>,
    currency:String,
    transaction_id:String,
    account_id:String,
    auth_user: User,
) -> Result<CustomResult, CustomError> {
    account_controller::transactions(
        db,
        TransactionsQueryData {
            currency:Some(currency) ,
            transaction_id:Some(transaction_id),
            account_id:Some(account_id),
        },
        auth_user
    ).await
} 