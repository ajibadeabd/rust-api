
use mongodb::{
    bson::{oid::ObjectId,doc},
    sync::{Collection},
    results::InsertOneResult, options::FindOneOptions
};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};





extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub password: String,
        // #[serde(skip)]
        pub created_at: Option<DateTime<Utc>>,
        // #[serde(skip)]
        pub updated_at: Option<DateTime<Utc>>,
}
 
pub struct Init<'a> {
    col: &'a Collection<User>,
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<User>) -> Self {
        Init { col }
    }

    pub fn save(&self, mut user: &mut User)->Result<InsertOneResult, mongodb::error::Error> {
        user.created_at = Some(Utc::now());
        user.updated_at = Some(Utc::now());
        user.password = hash(user.password.to_string(),DEFAULT_COST).unwrap();
        self.col.insert_one(user, None)
    }
    pub fn find_one(&self, email: &str)->Result<std::option::Option<User>, mongodb::error::Error> {
        self.col.find_one(doc! {"email":email}, None)
    }
    // pub fn find_one(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
    //     let options = FindOneOptions::builder().projection(doc! { "_id": 0 }).build();
    //     self.col.find_one(doc! {"email": email}, options)
    // }
    
}
 