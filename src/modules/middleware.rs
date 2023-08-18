 
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use rocket::http::Status;
use rocket::request::{Outcome, FromRequest};
use rocket::{Request, State};

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
        &ObjectId::parse_str(&token.claims.user_id).unwrap(),
        Some(doc!{"password":0}
    )
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

 




// struct Person<'r> {
//     name: &'r str,
//     age: u16
// }
// #[derive(Debug)]
// enum Error {
//     TooLarge,
//     NoColon,
//     InvalidAge,
//     Io(std::io::Error),
// }

// #[rocket::async_trait]
// impl<'r> FromData<'r> for Person<'r> {
//     type Error = Error;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
//         use Error::*;
//         use rocket::outcome::Outcome::*;

//         // Ensure the content type is correct before opening the data.
//         let person_ct = ContentType::new("application", "x-person");
//         if req.content_type() != Some(&person_ct) {
//             return Forward(data);
//         }

//         // Use a configured limit with name 'person' or fallback to default.
//         let limit = req.limits().get("person").unwrap_or(256.bytes());

//         // Read the data into a string.
//         let string = match data.open(limit).into_string().await {
//             Ok(string) if string.is_complete() => string.into_inner(),
//             Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
//             Err(e) => return Failure((Status::InternalServerError, Io(e))),
//         };

//         // We store `string` in request-local cache for long-lived borrows.
//         let string = request::local_cache!(req, string);

//         // Split the string into two pieces at ':'.
//         let (name, age) = match string.find(':') {
//             Some(i) => (&string[..i], &string[(i + 1)..]),
//             None => return Failure((Status::UnprocessableEntity, NoColon)),
//         };

//         // Parse the age.
//         let age: u16 = match age.parse() {
//             Ok(age) => age,
//             Err(_) => return Failure((Status::UnprocessableEntity, InvalidAge)),
//         };

//         Success(Person { name, age })
//     }
// }
