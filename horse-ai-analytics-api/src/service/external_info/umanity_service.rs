use std::collections::HashMap;

use async_graphql::*;
use regex::Regex;
use scraper::ElementRef;
use url::Url;

use crate::{
    graphql_object::{horse_enum::ErrorType, horse_model},
    struct_const_def::prompt_def,
};

use super::external_info_common_service;

// urlに指定されたコードから日付とコードを取得
pub fn get_race_code_and_date_from_url_code(umanity_url: &String) -> Result<(String, String)> {
    // urlからコードを取得
    let race_code = match Url::parse(umanity_url).and_then(|u| {
        Ok(u.query_pairs()
            .find(|(k, _)| k == "code")
            .and_then(|param| Some(param.1.to_string())))
    }) {
        Ok(Some(code)) => code,
        _ => {
            return Err(Error::new("Can not get param")
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    };

    // 日付の文字列
    match (
        (&race_code).get(0..4),
        (&race_code).get(4..6),
        (&race_code).get(6..8),
    ) {
        (Some(y), Some(m), Some(d)) => Ok((
            race_code.clone(),
            (y.to_string() + "/" + m + "/" + d).to_string(),
        )),
        _ => {
            Err(Error::new("Parse error").extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    }
}

// htmlからレース名を取得
pub fn get_race_name_from_html(html_text: &String) -> Result<String> {
    let doc = scraper::Html::parse_document(html_text);

    let mut race_name = "".to_string();

    if let Some(title_root) = doc
        .select(&scraper::Selector::parse(".race_info .detail").unwrap())
        .next()
    {
        for title_elem in title_root.child_elements() {
            if title_elem.value().name() == "h2" {
                race_name = title_elem.text().collect::<Vec<_>>()[0].trim().to_string() + &race_name
            } else {
                let mut text_index = 0;
                if let Some(first_child) = title_elem.child_elements().next() {
                    if first_child.value().name() == "time" {
                        text_index = 3;
                    }
                }
                let text_vec = title_elem.text().collect::<Vec<_>>();
                if text_vec.clone().len() > text_index {
                    race_name = race_name + " " + text_vec[text_index].trim()
                }
            }
        }
    }

    Ok(race_name.to_string())
}

// race_7のページから近走成績を取得
pub async fn get_recent_results_from_race_7(race_code: &String) -> HashMap<String, String> {
    let mut recent_results = HashMap::new();
    let url = format!(
        "https://umanity.jp/racedata/race_7.php?t=1&code={race_code_param}",
        race_code_param = race_code
    );
    // 近走成績のurlからhtmlを取得
    let html = match external_info_common_service::get_html_from_url(&url, None).await {
        Ok(text) => text,
        Err(_) => return recent_results,
    };
    let doc = scraper::Html::parse_document(&html);

    for horse_tr_elem in
        doc.select(&scraper::Selector::parse("table#grace_table1 tbody tr").unwrap())
    {
        let td_list: Vec<ElementRef> = horse_tr_elem.child_elements().collect();
        // 馬のコード
        if let Some(name_and_id_elem) = td_list[2]
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
        {
            if let Some(href) = name_and_id_elem.value().attr("href") {
                let horse_code = get_horse_id_from_url(&("https:".to_string() + href));
                let mut recent_result_vec: Vec<String> = Vec::new();
                // 1走前
                let before1 = get_recent_results_td_from_race_7(td_list[7]);
                if (&before1).len() > 0 {
                    recent_result_vec.push(before1)
                }
                // 2走前
                let before2 = get_recent_results_td_from_race_7(td_list[8]);
                if (&before2).len() > 0 {
                    recent_result_vec.push(before2)
                }
                // 3走前
                let before3 = get_recent_results_td_from_race_7(td_list[9]);
                if (&before3).len() > 0 {
                    recent_result_vec.push(before3)
                }
                recent_results.insert(horse_code, recent_result_vec.join(" | "));
            }
        }
    }

    recent_results
}

// race_7のページから出馬情報を取得
pub fn get_horse_info_from_race_7(html_text: &String) -> Result<Vec<prompt_def::PromptHorseInfo>> {
    let mut horse_info_list: Vec<prompt_def::PromptHorseInfo> = Vec::new();

    let doc: scraper::Html = scraper::Html::parse_document(html_text);
    for horse_tr_elem in
        doc.select(&scraper::Selector::parse("table#grace_table1 tbody tr").unwrap())
    {
        let mut horse_info = prompt_def::PromptHorseInfo::new();

        let td_list: Vec<ElementRef> = horse_tr_elem.child_elements().collect();

        // 馬の名前とidを取得
        if let Some(name_and_id_elem) = td_list[2]
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
        {
            horse_info.name = name_and_id_elem.text().collect::<Vec<_>>()[0]
                .trim()
                .to_string();

            if let Some(href) = name_and_id_elem.value().attr("href") {
                horse_info.umanity_code = get_horse_id_from_url(&("https:".to_string() + href))
            }
        }
        // 性齢を取得
        horse_info.gender_and_age = (td_list[3].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 負担重量を取得
        horse_info.charge_weight = td_list[4].text().collect::<Vec<_>>()[0].trim().to_string();
        // 調教師を取得
        horse_info.trainer = (td_list[5].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 所属を取得
        horse_info.belonging = (td_list[6].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 戦績を取得
        horse_info.all_results = (td_list[7].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 獲得賞金の合計を取得
        let re = Regex::new(r",").unwrap();
        horse_info.career_prize_money = re
            .replace_all(td_list[8].text().collect::<Vec<_>>()[0].trim(), "")
            .parse()
            .unwrap();
        // 父を取得
        horse_info.father = (td_list[9].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 母を取得
        horse_info.mother = (td_list[10].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 母父を取得
        horse_info.mother_father = (td_list[11].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();

        if horse_info.name != prompt_def::HYPHEN {
            horse_info_list.push(horse_info)
        }
    }

    Ok(horse_info_list)
}

// race_8_1のページから出馬情報を取得
pub async fn get_horse_info_from_race_8_1(
    html_text: &String,
) -> Result<Vec<prompt_def::PromptHorseInfo>> {
    let mut horse_info_list: Vec<prompt_def::PromptHorseInfo> = Vec::new();

    let doc: scraper::Html = scraper::Html::parse_document(html_text);
    for horse_tr_elem in
        doc.select(&scraper::Selector::parse("table tr.odd-row,table tr.even-row").unwrap())
    {
        let mut horse_info = prompt_def::PromptHorseInfo::new();

        let td_list: Vec<ElementRef> = horse_tr_elem.child_elements().collect();

        // 枠番
        if let Ok(waku) = (td_list[0].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string()
            .parse::<i32>()
        {
            horse_info.waku_num = Some(waku);
        }
        // 馬番
        if let Ok(uma_num) = (td_list[0].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string()
            .parse::<i32>()
        {
            horse_info.uma_num = Some(uma_num);
        }
        // 馬の名前とidを取得
        if let Some(name_and_id_elem) = td_list[4]
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
        {
            horse_info.name = name_and_id_elem.text().collect::<Vec<_>>()[0]
                .trim()
                .to_string();

            if let Some(href) = name_and_id_elem.value().attr("href") {
                horse_info.umanity_code = get_horse_id_from_url(&("https:".to_string() + href))
            }
        }
        // 性齢を取得
        horse_info.gender_and_age = (td_list[5].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 負担重量を取得
        horse_info.charge_weight = td_list[6].text().collect::<Vec<_>>()[0].trim().to_string();
        // 調教師を取得
        horse_info.trainer = (td_list[8].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();
        // 所属を取得
        horse_info.belonging = (td_list[9].text().collect::<Vec<_>>()[0])
            .trim()
            .to_string();

        if horse_info.name != prompt_def::HYPHEN {
            horse_info_list.push(horse_info)
        }
    }

    Ok(horse_info_list)
}

// race_8_9のページからオッズ情報を取得
pub async fn get_odds_info_from_race_8_9(
    race_code: &String,
) -> Option<horse_model::OddsInfoResponse> {
    let mut odds_info_list: Vec<horse_model::OddsInfo> = Vec::new();
    let url = format!(
        "https://umanity.jp/racedata/race_8_9.php?code={race_code_param}",
        race_code_param = race_code
    );
    let html = match external_info_common_service::get_html_from_url(&url, None).await {
        Ok(text) => text,
        Err(_) => return None,
    };
    let doc = scraper::Html::parse_document(&html);
    for horse_tr_elem in
        doc.select(&scraper::Selector::parse("table tr.odd-row,table tr.even-row").unwrap())
    {
        let td_list: Vec<ElementRef> = horse_tr_elem.child_elements().collect();

        // 馬の名前
        let name_opt = td_list[4]
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|elem| Some(elem.text().collect::<Vec<_>>()[0].trim().to_string()));
        match name_opt {
            Some(name) => {
                // オッズ
                let odds = td_list[5].text().collect::<Vec<_>>()[0].trim().to_string();
                odds_info_list.push(horse_model::OddsInfo {
                    horse_name: name,
                    odds,
                })
            }
            _ => {}
        }
    }

    if (&odds_info_list).len() < 1 {
        return None;
    }
    odds_info_list.sort_by(|a, b| {
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

    Some(horse_model::OddsInfoResponse {
        odds_url: format!(
            "https://umanity.jp/racedata/race_mark.php?code={race_code_param}",
            race_code_param = race_code
        ),
        odds_list: odds_info_list,
    })
}

// 馬のurlからidを取得
pub fn get_horse_id_from_url(url: &String) -> String {
    match Url::parse(url).and_then(|u| {
        Ok(u.query_pairs()
            .find(|(k, _)| k == "code")
            .and_then(|param| Some(param.clone().1.to_string())))
    }) {
        Ok(Some(code)) => code,
        _ => prompt_def::HYPHEN.to_string(),
    }
}

// race_7のテーブルブロックから近走成績を取得
fn get_recent_results_td_from_race_7(td_elem: ElementRef) -> String {
    let mut recent_result_vec: Vec<String> = Vec::new();
    for recent_result_elem in td_elem.select(&scraper::Selector::parse("div").unwrap()) {
        let texts = recent_result_elem.text().collect::<Vec<_>>();
        if texts.len() > 0 {
            recent_result_vec.push(texts[0].to_string())
        }
    }
    recent_result_vec.join("-")
}
