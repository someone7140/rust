use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "novel_settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub novel_id: String,
    pub owner_user_account_id: String,
    pub parent_setting_id: Option<String>,
    pub display_order: Option<i32>,
    pub attributes: Vec<String>,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
