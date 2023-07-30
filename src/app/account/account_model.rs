use chrono::{DateTime, Utc};
use mongodb::{
    bson::{oid::ObjectId,doc, Document},
    sync::{Collection, ClientSession},
    results::{InsertOneResult, UpdateResult}, options::{FindOneOptions, UpdateModifications, UpdateOptions}
};
use serde::{Serialize, Deserialize};

use crate::app::user::user_model::serialize_object_ids;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Account {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub balance: f64,
        pub locked_balance: f64,
        pub user_id: Option<ObjectId>,
        pub channel: String,
        pub currency: String,
        #[serde(serialize_with = "serialize_object_ids")]
        pub transaction: Option<Vec<ObjectId>>,
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
            transaction: None,
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
        self.col.find_one(doc!{"_id":object_id}, None)
    }
    pub fn update_one(&self, filter_by:Document,update:UpdateModifications,update_option:Option<UpdateOptions>,session: Option<&mut ClientSession>)->Result<UpdateResult, mongodb::error::Error> {
        if let Some(session) = session {
         return  self.col.update_one_with_session(filter_by,update,update_option,session)
        }
         self.col.update_one(filter_by,update,update_option)
 }
}
 