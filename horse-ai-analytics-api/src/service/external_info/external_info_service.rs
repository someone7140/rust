use async_graphql::*;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;

use crate::service::external_info::umanity_service;

// ウマニティの出馬表urlからプロンプトを取得
pub async fn get_race_info_from_umanity_url(
    url: String,
) -> Result<horse_model::GetRaceInfoResponse> {
    // urlからhtml取得
    let html = match get_html_from_url(&url).await {
        Ok(t) => t,
        Err(error) => return Err(error),
    };
    // レースコードと日付を取得
    let (race_code_umanity, race_date_yyyy_mm_dd) =
        match umanity_service::get_race_code_and_date_from_url_code(url) {
            Ok(t) => t,
            Err(error) => return Err(error),
        };
    // レース名を取得
    let race_name = match umanity_service::get_race_name_from_html(html) {
        Ok(n) => n,
        Err(error) => return Err(error),
    };

    return Ok(horse_model::GetRaceInfoResponse {
        race_name,
        race_date_yyyy_mm_dd,
        prompt: "test".to_string(),
    });
}

// urlからhtmlを取得
async fn get_html_from_url(url: &String) -> Result<String> {
    match reqwest::get(url).await {
        Ok(r) => match r.text().await {
            Ok(text) => Ok(text),
            Err(error) => {
                return Err(Error::new(error.to_string())
                    .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
            }
        },
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    }
}
