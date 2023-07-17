


use bcrypt::{DEFAULT_COST, hash, verify};
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};
use serde::{Serialize,Deserialize};
use crate::{
    database::{
        Database
    },
    modules::{
        generic_type::{ResponseType, GenericResponse},
        util::{self, encode_token_and_refresh}}
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
            println!("email already registered");
            return Err(Status::BadRequest)},
    }
}


pub fn sign_in(db: &State<Database>,user:Json<UserLoginRequestType>)
->ResponseType<Option<LoginResponse>>
{
    let user_detail = db.user().find_one(&user.email);
    
    match user_detail {
        Ok(None)=>Err(Status::BadGateway),
        Err(_)=>Err(Status::BadGateway),
        Ok(Some(registered_user))=>{
          let is_password_valid  = verify(&user.password,&registered_user.password);
          if let Ok(false) =is_password_valid{
            println!("pass wrong");
          }
          println!("{:?}",registered_user.id.unwrap());
          let token = encode_token_and_refresh(registered_user.id.unwrap(), "secret","",3,4000000000).unwrap();
          
          let login_response = LoginResponse {
            user_detail: registered_user,
            access_token:token.token,
            refresh_token:token.refresh_token
        };
          
          let response_json = GenericResponse {
            status: "success".to_string(),
            message: user.email.to_string() + "has successfully logged in.",
            data: Some(login_response),
        };
        Ok(Json(response_json))
        },

    }
}