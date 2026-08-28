use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(integer(UserRoles::UserId))
                    .col(integer(UserRoles::RoleId))
                    .primary_key(
                        Index::create()
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(
                                super::m20260629_000001_create_users::Users::Table,
                                super::m20260629_000001_create_users::Users::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::RoleId)
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
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRoles {
    Table,
    UserId,
    RoleId,
}
