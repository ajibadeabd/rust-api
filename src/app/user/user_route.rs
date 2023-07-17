
use rocket::{http::Status, serde::json::Json,State, figment::value::Value};
use crate::{
    database::{
        Database
    }, app::user::types::LoginResponse ,
    modules::{generic_type::ResponseType}
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
    request: &mut Request<'_>,
)
    ->rocket::response::status::Custom<ResponseType<Option<LoginResponse>>>
     {
       let response =  user_controller::sign_in(db,user);
       rocket::response::status::Custom(Status::Ok, response)
}

// #[post("/sign_in", data = "<user>")]
// pub async fn sign_in(
//     db: &State<Database>,
//     user: Json<UserLoginRequestType>,
//     user_data: &State<Option<User>>,
// ) -> rocket::response::status::Custom<ResponseType<Option<LoginResponse>>> {
//     match user_data.as_ref() {
//         Some(user) => {
//             // Access the user data here
//             let name = &user.name;
//             println!("User: {}", name);
//             // Continue with your sign-in logic using the user data
            
//             let response = user_controller::sign_in(db, user);
//             rocket::response::status::Custom(Status::Ok, response)
//         }
//         None => {
//             // User data not available
//             rocket::response::status::Custom(
//                 Status::InternalServerError,
//                 ResponseType::new_error("User data not found"),
//             )
//         }
//     }
// }
