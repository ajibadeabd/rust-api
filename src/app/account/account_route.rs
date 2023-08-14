
use rocket::{ serde::json::Json,State,  };


use crate::{ modules::{ response_handler::{ CustomError, CustomResult }, middleware::XStoreKeyHeader
}, database::Database, app::{user::user_model::User, account::account_type::{DepositAccountData, WithdrawAccountData, TransferPaymentData, PaymentEventRequestBody}}};
use super::{
    account_type::{AccountData, TransactionsQueryData},
    account_controller
};




 
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

 
// Route handler for /transactions (matches all POST requests to /transactions with query parameters)
#[get("/transactions?<currency>&<transaction_id>&<account_id>&<limit>&<page>")]
pub async fn transactions(
    db: &State<Database>,
    currency:Option<String>,
    transaction_id:Option<String>,
    account_id:Option<String>,
    page:Option<String>,
    limit:Option<String>,
    auth_user: User,
) -> Result<CustomResult, CustomError> {
    account_controller::transactions(
        db,
        TransactionsQueryData {
            currency ,
            transaction_id ,
            account_id ,
            page ,
           limit ,
        },
        auth_user
    ).await
} 
// Route handler for /transactions (matches all POST requests to /transactions with query parameters)
#[post("/callback/<provider>" , data = "<payload>") ]
pub async fn webhook(
    db: &State<Database>,
    x_paystack_signature: XStoreKeyHeader, 
    provider:String,
    payload:Json<PaymentEventRequestBody>
) -> Result<(), CustomError> {
    let _s= payload.data.source.as_ref().unwrap();
    // println!("{:?} ",&payload.data.source.unwrap());
    // println!("{:?} ",s.0);
    //println!("{:?} ",payload.data.log);
    // println!("{:?} ",payload.data.event_specific);
   account_controller::webhook(db, x_paystack_signature.token, provider,payload).await 
    
} 