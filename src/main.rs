#[macro_use]
extern crate rocket;


mod app;
mod modules;
mod database;

use app::{
    user::user_route::{add_user,sign_in,profile},
    account::account_route::{ account_creation ,deposit,withdraw,transfer_funds,transactions,webhook}
};

use modules::cors::make_cors;
use shuttle_secrets::SecretStore;
 

// #[launch]
#[shuttle_runtime::main]
// fn rocket() -> _ {
pub async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,

) ->  shuttle_rocket::ShuttleRocket {
    let secret = if let Some(secret) = secret_store.get("MONGO_URI") {
        secret
    } else {
        print!("mp env");
        format!("")
        // return Err(anyhow!("secret was not found").into());
    };
    // rocket= mount_user_route(rocket);
let db=database::Database::init(&secret);
    let rocket = rocket::build()

    .mount("/api", routes![add_user,sign_in,profile])
    .mount("/api/account", routes![account_creation,deposit,withdraw,transfer_funds,transactions])
    .mount("/", routes![webhook])
    .attach(make_cors()).manage(db).into();

    //  rocket
     Ok(rocket)

}



