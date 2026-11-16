use std::error::Error;

use toasty::Db;

use crate::service::{
    basic_info_import_service::import_basic_info, budget_import_service::import_budget,
};

// CSVインポート
pub async fn import_csv(db: &mut Db) -> Result<(), Box<dyn Error>> {
    // 基本情報
    import_basic_info(db).await?;
    // 予算
    import_budget(db).await?;
    Ok(())
}
