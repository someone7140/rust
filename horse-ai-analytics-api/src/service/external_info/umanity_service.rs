use async_graphql::*;
use scraper::ElementRef;
use url::Url;

use crate::{graphql_object::horse_enum::ErrorType, struct_const_def::prompt_def};

// urlに指定されたコードから日付とコードを取得
pub fn get_race_code_and_date_from_url_code(umanity_url: &String) -> Result<(String, String)> {
    // urlからコードを取得
    let race_code = match Url::parse(umanity_url).and_then(|u| {
        Ok(u.query_pairs()
            .find(|(k, _)| k == "code")
            .and_then(|param| Some(param.clone().1.to_string())))
    }) {
        Ok(Some(code)) => code,
        _ => {
            return Err(Error::new("Can not get param")
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    };

    // 日付の文字列
    match (
        race_code.clone().get(0..4),
        race_code.clone().get(4..6),
        race_code.clone().get(6..8),
    ) {
        (Some(y), Some(m), Some(d)) => {
            Ok((race_code, (y.to_string() + "/" + m + "/" + d).to_string()))
        }
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
        horse_info.gender_and_age = (td_list[3].text().collect::<Vec<_>>()[0]).to_string();
        // 調教師を取得
        horse_info.trainer = (td_list[5].text().collect::<Vec<_>>()[0]).to_string();
        // 所属を取得
        horse_info.belonging = (td_list[6].text().collect::<Vec<_>>()[0]).to_string();

        if horse_info.name != prompt_def::HYPHEN {
            horse_info_list.push(horse_info)
        }
    }

    Ok(horse_info_list)
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
