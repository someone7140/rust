use chrono::{DateTime, FixedOffset, TimeZone, Utc};

// 日本のタイムゾーンでFixedOffsetを設定して現在日時を取得
pub fn get_now_jst_datetime_fixed_offset() -> DateTime<FixedOffset> {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    jst.from_utc_datetime(&Utc::now().naive_utc())
}
