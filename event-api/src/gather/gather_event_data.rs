use crate::model::db::event_collection::EventCollection;
use crate::model::db::event_info_collection::EventSearchMasterCollection;
use crate::util::date_util;
use chrono::{DateTime, Datelike};
use chrono_tz::Tz;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
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
    let future_koryupa = koryupa_gather(
        event_search_master._id.clone(),
        event_search_master.koryupa_key,
        event_date.clone(),
        event_date_time,
        update_time,
    );
    let future_kokuchpro = kokuchpro_gather(
        event_search_master._id.clone(),
        event_search_master.kokuchpro_key,
        event_date.clone(),
        event_date_time,
        update_time,
    );
    let future_twipla = twipla_gather(
        event_search_master._id.clone(),
        event_search_master.twipla_key,
        event_date.clone(),
        event_date_time,
        update_time,
    );
    // 結果取得
    let (result_tunagate, result_jmty, result_koryupa, result_kokuchpro, result_twipla) = futures::join!(
        future_tunagate,
        future_jmty,
        future_koryupa,
        future_kokuchpro,
        future_twipla
    );
    let (
        mut events_tunagate,
        mut events_jmty,
        mut events_koryupa,
        mut events_kokuchpro,
        mut events_twipla,
    ) = (
        result_tunagate?,
        result_jmty?,
        result_koryupa?,
        result_kokuchpro?,
        result_twipla?,
    );
    let mut result_vec: Vec<EventCollection> = Vec::new();
    result_vec.append(&mut events_tunagate);
    result_vec.append(&mut events_jmty);
    result_vec.append(&mut events_koryupa);
    result_vec.append(&mut events_kokuchpro);
    result_vec.append(&mut events_twipla);
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
    macro_rules! base_url {
        () => {
            "https://jmty.jp/{search_key}/com?keyword={event_date}"
        };
    }
    let event_date_time_ref = event_date_time;
    // 日付を日本語形式で指定
    let encoded_date_jp = utf8_percent_encode(
        &format!(
            "{month}月{day}日",
            month = event_date_time_ref.month(),
            day = event_date_time_ref.day()
        ),
        NON_ALPHANUMERIC,
    )
    .to_string();
    let url_jp_encoded = format!(
        base_url!(),
        search_key = search_key.clone(),
        event_date = encoded_date_jp
    );
    let resp_jp = reqwest::get(url_jp_encoded).await?;
    let html_url_jp = resp_jp.text().await?;
    let mut gather_events = EventCollection::from_jmty_html(
        html_url_jp,
        location_key.clone(),
        event_date.clone(),
        update_time,
    )?;
    // 日付をスラッシュ形式で指定
    let encoded_date_slash = utf8_percent_encode(
        &format!(
            "{month}/{day}",
            month = event_date_time_ref.month(),
            day = event_date_time_ref.day()
        ),
        NON_ALPHANUMERIC,
    )
    .to_string();
    let url_slash_encoded = format!(
        base_url!(),
        search_key = search_key.clone(),
        event_date = encoded_date_slash
    );
    let resp_slash = reqwest::get(url_slash_encoded).await?;
    let html_url_slash = resp_slash.text().await?;
    let mut gather_events_url_slash = EventCollection::from_jmty_html(
        html_url_slash,
        location_key.clone(),
        event_date.clone(),
        update_time,
    )?;
    gather_events.append(&mut gather_events_url_slash);
    // 結果に追加
    for gather_event in gather_events {
        let result_vec_clone = result_vec.clone();
        if !result_vec_clone
            .iter()
            .any(|r| r.site_event_id == gather_event.site_event_id)
        {
            result_vec.push(gather_event);
        }
    }
    return Ok(result_vec);
}

async fn koryupa_gather(
    location_key: String,
    search_key: String,
    event_date: String,
    event_date_time: DateTime<Tz>,
    update_time: i64,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let url = format!(
        "https://koryupa.jp/events/get_of_day/ymd:{event_date}/prf1:{search_key}",
        search_key = search_key,
        event_date = date_util::format_jst_date(event_date_time.clone(), "%Y%m%d"),
    );
    let resp = reqwest::get(url).await?;
    let html = resp.text().await?;
    let gather_events = EventCollection::from_koryupa_html(
        html,
        location_key.clone(),
        event_date.clone(),
        update_time,
    )?;
    return Ok(gather_events);
}

async fn kokuchpro_gather(
    location_key: String,
    search_key: String,
    event_date: String,
    event_date_time: DateTime<Tz>,
    update_time: i64,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let mut result_vec: Vec<EventCollection> = Vec::new();
    macro_rules! base_url {
        () => {
            "https://www.kokuchpro.com/s/{search_key}/date-{event_date}/?page={page}"
        };
    }
    let mut page = 1;
    let search_key_encoded = utf8_percent_encode(&search_key, NON_ALPHANUMERIC).to_string();

    loop {
        let url = format!(
            base_url!(),
            search_key = search_key_encoded,
            event_date = date_util::format_jst_date(event_date_time, "%Y%m%d"),
            page = page
        );
        let resp = reqwest::get(url).await?;
        let html = resp.text().await?;
        let result = EventCollection::from_kokuchpro_html(
            html,
            location_key.clone(),
            event_date.clone(),
            event_date_time,
            update_time,
        )?;
        let gather_events_refer = &result.0;
        let next_flag = &result.1;
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
        if !next_flag {
            break;
        }
        page = page + 1
    }
    return Ok(result_vec);
}

async fn twipla_gather(
    location_key: String,
    search_key: String,
    event_date: String,
    event_date_time: DateTime<Tz>,
    update_time: i64,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let mut result_vec: Vec<EventCollection> = Vec::new();
    macro_rules! base_url {
        () => {
            "https://twipla.jp/events/search/page~{page}/keyword~{search_key}/date~{event_date}"
        };
    }
    let mut page = 1;
    let search_key_encoded = utf8_percent_encode(&search_key, NON_ALPHANUMERIC).to_string();

    loop {
        let url = format!(
            base_url!(),
            search_key = search_key_encoded,
            event_date = event_date,
            page = page
        );
        let resp = reqwest::get(url).await?;
        let html = resp.text().await?;
        let result = EventCollection::from_twipla_html(
            html,
            location_key.clone(),
            event_date.clone(),
            event_date_time,
            update_time,
        )?;
        let gather_events_refer = &result.0;
        let next_flag = &result.1;
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
        if !next_flag {
            break;
        }
        page = page + 1
    }
    return Ok(result_vec);
}
