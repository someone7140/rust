use sea_orm::DatabaseConnection;
use shuttle_runtime::SecretStore;

#[derive(Clone)]
pub struct CommonContext {
    pub secrets: SecretStore,
    pub db_connect: DatabaseConnection,
}

#[derive(Clone)]
pub struct AuthContext {
    pub user_account_id: String,
}
