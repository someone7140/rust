use crate::model::api::search_condition_response::{
    SearchConditionResponse, SearchConditionResponseKV,
};
use crate::repository::search_condition_repository::{get_location_list, get_store_type_list};
use std::error::Error;

pub fn get_search_condition_response() -> Result<SearchConditionResponse, Box<dyn Error>> {
    let location_list = get_location_list()?
        .iter()
        .map(|l| SearchConditionResponseKV {
            key: l._id.clone(),
            value: l.name.clone(),
        })
        .collect();
    let store_type_list = get_store_type_list()?
        .iter()
        .map(|l| SearchConditionResponseKV {
            key: l._id.clone(),
            value: l.name.clone(),
        })
        .collect();
    return Ok(SearchConditionResponse {
        location: location_list,
        store_type: store_type_list,
    });
}
