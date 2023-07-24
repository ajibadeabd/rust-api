use mongodb::{bson::doc, options::UpdateModifications};
use rocket::{State, serde::json::Json, http::Status};

use crate::{

    database::Database, modules::{
        response_handler::{
            CustomError, CustomResult, generic_response
        }
    }, 
    app::{user::{
        user_model::User, user_service::update_user_account
    },
},
modules::{
    generic_type::{ResponseType, GenericResponse},
}
};

use super::{
    account_type::{AccountData, DepositAccountData},
    account_service::{self, get_account}, account_model::Account,
    transaction_service, transaction_model::Transaction,
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
                  Some(Status::Created.code)))
        },
        Err(_) => Err(CustomError::BadRequest("Error occurred while creating an account".to_string())),
    }
}

 




// deposit(db, account_data,auth_user)
pub fn initialize_deposit(db: &State<Database>,deposit_data: Json<DepositAccountData>,auth_user:User)
-> Result<(), CustomError>
// -> Result<CustomResult, CustomError>
{
    let new_account = account_service::get_account(db,doc!{
        "user_id":auth_user.id,
        "currency":deposit_data.currency.clone(),
       "channel":"INTERNAL",
    }, None).unwrap();
    println!("{:?}",new_account);

    match new_account {
        None=> Err(CustomError::BadRequest(format!("User has no account in {}", deposit_data.currency))),
        Some(user_account)=>{
            let provider_name = "".to_string();
            let new_transaction= Transaction::new(
                deposit_data.amount.clone(), 
                deposit_data.currency.clone(),
                0.0, 
                user_account.id, 
                provider_name);
            let new_transactionv = transaction_service::create_transaction(db,new_transaction);
            // let provider_name = get_provider_name();
            Ok(())
            

        }
    }

}