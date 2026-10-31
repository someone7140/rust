use std::error::Error;

use crate::use_case::basic_info_import_use_case::import_basic_info;

mod model {
    pub mod csv_struct;
    pub mod intermediate_data;
}

mod use_case {
    pub mod basic_info_import_use_case;
}

mod util {
    pub mod csv_util;
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
    import_basic_info(&mut db).await?;
    Ok(())
}
