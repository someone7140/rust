use crate::util::date_util;
use chrono::{DateTime, Datelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventCollection {
    pub site_id: String,
    pub site_event_id: String,
    pub location_key: String,
    pub title: String,
    pub url: String,
    pub event_date: String,
    pub event_time: Option<String>,
    pub update_time: i64,
}

impl EventCollection {
    pub fn from_tunagate_json(
        json: String,
        location_key: String,
        event_date: String,
        update_time: i64,
    ) -> Result<Vec<EventCollection>, Box<dyn Error>> {
        let mut result_vec: Vec<EventCollection> = Vec::new();
        let v: serde_json::Value = serde_json::from_str(&json)?;

        let empty_vec: Vec<serde_json::Value> = Vec::new();
        let circles = v["circles"].as_array().unwrap_or_else(|| &empty_vec);
        for circle in circles {
            let circle_id = circle["id"].as_i64().unwrap_or_else(|| -1);
            if circle_id >= 0 {
                let events = circle["events"].as_array().unwrap_or_else(|| &empty_vec);
                for event in events {
                    // イベントID
                    let site_event_id = event["id"].as_i64().unwrap_or_else(|| -1);
                    if site_event_id < 0 {
                        continue;
                    }
                    // URL
                    let event_url = format!(
                        "https://tunagate.com/circle/{circle_id}/events/{site_event_id}",
                        circle_id = circle_id,
                        site_event_id = site_event_id
                    );
                    // イベントタイトル
                    let event_title = event["title"].as_str().unwrap_or_else(|| "");
                    // 時間
                    let event_date_formatted =
                        event["event_date_formatted"].as_str().unwrap_or_else(|| "");
                    let event_date_formatted_arr: Vec<&str> =
                        event_date_formatted.split(" ").collect();
                    let event_time = if event_date_formatted_arr.clone().len() < 2 {
                        ""
                    } else {
                        event_date_formatted_arr[1]
                    };
                    // 結果をVecに追加
                    result_vec.push(EventCollection {
                        site_id: "tunagate".to_string(),
                        site_event_id: site_event_id.to_string(),
                        location_key: location_key.clone(),
                        title: event_title.to_string(),
                        url: event_url,
                        event_date: event_date.clone(),
                        event_time: Some(event_time.to_string()),
                        update_time: update_time,
                    })
                }
            }
        }
        return Ok(result_vec);
    }

    pub fn from_jmty_html(
        html: String,
        location_key: String,
        event_date: String,
        update_time: i64,
    ) -> Result<Vec<EventCollection>, Box<dyn Error>> {
        let mut result_vec: Vec<EventCollection> = Vec::new();
        let doc = scraper::Html::parse_document(&html);
        let title_a_tag = scraper::Selector::parse(
            "li.p-articles-list-item div.p-item-content-info div.p-item-title a",
        )
        .unwrap();
        for node in doc.select(&title_a_tag) {
            let node_ref = &node;
            if let Some(href) = node_ref.value().attr("href") {
                let href_split: Vec<&str> = href.clone().split("/").collect();
                let href_split_size = href_split.len();
                let site_event_id = href_split[href_split_size - 2].to_string()
                    + "/"
                    + href_split[href_split_size - 1];
                let event_title = node_ref.text().collect::<Vec<_>>()[0].trim();
                // 結果をVecに追加
                result_vec.push(EventCollection {
                    site_id: "jmty".to_string(),
                    site_event_id: site_event_id.to_string(),
                    location_key: location_key.clone(),
                    title: event_title.to_string(),
                    url: href.to_string(),
                    event_date: event_date.clone(),
                    event_time: None,
                    update_time: update_time,
                })
            }
        }
        return Ok(result_vec);
    }

    pub fn from_koryupa_html(
        html: String,
        location_key: String,
        event_date: String,
        update_time: i64,
    ) -> Result<Vec<EventCollection>, Box<dyn Error>> {
        let mut result_vec: Vec<EventCollection> = Vec::new();
        let doc = scraper::Html::parse_document(&html);
        let a_tag = scraper::Selector::parse("a.event_image_block").unwrap();
        for a_node in doc.select(&a_tag) {
            let a_node_ref = &a_node;
            if let Some(href) = a_node_ref.value().attr("href") {
                // URL
                let url = "https://koryupa.jp".to_string() + href.clone();
                // サイトID
                let href_split: Vec<&str> = href.clone().split("/").collect();
                let href_split_size = href_split.len();
                let site_event_id = href_split[href_split_size - 2];
                // タイトル
                let href_child_doc = scraper::Html::parse_document(&a_node_ref.inner_html());
                let title_tag = scraper::Selector::parse("p.title").unwrap();
                if let Some(title) = href_child_doc.select(&title_tag).next() {
                    let event_title = title.text().collect::<Vec<_>>()[0].trim();
                    // 結果をVecに追加
                    result_vec.push(EventCollection {
                        site_id: "koryupa".to_string(),
                        site_event_id: site_event_id.to_string(),
                        location_key: location_key.clone(),
                        title: event_title.to_string(),
                        url: url,
                        event_date: event_date.clone(),
                        event_time: None,
                        update_time: update_time,
                    })
                }
            }
        }
        return Ok(result_vec);
    }

    pub fn from_kokuchpro_html(
        html: String,
        location_key: String,
        event_date: String,
        event_date_time: DateTime<Tz>,
        update_time: i64,
    ) -> Result<(Vec<EventCollection>, bool), Box<dyn Error>> {
        let mut result_vec: Vec<EventCollection> = Vec::new();
        let doc = scraper::Html::parse_document(&html);
        // 日付の文字列
        let event_date_formatted = format!(
            "{year}年{month}月{day}日",
            year = event_date_time.year(),
            month = event_date_time.month(),
            day = event_date_time.day()
        );
        // イベント情報の取得
        let event_info_tag = scraper::Selector::parse("div.event_info_box").unwrap();
        let event_info_select = doc.select(&event_info_tag);
        for event_info_node in event_info_select {
            let event_info_doc = scraper::Html::parse_document(&event_info_node.inner_html());
            // タイトルとURL
            let title_url_tag = scraper::Selector::parse("div.event_name_wrapper a.url").unwrap();
            if let Some(title_url_node) = event_info_doc.select(&title_url_tag).next() {
                let title_url_node_ref = &title_url_node;
                if let Some(href) = title_url_node_ref.value().attr("href") {
                    let event_title = title_url_node_ref.text().collect::<Vec<_>>()[0].trim();
                    let href_split: Vec<&str> = href.clone().split("/").collect();
                    let href_split_size = href_split.len();
                    let site_event_id = href_split[href_split_size - 3].to_string()
                        + "/"
                        + href_split[href_split_size - 2];
                    // 日付・時間
                    let title_url_tag = scraper::Selector::parse(
                        "div.event_detail_wrapper td.event_date span.dtstart",
                    )
                    .unwrap();
                    if let Some(date_node) = event_info_doc.select(&title_url_tag).next() {
                        let date_node_ref = &date_node;
                        // 時間
                        let event_time = date_node_ref.text().collect::<Vec<_>>()[1].trim();
                        // 日付
                        let date_tag = scraper::Selector::parse("a.event_date_link").unwrap();
                        if let Some(date_link) = date_node_ref.select(&date_tag).next() {
                            let date_txt_ref = date_link.text().collect::<Vec<_>>()[0].trim();
                            // 指定した日付で始まる場合はvecに追加
                            if date_txt_ref.starts_with(&event_date_formatted) {
                                // 結果をVecに追加
                                result_vec.push(EventCollection {
                                    site_id: "kokuchpro".to_string(),
                                    site_event_id: site_event_id.to_string(),
                                    location_key: location_key.clone(),
                                    title: event_title.to_string(),
                                    url: href.to_string(),
                                    event_date: event_date.clone(),
                                    event_time: Some(event_time.to_string()),
                                    update_time: update_time,
                                })
                            } else {
                                // 他の日付の場合が入ってる場合はこの時点でreturn
                                return Ok((result_vec, false));
                            }
                        }
                    }
                }
            }
        }
        if result_vec.len() == 0 {
            return Ok((result_vec, false));
        }
        return Ok((result_vec, true));
    }

    pub fn from_twipla_html(
        html: String,
        location_key: String,
        event_date: String,
        event_date_time: DateTime<Tz>,
        update_time: i64,
    ) -> Result<(Vec<EventCollection>, bool), Box<dyn Error>> {
        let mut result_vec: Vec<EventCollection> = Vec::new();
        let doc = scraper::Html::parse_document(&html);
        // 日付の文字列
        let event_date_formatted = date_util::format_jst_date(event_date_time, "%Y/%m/%d");
        // イベント情報の取得
        let event_info_tag = scraper::Selector::parse("ol.links").unwrap();
        if let Some(event_info_node) = doc.select(&event_info_tag).next() {
            let event_list_doc = scraper::Html::parse_document(&event_info_node.inner_html());
            // イベントのリストでループ
            for event_li_node in event_list_doc.select(&scraper::Selector::parse("li").unwrap()) {
                // 日付チェック
                if let Some(date_node) = event_li_node
                    .select(&scraper::Selector::parse("strong.black").unwrap())
                    .next()
                {
                    let setting_date_time = date_node.text().collect::<Vec<_>>()[0].trim();
                    // 日付が指定したもので始まる
                    if setting_date_time.starts_with(&event_date_formatted) {
                        // URL
                        if let Some(href_node) = event_li_node
                            .select(&scraper::Selector::parse("a").unwrap())
                            .next()
                        {
                            if let Some(href) = href_node.value().attr("href") {
                                // サイトID取得
                                let href_split: Vec<&str> = href.clone().split("/").collect();
                                let site_event_id = href_split[href_split.len() - 1];
                                // 時間
                                let setting_date_time_split: Vec<&str> =
                                    setting_date_time.split(" ").collect();
                                let event_time = if setting_date_time_split.len() > 1 {
                                    Some(setting_date_time_split[1].to_string())
                                } else {
                                    None
                                };
                                // タイトル
                                let event_title =
                                    event_li_node.text().collect::<Vec<_>>()[7].trim();
                                // 結果をVecに追加
                                result_vec.push(EventCollection {
                                    site_id: "twipla".to_string(),
                                    site_event_id: site_event_id.to_string(),
                                    location_key: location_key.clone(),
                                    title: event_title.to_string(),
                                    url: "https://twipla.jp".to_string() + href,
                                    event_date: event_date.clone(),
                                    event_time: event_time,
                                    update_time: update_time,
                                })
                            }
                        }
                    } else {
                        return Ok((result_vec, false));
                    }
                }
            }
        }
        if result_vec.len() == 0 {
            return Ok((result_vec, false));
        }
        return Ok((result_vec, true));
    }
}
