use bcrypt::{DEFAULT_COST, hash, verify};
use mongodb::{bson::{oid::ObjectId, doc}, options::UpdateModifications, Cursor};
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};
use serde::{Serialize,Deserialize};
use crate::{
    database::{
        Database
    },
    modules::{
        util::{self, encode_token_and_refresh}, response_handler::{CustomError, CustomResult, generic_response}
    }, app::account::{account_service::create_new_account, account_model::Account
        }
};

use super::{
    user_model::User,
    types::{UserLoginRequestType, LoginResponse, }, user_service::{update_user_account, self}
};
 
pub fn sign_up(db: &State<Database>,mut user:Json<User>)
// -> ResponseType<Option<String>>
-> Result<CustomResult, CustomError>
{
    
    let user_detail = db.user().save(&mut user);
    match user_detail {
        Ok(res) => Ok({
            let  new_account_data = Account::new(
                "INTERNAL".to_string(),
                "NGN".to_string(),
                res.inserted_id.as_object_id() 
            );
            let user_account = create_new_account(db,new_account_data);
                if let Ok(account) = user_account {
            let update_doc = UpdateModifications::Document( doc! { "$set": { "accounts": [account.inserted_id] }  });
           let _ = update_user_account(db,doc!{"_id": res.inserted_id.as_object_id() },update_doc,None);
                }
            
          generic_response::<Option<String>>(
            "has successfully logged in.",
           None,
           None
       )
        }),
        Err(err) => {
            return Err(CustomError::BadRequest("Email already registered".to_string()))},
    }
}


pub fn sign_in(db: &State<Database>,user:Json<UserLoginRequestType>)
-> Result<CustomResult, CustomError>

{
    let user_detail = db.user().find_one("email",&user.email);
    
    match user_detail {
        Ok(None)=>Err(CustomError::NotFound("User not found".to_string())),
        Err(_)=>Err(CustomError::NotFound("Unable to log in".to_string())),
        Ok(Some(registered_user))=>{
          let is_password_valid  = verify(&user.password,&registered_user.password);
          if let Ok(false) =is_password_valid{
           return  Err(CustomError::NotFound("Not Found".to_string()))
          }
          let token = encode_token_and_refresh(registered_user.id.unwrap(), "secret","",3,4000000000).unwrap();
          
          let login_response = LoginResponse {
            user_detail: registered_user,
            access_token:token.token,
            refresh_token:token.refresh_token
        };
        
         Ok(generic_response(
             "has successfully logged in.",
            Some(login_response),
            None
        ))
        },

    }
    
} 
 