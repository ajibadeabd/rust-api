#[macro_use]
extern crate rocket;


mod app;
mod modules;
mod database;

use app::{
    user::user_route::{add_user,sign_in,profile},
    account::account_route::{ account_creation ,deposit,withdraw,transfer_funds,transactions,webhook,dashboard,get_deposit}
};

use modules::{cors::make_cors, error_handler::{internal_error, not_found,bad_input}};
use shuttle_secrets::SecretStore;
//  use crate::module::error_handler::{
//     not_found,
//     internal_error

// };

// #[launch]
#[shuttle_runtime::main]
// fn rocket() -> _ {
pub async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,

) ->  shuttle_rocket::ShuttleRocket {
    let secret = if let Some(secret) = secret_store.get("MONGO_URI") {
        secret
    } else {
        format!("")
        // return Err(anyhow!("secret was not found").into());
    };
    // rocket= mount_user_route(rocket);
let db=database::Database::init(&secret);
    let rocket = rocket::build()

    .mount("/api", routes![add_user,sign_in,profile])
    .mount("/api/account", routes![dashboard,account_creation,deposit,withdraw,transfer_funds,transactions,get_deposit])
    .mount("/", routes![webhook])
    .register("/", catchers![internal_error, not_found, bad_input])
    .attach(make_cors()).manage(db).into();

    //  rocket
     Ok(rocket)

}



