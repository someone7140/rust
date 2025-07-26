use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::user_accounts;
use entity::user_accounts::Entity as UserAccountEntity;

// idをキーにユーザーを取得
pub async fn get_user_account_by_id(
    db: &DatabaseConnection,
    id: String,
) -> Option<user_accounts::Model> {
    return UserAccountEntity::find()
        .filter(user_accounts::Column::Id.eq(id))
        .one(db)
        .await
        .unwrap();
}

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

// ユーザーのimage_urlを更新
pub async fn update_user_account_image_url(
    db: &DatabaseConnection,
    user_account: user_accounts::ActiveModel,
    image_url: Option<String>,
) -> Option<DbErr> {
    let mut update_user_account = user_account;
    update_user_account.image_url = Set(image_url);
    let result = update_user_account.update(db).await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

// ユーザーの入力情報を更新
pub async fn update_user_account_input_info(
    db: &DatabaseConnection,
    user_account: user_accounts::ActiveModel,
    user_setting_id: String,
    name: String,
) -> Option<DbErr> {
    let mut update_user_account = user_account;
    update_user_account.user_setting_id = Set(user_setting_id);
    update_user_account.name = Set(name);
    let result = update_user_account.update(db).await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}
