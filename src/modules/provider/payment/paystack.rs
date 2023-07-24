use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::error;

const SECRET_KEY: &str = env!("PAYSTACK_SECRET_KEY");

#[derive(Debug, Serialize, Deserialize)]
struct CheckoutResponse {
    transactionReference: String,
    checkoutUrl: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransferResponse {
    providerReference: String,
    transactionReference: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaymentEvent {
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
struct TransactionDTO {
    transactionReference: String,
    userEmail: String,
    amount: f64,
    currency: String,
    callbackUrl: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransferDTO {
    bankAccountName: String,
    bankAccountNumber: String,
    bankCode: String,
    currency: String,
}

fn create_authorization_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", SECRET_KEY).parse().unwrap(),
    );
    headers
}

async fn initialize_transaction(dto: TransactionDTO) -> Result<CheckoutResponse, Box<dyn std::error::Error>> {
    let payload = json!({
        "reference": dto.transactionReference,
        "email": dto.userEmail,
        "amount": (dto.amount * 100.0) as i64,
        "currency": dto.currency,
        "callback_url": dto.callbackUrl,
        "channels": ["card", "bank", "ussd", "mobile_money", "bank_transfer"],
        "metadata": {}
    });

    let client = Client::new();
    let res = client.post("https://api.paystack.co/transaction/initialize")
        .headers(create_authorization_header())
        .json(&payload)
        .send()
        .await?;

    let res_data: HashMap<String, serde_json::Value> = res.json().await?;
    if let Some(status) = res_data.get("status").and_then(|s| s.as_bool()) {
        if !status {
            return Err("Unable to initialize transaction".into());
        }
    }

    let data = res_data.get("data").ok_or("Missing data field")?;
    let reference = data.get("reference").ok_or("Missing reference field")?.as_str().ok_or("Invalid reference field")?.to_string();
    let authorization_url = data.get("authorization_url").ok_or("Missing authorization_url field")?.as_str().ok_or("Invalid authorization_url field")?.to_string();

    Ok(CheckoutResponse {
        transactionReference: reference,
        checkoutUrl: authorization_url,
    })
}

async fn initialize_transfer(dto: TransferDTO) -> Result<TransferResponse, Box<dyn std::error::Error>> {
    let payload = json!({
        "type": "nuban",
        "name": dto.bankAccountName,
        "account_number": dto.bankAccountNumber,
        "bank_code": dto.bankCode,
        "currency": dto.currency,
    });

    let client = Client::new();
    let res = client.post("https://api.paystack.co/transferrecipient")
        .headers(create_authorization_header())
        .json(&payload)
        .send()
        .await?;

    let res_data: HashMap<String, serde_json::Value> = res.json().await?;
    if let Some(status) = res_data.get("status").and_then(|s| s.as_bool()) {
        if !status {
            return Err("Unable to initialize transfer".into());
        }
    }

    let data = res_data.get("data").ok_or("Missing data field")?;
    let provider_reference = data.get("id").ok_or("Missing id field")?.as_str().ok_or("Invalid id field")?.to_string();
    let transaction_reference = data.get("reference").ok_or("Missing reference field")?.as_str().ok_or("Invalid reference field")?.to_string();

    Ok(TransferResponse {
        providerReference: provider_reference,
        transactionReference: transaction_reference,
    })
}

async fn get_account_name(account_number: &str, bank_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.paystack.co/bank/resolve?account_number={}&bank_code={}", account_number, bank_code);
    let client = Client::new();
    let res = client.get(&url)
        .headers(create_authorization_header())
        .send()
        .await?;

    let res_data: HashMap<String, serde_json::Value> = res.json().await?;
    let account_name = res_data.get("data")
        .and_then(|data| data.get("account_name"))
        .and_then(|account_name| account_name.as_str())
        .unwrap_or("");

    Ok(account_name.to_string())
}

fn verify_webhook_payload(signature: &str, payload: &serde_json::Value) -> Option<PaymentEvent> {
    let payload_string = payload.to_string();
    let mut mac = Hmac::<Sha512>::new_varkey(SECRET_KEY.as_bytes()).unwrap();
    mac.update(payload_string.as_bytes());
    let hash = mac.finalize().into_bytes();
    let hash_string = hex::encode(hash);

    if signature != hash_string {
        return None;
    }

    let event_type = payload.get("event").and_then(|event| event.as_str()).unwrap_or("");
    let data = payload.get("data").unwrap_or(&serde_json::Value::Null);

    let event = match event_type {
        "charge.success" => PaymentEvent {
            status: true,
            transactionType: TransactionType::DEPOSIT,
            transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
            providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
        },
        "charge.failed" => PaymentEvent {
            status: false,
            transactionType: TransactionType::DEPOSIT,
            transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
            providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
        },
        "transfer.success" => PaymentEvent {
            status: true,
            transactionType: TransactionType::WITHDRAWAL,
            transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
            providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
        },
        "transfer.failed" | "transfer.reversed" => PaymentEvent {
            status: false,
            transactionType: TransactionType::WITHDRAWAL,
            transactionId: data.get("reference").and_then(|reference| reference.as_str()).unwrap_or("").to_string(),
            providerReference: data.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string(),
        },
        _ => {
            error(format!("Unhandled event type: {}", event_type).as_str());
            return None;
        }
    };

    Some(event)
}
