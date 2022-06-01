use mongodb::sync::Client;
use mongodb::sync::Database;
use std::env;
use std::error::Error;

pub fn get_mongodb_db_connection() -> Result<Database, Box<dyn Error>> {
    let client = Client::with_uri_str(&env::var("DB_CONNECTION")?)?;
    let database = client.database(&env::var("DB_NAME")?);
    return Ok(database);
}
