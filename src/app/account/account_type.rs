use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    pub currency:String,
    pub channel:String

}