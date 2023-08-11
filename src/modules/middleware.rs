 
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{Outcome, FromRequest};
use rocket::{Request, Data, State};

use crate::app::user::user_model::User;
use crate::database::Database;
use crate::modules::util::{decode_jwt, DecodeJwtHelper};
 


 
// // Allows a route to access the time a request was initiated.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) ->  Outcome<Self, ()> {

        let bearer_token = request.headers().get_one("Authorization").unwrap_or("");

        if !bearer_token.starts_with("Bearer ") {
           return  Outcome::Failure((Status::Unauthorized,()));
        }
        let token = &bearer_token[7..];
        let decode_token_response = decode_jwt(token, "secret");

        match decode_token_response {
        DecodeJwtHelper::Err=>  {
           return  Outcome::Failure((Status::Unauthorized,()));
        }
        DecodeJwtHelper::Ok(token)=>{
       let database = request.guard::<&State<Database>>().await;
       let user = database.unwrap().user().find_by_id(
        &ObjectId::parse_str(&token.claims.user_id).unwrap()
    );
       match user {
        Ok(Some(user)) =>  Outcome::Success(user),
        _=>{
            Outcome::Failure((Status::Unauthorized,()))}
    }
        }
        }
    }
}
#[derive(Debug )]

pub struct XStoreKeyHeader{
    pub token:String
}

// Implement FromRequest for the newtype
#[rocket::async_trait]

impl<'r> FromRequest<'r> for XStoreKeyHeader {
    type Error = ();

   async  fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let header = request.headers().get_one("x-paystack-signature").unwrap_or("");
    if header==""{
        Outcome::Failure((Status::Unauthorized,()))
    }else{
            Outcome::Success(XStoreKeyHeader{token:header.to_string()} )
         }
   }
}


  