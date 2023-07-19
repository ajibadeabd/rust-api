 
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{Outcome, FromRequest};
use rocket::{Request, Data, Response, State};
use std::sync::Arc;

use crate::app::user::user_model::User;
use crate::database::Database;
use crate::modules::util::{decode_jwt, DecodeJwtHelper};

// // // pub struct IncomingRequest{
// // //     // pub db: &Database,
// // //     pub user_data: Option<User>,
// // //     pub db: Arc<Database>,
// // // }

// // // #[rocket::async_trait]
// // // // impl Fairing for IncomingRequest {
// // //     impl FromRequest<'_> for IncomingRequest {
// // //         type Error = ();
// // //     // fn info(&self) -> Info {
// // //     //     Info {
// // //     //         name: "Incoming Request",
// // //     //         kind: Kind::Request | Kind::Response,
// // //     //     }
// // //     // }
// // //     // pub type Outcome<S, E> = outcome::Outcome<S, (Status, E), ()>;
// // //     // pub type Outcome<S, E> = outcome::Outcome<S, (Status, E), ()>;
// // //     async fn from_request(request: &Request<'_>) -> Outcome<Self, ()> {
// // //     // async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>){
// // //         if let Some(auth_header) = request.headers().get_one("Authorization") {
// // //             if auth_header.starts_with("Bearer ") {
// // //                 let token = &auth_header[7..]; // Extract the token by removing "Bearer "
// // //                 //println!("Bearer Token: {}", token);
// // //                 let res = decode_jwt(token, "secret");
// // //                 //println!("{:?}", res);
// // //                 if let DecodeJwtHelper::Err = res {
// // //                     // Err(Status::Unauthorized)
// // //                 }
// // //                 let d = Self.db.user();
// //                 let user_details = Self.db.user().find_one("email","kormddn");
// //                 // if let Ok(None) =  user_details {
// //                 //     println!("no user found");
// //                 // // Outcome::Success(us)

// //                 // }
// // //                 // Outcome::Success(user_details.unwrap())

// // //                 match user_details {
// // //                     Err(_)=>println!("Error occur while fetching user details"),
// // //                     Ok(None)=>println!("no user found"),
// // //                     Ok(Some(user))=>{
// // //                 println!("{:?}",user);

// // //                 request.local_cache(|| {
// // //                     user.clone()
// // //                     // self.user_data = Some(user);
// // //                     // self.user_data.as_ref().unwrap().clone()
// // //                 });

// // //                 // self.user_data = Some(user.clone());
                
// // //                // Outcome::Success(user)
// // //                     },

// // //                 }
// // //             }
// // //         }
// // //         // Outcome::Failure((Status::BadRequest, "Input too short".to_string()))


// // //         // Access the database instance using self.db
// // //         //let collection = self.db.collection("my_collection");
// // //         // Perform database operations as needed
// // //         // ...
// // //     }

// // //     // async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
// // //     //     // No changes to the response needed
// // //     //     println!("Outgoing Response");
// // //     // }
// // // }

// // use rocket::fairing::{Fairing, Info, Kind};
// // use rocket::http::Status;
// // use rocket::request::{Outcome, FromRequest};
// // use rocket::{Request, Data, Response};
// // use std::sync::Arc;

// // use crate::app::user::user_model::User;
// // use crate::database::Database;
// // use crate::modules::util::{decode_jwt, DecodeJwtHelper};

// // pub struct IncomingRequest {
// //     pub user_data: Option<User>,
// //     pub db: Arc<Database>,
// // }

// // #[rocket::async_trait]
// // impl FromRequest<'r_> for &'r IncomingRequest {
// //     // impl<'r> FromRequest<'r> for &'r RequestId {
// //     type Error = ();

// //     async fn from_request(request: &Request<'_>) {
// //         if let Some(auth_header) = request.headers().get_one("Authorization") {
// //             if auth_header.starts_with("Bearer ") {}
// //             //     let token = &auth_header[7..];
// //             //     let res = decode_jwt(token, "secret");
// //             //     if let DecodeJwtHelper::Err = res {
// //             //         return Outcome::Failure((Status::Unauthorized, ()));
// //             //     }
// //             //     let db = request.guard::<rocket::State<Arc<Database>>>().await;
// //             //     // match db {
// //             //     //     Some(db) => {
// //             //     //         let user_details = db.user().find_one("email", "kormddn");
// //             //     //         match user_details {
// //             //     //             Err(_) => println!("Error occurred while fetching user details"),
// //             //     //             Ok(None) => println!("No user found"),
// //             //     //             Ok(Some(user)) => {
// //             //     //                 println!("{:?}", user);
// //             //     //                 // return Outcome::Success(IncomingRequest {
// //             //     //                 //     user_data: Some(user),
// //             //     //                 //     db,
// //             //     //                 // });
// //             //     //                 ()
// //             //     //             }
// //             //     //         }
// //             //     //     }
// //             //     //     None => println!("Failed to get database connection"),
// //             //     // }
// //             // }
// //         }
// //         // Outcome::Failure((Status::BadRequest, ()))
// //     }
// // }

// use rocket::fairing::{Fairing, Info, Kind};
// use rocket::http::Status;
// use rocket::request::{Outcome, FromRequest};
// use rocket::{Request, Data, Response};
// use std::convert::Infallible;
// use std::sync::Arc;

// use crate::app::user::user_model::User;
// use crate::database::Database;
// use crate::modules::util::{decode_jwt, DecodeJwtHelper};


// // pub struct IncomingRequest {
// //     // pub user_data: Option<User>,
// //     pub db: Arc<Database>,
// // }
// // // struct RequestHeaders<'h>(&'h HeaderMap<'h>);

// // #[rocket::async_trait]
// // impl<'r> FromRequest<'r> for IncomingRequest {
// //     type Error = Infallible;

// //     async  fn from_request(request: &'r Request<'_>) -> Outcome<&'static str, Status> {
// //         // if let Some(auth_header) = request.headers().get_one("Authorization") {
// //         // Outcome::Success(auth_header)

// //         // }
// //         if let Some(auth_header) = request.headers().get_one("Authorization") {
// //             return Outcome::Success(&auth_header.to_string());
// //         }
// //         Outcome::Failure((Status::Unauthorized, Status::Unauthorized))
// //         // Outcome::Failure(())
// //         // Ok(())


// //     }
// // }

use std::time::SystemTime;





/// Fairing for timing requests.
pub struct RequestTimer;

/// Value stored in request-local state.
#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

#[rocket::async_trait]
impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info {
            name: "Request Timer",
            kind: Kind::Request | Kind::Response
        }
    }

    /// Stores the start time of the request in request-local state.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // Store a `TimerStart` instead of directly storing a `SystemTime`
        // to ensure that this usage doesn't conflict with anything else
        // that might store a `SystemTime` in request-local cache.
        request.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    // / Adds a header to the response indicating how long the server took to
    // / process the request.
    // async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
    //     let start_time = req.local_cache(|| TimerStart(None));
    //     if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
    //         let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
    //         res.set_raw_header("X-Response-Time", format!("{} ms", ms));
    //     }
    // }
}

/// Request guard used to retrieve the start time of a request.
#[derive(Copy, Clone,Debug)]
pub struct StartTime(pub SystemTime);

// Allows a route to access the time a request was initiated.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for StartTime {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) ->  Outcome<Self, ()> {
         let database = request.guard::<&State<Database>>().await;
           // .map(|my_config| my_config);
       println!("{:?}",database.unwrap().user().find_one("email", "kosrfdn"));

       
        match *request.local_cache(|| TimerStart(None)) {
            TimerStart(Some(time)) =>  Outcome::Success(StartTime(time)),
            TimerStart(None) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}