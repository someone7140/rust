use std::error::Error;

use crate::handler::import_handler::import_csv;

mod handler {
    pub mod import_handler;
}

mod model {
    pub mod csv_struct;
}

mod service {
    pub mod basic_info_import_service;
    pub mod budget_import_service;
}

mod util {
    pub mod csv_util;
    pub mod db_util;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    // DB接続
    let db_connect_str = std::env::var("DB_CONNECT")?;
    let mut db = toasty::Db::builder()
        .models(toasty::models!(entity::*))
        .connect(&db_connect_str)
        .await?;

    // CSV取り込み
    import_csv(&mut db).await?;
    Ok(())
}
