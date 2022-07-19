use pagetop::prelude::*;

#[derive(Iden)]
enum Role {
    Table,              // role: Store user roles.

    Rid,                // Primary Key: Unique role ID.
    Name,               // Unique role name.
    Weight,             // The weight of this role in listings and the user interface.
}

pub_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Rid)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Role::Name)
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Role::Weight)
                            .integer()
                            .not_null()
                            .default(10),
                    )
                    // INDEXES.
                    .index(
                        Index::create()
                            .name("weight-name")
                            .col(Role::Weight)
                            .col(Role::Name),
                    )
                    .to_owned(),
            )
            .await?;

        // Built-in roles.
        app::db::exec::<InsertStatement>(
            Query::insert()
                .into_table(Role::Table)
                .columns(vec![Role::Name, Role::Weight])
                .values_panic(vec!["anonymous".into(), "1".into()])
                .values_panic(vec!["authenticated".into(), "2".into()])
                .values_panic(vec!["administrator".into(), "3".into()]),
        )
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}
