// use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, Document}, options::FindOneOptions};
// use rocket::{State, serde::json::Json, http::Status};

// use crate::{database::Database, modules::response_handler::CustomError, app::user::user_model::User};

// use super::{account_type::AccountData, account_model::Account};

use mongodb::{bson::{Document, doc}, options::{UpdateModifications, UpdateOptions}, results::UpdateResult};
use rocket::{State, http::ext::IntoCollection};

use crate::database::Database;

use super::user_model::User;


 

pub fn update_user_account(db: &State<Database>,filter_by:Document,update_doc:UpdateModifications,update_option:Option<UpdateOptions>)
-> Result<UpdateResult,mongodb::error::Error>
{
    db.user().update_one(filter_by,
    update_doc,
    None)
}


// pub fn get_all_user(db: &State<Database>)
// -> Result<mongodb::sync::Cursor<User>,mongodb::error::Error>
// {
//     let users = db.user().find_all(doc!{}, None);
//     // users.collect::<Result<Vec<User>, Error>>(
//         users 
//     // cursor.collect::<Result<Vec<User>, Error>>()
// }
