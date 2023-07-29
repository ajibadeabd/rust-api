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
    // #[serde(rename = "type")]
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

// pub async fn initialize_transaction(dto: TransactionDTO) -> Result<CheckoutResponse, Box<dyn std::error::Error>> {
//     println!("frfrf {:?}",dto.transactionReference);
//     let payload = InitializeTransactionDTO  {
//         reference: dto.transactionReference,
//         email: dto.userEmail,
//         amount: (dto.amount * 100.0) ,
//         currency: dto.currency,
//         callback_url: dto.callbackUrl,
//         channels: Vec::from([
//             "card".to_owned(), 
//             "bank".to_owned(), 
//             "ussd".to_owned(), 
//             "mobile_money".to_owned(), 
//             "bank_transfer".to_owned(), 
//             ]),
//         metadata: HashMap::new()
//     };


//     let client = Client::new();
//     let res = client.post("https://api.paystack.co/transaction/initialize")
//         .headers(create_authorization_header())
//         // .json(&payload)
//         .body(&payload)
//         .send()
//         .await?;

//     // let res_data: HashMap<String, serde_json::Value> = res.
//     let res_data = res.text().await;
//     println!("{:?}",res_data);
//     // if let Some(status) = res_data.get("status").and_then(|s| s.as_bool()) {
//         // if res.status() {
//         //     return Err("Unable to initialize transaction".into());
//         // }
//     // }

//     // let data = res_data.get("data").ok_or("Missing data field")?;
//     // let reference = data.get("reference").ok_or("Missing reference field")?.as_str().ok_or("Invalid reference field")?.to_string();
//     // let authorization_url = data.get("authorization_url").ok_or("Missing authorization_url field")?.as_str().ok_or("Invalid authorization_url field")?.to_string();

//     Ok(CheckoutResponse {
//         transactionReference: "reference".to_string(),
//         checkoutUrl: "authorization_url".to_string(),
//     })
// }
#[derive(Debug,Clone)]
pub struct PaystackApi ;

#[derive(Debug, Serialize,Deserialize,Clone)]
pub struct ResponseData {
    status: bool,
    message: String,
    data: ResponseDataDetails,
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
        let initialize_response:ResponseData = from_str(&response).unwrap();
        let response  = DepositResponseDataDetails{
            transaction_reference:initialize_response.data.reference,
            checkout_url:initialize_response.data.authorization_url
        };
         Ok(response)
    }
    }
    pub async fn initialize_transfer(&self, payload: TransferPAymentPayload) 
-> Result<(), CustomError> {
   

    let client = Client::new();
    let response_data = client.post("https://api.paystack.co/transferrecipient")
        .headers(create_authorization_header())
        .body(&payload)
        .send()
        .await.unwrap();
    
    if response_data.status().is_success()==false {
        return Err(CustomError::BadRequest("Unable to process transaction".to_string()));
} else{
    let response  = response_data.text().await.unwrap();
    let initialize_response:ResponseData = from_str(&response).unwrap();
    let response  = DepositResponseDataDetails{
        transaction_reference:initialize_response.data.reference,
        checkout_url:initialize_response.data.authorization_url
    };
    println!("{:?}",response);
     Ok(())
    //  Ok(response)
}

}
}

 

// async fn get_account_name(account_number: &str, bank_code: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let url = format!("https://api.paystack.co/bank/resolve?account_number={}&bank_code={}", account_number, bank_code);
//     let client = Client::new();
//     let res = client.get(&url)
//         .headers(create_authorization_header())
//         .send()
//         .await?;

//     let res_data: HashMap<String, serde_json::Value> = res.json().await?;
//     let account_name = res_data.get("data")
//         .and_then(|data| data.get("account_name"))
//         .and_then(|account_name| account_name.as_str())
//         .unwrap_or("");

//     Ok(account_name.to_string())
// }

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
