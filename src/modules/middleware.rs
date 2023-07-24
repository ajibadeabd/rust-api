 
// // use rocket::fairing::{Fairing, Info, Kind};
// // use rocket::http::Status;
// // use rocket::request::{Outcome, FromRequest};
// // use rocket::{Request, Data, Response};
// // use std::sync::Arc;

// // use crate::app::user::user_model::User;
// // use crate::database::Database;
// // use crate::modules::util::{decode_jwt, DecodeJwtHelper};

// //  pub struct IncomingRequest {
// //     pub user_data: Option<User>,
// //     pub db: Arc<Database>,
// // }


// // // // #[rocket::async_trait]
// // // // impl Fairing for IncomingRequest {
// // // //     // This is a request and response fairing named "GET/POST Counter".
// // // //     fn info(&self) -> Info {
// // // //         Info {
// // // //             name: "GET/POST Counter",
// // // //             kind: Kind::Request | Kind::Response
// // // //         }
// // // //     }

// // // //     async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
// // // //         if let Some(auth_header) = request.headers().get_one("Authorization") {
// // // //             if auth_header.starts_with("Bearer ") {
// // // //                 let token = &auth_header[7..];
// // // //                 let res = decode_jwt(token, "secret");
// // // //                 let user_details = self.db.user().find_one("email","kormddn");
// // // //                 if let Ok(None) =  user_details {
// // // //                     println!("no user found");
// // // //                     return 
// // // //                 }
// // // //                 request.local_cache(||user_details.unwrap());
// // // //             }
// // // //         }
// // // //     }

// // // //     async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        
// // // //     }
// // // // }


// // // // use rocket::Outcome;
// // // // use rocket::request::{self, Request, FromRequest};

// // // // pub extern crate crypto;
// // // // pub extern crate jwt;
// // // // pub extern crate rustc_serialize;

// // // // use user::auth::crypto::sha2::Sha256;
// // // // use self::jwt::{
// // // //     Header,
// // // //     Registered,
// // // //     Token,
// // // // };


// // // pub struct ApiKey(pub String);

// // // pub fn read_token(key: &str) -> Result<String, String> {
// // //     // let token = Token::<Header, Registered>::parse(key)
// // //     //     .map_err(|_| "Unable to parse key".to_string())?;
// // //     // if token.verify(b"secret_key", Sha256::new()) {
// // //     //     token.claims.sub.ok_or("Claims not valid".to_string())
// // //     // } else {
// // //     //     Err("Token not valid".to_string())
// // //     // }
// // // }

// // // impl<'a> FromRequest<'a> for ApiKey {
// // //     type Error = ();

// // //     fn from_request(request: &'a Request) -> Outcome<&'a str, ()> {
// // //         let keys: Vec<_> = request.headers().get("Authentication").collect();
// // //         if keys.len() != 1 {
// // //             return Outcome::Forward(());
// // //         }
// // //         Outcome::Success("ApiKey(Ok(claim).unwrap())")
// // //         // match read_token(keys[0]) {
// // //         //     Ok(claim) => Outcome::Success(ApiKey(claim)),
// // //         //     Err(_) => Outcome::Forward(())
// // //         // }
// // //     }
// // // }

// // // use rocket::Outcome;
// // // use rocket::request::{self, Request, FromRequest};

// // // pub extern crate crypto;
// // // pub extern crate jwt;
// // // pub extern crate rustc_serialize;

// // // use user::auth::crypto::sha2::Sha256;
// // // use self::jwt::{
// // //     Header,
// // //     Registered,
// // //     Token,
// // // };


// pub struct Auth;

// pub fn read_token(key: &str) -> Result<String, String> {
//     let token = Token::<Header, Registered>::parse(key)
//         .map_err(|_| "Unable to parse key".to_string())?;
//     if token.verify(b"secret_key", Sha256::new()) {
//         token.claims.sub.ok_or("Claims not valid".to_string())
//     } else {
//         Err("Token not valid".to_string())
//     }
// }

// use rocket::{request::FromRequest, http::hyper::Request, outcome::Outcome};

// #[rocket::async_trait]
// impl<'r> FromRequest< 'r> for Auth {
//     type Error = ();

//     fn from_request(request: Request<'r>) -> Outcome<Self, ()> {
//         let keys: Vec<_> = request.headers().get("Authentication").collect();
//         if keys.len() != 1 {
//             return Outcome::Forward(());
//         }
//         match read_token(keys[0]) {
//             Ok(claim) => Outcome::Success(Auth),
//             Err(_) => Outcome::Forward(())
//         }
//     }
// }
// // #[rocket::async_trait]
// // impl<'r> FromRequest<'r> for ApiKey {
// //     type Error = ();

// //     async fn from_request(request: &'r Request<'_>) -> Outcome<'r, Self> {
// //         let keys: Vec<_> = request.headers().get("Authentication").collect();
// //         if keys.len() != 1 {
// //             return Outcome::Forward(());
// //         }
// //         match read_token(keys[0]) {
// //             Ok(claim) => Outcome::Success(ApiKey(claim)),
// //             Err(_) => Outcome::Forward(()),
// //         }
// //     }
// // }

// // macro_rules! my_decorator {
// //     ($func:expr) => {
// //         fn decorated_function() {
// //             println!("Before function call");
// //             $func();
// //             println!("After function call");
// //         }
// //     };
// // }