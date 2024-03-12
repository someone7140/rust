use std::collections::HashMap;

use crate::struct_const_def::prompt_def;

use super::external_info_common_service;

// ウマニティのコードをもとにnetkeibaの情報を取得
pub async fn get_netkeiba_info_from_umanity_code(
    umanity_code: &String,
) -> HashMap<String, prompt_def::PromptHorseInfo> {
    let mut netkeiba_horse_info_map = HashMap::<String, prompt_def::PromptHorseInfo>::new();
    if let Some(netkeiba_code) = get_netkeiba_race_code_from_umanity_code(&umanity_code) {
        let netkeiba_url = format!(
            "https://race.netkeiba.com/race/shutuba.html?race_id={netkeiba_code_param}",
            netkeiba_code_param = &netkeiba_code
        );
        match external_info_common_service::get_html_from_url(&netkeiba_url, Some("euc-jp")).await {
            Ok(net_keiba_html) => {
                netkeiba_horse_info_map = get_netkeiba_info(&net_keiba_html);
            }
            _ => {}
        };
    }
    netkeiba_horse_info_map
}

// ウマニティのコードからnetkeibaのコードに変換
fn get_netkeiba_race_code_from_umanity_code(umanity_code: &String) -> Option<String> {
    match ((&umanity_code).get(0..4), (&umanity_code).get(8..16)) {
        (Some(date), Some(race_code)) => Some(date.to_string() + race_code),
        _ => None,
    }
}

// net競馬の出馬表htmlを馬名をキーにMap形式で取得
fn get_netkeiba_info(html_text: &String) -> HashMap<String, prompt_def::PromptHorseInfo> {
    let mut netkeiba_horse_info_map = HashMap::<String, prompt_def::PromptHorseInfo>::new();
    let doc: scraper::Html = scraper::Html::parse_document(html_text);
    for horse_tr_elem in doc
        .select(&scraper::Selector::parse("table.Shutuba_Table.RaceTable01 tr.HorseList").unwrap())
    {
        let mut horse_info = prompt_def::PromptHorseInfo::new();

        // 馬の名前を取得
        if let Some(name_elem) = horse_tr_elem
            .select(&scraper::Selector::parse("td.HorseInfo .HorseName").unwrap())
            .next()
        {
            horse_info.name = name_elem.text().collect::<Vec<_>>()[0].trim().to_string();
        }

        // 騎手を取得
        if let Some(jockey_elem) = horse_tr_elem
            .select(&scraper::Selector::parse("td.Jockey a").unwrap())
            .next()
        {
            let jockey = jockey_elem.text().collect::<Vec<_>>()[0].trim();
            if jockey.len() > 0 {
                horse_info.jockey = Some(jockey.to_string())
            }
        }

        // 馬体重(増減)を取得
        if let Some(horse_weight_elem) = horse_tr_elem
            .select(&scraper::Selector::parse("td.Weight").unwrap())
            .next()
        {
            let horse_weight = horse_weight_elem
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();
            if horse_weight.len() > 0 {
                horse_info.horse_weight = Some(horse_weight)
            }
        }

        if (&horse_info).name != prompt_def::HYPHEN {
            netkeiba_horse_info_map.insert(horse_info.clone().name, horse_info);
        }
    }
    netkeiba_horse_info_map
}
