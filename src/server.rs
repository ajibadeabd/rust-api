#[macro_use]
extern crate rocket;


mod app;
mod modules;

use modules::{middleware};
use app::{
    user::{
        user_route::{health_checker_handler,add_user},
        // mount as mount_user_route
    },
};


 

#[launch]
fn rocket() -> _ {
    // rocket= mount_user_route(rocket);

    let rocket = rocket::build()

    .attach(middleware::IncomingRequest)
    .mount("/api", routes![health_checker_handler,add_user]);
     rocket
}
