use mongodb::{bson::{doc, oid::ObjectId}, options::UpdateModifications};
use rocket::{State, serde::json::Json, http::Status};

use crate::{

    database::Database, modules::{
        response_handler::{
            CustomError, CustomResult, generic_response
        }
    }, 
    app::{user::{
        user_model::User, user_service::update_user_account
    }, account::transaction_service
}
};

use super::{
    account_type::{AccountData, DepositAccountData, WithdrawAccountData, TransferPaymentData, TransactionsQueryData, PaymentEventRequestBody, DashboardResponse, TransactionType},
    account_service::{self, get_account}, account_model::Account,
};




 
pub fn create_account(db: &State<Database>, account_data: Json<AccountData>,auth_user:User)
-> Result<CustomResult, CustomError>
{
    let is_account_exist = get_account(db,doc!{
        "user_id":auth_user.id,
        "currency":account_data.currency.to_string(),
       "channel": account_data.channel.to_string(),
    }, None);
    
    if let Ok(Some(_)) = is_account_exist {
       return  Err(CustomError::BadRequest("Account already created".to_string()))
    }
    let  new_account_data = Account::new(
        account_data.channel.to_string(),
         account_data.currency.to_string(),
        auth_user.id
    );
    let new_account = account_service::create_new_account(db, new_account_data);
    
    match new_account {
        Ok(_) => {
            let update_doc = UpdateModifications::Document(doc!  { "$push": { "accounts": new_account.unwrap().inserted_id }  });
            let _ = update_user_account(
                db,
                doc!{"_id":auth_user.id},
            update_doc,
            None);
            Ok(generic_response::<Option<String>>(
                "Account created successfully",
                 None,
                  Some(Status::Created.code))
                )
        },
        Err(_) => Err(CustomError::BadRequest("Error occurred while creating an account".to_string())),
    }
}

 



// initialize_withdrawal
pub async fn initialize_withdrawal(db: &State<Database>,withdraw_data: Json<WithdrawAccountData >,user_id:Option<ObjectId>)
-> Result<CustomResult, CustomError>
{
    let new_account = account_service::get_account(db,doc!{
        "user_id":user_id,
        "currency":withdraw_data.currency.clone(),
       "channel":"INTERNAL",
    }, None).unwrap();
    match new_account {
        None=> Err(CustomError::BadRequest(format!("User has no account in {}", withdraw_data.currency))),
        Some(user_account)=>{
              if user_account.balance < withdraw_data.amount {
                return Err(CustomError::BadRequest(format!("Account has insufficient funds." )));
              }
            let transfer_response = transaction_service::initialize_withdrawal(db,withdraw_data, user_account.id).await?;
            
            Ok(generic_response ("Withdraw transaction successfully initiated.",Some(transfer_response),Some(Status::Created.code)))

        }
    }
}
pub async fn initialize_deposit(db: &State<Database>,deposit_data: Json<DepositAccountData>,auth_user:User)
-> Result<CustomResult, CustomError>
{
    let new_account = account_service::get_account(db,doc!{
        "user_id":auth_user.id,
        "currency":&deposit_data.currency,
        "channel":"INTERNAL",
    }, None).unwrap();

    match new_account {
        None=> Err(CustomError::BadRequest(format!("User has no account in {}", deposit_data.currency))),
        Some(user_account)=>{
           let response =  transaction_service::initialize_deposit(db,deposit_data, user_account.id, auth_user.email).await;
    Ok(generic_response ("Deposit link successfully created.",Some(response.unwrap()),Some(Status::Created.code)))
        }
    }
}
pub async fn get_deposit(db: &State<Database>, auth_user:User)
-> Result<CustomResult, CustomError>
{
     let query =  TransactionsQueryData {
        transaction_id: None,
        limit: None,
        account_id: None,
        currency: None,
        page: None,
        transaction_type:Some(TransactionType::DEPOSIT)
      };
    let response =  transaction_service::transactions(db,Some(query),&auth_user);

    Ok(generic_response ("Deposit link successfully created.",Some(response),Some(Status::Created.code)))
}




pub async fn transfer_funds(db: &State<Database>,transfer_data: Json<TransferPaymentData >,auth_user:User)
-> Result<CustomResult, CustomError>
{
     let response = transaction_service::transfer_fund(db,transfer_data,auth_user
).await?;
    Ok(generic_response ("Transfer transaction successfully done.",Some(response),Some(Status::Created.code)))
}


pub async fn dashboard(db: &State<Database>,auth_user:User)
-> Result<CustomResult, CustomError>
{
    
    //  let accounts  = account_service::accounts_with_transaction(db,None,&auth_user.id);
    // //  let response = DashboardResponse{
    // //     accounts,
    // //     transactions`    
    // //  };

      let query_param = TransactionsQueryData {
        // limit:Some("2".to_owned()),
        transaction_id: None,
        limit: None,
        account_id: None,
        currency: None,
        page: None,
        transaction_type:None
      };
     let accounts = account_service::accounts(db,None,&auth_user.id);
     let transactions = transaction_service::transactions(db,Some(query_param),&auth_user);
let response = DashboardResponse {
    accounts,
    transactions

};

    Ok(generic_response ("Transfer transaction successfully done.",Some(response),Some(Status::Created.code)))
}


pub async fn transactions(db: &State<Database>,transaction_data: TransactionsQueryData,auth_user:User)
-> Result<CustomResult, CustomError>
{
     let transactions = transaction_service::transactions(db,Some(transaction_data),&auth_user);

    Ok(generic_response ("Transfer transaction successfully done.",Some(transactions),Some(Status::Created.code)))
}

pub async fn accounts(db: &State<Database>,currency:Option<String>,auth_user:User)
-> Result<CustomResult, CustomError>
{
     let accounts = account_service::accounts(db,currency,&auth_user.id);
    Ok(generic_response ("Transfer transaction successfully done.",Some(accounts),Some(Status::Created.code)))
}
 



pub async fn webhook(
    db: &State<Database>,
    x_paystack_signature: String,
    provider: String,
    payload:Json<PaymentEventRequestBody>,
) -> Result<(), CustomError> {
 
    let _transactions = account_service::webhook(db,x_paystack_signature,provider,    payload);
    // Ok(generic_response ("Transfer transaction successfully done.",Some(transactions),Some(Status::Created.code)))

    Ok(())
} 