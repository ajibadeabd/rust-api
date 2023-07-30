use hmac::{Hmac, Mac};
use rocket::futures::future::ok;
use rocket::serde::json;
use sha2::Sha512;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Body};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap,env};
use log::error;
use dotenv::dotenv;
use serde_json::from_str;

use crate::modules::response_handler::CustomError;

// use rocket::{serde::json::Json};

fn get_key()->String{

dotenv().ok();
// const SECRET_KEY :String= env::var("PAYSTACK_SECRET_KEY").to_owned().unwrap();
// SECRET_KEY

let uri = match env::var("PAYSTACK_SECRET_KEY") {
    Ok(database_url) => database_url.to_string(),
    Err(_) => format!("Error loading env variable"),
};
uri

}
// const   SECRET_KEY :&str = "env::var(PAYSTACK_SECRET_KEY).unwrap_or";

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutResponse {
    transactionReference: String,
    checkoutUrl: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransferResponse {
    providerReference: String,
    transactionReference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentEvent {
    status: bool,
    #[serde(rename = "type")]
    transactionType: TransactionType,
    transactionId: String,
    providerReference: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDTO {
   pub  transactionReference: String,
   pub userEmail: String,
   pub amount: f64,
   pub currency: String,
   pub callbackUrl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferPAymentPayload {
    #[serde(rename = "type")]
    pub account_type: String,
    pub name: String,
    pub account_number: String,
    pub bank_code: String,
    pub currency: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct TransferDTO {
    bankAccountName: String,
    bankAccountNumber: String,
    bankCode: String,
    currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeTransactionDTO {
    reference: String,
    email: String,
    amount: f64,
    currency: String,
    callback_url: String,
    channels: Vec<String>,
    metadata: HashMap<String, serde_json::Value>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientDetails {
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub currency: String,
    pub description: Option<String>,
    pub domain: String,
    pub email: Option<String>,
    pub id: u64,
    pub integration: u64,
    pub metadata: Option<String>,
    pub name: String,
    pub recipient_code: String,
    pub r#type: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub is_deleted: bool,
    pub details: RecipientAccountDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientAccountDetails {
    pub authorization_code: Option<String>,
    pub account_number: String,
    pub account_name: String,
    pub bank_code: String,
    pub bank_name: String,
}


// Implement Into<reqwest::Body> for &InitializeTransactionDTO
impl Into<Body> for &InitializeTransactionDTO {
    fn into(self) -> Body {
        // Serialize the DTO to JSON and create a Body from it
        Body::from(serde_json::to_string(self).unwrap())
    }
}

impl Into<Body> for &TransferPAymentPayload {
    fn into(self) -> Body {
        // Serialize the DTO to JSON and create a Body from it
        Body::from(serde_json::to_string(self).unwrap())
    }
}


fn create_authorization_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", get_key()).parse().unwrap(),
    );
    headers
}
 
 
#[derive(Debug,Clone)]
pub struct PaystackApi ;
 
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ResponseData<T> {
    status: bool,
    message: String,
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseTransferData {
    account_number: String,
    account_name: String,
    bank_id: u32,
}
#[derive(Debug, Serialize,Deserialize,Clone)]
pub struct ResponseDataDetails {
    authorization_url: String,
    access_code:Option<String>,
    reference: String,
}
#[derive(Debug, Serialize,Deserialize,Clone)]
pub struct DepositResponseDataDetails {
    checkout_url: String,
    transaction_reference: String,
}

impl PaystackApi {

    pub async fn initialize_transaction(&self,dto: TransactionDTO) 
    -> Result<DepositResponseDataDetails, CustomError> 
    {
        let payload = InitializeTransactionDTO  {
            reference: dto.transactionReference,
            email: dto.userEmail,
            amount: (dto.amount * 100.0) ,
            currency: dto.currency,
            callback_url: dto.callbackUrl,
            channels: Vec::from([
                "card".to_owned(), 
                "bank".to_owned(), 
                "ussd".to_owned(), 
                "mobile_money".to_owned(), 
                "bank_transfer".to_owned(), 
                ]),
            metadata: HashMap::new()
        };
    
    
        let client = Client::new();
        let response_data = client.post("https://api.paystack.co/transaction/initialize")
            .headers(create_authorization_header())
            .body(&payload)
            .send()
            .await.unwrap();
    if response_data.status().is_success()==false {
            return Err(CustomError::BadRequest("Unable to process transaction".to_string()));
    } else{
        let response  = response_data.text().await.unwrap();
        let initialize_response:ResponseData::<ResponseDataDetails> = from_str(&response).unwrap();
        let initialize_response_data = initialize_response.data.unwrap();

        let response  = DepositResponseDataDetails{
            transaction_reference:initialize_response_data.reference,
            checkout_url:initialize_response_data.authorization_url
        };
         Ok(response)
    }
    }
    pub async fn initialize_transfer(&self, payload: TransferPAymentPayload) 
-> Result<RecipientDetails, CustomError> {
   

    let client = Client::new();
    let response_data = client.post("https://api.paystack.co/transferrecipient")
        .headers(create_authorization_header())
        .body(&payload)
        .send()
        .await.unwrap();
    
    if response_data.status().is_success()==false {
        let response  = response_data.text().await.unwrap();
        let initialize_response:ResponseData<String> = from_str(&response).unwrap();
            return Err(CustomError::BadRequest(initialize_response.message));
    // let response  = response_data.text().await.unwrap();
    //     return Err(CustomError::BadRequest("Unable to process transaction".to_string()));
} else{
    let response  = response_data.text().await.unwrap();
    let initialize_transfer_response: ResponseData::<RecipientDetails>  = from_str(&response).unwrap();
     Ok(initialize_transfer_response.data.unwrap())
}

}
 

pub async fn get_account_name(&self,account_number: &str, bank_code: &str) -> Result<String, CustomError> {
    let url = format!("https://api.paystack.co/bank/resolve?account_number={}&bank_code={}", account_number, bank_code);
    let client = Client::new();
    let response_data = client.get(&url)
        .headers(create_authorization_header())
        .send()
        .await.unwrap(); 
    if response_data.status().is_success()==false {
        let response  = response_data.text().await.unwrap();
        let initialize_response:ResponseData::<String> = from_str(&response).unwrap();
            return Err(CustomError::BadRequest(initialize_response.message));
    } else{
        let response  = response_data.text().await.unwrap();

        let initialize_response:ResponseData::<ResponseTransferData > = from_str(&response).unwrap();
         Ok(initialize_response.data.unwrap().account_name)
    }
}
}

// fn verify_webhook_payload(signature: &str, payload: &serde_json::Value) -> Option<PaymentEvent> {
//     let payload_string = payload.to_string();
//     let mut mac = Hmac::<Sha512>::new_varkey(SECRET_KEY.as_bytes()).unwrap();
//     mac.update(payload_string.as_bytes());
//     let hash = mac.finalize().into_bytes();
//     let hash_string = "hex::encode(hash)";

//     if signature != hash_string {
//         return None;
//     }

//     let event_type = payload.get("event").and_then(|event| event.as_str()).unwrap_or("");
//     let data = payload.get("data").unwrap_or(&serde_json::Value::Null);

//     let event = match event_type {
//         "charge.success" => PaymentEvent {
//             status: true,
//             transactionType: TransactionType::DEPOSIT,
//             transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
//             providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
//         },
//         "charge.failed" => PaymentEvent {
//             status: false,
//             transactionType: TransactionType::DEPOSIT,
//             transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
//             providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
//         },
//         "transfer.success" => PaymentEvent {
//             status: true,
//             transactionType: TransactionType::WITHDRAWAL,
//             transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
//             providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
//         },
//         "transfer.failed" | "transfer.reversed" => PaymentEvent {
//             status: false,
//             transactionType: TransactionType::WITHDRAWAL,
//             transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
//             providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
//         },
//         _ => {
//             Err(format!("Unhandled event type: {}", event_type).as_str());
//             return None;
//         }
//     };

//     Some(event)
// }
