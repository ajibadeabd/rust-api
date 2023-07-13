
use rocket::{ http::Status, serde::json::Json,State, figment::value::Value};
use serde::{Serialize,Deserialize};
use crate::{
    database::{
        user::User,
        Database
    },
    // app::
};
use mongodb::{
    bson::oid::ObjectId,
    sync::{Collection},
    results::InsertOneResult
};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

// #[derive(Debug, Deserialize)]
// pub struct User {
//     username: String,
//     password: String,
// }

#[get("/user")]
pub async fn health_checker_handler() -> rocket::response::status::Custom<Json<GenericResponse>> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    rocket::response::status::Custom(Status::Ok, Json(response_json))
}
 
#[post("/sign_up", data = "<user>")]
pub async fn add_user(
    db: &State<Database>,
    user: Json<User>) 
    -> Result<rocket::response::status::Custom<Json<GenericResponse>>,Status>
     {
      let user_detail = db.user().find_one(&user.name);
 
    let _=match user_detail {
        Ok(user) => Err(user.unwrap().name.to_string() +"already exist"),
        _=>Ok(""),
    };

      let user_detail = db.user().save(&user);

      match user_detail {
          Ok(user_id) => Ok({
            let response_json = GenericResponse {
                status: "success".to_string(),
                message: user.name.to_string() + " account created"
            };
            rocket::response::status::Custom(Status::Ok, Json(response_json))
          }),
          Err(_) => Err(Status::InternalServerError),
      }

    // Json(response_json)
}