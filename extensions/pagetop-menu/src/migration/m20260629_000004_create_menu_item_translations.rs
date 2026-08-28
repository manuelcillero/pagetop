use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MenuItemTranslations::Table)
                    .if_not_exists()
                    .col(integer(MenuItemTranslations::ItemId))
                    .col(string_len(MenuItemTranslations::Lang, 35))
                    .col(string_len(MenuItemTranslations::Title, 255))
                    .primary_key(
                        Index::create()
                            .col(MenuItemTranslations::ItemId)
                            .col(MenuItemTranslations::Lang),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_menu_item_translations_item_id")
                    .from(MenuItemTranslations::Table, MenuItemTranslations::ItemId)
                    .to(MenuItems::Table, MenuItems::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MenuItemTranslations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MenuItemTranslations {
    Table,
    ItemId,
    Lang,
    Title,
}

#[derive(DeriveIden)]
enum MenuItems {
    Table,
    Id,
}
