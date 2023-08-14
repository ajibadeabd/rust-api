
use mongodb::{
    bson::{oid::ObjectId,doc, Document},
    sync::{Collection},
    results::{InsertOneResult, UpdateResult}, options::{UpdateModifications, UpdateOptions}
};
use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};



extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};

use serde::Serializer;

pub fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
      Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
      None => serializer.serialize_none()
    }
}
pub fn serialize_object_ids<S>(object_ids: &Option<Vec<ObjectId>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_ids {
        Some(ref object_ids) => {
            let object_id_strings: Vec<_> = object_ids.iter().map(|id| id.to_string()).collect();
            serializer.serialize_some(&object_id_strings)
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct User {
        #[serde(
            rename = "_id",
            skip_serializing_if = "Option::is_none",
            serialize_with = "serialize_object_id"
        )]
        pub id: Option<ObjectId>,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        #[serde(serialize_with = "serialize_object_ids")]
        pub accounts: Option<Vec<ObjectId>>,
        pub password: String,
        pub created_at: Option<DateTime<Utc>>,
        pub updated_at: Option<DateTime<Utc>>,
}
 
pub struct Init<'a> {
    col: &'a Collection<User>,
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<User>) -> Self {
        Init { col }
    }

    pub fn save(&self, user: &mut User)->Result<InsertOneResult, mongodb::error::Error> {
        user.created_at = Some(Utc::now());
        user.updated_at = Some(Utc::now());
        user.password = hash(user.password.to_string(),DEFAULT_COST).unwrap();
        self.col.insert_one(user, None)
    }
    pub fn find_one(&self, find_by: &str,find_with: &str)->Result<std::option::Option<User>, mongodb::error::Error> {
        self.col.find_one(doc! {find_by:find_with}, None)
    }
    pub fn find_by_id(&self, object_id: &ObjectId)->Result<std::option::Option<User>, mongodb::error::Error> {
        self.col.find_one(doc!{"_id":object_id}, None)
    }
    pub fn update_one(&self, filter_by:Document,update:UpdateModifications,update_option:Option<UpdateOptions>)->Result<UpdateResult, mongodb::error::Error> {
        self.col.update_one(filter_by,update,update_option)
    }

    // pub fn find_all(&self, filter_by:Document,find_option:Option<FindOptions>)->SmallVec<mongodb::sync::Cursor<User>, mongodb::error::Error> {
    //     self.col.find(filter_by,find_option).into_collection()
    // }
    
}
 