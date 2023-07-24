 
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{Outcome, FromRequest};
use rocket::{Request, Data, State};

use crate::app::user::user_model::User;
use crate::database::Database;
use crate::modules::response_handler::CustomError;
use crate::modules::util::{decode_jwt, DecodeJwtHelper};
 
use std::time::SystemTime;





// /// Fairing for timing requests.
// pub struct RequestTimer;

// /// Value stored in request-local state.
// #[derive(Copy, Clone)]
// struct TimerStart(Option<SystemTime>);

// #[rocket::async_trait]
// impl Fairing for RequestTimer {
//     fn info(&self) -> Info {
//         Info {
//             name: "Request Timer",
//             kind: Kind::Request | Kind::Response
//         }
//     }

//     /// Stores the start time of the request in request-local state.
//     async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
//         // Store a `TimerStart` instead of directly storing a `SystemTime`
//         // to ensure that this usage doesn't conflict with anything else
//         // that might store a `SystemTime` in request-local cache.
//         request.local_cache(|| TimerStart(Some(SystemTime::now())));
//     }

//     // / Adds a header to the response indicating how long the server took to
//     // / process the request.
//     // async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
//     //     let start_time = req.local_cache(|| TimerStart(None));
//     //     if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
//     //         let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
//     //         res.set_raw_header("X-Response-Time", format!("{} ms", ms));
//     //     }
//     // }
// }

// /// Request guard used to retrieve the start time of a request.
// #[derive(Copy, Clone,Debug)]
// pub struct StartTime(pub SystemTime);

// // Allows a route to access the time a request was initiated.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) ->  Outcome<Self, ()> {
        
        // if let Some(auth_header) = request.headers().get_one("Authorization") {
        //                 if auth_header.starts_with("Bearer ") {
        //                     let token = &auth_header[7..];
        //                     let res = decode_jwt(token, "secret");
        //                     let user_details = self.db.user().find_one("email","kormddn");
        //                     if let Ok(None) =  user_details {
        //                         println!("no user found");
        //                         return 
        //                     }
        //                 }
        //             }

        let bearer_token = request.headers().get_one("Authorization").unwrap_or("");
        // match  {
            
        // }
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
            println!("{:?}",token.claims.user_id);
       let database = request.guard::<&State<Database>>().await;
       let user = database.unwrap().user().find_by_id(
        &ObjectId::parse_str(&token.claims.user_id).unwrap()
    );
       match user {
        Ok(Some(user)) =>  Outcome::Success(user),
        _=>{
            print!("e no dea");
            
            Outcome::Failure((Status::Unauthorized,()))}
    }
        }
        }
    }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for User{
//     async fn from_request(request: &'r Request<'_>) ->  Outcome<Self, ()> {
//         let database = request.guard::<&State<Database>>().await;
//           // .map(|my_config| my_config);
//       println!("{:?}",database.unwrap().user().find_one("email", "kosrfdn"));

      
//        match *request.local_cache(|| TimerStart(None)) {
//            TimerStart(Some(time)) =>  Outcome::Success(StartTime(time)),
//            TimerStart(None) => Outcome::Failure((Status::InternalServerError, ())),
//        }
//    }

// }


// impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
//     type Error = LoginError;
//     async fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {

//     }
// }


// use serde::{Serialize, Deserialize};
// use std::io::Cursor;
// use rocket::http::Status;
// use rocket::request::Request;
// use rocket::response::{self, Response, Responder};
// use rocket::http::ContentType;

// use super::generic_type::GenericResponse;
// use super::response_handler::CustomError;

// #[derive(Serialize)]
// pub struct IsAdmin;

 
 
 

// // impl std::fmt::Display for IsAdmin {
// //     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
// //         write!(fmt, "{}", self.result)
// //     }
// // }
 

// impl<'r> Responder<'r, 'static> for IsAdmin {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<(), Status> {
//         // Ok(())
//         Err(CustomError::NotFound("Unable to create an account".to_string()))
//     }
// }
  
 
