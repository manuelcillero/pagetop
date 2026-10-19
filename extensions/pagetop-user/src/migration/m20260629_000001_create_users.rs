use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Users::Table)
                    .col(pk_auto(Users::Id))
                    // El nombre de usuario y el email con sus mayúsculas (ver `identity.rs`).
                    .col(string_len(Users::Username, 64))
                    .col(string_len(Users::Email, 254))
                    // Formas para buscar y comparar sin distinguir mayúsculas, que pueden ser más
                    // largas que el original (en minúsculas, un carácter puede ocupar dos), y
                    // *hash* del esqueleto de confusión del nombre (ver `identity.rs`).
                    .col(string_len_uniq(Users::UsernameKey, 255))
                    .col(char_len_uniq(Users::UsernameSkeleton, 64))
                    .col(string_len_uniq(Users::EmailKey, 255))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .col(string(Users::PasswordHash))
                    // 0=Blocked, 1=Active, 2=Pending
                    .col(small_integer(Users::Status).default(1))
                    .col(string_len_null(Users::Language, 16))
                    .col(string_len_null(Users::Timezone, 64))
                    .col(string_len_null(Users::Theme, 64))
                    .col(string_len_null(Users::DisplayName, 128))
                    // Texto libre "Sobre mí" que el propio usuario escribe en su perfil.
                    .col(text_null(Users::About))
                    .col(timestamp_null(Users::LastLoginAt))
                    .col(timestamp_null(Users::LastAccessAt))
                    .col(integer(Users::FailedLoginCount).default(0))
                    .col(timestamp_null(Users::FailedLoginAt))
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
    UsernameKey,
    UsernameSkeleton,
    EmailKey,
    EmailVerifiedAt,
    PasswordHash,
    Status,
    Language,
    Timezone,
    Theme,
    DisplayName,
    About,
    LastLoginAt,
    LastAccessAt,
    FailedLoginCount,
    FailedLoginAt,
    LockedUntil,
    IsAdmin,
}
