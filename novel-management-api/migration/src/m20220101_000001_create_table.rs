use entity::{novel_settings, novels, user_accounts};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // user_accountsのテーブル
        manager
            .create_table(
                Table::create()
                    .table(user_accounts::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_accounts::Column::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user_accounts::Column::UserSettingId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(user_accounts::Column::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(user_accounts::Column::Gmail)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(user_accounts::Column::ImageUrl).string())
                    .col(
                        ColumnDef::new(user_accounts::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // novelsのテーブル
        manager
            .create_table(
                Table::create()
                    .table(novels::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(novels::Column::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(novels::Column::Title).string().not_null())
                    .col(ColumnDef::new(novels::Column::Description).text())
                    .col(
                        ColumnDef::new(novels::Column::OwnerUserAccountId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(novels::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_novels_owner_user_account_id")
                    .table(novels::Entity)
                    .col(novels::Column::OwnerUserAccountId)
                    .to_owned(),
            )
            .await?;

        // novel_settingsのテーブル
        manager
            .create_table(
                Table::create()
                    .table(novel_settings::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(novel_settings::Column::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(novel_settings::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(novel_settings::Column::NovelId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(novels::Column::OwnerUserAccountId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(novel_settings::Column::ParentSettingId).string())
                    .col(ColumnDef::new(novel_settings::Column::DisplayOrder).integer())
                    .col(
                        ColumnDef::new(novel_settings::Column::Attributes)
                            .array(ColumnType::String(StringLen::default())),
                    )
                    .col(ColumnDef::new(novel_settings::Column::Description).text())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_novel_settings_novel_id")
                    .table(novel_settings::Entity)
                    .col(novel_settings::Column::NovelId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_novel_settings_owner_user_account_id")
                    .table(novel_settings::Entity)
                    .col(novel_settings::Column::OwnerUserAccountId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // novel_settingsのテーブル
        manager
            .drop_index(
                Index::drop()
                    .name("idx_novel_settings_novel_id")
                    .table(novel_settings::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_novel_settings_owner_user_account_id")
                    .table(novel_settings::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(novel_settings::Entity).to_owned())
            .await?;

        // novelsのテーブル
        manager
            .drop_index(
                Index::drop()
                    .name("idx_novels_owner_user_account_id")
                    .table(novels::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(novels::Entity).to_owned())
            .await?;

        // user_accountsのテーブル
        manager
            .drop_table(Table::drop().table(user_accounts::Entity).to_owned())
            .await
    }
}
