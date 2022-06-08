use crate::model::db::event_collection::EventCollection;
use crate::model::db::event_info_collection::EventSearchMasterCollection;
use crate::util::date_util;
use chrono::DateTime;
use chrono_tz::Tz;
use futures::future;
use std::error::Error;
use std::{thread, time};

pub async fn get_event_data(
    event_search_master: EventSearchMasterCollection,
    event_date: String,
    update_time: i64,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let event_date_time = date_util::parse_str_jst_date(event_date.clone())?;
    // 各サイトの収集を並行で行う
    let future_tunagate = tunagate_gather(
        event_search_master._id.clone(),
        event_search_master.tunagate_key,
        event_date.clone(),
        update_time,
    );
    let future_jmty = jmty_gather(
        event_search_master._id.clone(),
        event_search_master.jmty_key,
        event_date.clone(),
        event_date_time,
        update_time,
    );
    // 結果取得
    let (result_tunagate, result_jmty) = future::join(future_tunagate, future_jmty).await;
    let (mut events_tunagate, mut events_jmty) = (result_tunagate?, result_jmty?);
    let mut result_vec: Vec<EventCollection> = Vec::new();
    result_vec.append(&mut events_tunagate);
    result_vec.append(&mut events_jmty);

    return Ok(result_vec);
}

async fn tunagate_gather(
    location_key: String,
    search_key: String,
    event_date: String,
    update_time: i64,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let mut result_vec: Vec<EventCollection> = Vec::new();
    macro_rules! base_url {() => ("https://tunagate.com/api/circle/search?pref_key={search_key}&event_date={event_date}&page={page}")}
    let mut page = 1;
    let event_date_ref = &event_date;

    loop {
        let url = format!(
            base_url!(),
            search_key = search_key,
            event_date = event_date_ref,
            page = page
        );
        let resp = reqwest::get(url).await?;
        let json_str = resp.text().await?;
        let gather_events = EventCollection::from_tunagate_json(
            json_str,
            location_key.clone(),
            event_date_ref.clone(),
            update_time,
        )?;
        let gather_events_refer = &gather_events;
        if gather_events_refer.len() == 0 {
            break;
        } else {
            // keyが無ければvecに追加
            let result_vec_clone = result_vec.clone();
            let add_events_filtered = gather_events_refer.iter().filter(|g| {
                !result_vec_clone
                    .iter()
                    .any(|r| r.site_event_id == g.site_event_id)
            });
            for add_event in add_events_filtered {
                result_vec.push(add_event.clone());
            }
            thread::sleep(time::Duration::from_millis(500));
            page = page + 1
        }
    }
    return Ok(result_vec);
}

async fn jmty_gather(
    location_key: String,
    search_key: String,
    event_date: String,
    event_date_time: DateTime<Tz>,
    update_time: i64,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let mut result_vec: Vec<EventCollection> = Vec::new();
    return Ok(result_vec);
}
