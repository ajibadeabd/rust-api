use rocket::{serde::json::Json, http::Status};
use serde::Serialize;

use super::response_handler::CustomError;


#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}
pub type ResponseType<T> = Result<Json<GenericResponse<T>>,CustomError>;

