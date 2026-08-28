use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MenuTranslations::Table)
                    .if_not_exists()
                    .col(integer(MenuTranslations::MenuId))
                    .col(string_len(MenuTranslations::Lang, 35))
                    .col(string_len(MenuTranslations::Title, 128))
                    .col(text_null(MenuTranslations::Description))
                    .primary_key(
                        Index::create()
                            .col(MenuTranslations::MenuId)
                            .col(MenuTranslations::Lang),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_menu_translations_menu_id")
                    .from(MenuTranslations::Table, MenuTranslations::MenuId)
                    .to(Menus::Table, Menus::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MenuTranslations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MenuTranslations {
    Table,
    MenuId,
    Lang,
    Title,
    Description,
}

#[derive(DeriveIden)]
enum Menus {
    Table,
    Id,
}
