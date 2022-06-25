use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::{Asia::Tokyo, Tz};

use std::error::Error;

pub const DATE_SEC: i64 = 60 * 60 * 24;

// 日本時間当日を0時0分0秒で返す
pub fn get_now_jst_date() -> DateTime<Tz> {
    let utc = Utc::now().naive_utc();
    let jst = Tokyo.from_utc_datetime(&utc);
    return Tokyo
        .ymd(jst.year(), jst.month(), jst.day())
        .and_hms(0, 0, 0);
}

// 日本時間当日
pub fn get_now_jst_date_time() -> DateTime<Tz> {
    let utc = Utc::now().naive_utc();
    return Tokyo.from_utc_datetime(&utc);
}

// 日付の文字列をDateにパース
pub fn parse_str_jst_date(jst_str_date: String) -> Result<DateTime<Tz>, Box<dyn Error>> {
    let naive_date = NaiveDate::parse_from_str(&jst_str_date, "%Y-%m-%d")?;
    let japan_date_time = Tokyo
        .ymd(naive_date.year(), naive_date.month(), naive_date.day())
        .and_hms(0, 0, 0);
    return Ok(japan_date_time);
}

// 日付を文字列にフォーマット
pub fn format_jst_date(jst_date_time: DateTime<Tz>, format_str: &str) -> String {
    return jst_date_time.format(format_str).to_string();
}
