use mongodb::{
    bson::{oid::ObjectId,doc},
    sync::{Collection},
    results::InsertOneResult
};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Transaction {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub balance: f64,
        pub locked_balance: f64,
        pub user_id: String,
        pub channel: String,
        pub currency: String,
        pub updated_at: String,
        pub created_at: String,
}
 
pub struct Init<'a> {
    col: &'a Collection<Transaction>,
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<Transaction>) -> Self {
        Init { col }
    }

    pub fn save(&self, user: &Transaction)->Result<InsertOneResult, mongodb::error::Error> {
        self.col.insert_one(user, None)
    }
    pub fn find_one(&self, user: &str)->Result<std::option::Option<Transaction>, mongodb::error::Error> {
        self.col.find_one(doc! {"":user}, None)
    }
}
 