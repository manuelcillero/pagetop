use pagetop::prelude::*;

#[derive(Iden)]
enum Role {
    Table,              // Store user roles.

    Rid,                // Primary Key: Unique role ID.
    Name,               // Unique role name.
    Weight,             // The weight of this role in listings and the user interface.
}

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(Role::Table)
            .if_not_exists()
            .col(ColumnDef::new(Role::Rid)
                .unsigned()
                .not_null()
                .auto_increment()
                .primary_key()
            )
            .col(ColumnDef::new(Role::Name)
                .string_len(64)
                .not_null()
                .unique_key()
            )
            .col(ColumnDef::new(Role::Weight)
                .integer()
                .not_null()
                .default(0)
            )
            // INDEXES.
            .index(Index::create()
                .name("name-weight")
                .col(Role::Name)
                .col(Role::Weight)
            )
            .to_owned()
        )
        .await?;

        app::db::exec::<InsertStatement>(Query::insert()
            .into_table(Role::Table)
            .columns(vec![Role::Name])
            .values_panic(vec!["anonymous".into()])
            .values_panic(vec!["authenticated".into()])
        )
        .await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop()
            .table(Role::Table)
            .to_owned()
        )
        .await
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        module_name!()
    }
}
