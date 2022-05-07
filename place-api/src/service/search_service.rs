use crate::model::api::search_condition_response::SearchConditionResponse;
use crate::model::api::search_condition_response::SearchConditionResponseKV;

pub fn get_search_condition_response() -> SearchConditionResponse {
    return SearchConditionResponse {
        store_type: vec![SearchConditionResponseKV {
            key: "bar".to_string(),
            value: "バー".to_string(),
        }],
        location: vec![SearchConditionResponseKV {
            key: "ikebukuro_tokyo".to_string(),
            value: "池袋（東京）".to_string(),
        }],
    };
}
