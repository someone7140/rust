use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Condition, QueryOrder};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::novels;
use entity::novels::Entity as NovelEntity;

// 小説を登録
pub async fn register_novel(db: &DatabaseConnection, novel: novels::ActiveModel) -> Option<DbErr> {
    let result = NovelEntity::insert(novel).exec(db).await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// 小説を編集
pub async fn edit_novel(
    db: &DatabaseConnection,
    novel: novels::ActiveModel,
    title: String,
    description: Option<String>,
) -> Option<DbErr> {
    let mut update_novel = novel;
    update_novel.title = Set(title);
    update_novel.description = Set(description);
    let result = update_novel.update(db).await;

    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// 小説を削除
pub async fn delete_novel(db: &DatabaseConnection, novel: novels::ActiveModel) -> Option<DbErr> {
    let result = novel.delete(db).await;

    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// idをキーに小説を取得
pub async fn get_novel_by_id(
    db: &DatabaseConnection,
    user_account_id: String,
    novel_id: String,
) -> Option<novels::Model> {
    return NovelEntity::find()
        .filter(
            Condition::all()
                .add(novels::Column::OwnerUserAccountId.gte(user_account_id))
                .add(novels::Column::Id.eq(novel_id)),
        )
        .one(db)
        .await
        .unwrap();
}

// 小説の一覧を取得
pub async fn get_my_novels(db: &DatabaseConnection, user_account_id: String) -> Vec<novels::Model> {
    return NovelEntity::find()
        .filter(novels::Column::OwnerUserAccountId.gte(user_account_id))
        .order_by_desc(novels::Column::CreatedAt)
        .all(db)
        .await
        .unwrap();
}
