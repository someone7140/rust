use serde_json::Value;

use crate::graphql_object::horse_model::{OddsInfo, OddsInfoResponse};
use crate::service::external_info::{
    external_info_common_service, umanity_service::get_common_race_code_from_umanity_code,
};

// ウマニティのコードをもとにtospoのオッズ情報を取得
pub async fn get_tospo_odds_info_from_umanity_code(
    umanity_code: &String,
) -> Option<OddsInfoResponse> {
    let mut odds_url = "".to_string();
    let mut api_url = "".to_string();
    if let Some(race_code) = get_common_race_code_from_umanity_code(&umanity_code) {
        odds_url = format!(
            "https://tospo-keiba.jp/race/{race_code_param}/card",
            race_code_param = &race_code
        );
        api_url = format!(
            "https://tospo-keiba.jp/race/detail/{race_code_param}/card",
            race_code_param = &race_code
        );
    } else {
        return None;
    }

    let odds_list = match external_info_common_service::get_contents_from_url(&api_url, None).await
    {
        Ok(tospo_json) => get_tospo_odds_info(&tospo_json).await,
        _ => return None,
    };

    if (&odds_list).len() < 1 {
        return None;
    }

    return Some(OddsInfoResponse {
        odds_url,
        odds_list,
    });
}

async fn get_tospo_odds_info(json_text: &String) -> Vec<OddsInfo> {
    let mut odds_list: Vec<OddsInfo> = vec![];

    let json_value_result: Result<Value, serde_json::Error> = serde_json::from_str(json_text);
    let json_value = match json_value_result {
        Ok(value) => value,
        _ => return odds_list,
    };

    let json_value = &json_value["body"]["raceEntryList"].as_array();
    match json_value.clone() {
        Some(value) => {
            let race_entry_list = value.clone();
            for race_entry in race_entry_list {
                let horse_name_opt = race_entry["horseName"].as_str();
                let odds_opt = race_entry["odds"].as_f64();
                if let (Some(horse_name), Some(odds)) = (horse_name_opt, odds_opt) {
                    odds_list.push(OddsInfo {
                        horse_name: horse_name.to_string(),
                        odds: odds.to_string(),
                    })
                }
            }
        }
        _ => return odds_list,
    };

    odds_list.sort_by(|a, b| {
        let a_odds = match a.odds.parse::<f32>() {
            Ok(odds) => odds,
            _ => 9999999999.0,
        };
        let b_odds = match b.odds.parse::<f32>() {
            Ok(odds) => odds,
            _ => 9999999999.0,
        };

        a_odds.partial_cmp(&b_odds).unwrap()
    });

    return odds_list;
}
