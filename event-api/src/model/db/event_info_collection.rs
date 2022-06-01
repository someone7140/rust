use crate::util::date_util;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventUpdateHistoryCollection {
    pub location_key: String,
    pub event_date: String,
    pub update_time: i64,
}

pub trait EventUpdateHistoryTrait {
    fn is_update_target(&self, target_time: i64) -> bool;
    fn is_delete_target(&self, target_time: i64) -> bool;
}

impl EventUpdateHistoryTrait for EventUpdateHistoryCollection {
    fn is_update_target(&self, target_time: i64) -> bool {
        // 現在時時刻から2日以内の更新なら対象外
        let now_time = date_util::get_now_jst_date_time().timestamp();
        if now_time - self.update_time < date_util::DATE_SEC * 2 {
            return false;
        }

        // target_time以降
        return match date_util::parse_str_jst_date(self.event_date.clone()) {
            Ok(r) => r.timestamp() >= target_time,
            Err(_e) => false,
        };
    }
    fn is_delete_target(&self, target_time: i64) -> bool {
        // target_timeより前
        return match date_util::parse_str_jst_date(self.event_date.clone()) {
            Ok(r) => r.timestamp() < target_time,
            Err(_e) => false,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSearchMasterCollection {
    pub _id: String,
    pub tunagate_key: String,
    pub jmty_key: String,
    pub koryupa_key: String,
    pub kokuchpro_key: String,
}
