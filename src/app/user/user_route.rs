
use rocket::{http::Status, serde::json::Json,State, figment::value::Value};
use crate::{
    database::{
        Database
    }, app::user::types::LoginResponse ,
    modules::{generic_type::ResponseType,  response_handler::{ CustomError, CustomResult}}
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
-> Result<CustomResult, CustomError>
     {
      user_controller::sign_up(db,user)
}


#[post("/sign_in", data = "<user>")]
pub fn sign_in(
    db: &State<Database>,
    user: Json<UserLoginRequestType>,
)-> Result<CustomResult, CustomError>{
     user_controller::sign_in(db,user)
}

// #[get("/")]
// pub fn get_all_user(db: &State<Database>)-> Result<CustomResult, CustomError>{
//      user_controller::get_all_user(db)
// }