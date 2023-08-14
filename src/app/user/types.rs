// use crate::{
//     app::{user::{user_controller}},
// };

use serde::{Deserialize, Serialize};

use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};


use super::{
    user_model::User
};

 

#[derive(Debug, Serialize, Deserialize,JsonSchema)]
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