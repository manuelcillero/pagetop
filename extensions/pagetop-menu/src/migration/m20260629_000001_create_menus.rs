use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Menus::Table)
                    .col(pk_auto(Menus::Id))
                    .col(string_len_uniq(Menus::MachineName, 64))
                    .col(boolean(Menus::Locked).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Menus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Menus {
    Table,
    Id,
    MachineName,
    Locked,
}
