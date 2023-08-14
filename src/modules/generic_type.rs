
use serde::Serialize;




#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

