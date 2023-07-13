use std::env;
use dotenv::dotenv;

use mongodb::{
    sync::{Client, Collection},
};


pub mod user;
mod product;




pub struct Database {
    user_schema: Collection<user::User>,
    product_schema: Collection<product::Product>,
}
impl Database {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(database_url) => database_url.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let user_schema: Collection<user::User> = db.collection("User");
        let product_schema: Collection<product::Product> = db.collection("Product");
        Database { user_schema, product_schema }
    }
    pub fn user(&self)->user::Init{
        user::Init::init(&self.user_schema)
    }
}