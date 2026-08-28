use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UserTokens::Table)
                    .col(pk_auto(UserTokens::Id))
                    .col(integer(UserTokens::UserId))
                    // "email_verification" | "password_reset"
                    .col(string_len(UserTokens::Kind, 32))
                    // SHA-256 del token plano (nunca se almacena el token en claro)
                    .col(char_len_uniq(UserTokens::TokenHash, 64))
                    .col(timestamp(UserTokens::ExpiresAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(UserTokens::ConsumedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserTokens::Table, UserTokens::UserId)
                            .to(
                                super::m20260629_000001_create_users::Users::Table,
                                super::m20260629_000001_create_users::Users::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserTokens {
    Table,
    Id,
    UserId,
    Kind,
    TokenHash,
    ExpiresAt,
    ConsumedAt,
}
