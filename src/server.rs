#[macro_use]
extern crate rocket;


mod app;
mod modules;
mod database;

use app::{
    user::user_route::{add_user,sign_in},
    account::account_route::{ account_creation ,deposit,withdraw,transfer_funds,transactions,webhook}
};
 

// #[launch]
#[shuttle_runtime::main]
// fn rocket() -> _ {
pub async fn rocket() ->  shuttle_rocket::ShuttleRocket {

    // rocket= mount_user_route(rocket);
let db=database::Database::init();
    let rocket = rocket::build()

    .mount("/api", routes![add_user,sign_in])
    .mount("/api/account", routes![account_creation,deposit,withdraw,transfer_funds,transactions])
    .mount("/", routes![webhook])
    .manage(db).into();

    //  rocket
     Ok(rocket)

}



