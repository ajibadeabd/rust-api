 
use mongodb::{bson::{Document}, options::{UpdateModifications, UpdateOptions}, results::UpdateResult};
use rocket::{State};

use crate::database::Database;
 
 

pub fn update_user_account(db: &State<Database>,filter_by:Document,update_doc:UpdateModifications,update_option:Option<UpdateOptions>)
-> Result<UpdateResult,mongodb::error::Error>
{
    db.user().update_one(filter_by,
    update_doc,
    None)
}

 