use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::user_accounts;
use entity::user_accounts::Entity as UserAccountEntity;

// gmailをキーにユーザーを取得
pub async fn get_user_account_by_gmail(
    db: &DatabaseConnection,
    gmail: String,
) -> Option<user_accounts::Model> {
    return UserAccountEntity::find()
        .filter(user_accounts::Column::Gmail.eq(gmail))
        .one(db)
        .await
        .unwrap();
}

// user_setting_idをキーにユーザーを取得
pub async fn get_user_account_by_user_setting_id(
    db: &DatabaseConnection,
    user_setting_id: String,
) -> Option<user_accounts::Model> {
    return UserAccountEntity::find()
        .filter(user_accounts::Column::UserSettingId.eq(user_setting_id))
        .one(db)
        .await
        .unwrap();
}

// ユーザーを登録
pub async fn register_user_account(
    db: &DatabaseConnection,
    user_account: user_accounts::ActiveModel,
) -> Option<DbErr> {
    let result = UserAccountEntity::insert(user_account).exec(db).await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}
