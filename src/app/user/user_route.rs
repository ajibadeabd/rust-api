
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};
use crate::{
    database::{
        Database
    }, app::user::types::LoginResponse ,
    modules::{generic_type::ResponseType}
};
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
pub async fn sign_in(
    db: &State<Database>,
    user: Json<UserLoginRequestType>) 
    ->rocket::response::status::Custom<ResponseType<Option<LoginResponse>>>
     {
       let response =  user_controller::sign_in(db,user);
       rocket::response::status::Custom(Status::Ok, response)
}