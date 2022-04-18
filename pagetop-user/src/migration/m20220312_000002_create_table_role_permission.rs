use pagetop::db::migration::*;

#[derive(Iden)]
enum RolePermission { Table,
    Rid,
    Permission,
    Module,
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
                // Stores the permissions assigned to user roles.
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    // Foreign Key: Role::Rid.
                    .col(ColumnDef::new(RolePermission::Rid)
                        .unsigned()
                        .not_null()
                    )
                    // A single permission granted to the role identified by Rid.
                    .col(ColumnDef::new(RolePermission::Permission)
                        .string_len(128)
                        .not_null()
                    )
                    // The module declaring the permission.
                    .col(ColumnDef::new(RolePermission::Module)
                        .string_len(255)
                        .not_null()
                    )
                    // INDEXES.
                    .primary_key(Index::create()
                        .col(RolePermission::Rid)
                        .col(RolePermission::Permission)
                    )
                    .index(Index::create()
                        .name("permission")
                        .col(RolePermission::Permission)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permission-rid")
                            .from(RolePermission::Table, RolePermission::Rid)
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
                .table(RolePermission::Table)
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
