use pagetop::prelude::*;

#[rustfmt::skip]
#[derive(Iden)]
enum RolePermission {
    Table,              // role_permission: Stores the permissions assigned to user roles.

    Rid,                // Foreign Key: Role::Rid.
    Permission,         // A single permission granted to the role identified by Rid.
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
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolePermission::Rid).unsigned().not_null())
                    .col(
                        ColumnDef::new(RolePermission::Permission)
                            .string_len(128)
                            .not_null(),
                    )
                    // INDEXES.
                    .primary_key(
                        Index::create()
                            .col(RolePermission::Rid)
                            .col(RolePermission::Permission),
                    )
                    .index(
                        Index::create()
                            .name("permission")
                            .col(RolePermission::Permission),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permission-rid")
                            .from(RolePermission::Table, RolePermission::Rid)
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
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await
    }
}
