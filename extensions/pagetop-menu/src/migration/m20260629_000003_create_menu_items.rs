use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(MenuItems::Table)
                    .col(pk_auto(MenuItems::Id))
                    .col(integer(MenuItems::MenuId))
                    .col(integer_null(MenuItems::ParentId))
                    .col(string_len(MenuItems::Url, 2048))
                    .col(integer(MenuItems::Weight).default(0))
                    .col(boolean(MenuItems::Enabled).default(true))
                    .col(boolean(MenuItems::Expanded).default(false))
                    .col(string_len(MenuItems::Provider, 64).default("user"))
                    .col(string_len_null(MenuItems::ExternalKey, 128))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_menu_items_menu_id")
                    .from(MenuItems::Table, MenuItems::MenuId)
                    .to(Menus::Table, Menus::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Índice para listar hijos ordenados.
        manager
            .create_index(
                Index::create()
                    .name("idx_menu_items_menu_parent_weight")
                    .table(MenuItems::Table)
                    .col(MenuItems::MenuId)
                    .col(MenuItems::ParentId)
                    .col(MenuItems::Weight)
                    .to_owned(),
            )
            .await?;

        // Índice de unicidad para upserts por extensión.
        manager
            .create_index(
                Index::create()
                    .name("idx_menu_items_provider_key")
                    .table(MenuItems::Table)
                    .col(MenuItems::Provider)
                    .col(MenuItems::ExternalKey)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MenuItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MenuItems {
    Table,
    Id,
    MenuId,
    ParentId,
    Url,
    Weight,
    Enabled,
    Expanded,
    Provider,
    ExternalKey,
}

#[derive(DeriveIden)]
enum Menus {
    Table,
    Id,
}
