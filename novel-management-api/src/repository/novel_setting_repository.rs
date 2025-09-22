use sea_orm::prelude::Expr;
use sea_orm::sea_query::NullOrdering;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, Order,
    QueryFilter, QueryOrder,
};

use entity::novel_settings;
use entity::novel_settings::Entity as NovelSettingEntity;

// 小説の設定一覧を取得
pub async fn get_novel_settings(
    db: &DatabaseConnection,
    user_account_id: String,
    novel_id: String,
) -> Vec<novel_settings::Model> {
    NovelSettingEntity::find()
        .filter(
            Condition::all()
                .add(novel_settings::Column::OwnerUserAccountId.eq(user_account_id))
                .add(novel_settings::Column::NovelId.eq(novel_id)),
        )
        .order_by_with_nulls(
            novel_settings::Column::DisplayOrder,
            Order::Asc,
            NullOrdering::Last,
        )
        .all(db)
        .await
        .unwrap()
}

// 設定の親IDをキーに取得
pub async fn get_novel_settings_by_parent_setting_id(
    db: &DatabaseConnection,
    user_account_id: String,
    parent_setting_id: String,
) -> Vec<novel_settings::Model> {
    novel_settings::Entity::find()
        .filter(
            Condition::all()
                .add(novel_settings::Column::OwnerUserAccountId.eq(user_account_id))
                .add(novel_settings::Column::ParentSettingId.eq(parent_setting_id)),
        )
        .order_by_with_nulls(
            novel_settings::Column::DisplayOrder,
            Order::Asc,
            NullOrdering::Last,
        )
        .all(db)
        .await
        .unwrap()
}

// 設定をIDをキーに取得
pub async fn get_novel_setting_by_setting_id(
    db: &DatabaseConnection,
    user_account_id: String,
    setting_id: String,
) -> Option<novel_settings::Model> {
    novel_settings::Entity::find()
        .filter(
            Condition::all()
                .add(novel_settings::Column::OwnerUserAccountId.eq(user_account_id))
                .add(novel_settings::Column::Id.eq(setting_id)),
        )
        .one(db)
        .await
        .unwrap()
}

// 設定を追加
pub async fn add_settings(
    db: &DatabaseConnection,
    settings: Vec<novel_settings::ActiveModel>,
) -> Option<DbErr> {
    let result = novel_settings::Entity::insert_many(settings).exec(db).await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// 設定を更新
pub async fn update_settings(
    db: &DatabaseConnection,
    settings: Vec<novel_settings::Model>,
) -> Option<DbErr> {
    for setting in settings {
        if let Err(error) = novel_settings::Entity::update_many()
            .filter(
                Condition::all()
                    .add(novel_settings::Column::Id.eq(setting.id))
                    .add(
                        novel_settings::Column::OwnerUserAccountId
                            .eq(setting.owner_user_account_id),
                    ),
            )
            .col_expr(novel_settings::Column::Name, Expr::value(setting.name))
            .col_expr(
                novel_settings::Column::ParentSettingId,
                Expr::value(setting.parent_setting_id),
            )
            .col_expr(
                novel_settings::Column::Description,
                Expr::value(setting.description),
            )
            .col_expr(
                novel_settings::Column::DisplayOrder,
                Expr::value(setting.display_order),
            )
            .col_expr(
                novel_settings::Column::Attributes,
                Expr::value(setting.attributes),
            )
            .exec(db)
            .await
        {
            return Some(error);
        }
    }
    None
}

// 設定を削除
pub async fn delete_novel_setting(
    db: &DatabaseConnection,
    novel_setting: novel_settings::ActiveModel,
) -> Option<DbErr> {
    let result = novel_setting.delete(db).await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// 設定のIDをキー（複数）で削除
pub async fn delete_settings_by_ids(
    db: &DatabaseConnection,
    user_account_id: String,
    setting_ids: Vec<String>,
) -> Option<DbErr> {
    let delete_result = novel_settings::Entity::delete_many()
        .filter(
            Condition::all()
                .add(novel_settings::Column::OwnerUserAccountId.eq(user_account_id))
                .add(novel_settings::Column::Id.is_in(setting_ids)),
        )
        .exec(db)
        .await;

    match delete_result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// 設定の親IDをキーに削除
pub async fn delete_settings_by_parent_setting_id(
    db: &DatabaseConnection,
    user_account_id: String,
    parent_setting_id: String,
) -> Option<DbErr> {
    let delete_result = novel_settings::Entity::delete_many()
        .filter(
            Condition::all()
                .add(novel_settings::Column::OwnerUserAccountId.eq(user_account_id))
                .add(novel_settings::Column::ParentSettingId.eq(parent_setting_id)),
        )
        .exec(db)
        .await;

    match delete_result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// 小説IDをキーに削除
pub async fn delete_settings_by_novel_id(
    db: &DatabaseConnection,
    user_account_id: String,
    novel_id: String,
) -> Option<DbErr> {
    let delete_result = novel_settings::Entity::delete_many()
        .filter(
            Condition::all()
                .add(novel_settings::Column::OwnerUserAccountId.eq(user_account_id))
                .add(novel_settings::Column::NovelId.eq(novel_id)),
        )
        .exec(db)
        .await;

    match delete_result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}
