
use rocket::{ http::Status, serde::json::Json, figment::value::Value};
use serde::{Serialize,Deserialize};


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

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
pub async fn add_user(user: Json<User>) -> Json<GenericResponse> {
    let response_json = GenericResponse {
        status: user.username.to_string(),
        message: user.password.to_string(),
    };

    Json(response_json)
}