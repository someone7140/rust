use async_graphql::*;
use scraper::ElementRef;
use url::Url;

use crate::graphql_object::horse_enum::ErrorType;

// urlに指定されたコードから日付とコードを取得
pub fn get_race_code_and_date_from_url_code(umanity_url: String) -> Result<(String, String)> {
    // urlからコードを取得
    let race_code = match Url::parse(&umanity_url).and_then(|u| {
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
pub fn get_race_name_from_html(html_text: String) -> Result<String> {
    let doc = scraper::Html::parse_document(&html_text);
    let race_info_selector = match scraper::Selector::parse(".race_info .detail") {
        Ok(s) => s,
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    };

    let mut race_name = "".to_string();

    if let Some(title_root) = doc.select(&race_info_selector).next() {
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
