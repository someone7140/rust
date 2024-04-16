use async_graphql::*;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;

use crate::service::external_info::external_info_common_service;
use crate::service::external_info::netkeiba_service;
use crate::service::external_info::tospo_keiba_service;
use crate::service::external_info::umanity_service;
use crate::struct_const_def::prompt_def;

// ウマニティの出馬表urlからプロンプト情報などを取得
pub async fn get_race_info_from_umanity_url(
    url: String,
) -> Result<horse_model::GetRaceInfoResponse> {
    // urlからhtml取得
    let html = match external_info_common_service::get_contents_from_url(&url, None).await {
        Ok(text) => text,
        Err(error) => return Err(error),
    };
    // ウマニティのレースコードと日付を取得
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
    // オッズ用の情報
    let mut odds_info: Option<horse_model::OddsInfoResponse> = None;

    // 指定されたウマニティのページによって呼ぶ処理を変える
    match external_info_common_service::get_page_from_url(&url) {
        Ok(page) => {
            if page == "race_7.php" {
                // 出馬表の取得
                if let Ok(results) = umanity_service::get_horse_info_from_race_7(&html) {
                    // 付加情報の取得
                    let recent_results_map =
                        umanity_service::get_recent_results_from_race_7(&race_code_umanity).await;
                    let time_results_map =
                        umanity_service::get_time_results_from_race_7(&race_code_umanity).await;
                    let netkeiba_horse_info_map =
                        netkeiba_service::get_netkeiba_info_from_umanity_code(&race_code_umanity)
                            .await;
                    for info in results {
                        let mut mut_info = info.clone();
                        // 近走成績を付加
                        if let Some(recent_results) =
                            (&recent_results_map).get(&(info.umanity_code))
                        {
                            mut_info.recent_results = recent_results.to_string()
                        }
                        // タイムを付加
                        if let Some(time_results) = (&time_results_map).get(&(info.umanity_code)) {
                            mut_info.time_results = time_results.to_string()
                        }
                        // netkeibaの情報を付加
                        if let Some(net_keiba_info) = (&netkeiba_horse_info_map).get(&(info.name)) {
                            mut_info.jockey = net_keiba_info.jockey.clone();
                            mut_info.horse_weight = net_keiba_info.horse_weight.clone();
                        }

                        horse_info_list.push(mut_info);
                    }
                };
            } else {
                // 8_1のページの出馬表のurlからhtmlを取得
                let horse_list_url_page_8_1 = format!(
                    "https://umanity.jp/racedata/race_8_1.php?code={race_code_param}",
                    race_code_param = &race_code_umanity
                );
                let html_8_1 = match external_info_common_service::get_contents_from_url(
                    &horse_list_url_page_8_1,
                    None,
                )
                .await
                {
                    Ok(text) => text,
                    Err(error) => return Err(error),
                };
                if let Ok(results) = umanity_service::get_horse_info_from_race_8_1(&html_8_1).await
                {
                    // 付加情報の取得
                    let mut horse_info_page_7_list: Vec<prompt_def::PromptHorseInfo> = Vec::new();
                    if let Ok(html_page_7) = external_info_common_service::get_contents_from_url(
                        &format!(
                            "https://umanity.jp/racedata/race_7.php?code={race_code_param}",
                            race_code_param = &race_code_umanity
                        ),
                        None,
                    )
                    .await
                    {
                        if let Ok(page_7_info) =
                            umanity_service::get_horse_info_from_race_7(&html_page_7)
                        {
                            horse_info_page_7_list = page_7_info;
                        }
                    }
                    let recent_results_map =
                        umanity_service::get_recent_results_from_race_7(&race_code_umanity).await;
                    let time_results_map =
                        umanity_service::get_time_results_from_race_7(&race_code_umanity).await;
                    let netkeiba_horse_info_map =
                        netkeiba_service::get_netkeiba_info_from_umanity_code(&race_code_umanity)
                            .await;
                    odds_info = tospo_keiba_service::get_tospo_odds_info_from_umanity_code(
                        &race_code_umanity,
                    )
                    .await;

                    for info in results {
                        let mut mut_info = info.clone();
                        // race_7のページ情報を付加
                        if let Some(race_7_result) = horse_info_page_7_list
                            .iter()
                            .find(|p| &(p.umanity_code) == &(info).umanity_code)
                        {
                            mut_info.career_prize_money = race_7_result.career_prize_money.clone();
                            mut_info.all_results = race_7_result.all_results.clone();
                            mut_info.father = race_7_result.father.clone();
                            mut_info.mother = race_7_result.mother.clone();
                            mut_info.mother_father = race_7_result.mother_father.clone();
                        }
                        // 近走成績を付加
                        if let Some(recent_results) =
                            (&recent_results_map).get(&(info.umanity_code))
                        {
                            mut_info.recent_results = recent_results.to_string()
                        }
                        // タイムを付加
                        if let Some(time_results) = (&time_results_map).get(&(info.umanity_code)) {
                            mut_info.time_results = time_results.to_string()
                        }
                        // netkeibaの情報を付加
                        if let Some(net_keiba_info) = (&netkeiba_horse_info_map).get(&(info.name)) {
                            mut_info.jockey = net_keiba_info.jockey.clone();
                            mut_info.horse_weight = net_keiba_info.horse_weight.clone();
                        }
                        horse_info_list.push(mut_info);
                    }
                }
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
        odds: odds_info,
    });
}

fn get_prompt_text_from_info_list(
    race_name: &String,
    race_date_yyyy_mm_dd: &String,
    info_list: Vec<prompt_def::PromptHorseInfo>,
) -> String {
    // 最初の文言
    let mut prompt = format!(
        "{race_date_param}に行われる競馬のレース「{race_name_param}」で、以下のCSV形式の馬情報をもとに順位の見解を簡潔に教えて\n\n",
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
    let mut record_list: Vec<String> = Vec::new();
    for info in info_list {
        let mut column_value_vec = vec!["".to_string(); COLUMN_LEN];

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
        // 騎手
        if let Some(jockey) = &info.jockey {
            column_value_vec[4] = jockey.to_string()
        }
        // 負担重量
        column_value_vec[5] = info.charge_weight.to_string();
        // 馬体重
        if let Some(horse_weight) = &info.horse_weight {
            column_value_vec[6] = horse_weight.to_string()
        }
        // 所属
        column_value_vec[7] = info.belonging;
        // 調教師
        column_value_vec[8] = info.trainer;
        // 父
        column_value_vec[9] = info.father;
        // 母
        column_value_vec[10] = info.mother;
        // 全成績
        column_value_vec[11] = info.all_results;
        // 近走成績
        column_value_vec[12] = info.recent_results;
        // 合計獲得賞金
        column_value_vec[13] = info.career_prize_money;
        // 持ちタイム
        column_value_vec[14] = info.time_results;

        record_list.push(column_value_vec.join(","))
    }
    prompt = prompt + &(record_list.join("\n"));

    prompt
}
