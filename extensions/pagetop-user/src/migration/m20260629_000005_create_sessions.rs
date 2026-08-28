use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    // sid: 64 chars hex (32 bytes aleatorios)
                    .col(char_len(Sessions::Sid, 64).primary_key())
                    .col(integer(Sessions::UserId))
                    // Datos de sesión en JSON (flash messages, etc.)
                    .col(text(Sessions::Data).default("{}"))
                    .col(timestamp_null(Sessions::LastActivityAt))
                    .col(timestamp(Sessions::ExpiresAt).default(Expr::current_timestamp()))
                    .col(timestamp(Sessions::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Sessions::Table, Sessions::UserId)
                            .to(
                                super::m20260629_000001_create_users::Users::Table,
                                super::m20260629_000001_create_users::Users::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sessions_user_id")
                    .table(Sessions::Table)
                    .col(Sessions::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sessions_expires_at")
                    .table(Sessions::Table)
                    .col(Sessions::ExpiresAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sessions {
    Table,
    Sid,
    UserId,
    Data,
    LastActivityAt,
    ExpiresAt,
    CreatedAt,
}
