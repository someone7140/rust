use async_graphql::*;

#[derive(InputObject)]
pub struct NovelSettingRegisterInput {
    pub id: Option<String>,
    #[graphql(validator(min_length = 1))]
    pub name: String,
    #[graphql(validator(min_length = 1))]
    pub novel_id: String,
    pub parent_setting_id: Option<String>,
    pub display_order: Option<i32>,
    pub attributes: Vec<String>,
    pub description: Option<String>,
}

#[derive(SimpleObject)]
pub struct NovelSettingResponse {
    pub id: String,
    pub name: String,
    pub novel_id: String,
    pub parent_setting_id: Option<String>,
    pub display_order: Option<i32>,
    pub attributes: Vec<String>,
    pub description: Option<String>,
}
