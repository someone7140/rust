use mongodb::Database;
use shuttle_secrets::SecretStore;

#[derive(Clone)]
pub struct CommonContext {
    pub secrets: SecretStore,
    pub mongo_db: Database,
}

#[derive(Eq, PartialEq, Clone)]
pub struct AuthContext {
    pub account_id: String,
}
