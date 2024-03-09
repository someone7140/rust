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
    let html: String = match external_info_common_service::get_html_from_url(&url).await {
        Ok(text) => text,
        Err(error) => return Err(error),
    };
    // レースコードと日付を取得
    let (race_code_umanity, race_date_yyyy_mm_dd) =
        match umanity_service::get_race_code_and_date_from_url_code(&url) {
            Ok(tuple) => tuple,
            Err(error) => return Err(error),
        };
    // レース名を取得
    let race_name = match umanity_service::get_race_name_from_html(&html) {
        Ok(name) => name,
        Err(error) => return Err(error),
    };

    // プロンプト用の情報
    let mut horse_info_list: Vec<prompt_def::PromptHorseInfo> = Vec::new();
    // 指定されたページによって呼ぶ処理を変える
    match external_info_common_service::get_page_from_url(&url) {
        Ok(page) => {
            if page == "race_7.php" {
                // 出馬表の取得
                if let Ok(results) = umanity_service::get_horse_info_from_race_7(&html).await {
                    // 付加情報の取得
                    let recent_results_map =
                        umanity_service::get_recent_results_from_race_7(&race_code_umanity).await;
                    for info in results {
                        let mut mut_info = info.clone();
                        // 近走成績を付加
                        if let Some(recent_results) =
                            (&recent_results_map).get(&(info.umanity_code))
                        {
                            mut_info.recent_results = recent_results.to_string()
                        }

                        horse_info_list.push(mut_info);
                    }
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
        race_name: race_name.clone(),
        race_date_yyyy_mm_dd: race_date_yyyy_mm_dd.clone(),
        prompt: get_prompt_text_from_info_list(&race_name, &race_date_yyyy_mm_dd, horse_info_list),
    });
}

fn get_prompt_text_from_info_list(
    race_name: &String,
    race_date_yyyy_mm_dd: &String,
    info_list: Vec<prompt_def::PromptHorseInfo>,
) -> String {
    // 最初の文言
    let mut prompt = format!(
        "{race_date_param}に行われる競馬のレース「{race_name_param}」で、以下のCSV形式の馬情報をもとに順位の見解だけ教えて\n\n",
        race_date_param = race_date_yyyy_mm_dd,
        race_name_param = race_name,
    );
    // 列名の設定
    prompt = prompt
        + &(prompt_def::CSV_COLUMN_MAP
            .values()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(","))
        + "\n";
    // 値の設定
    const COLUMN_LEN: usize = prompt_def::CSV_COLUMN_MAP.len();
    const EMPTY_STRING: String = String::new();
    let mut record_list: Vec<String> = Vec::new();
    for info in info_list {
        let mut column_value_vec = [EMPTY_STRING; COLUMN_LEN];

        // 枠番
        if let Some(waku) = info.waku_num {
            column_value_vec[0] = waku.to_string();
        }
        // 馬番
        if let Some(uma_num) = &info.uma_num {
            column_value_vec[1] = uma_num.to_string();
        }
        // 馬名
        column_value_vec[2] = info.name;
        // 性齢
        column_value_vec[3] = info.gender_and_age;
        // 負担重量
        column_value_vec[4] = info.charge_weight.to_string();
        // 所属
        column_value_vec[5] = info.belonging;
        // 調教師
        column_value_vec[6] = info.trainer;
        // 全成績
        column_value_vec[7] = info.all_results;
        // 合計獲得賞金
        column_value_vec[8] = info.career_prize_money;
        // 父
        column_value_vec[9] = info.father;
        // 母
        column_value_vec[10] = info.mother;
        // 母父
        column_value_vec[11] = info.mother_father;

        record_list.push(column_value_vec.join(","))
    }
    prompt = prompt + &(record_list.join("\n"));

    prompt
}
