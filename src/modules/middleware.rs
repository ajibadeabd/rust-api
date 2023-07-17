use std::sync::Arc;

use mongodb::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::{Request, Data, Response};

use crate::modules::util::{decode_jwt, DecodeJwtHelper};

#[rocket::async_trait]
 impl Fairing for IncomingRequest {
    // This is a request and response fairing named "Incoming Request".
    fn info(&self) -> Info {
        Info {
            name: "Incoming Request",
            kind: Kind::Request | Kind::Response
        }
    }

     async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..]; // Extract the token by removing "Bearer "
                println!("Bearer Token: {}", token);
                let res = decode_jwt(token,"secret");
                println!("{:?}",res);
                if let DecodeJwtHelper::Err = res {
                    // Err(Status::Unauthorized)
                }


            }
        }
    }


    async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
        // No changes to the response needed.
        println!("outgoing Response"); 

    }
}
pub struct IncomingRequest {
    db: Arc<Client>
}
