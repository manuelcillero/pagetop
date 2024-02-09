use pagetop::prelude::*;

#[rustfmt::skip]
#[derive(Iden)]
enum UserRole {
    Table,              // user_role: Maps users to roles.

    Uid,                // Foreign Key: User::Uid for user.
    Rid,                // Foreign Key: Role::Rid for role.
}

#[derive(Iden)]
enum User {
    Table,
    Uid,
    /* ... */
}

#[derive(Iden)]
enum Role {
    Table,
    Rid,
    /* ... */
}

new_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRole::Uid).unsigned().not_null())
                    .col(ColumnDef::new(UserRole::Rid).unsigned().not_null())
                    // INDEXES.
                    .primary_key(Index::create().col(UserRole::Uid).col(UserRole::Rid))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role-uid")
                            .from(UserRole::Table, UserRole::Uid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role-rid")
                            .from(UserRole::Table, UserRole::Rid)
                            .to(Role::Table, Role::Rid)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}
