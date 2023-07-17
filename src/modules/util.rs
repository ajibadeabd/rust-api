// use crate::models::tokens::Token;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum EncodeJwtHelper {
    Ok(String),
    Err,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}
#[derive(Debug)]
pub enum DecodeJwtHelper {
    Ok(TokenData<Claims>),
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}





pub fn encode_jwt(id: ObjectId, secret: &str, expiration: i64) -> EncodeJwtHelper {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        user_id: id.to_string(),
        exp: expiration as usize,
    };
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => EncodeJwtHelper::Ok(token),
        Err(_) => EncodeJwtHelper::Err,
    }
}

pub fn decode_jwt(token: &str, secret: &str) -> DecodeJwtHelper {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    match token {
        Ok(token_string) => DecodeJwtHelper::Ok(token_string),
        Err(err) => {
            println!("{:?}",err);
            DecodeJwtHelper::Err},
    }
}

pub fn encode_token_and_refresh(
    id: ObjectId,
    jwt_secret: &'static str,
    refresh_token_secret: &'static str,
    expiration_refresh_token: i64,
    expiration_token: i64,
) -> Result<Token, ()> {
    match encode_jwt(id, jwt_secret, expiration_token) {
        EncodeJwtHelper::Ok(token) => {
            match encode_jwt(id, refresh_token_secret, expiration_refresh_token) {
                EncodeJwtHelper::Ok(refresh_token) => Ok(Token {
                    token,
                    refresh_token,
                }),
                EncodeJwtHelper::Err => Err(()),
            }
        }
        EncodeJwtHelper::Err => Err(()),
    }
}