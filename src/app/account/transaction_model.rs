use chrono::{Utc, DateTime};
use mongodb::{
    bson::{oid::ObjectId,doc},
    sync::{Collection},
    results::InsertOneResult
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
        pub receiver_id: Option<ObjectId>,
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
        receiver_id: Option<ObjectId>,
        provider_name: String,
    ) -> Self {
        Self {
            amount,
            currency,
            id:None,
            fee,
            receiver_id,
            giver_id:"INTERNAL".to_string(),
            provider_name,
            transaction_type:TransactionType::DEPOSIT,
            status:TransactionStatus::PENDING,
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
    pub fn find_one(&self, transaction: &str)->Result<std::option::Option<Transaction>, mongodb::error::Error> {
        self.col.find_one(doc! {"":transaction}, None)
    }
}
 