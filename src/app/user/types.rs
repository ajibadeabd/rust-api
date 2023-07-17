// use crate::{
//     app::{user::{user_controller}},
// };
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};
use serde::{Deserialize, Serialize};

use crate::modules::util::Token;

use super::{
    user_model::User,
    user_controller
};

 

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginRequestType {
    pub password: String,
    pub email: String,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct LoginResponse {
    pub user_detail: User,
    pub refresh_token: String,
    pub  access_token: String,
}