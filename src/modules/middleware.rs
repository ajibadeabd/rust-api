// use rocket::fairing::{Fairing, Info, Kind};
// use rocket::http::Status;
// use rocket::{Request, Data, Response};

// use crate::modules::util::{decode_jwt, DecodeJwtHelper};

// #[rocket::async_trait]
//  impl Fairing for IncomingRequest {
//     // This is a request and response fairing named "Incoming Request".
//     fn info(&self) -> Info {
//         Info {
//             name: "Incoming Request",
//             kind: Kind::Request | Kind::Response
//         }
//     }

//      async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
//         if let Some(auth_header) = request.headers().get_one("Authorization") {
//             if auth_header.starts_with("Bearer ") {
//                 let token = &auth_header[7..]; // Extract the token by removing "Bearer "
//                 println!("Bearer Token: {}", token);
//                 let res = decode_jwt(token,"secret");
//                 println!("{:?}",res);
//                 if let DecodeJwtHelper::Err = res {
//                     // Err(Status::Unauthorized)
//                 }

//             }
//         }
//     }


//     async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
//         // No changes to the response needed.
//         println!("outgoing Response"); 

//     }
// }
// pub struct IncomingRequest;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::{Request, Data, Response};
use std::sync::Arc;

use crate::app::user::user_model::User;
use crate::database::Database;
use crate::modules::util::{decode_jwt, DecodeJwtHelper};

pub struct IncomingRequest {
    // pub db: &Database,
    pub db: Arc<Database>,
}

#[rocket::async_trait]
impl Fairing for IncomingRequest {
    fn info(&self) -> Info {
        Info {
            name: "Incoming Request",
            kind: Kind::Request | Kind::Response,
        }
    }
    // pub type Outcome<S, E> = outcome::Outcome<S, (Status, E), ()>;

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>){
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..]; // Extract the token by removing "Bearer "
                println!("Bearer Token: {}", token);
                let res = decode_jwt(token, "secret");
                println!("{:?}", res);
                if let DecodeJwtHelper::Err = res {
                    // Err(Status::Unauthorized)
                }
                let user_details = &self.db.user().find_one("email","kormddn");
                // if let Ok(None) =  user_details {
                //     println!("no user found");
                // // Outcome::Success(us)

                // }
                // Outcome::Success(user_details.unwrap())

                match user_details {
                    Err(_)=>println!("Error occur while fetching user details"),
                    Ok(None)=>println!("no user found"),
                    Ok(Some(user))=>{
                println!("{:?}",user);
                Outcome::Success(user)
                    },

                }
            }
        }

        // Access the database instance using self.db
        //let collection = self.db.collection("my_collection");
        // Perform database operations as needed
        // ...
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
        // No changes to the response needed
        println!("Outgoing Response");
    }
}
