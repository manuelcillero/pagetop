use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(integer(RolePermissions::RoleId))
                    // Clave del permiso como string con namespace: "provider.action"
                    .col(string_len(RolePermissions::PermissionKey, 190))
                    .col(timestamp(RolePermissions::GrantedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .col(RolePermissions::RoleId)
                            .col(RolePermissions::PermissionKey),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RolePermissions::Table, RolePermissions::RoleId)
                            .to(
                                super::m20260629_000002_create_roles::Roles::Table,
                                super::m20260629_000002_create_roles::Roles::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RolePermissions {
    Table,
    RoleId,
    PermissionKey,
    GrantedAt,
}
