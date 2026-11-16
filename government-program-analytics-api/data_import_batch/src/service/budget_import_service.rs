use crate::{
    model::csv_struct::Budget21Summary,
    util::{csv_util, db_util::batch_insert},
};
use entity::db_model::BudgetSummary;
use std::error::Error;
use toasty::{
    Db,
    stmt::{List, Query},
};

// 予算のインポート
pub async fn import_budget(db: &mut Db) -> Result<(), Box<dyn Error>> {
    // 各csvから取り込み
    import_budget21_summary(db).await?;

    Ok(())
}

// 2-1_予算・執行_サマリのcsv取り込み
pub async fn import_budget21_summary(db: &mut Db) -> Result<(), Box<dyn Error>> {
    let folder_path = std::env::var("IMPORT_DATA_FOLDER")?;
    let budget21_summary_itr = csv_util::read_csv::<Budget21Summary>("2-1", &folder_path)?;
    let budget21_summaries = budget21_summary_itr.collect::<Result<Vec<_>, _>>()?;

    let budget_summaries: Vec<BudgetSummary> = budget21_summaries
        .into_iter()
        .filter(|row| row.initial_budget.is_some())
        .map(|row| BudgetSummary {
            project_id: row.project_id,
            budget_year: row.budget_year,
            initial_budget: row.initial_budget.map(|v| v as i64).unwrap_or(0),
            supplementary_budget: row.supplementary_budget.map(|v| v as i64).unwrap_or(0),
            carried_over_budget: row.carried_over_budget.map(|v| v as i64).unwrap_or(0),
            contingency_budget: row.contingency_budget.map(|v| v as i64).unwrap_or(0),
            summary_budget: row.summary_budget.map(|v| v as i64).unwrap_or(0),
            executuon_budget: row.executuon_budget.map(|v| v as i64).unwrap_or(0),
            executuon_rate: row.executuon_rate,
            carried_forward_budget: row.carried_forward_budget.map(|v| v as i64).unwrap_or(0),
            request_budget: row.request_budget.map(|v| v as i64).unwrap_or(0),
            reason_for_change: row.reason_for_change,
        })
        .collect();

    // 全件クリア
    Query::<List<BudgetSummary>>::all()
        .delete()
        .exec(db)
        .await?;

    // 登録
    batch_insert::<BudgetSummary, _, _>(
        db,
        &budget_summaries,
        BudgetSummary::create_many,
        |b, item| {
            item.project_id(b.project_id)
                .budget_year(b.budget_year)
                .initial_budget(b.initial_budget)
                .supplementary_budget(b.supplementary_budget)
                .carried_over_budget(b.carried_over_budget)
                .contingency_budget(b.contingency_budget)
                .summary_budget(b.summary_budget)
                .executuon_budget(b.executuon_budget)
                .executuon_rate(b.executuon_rate)
                .carried_forward_budget(b.carried_forward_budget)
                .request_budget(b.request_budget)
                .reason_for_change(b.reason_for_change.clone())
        },
    )
    .await?;

    Ok(())
}
