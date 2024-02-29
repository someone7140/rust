use mongodb::Database;
use shuttle_secrets::SecretStore;

pub struct CommonContext {
    pub secrets: SecretStore,
    pub mongo_db: Database,
}
