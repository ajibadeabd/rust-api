use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Data, Response};

#[rocket::async_trait]
 impl Fairing for IncomingRequest {
    // This is a request and response fairing named "Incoming Request".
    fn info(&self) -> Info {
        Info {
            name: "Incoming Request",
            kind: Kind::Request | Kind::Response
        }
    }

    // Display a message for incoming requests.
    async fn on_request(&self, _: &mut Request<'_>, _: &mut Data<'_>) {
        println!("Incoming Request");
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
        // No changes to the response needed.
    }
}
pub struct IncomingRequest;
