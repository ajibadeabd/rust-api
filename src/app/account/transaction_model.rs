use chrono::{Utc, DateTime};
use mongodb::{
    bson::{oid::ObjectId,doc, Document},
    sync::{Collection, ClientSession},
    results::{InsertOneResult, UpdateResult}, options::{UpdateOptions, UpdateModifications}
};
use serde::{Serialize, Deserialize};

use super::account_type::{TransactionStatus, TransactionType};
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Transaction {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub amount: f64,
        pub currency: String,
        pub fee: f64,
        pub receiver_id: String,
        pub giver_id: String,
        pub provider_name: String,
        pub transaction_type: TransactionType,
        pub status: TransactionStatus,
        updated_at: Option<DateTime<Utc>>,
        created_at: Option<DateTime<Utc>>,
}
 


pub struct Init<'a> {
    col: &'a Collection<Transaction>,
}

impl Transaction {
    pub fn new(
        amount: f64,
        currency: String,
        fee: f64,
        receiver_id: String,
        giver_id: String,
        provider_name: String,
        transaction_type: TransactionType,
        status:TransactionStatus
    ) -> Self {
        Self {
            amount,
            currency,
            id:None,
            fee,
            receiver_id,
            giver_id,
            provider_name,
            transaction_type,
            status,
            updated_at:  Some(Utc::now()),
            created_at:  Some(Utc::now()),
        }
    }
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<Transaction>) -> Self {
        Init { col }
    }

    pub fn save(&self, transaction: &Transaction)->Result<InsertOneResult, mongodb::error::Error> {
        self.col.insert_one(transaction, None)
    }
    pub fn save_with_session(&self, transaction: &Transaction,session: &mut ClientSession)->Result<InsertOneResult, mongodb::error::Error> {
      
      println!("{:?}",transaction);
     let d =  self.col.insert_one_with_session(transaction, None,session)
   
   ;
   
   println!("{:?}",d);
   d 
    }
    pub fn find_one(&self, transaction: &str)->Result<std::option::Option<Transaction>, mongodb::error::Error> {
        self.col.find_one(doc! {"":transaction}, None)
    }
    pub fn update_one(&self, filter_by:Document,update:UpdateModifications,update_option:Option<UpdateOptions>,session: Option<&mut ClientSession>)->Result<UpdateResult, mongodb::error::Error> {
           if let Some(session) = session {
            return  self.col.update_one_with_session(filter_by,update,update_option,session)
           }
            self.col.update_one(filter_by,update,update_option)
    }
}
 