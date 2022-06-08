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
    pub event_time: String,
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
                        event_time: event_time.to_string(),
                        update_time: update_time,
                    })
                }
            }
        }
        return Ok(result_vec);
    }
}
