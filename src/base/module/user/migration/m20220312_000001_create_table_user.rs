use crate::db::migration::*;

#[derive(Iden)]
enum User {
    Table,
    Id,
    Title,
    Text,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220312_000001_create_table_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(User::Title)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Text)
                        .string()
                        .not_null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(User::Table)
                .to_owned()
            )
            .await
    }
}
