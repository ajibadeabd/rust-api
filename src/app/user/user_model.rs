
use mongodb::{
    bson::{oid::ObjectId,doc, Document},
    sync::{Collection},
    results::{InsertOneResult, UpdateResult}, options::{UpdateModifications, UpdateOptions, FindOptions, FindOneOptions}
};
use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};



extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};

use serde::Serializer;

use super::types::UserSignUpRequestType;

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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password:Option< String>,
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

    pub fn save(&self, user: &mut UserSignUpRequestType )->Result<InsertOneResult, mongodb::error::Error> {
        let new_user = User {
            created_at : Some(Utc::now()),
            updated_at : Some(Utc::now()),
            first_name:user.first_name.to_owned(),
            last_name:user.last_name.to_owned(),
            accounts:None,
            password:Some(user.password.to_owned()),
            email:user.email.to_owned(),
            id:None
        };
            let hashed_password = hash(user.password.to_owned(), DEFAULT_COST).unwrap();
            user.password = hashed_password;
        // }
        self.col.insert_one(&new_user, None)
    }
    pub fn find_one(&self, find_by: &str,find_with: &str,find_option:Option<Document>)->Result<std::option::Option<User>, mongodb::error::Error> {
      
    let find_one_options = FindOneOptions::builder()
    .projection( find_option)
    .build();
        
        self.col.find_one(doc! {find_by:find_with}, find_one_options)
    }
    pub fn find_by_id(&self, object_id: &ObjectId,find_option:Option<Document>)->Result<std::option::Option<User>, mongodb::error::Error> {
     
    let find_one_options = FindOneOptions::builder()
        .projection( find_option)
        .build();
    //    let option = 
       
        self.col.find_one(doc!{"_id":object_id}, find_one_options)
    }
    pub fn update_one(&self, filter_by:Document,update:UpdateModifications,update_option:Option<UpdateOptions>)->Result<UpdateResult, mongodb::error::Error> {
        self.col.update_one(filter_by,update,update_option)
    }

    // pub fn find_all(&self, filter_by:Document,find_option:Option<FindOptions>)->SmallVec<mongodb::sync::Cursor<User>, mongodb::error::Error> {
    //     self.col.find(filter_by,find_option).into_collection()
    // }
    
}
 