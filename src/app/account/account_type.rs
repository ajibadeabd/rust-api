use mongodb::bson::oid::ObjectId;
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

#[derive(Debug, Clone,  Serialize,  Deserialize)]
pub struct WithdrawAccountData {
    pub currency:String,
    pub amount:f64,
    pub bank_account_number:i64,
    pub bank_account_name:String,
    pub bank_code:i32
}

#[derive(Debug, Clone,  Serialize,  Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "failed")]
    FAILED,
    #[serde(rename = "success")]
    SUCCESS,
}

#[derive(Debug, Clone,  Serialize,  Deserialize)]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    DEPOSIT,
    #[serde(rename = "withdrawal")]
    WITHDRAWAL,
    #[serde(rename = "transfer")]
    TRANSFER,
}

#[derive(Debug, Clone, PartialEq, Serialize,Deserialize)]
pub enum SupportedCurrency {
    #[serde(rename = "NGN")]
    NGN,
}
