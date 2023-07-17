use chrono::{DateTime, Utc};
use mongodb::{
    bson::{oid::ObjectId,doc},
    sync::{Collection},
    results::InsertOneResult
};
use serde::{Serialize, Deserialize};

use super::account_type::AccountData;
#[derive(Debug, Serialize, Deserialize)]
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
    fn new()->Self{
        Self {
            locked_balance:0.0,
            balance:0.0,
            currency:"NG".to_string(),
            channel:"INTERNAL".to_string(),
            user_id:None,
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

    pub fn save(&self, account: &AccountData)->Result<InsertOneResult, mongodb::error::Error> {
        let mut new_account = Account::new();
        new_account.channel = account.channel.to_string();
        new_account.currency = account.currency.to_string();
        self.col.insert_one(new_account, None)
    }
    pub fn find_one(&self, user: &str)->Result<std::option::Option<Account>, mongodb::error::Error> {
        self.col.find_one(doc! {"":user}, None)
    }
}
 