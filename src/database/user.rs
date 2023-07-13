use mongodb::{
    bson::{oid::ObjectId,doc},
    sync::{Collection},
    results::InsertOneResult
};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub name: String,
        pub location: String,
        pub title: String,
}
 
pub struct Init<'a> {
    col: &'a Collection<User>,
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<User>) -> Self {
        Init { col }
    }

    pub fn save(&self, user: &User)->Result<InsertOneResult, mongodb::error::Error> {
        self.col.insert_one(user, None)
    }
    pub fn find_one(&self, user: &str)->Result<std::option::Option<User>, mongodb::error::Error> {
        self.col.find_one(doc! {"name":user}, None)
    }
}
 