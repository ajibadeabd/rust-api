
use rocket::{serde::json::Json,State};
use crate::{
    database::{
        Database
    },
    modules::{ response_handler::{ CustomError, CustomResult}}
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
 