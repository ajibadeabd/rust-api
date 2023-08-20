use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    pub currency:String,
    pub channel:String,
    pub user_id:Option<ObjectId>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositAccountData {
    pub currency:String,
    pub amount:f64
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferPaymentData {
    pub amount: f64,
    pub currency: String,
    pub receiver_id: String,
}

#[derive(Debug, Clone,  Serialize,  Deserialize)]
pub struct WithdrawAccountData {
    pub currency:String,
    pub amount:f64,
    pub bank_account_number:String,
    pub bank_code:String
}

#[derive(Debug, Clone,  Serialize,  Deserialize,PartialEq)]
pub enum TransactionStatus {
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "failed")]
    FAILED,
    #[serde(rename = "success")]
    SUCCESS,
}

#[derive(Debug, Clone,  Serialize,  Deserialize,PartialEq)]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    DEPOSIT,
    #[serde(rename = "withdrawal")]
    WITHDRAWAL,
    #[serde(rename = "transfer")]
    TRANSFER,
}





impl From<TransactionType> for Bson {
    fn from(transaction_type: TransactionType) -> Self {
        match transaction_type {
            TransactionType::DEPOSIT => Bson::String("deposit".to_string()),
            TransactionType::TRANSFER => Bson::String("transfer".to_string()),
            TransactionType::WITHDRAWAL => Bson::String("withdrawal".to_string()),
            // Handle other cases
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize,Deserialize)]
pub enum SupportedCurrency {
    #[serde(rename = "NGN")]
    NGN,
}



#[derive(Debug,Deserialize,Serialize)]
pub struct TransactionsQueryData {
   pub transaction_id: Option<String>,
   pub account_id:Option<String>,
   pub currency: Option<String>,
   pub limit: Option<String>,
   pub page: Option<String>,
   pub transaction_type: Option<TransactionType>,
   
 
}

#[derive(Debug,Deserialize,Serialize)]
pub struct DashboardResponse {
   pub accounts:Vec<Account>,
   pub transactions:Vec<Transaction>
 
}



use std::collections::HashMap;

use super::{account_model::Account, transaction_model::Transaction};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentEventRequestBody {
    pub event: String,
    pub data: PaymentEventData,
}


// #[derive(Debug, Deserialize, Serialize)]
// pub struct PaymentEventData {
//     pub amount: u32,
//     pub currency: String,
//     pub domain: String,
//     pub id: u32,
//     pub integration: Option<Integration>,
//     pub reference: String,
//     pub source: Option<String>,
//     pub status: String,
//     pub recipient: Option<Recipient>,
//     pub session: Option<Session>,
//     pub created_at: String,
//     pub updated_at: Option<String>,
//     #[serde(flatten)]
//     pub event_specific: Option<HashMap<String, serde_json::Value>>,
// }
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentEventData {
    pub amount: u32,
    pub currency: String,
    pub domain: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<Integration>,
    pub reference: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    
    pub source: Option<Source>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Recipient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub event_specific: Option<HashMap<String, serde_json::Value>>,
}

 

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Source {
    Concrete(ConcreteSource),
    String(String),
}
 

#[derive(Debug, Deserialize, Serialize)]
   struct ConcreteSource {
    pub domain: String,
  }

#[derive(Debug, Deserialize, Serialize)]
pub struct Integration {
    pub id: u32,
    pub is_live: bool,
    pub business_name: String,
}
 

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub active: bool,
    pub currency: String,
    pub domain: String,
    pub email: Option<String>,
    pub id: u32,
    pub integration: u32,
    pub name: String,
    pub recipient_code: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub is_deleted: bool,
    pub details: RecipientDetails,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipientDetails {
    pub authorization_code: Option<serde_json::Value>,
    pub account_number: String,
    pub account_name: Option<serde_json::Value>,
    pub bank_code: String,
    pub bank_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub provider: Option<String>,
    pub id: Option<String>,
}
