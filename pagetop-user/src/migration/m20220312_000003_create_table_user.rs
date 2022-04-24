use pagetop::prelude::*;

#[derive(Iden)]
enum User { Table,
    Uid,
    Name,
    Pass,
    Mail,
    Created,
    Changed,
    Access,
    Login,
    Status,
    Timezone,
}

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                // Stores user data.
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    // Primary Key: Unique user ID.
                    .col(ColumnDef::new(User::Uid)
                        .unsigned()
                        .not_null()
                        .primary_key()
                    )
                    // Unique user name.
                    .col(ColumnDef::new(User::Name)
                        .string_len(60)
                        .not_null()
                        .unique_key()
                    )
                    // User's password (hashed).
                    .col(ColumnDef::new(User::Pass)
                        .string_len(128)
                        .not_null()
                    )
                    // User's e-mail address.
                    .col(ColumnDef::new(User::Mail)
                        .string_len(255)
                    )
                    // Timestamp for when user was created.
                    .col(ColumnDef::new(User::Created)
                        .timestamp()
                        .not_null()
                    )
                    // Timestamp for when user was changed.
                    .col(ColumnDef::new(User::Changed)
                        .timestamp()
                        .not_null()
                    )
                    // Timestamp for previous time user accessed the site.
                    .col(ColumnDef::new(User::Access)
                        .timestamp()
                        .not_null()
                    )
                    // Timestamp for user's last login.
                    .col(ColumnDef::new(User::Login)
                        .timestamp()
                        .not_null()
                    )
                    // Whether the user is active(1) or blocked(0).
                    .col(ColumnDef::new(User::Status)
                        .boolean()
                        .not_null()
                    )
                    // User's time zone.
                    .col(ColumnDef::new(User::Timezone)
                        .string_len(32)
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

impl MigrationName for Migration {
    fn name(&self) -> &str {
        module_name!()
    }
}
