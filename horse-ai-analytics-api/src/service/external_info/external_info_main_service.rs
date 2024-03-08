use async_graphql::*;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;

use crate::service::external_info::external_info_common_service;
use crate::service::external_info::umanity_service;
use crate::struct_const_def::prompt_def;

// ウマニティの出馬表urlからプロンプトを取得
pub async fn get_race_info_from_umanity_url(
    url: String,
) -> Result<horse_model::GetRaceInfoResponse> {
    // urlからhtml取得
    let html = match external_info_common_service::get_html_from_url(&url).await {
        Ok(t) => t,
        Err(error) => return Err(error),
    };
    // レースコードと日付を取得
    let (race_code_umanity, race_date_yyyy_mm_dd) =
        match umanity_service::get_race_code_and_date_from_url_code(&url) {
            Ok(t) => t,
            Err(error) => return Err(error),
        };
    // レース名を取得
    let race_name = match umanity_service::get_race_name_from_html(&html) {
        Ok(n) => n,
        Err(error) => return Err(error),
    };

    // プロンプト用の情報
    let mut horse_info_list: Vec<prompt_def::PromptHorseInfo> = Vec::new();
    // 指定されたページによって呼ぶ処理を変える
    match external_info_common_service::get_page_from_url(&url) {
        Ok(page) => {
            if page == "race_7.php" {
                if let Ok(res) = umanity_service::get_horse_info_from_race_7(&html) {
                    horse_info_list = res
                };
            }
        }
        Err(error) => return Err(error),
    };

    if horse_info_list.len() < 1 {
        return Err(Error::new("Can not get horse info")
            .extend_with(|_, e| e.set("type", ErrorType::BadRequest)));
    }

    return Ok(horse_model::GetRaceInfoResponse {
        race_name,
        race_date_yyyy_mm_dd,
        prompt: "test".to_string(),
    });
}
