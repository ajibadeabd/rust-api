 
use mongodb::bson::oid::ObjectId;
use rocket::data::{FromData, self, ToByteUnit, ByteUnit};
use rocket::http::{Status, ContentType};
use rocket::request::{Outcome, FromRequest, self};
use rocket::{Request, Data, State};
use serde::Deserialize;

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

 



#[derive(Debug,Deserialize)]
pub struct PaystackRequestPayload {
    value:String,
    // age: u16
}
#[derive(Debug)]
enum Error {
    TooLarge,
    NoColon,
    InvalidAge,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for PaystackRequestPayload  {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        // let person_ct = ContentType::new("application", "x-person");
        // if req.content_type() != Some(&person_ct) {
        //     return Forward(data);
        // }

        // Use a configured limit with name 'person' or fallback to default.
        let limit = req.limits().get("person").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        let string = request::local_cache!(req, string);
        Success(PaystackRequestPayload {value: string.to_owned() })
    }
}


// // use rocket::data::{ByteUnit, FromData, Outcome};
// // use rocket::{Data, Request};
// // use rocket::http::Status;
// // use rocket::data::ByteUnit::default;
// // use rocket::data::ByteUnit::bytes;
// // use rocket::data::ByteUnit::limit;
// // use rocket::data::ByteUnit::max;

// pub struct RequestBody {
//     value: String,
// }
// #[rocket::async_trait]
// impl<'r> rocket::data::FromData<'r> for RequestBody {
//     type Error = std::io::Error;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> rocket::data::Outcome<'r, Self> {
//         use Error::*;
//         use rocket::outcome::Outcome::*;

       
//         // let limit = ByteUnit::default().limit(256); // Set your desired limit here
//        // let limit = _req.limits().get("person").unwrap_or(256.bytes());

//         let mut value = String::new();
//         // if let Err(e) = data.open(limit).read_to_string(&mut value).await {
//         //     return Outcome::Failure((Status::InternalServerError, e));
//         // }
//          // Use a configured limit with name 'person' or fallback to default.
//          let limit = req.limits().get("person").unwrap_or(256.bytes());

//          // Read the data into a string.
//          let string = match data.open(limit).into_string().await {
//              Ok(string) if string.is_complete() => string.into_inner(),
//              Ok(_) => return Failure((Status::PayloadTooLarge, Error::TooLarge)),
//              Err(e) => return Failure((Status::InternalServerError, Io(e))),
//          };
 

//        Outcome::Success(RequestBody { value:string })
//     }
// }
