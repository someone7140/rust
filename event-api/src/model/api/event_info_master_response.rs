use serde::Serialize;

#[derive(Serialize)]
pub struct EventInfoMasterResponse {
    pub locations: Vec<EventInfoMasterResponseKV>,
    pub sites: Vec<EventInfoMasterResponseKV>,
}

#[derive(Serialize)]
pub struct EventInfoMasterResponseKV {
    pub key: String,
    pub label: String,
}
