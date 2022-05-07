use serde::Serialize;

#[derive(Serialize)]
pub struct SearchConditionResponse {
    pub store_type: Vec<SearchConditionResponseKV>,
    pub location: Vec<SearchConditionResponseKV>,
}

#[derive(Serialize)]
pub struct SearchConditionResponseKV {
    pub key: String,
    pub value: String,
}
