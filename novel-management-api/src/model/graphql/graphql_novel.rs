use async_graphql::*;

#[derive(SimpleObject)]
pub struct NovelResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}
