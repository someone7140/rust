use serde::{Deserialize, Serialize};

pub const ACCOUNT_USERS_COLLECTION: &str = "account_users";

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountUsers {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub user_setting_id: String,
    pub gmail: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
