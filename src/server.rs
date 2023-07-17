#[macro_use]
extern crate rocket;


mod app;
mod modules;
mod database;

use std::sync::Arc;

use modules::{middleware};
use app::{
    user::user_route::{add_user,sign_in},
    account::account_route::{ account_creation }
    
};
// use crate::app::
use serde::Serialize;

 

#[launch]
fn rocket() -> _ {
    // rocket= mount_user_route(rocket);
let db=database::Database::init();
    let rocket = rocket::build()

    .mount("/api", routes![add_user,sign_in]);
    let rocket = rocket.mount("/api/account", routes![account_creation])
    .attach(middleware::IncomingRequest {  db: Arc::new(db.copy()) })
    .manage(db);

     rocket
}
