use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Users::Table)
                    .col(pk_auto(Users::Id))
                    .col(string_len_uniq(Users::Username, 64))
                    .col(string_len_uniq(Users::Email, 254))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .col(string(Users::PasswordHash))
                    // 0=Blocked, 1=Active, 2=Pending
                    .col(small_integer(Users::Status).default(1))
                    .col(string_len_null(Users::Language, 16))
                    .col(string_len_null(Users::Timezone, 64))
                    .col(string_len_null(Users::DisplayName, 128))
                    .col(timestamp_null(Users::LastLoginAt))
                    .col(timestamp_null(Users::LastAccessAt))
                    .col(integer(Users::FailedLoginCount).default(0))
                    .col(timestamp_null(Users::LockedUntil))
                    // Acceso irrestricto al sistema, sin pasar por roles ni permisos.
                    .col(boolean(Users::IsAdmin).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    EmailVerifiedAt,
    PasswordHash,
    Status,
    Language,
    Timezone,
    DisplayName,
    LastLoginAt,
    LastAccessAt,
    FailedLoginCount,
    LockedUntil,
    IsAdmin,
}
