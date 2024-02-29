use serde::{Deserialize, Serialize};

pub const ACCOUNT_USERS_COLLECTION: &str = "account_users";

#[derive(Serialize, Deserialize)]
pub struct AccountUsers {
    #[serde(rename = "_id", skip_serializing)]
    id: String,
    name: String,
    user_setting_id: String,
    gmail: Option<String>,
    email: Option<String>,
    password: Option<String>,
}
