
use rocket::{http::Status, serde::json::Json,State, figment::value::Value};
use crate::{
    database::{
        Database
    }, app::user::types::LoginResponse ,
    modules::{generic_type::ResponseType, middleware_copy::StartTime, response_handler::{ CustomError, CustomResult}}
};
use rocket::request::{Request};
use super::{
    user_model::User,
    user_controller,
    types::UserLoginRequestType
};

#[post("/sign_up", data = "<user>")]
pub async fn add_user(
    db: &State<Database>,
    user: Json<User>) 
    ->rocket::response::status::Custom<ResponseType<Option<String>>>
     {
       let response =  user_controller::sign_up(db,user);
       rocket::response::status::Custom(Status::Ok, response)
}


#[post("/sign_in", data = "<user>")]
pub fn sign_in(
    db: &State<Database>,
    user: Json<UserLoginRequestType>,
)

-> Result<CustomResult, CustomError>{
     user_controller::sign_in(db,user)
}