use pagetop::prelude::*;

#[derive(Iden)]
enum UserRole { Table,
    Uid,
    Rid,
}
#[derive(Iden)]
enum User { Table,
    Uid,
//  ...
}
#[derive(Iden)]
enum Role { Table,
    Rid,
//  ...
}

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                // Maps users to roles.
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    // Foreign Key: User::Uid for user.
                    .col(ColumnDef::new(UserRole::Uid)
                        .unsigned()
                        .not_null()
                    )
                    // Foreign Key: Role::Rid for role.
                    .col(ColumnDef::new(UserRole::Rid)
                        .unsigned()
                        .not_null()
                    )
                    // INDEXES.
                    .primary_key(Index::create()
                        .col(UserRole::Uid)
                        .col(UserRole::Rid)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role-uid")
                            .from(UserRole::Table, UserRole::Uid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role-rid")
                            .from(UserRole::Table, UserRole::Rid)
                            .to(Role::Table, Role::Rid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(UserRole::Table)
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
