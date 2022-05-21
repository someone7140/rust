use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCollection {
    pub _id: String,
    pub name: String,
    pub loc: Vec<f64>,
    pub sort_order: i32,
    pub update_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreTypeCollection {
    pub _id: String,
    pub name: String,
    pub sort_order: i32,
}
