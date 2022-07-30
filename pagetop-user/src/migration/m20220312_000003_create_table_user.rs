use pagetop::prelude::*;

#[rustfmt::skip]
#[derive(Iden)]
enum User {
    Table,              // user: Stores user data.

    Uid,                // Primary Key: Unique user ID.
    Name,               // Unique user name.
    Pass,               // User's password (hashed).
    Mail,               // User's e-mail address.
    Created,            // Timestamp for when user was created.
    Changed,            // Timestamp for when user was changed.
    Access,             // Timestamp for previous time user accessed the site.
    Login,              // Timestamp for user's last login.
    Status,             // Whether the user is active(1) or blocked(0).
    Timezone,           // User's time zone.
}

pub_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Uid)
                            .unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string_len(60)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Pass).string_len(128).not_null())
                    .col(ColumnDef::new(User::Mail).string_len(255))
                    .col(ColumnDef::new(User::Created).timestamp().not_null())
                    .col(ColumnDef::new(User::Changed).timestamp().not_null())
                    .col(ColumnDef::new(User::Access).timestamp().not_null())
                    .col(ColumnDef::new(User::Login).timestamp().not_null())
                    .col(ColumnDef::new(User::Status).boolean().not_null())
                    .col(ColumnDef::new(User::Timezone).string_len(32))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
