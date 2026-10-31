use std::path::PathBuf;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let db_connect_str = std::env::var("DB_CONNECT")?;

    let db = toasty::Db::builder()
        .models(toasty::models!(entity::*))
        .connect(&db_connect_str)
        .await?;

    // Toasty.tomlを読み込む
    let migration_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut config = Config::load_from(&migration_dir.join("Toasty.toml"))?;
    config.migration.path = migration_dir.join(&config.migration.path);

    ToastyCli::with_config(db, config).parse_and_run().await?;
    Ok(())
}
