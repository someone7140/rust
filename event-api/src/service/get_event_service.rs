use crate::model::api::event_info_master_response::{
    EventInfoMasterResponse, EventInfoMasterResponseKV,
};
use crate::model::db::event_collection::EventCollection;
use crate::repository::event_repository;
use std::error::Error;

pub fn get_event_info_master() -> EventInfoMasterResponse {
    return EventInfoMasterResponse {
        locations: vec![EventInfoMasterResponseKV {
            key: "tokyo".to_string(),
            label: "東京".to_string(),
        }],
        sites: vec![
            EventInfoMasterResponseKV {
                key: "tunagate".to_string(),
                label: "つなげーと".to_string(),
            },
            EventInfoMasterResponseKV {
                key: "jmty".to_string(),
                label: "ジモティー".to_string(),
            },
            EventInfoMasterResponseKV {
                key: "koryupa".to_string(),
                label: "コリュパ".to_string(),
            },
            EventInfoMasterResponseKV {
                key: "kokuchpro".to_string(),
                label: "こくちーずプロ".to_string(),
            },
            EventInfoMasterResponseKV {
                key: "twipla".to_string(),
                label: "TwiPla".to_string(),
            },
        ],
    };
}

pub async fn get_event_info_list(
    location_key: String,
    event_date: String,
) -> Result<Vec<EventCollection>, Box<dyn Error>> {
    let results = event_repository::get_events(location_key, event_date)?;
    return Ok(results);
}
