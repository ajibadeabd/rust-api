use chrono::{DateTime, Utc};
use mongodb::{
    bson::{oid::ObjectId,doc, Document},
    sync::{Collection},
    results::InsertOneResult, options::FindOneOptions
};
use serde::{Serialize, Deserialize};

use super::account_type::AccountData;
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Account {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub balance: f64,
        pub locked_balance: f64,
        pub user_id: Option<ObjectId>,
        pub channel: String,
        pub currency: String,
        pub updated_at: Option<DateTime<Utc>>,
        pub created_at: Option<DateTime<Utc>>,
}
impl Account {
    pub fn new(channel:String,currency:String,user_id:Option<ObjectId>)->Self{
        Self {
            locked_balance:0.0,
            balance:0.0,
            currency,
            channel,
            user_id,
            id:None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }

    }
}
 
pub struct Init<'a> {
    col: &'a Collection<Account>,
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<Account>) -> Self {
        Init { col }
    }

    pub fn save(&self, account: &Account)->Result<InsertOneResult, mongodb::error::Error> {
        self.col.insert_one(account, None)
    }
    pub fn find_one(&self, find_by:Document,filter_by:Option<FindOneOptions>)->Result<std::option::Option<Account>, mongodb::error::Error> {
        self.col.find_one(find_by,filter_by)
    }
    pub fn find_by_id(&self, object_id: &ObjectId)->Result<std::option::Option<Account>, mongodb::error::Error> {
       let s = doc!{"_id":object_id};
        self.col.find_one(doc!{"_id":object_id}, None)
    }
}
 