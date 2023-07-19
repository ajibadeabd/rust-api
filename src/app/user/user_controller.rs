


use bcrypt::{DEFAULT_COST, hash, verify};
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};
use serde::{Serialize,Deserialize};
use crate::{
    database::{
        Database
    },
    modules::{
        generic_type::{ResponseType, GenericResponse},
        util::{self, encode_token_and_refresh}, response_handler::{CustomError, CustomResult, generic_response}}
};

use super::{
    user_model::User,
    types::{UserLoginRequestType, LoginResponse, }
};
 
pub fn sign_up(db: &State<Database>,mut user:Json<User>)
-> ResponseType<Option<String>>
{
      
    
    let user_detail = db.user().save(&mut user);
    match user_detail {
        Ok(_) => Ok({
          let response_json = GenericResponse {
              status: "success".to_string(),
              message: user.email.to_string() + " account created",
              data:None
          };
          Json(response_json)
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
 