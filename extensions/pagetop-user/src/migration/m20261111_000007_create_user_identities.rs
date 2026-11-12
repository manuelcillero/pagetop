use pagetop_seaorm::migration::*;

use super::m20260629_000001_create_users::Users;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserIdentities::Table)
                    .if_not_exists()
                    .col(pk_auto(UserIdentities::Id))
                    .col(integer(UserIdentities::UserId))
                    // Identificador del proveedor (`AuthProvider::id()`).
                    .col(string_len(UserIdentities::Provider, 64))
                    // Identificador de la persona tal como lo da el proveedor, y su forma
                    // normalizada para buscar, con el ámbito que el proveedor necesite. Nulos en
                    // una identidad pendiente.
                    .col(string_len_null(UserIdentities::ExternalId, 320))
                    .col(string_len_null(UserIdentities::ExternalKey, 320))
                    // Marca que un administrador pone en una cuenta para que el proveedor pueda
                    // vincularla, con el nombre o email con el que se espera a la persona.
                    .col(boolean(UserIdentities::Pending).default(false))
                    .col(string_len_null(UserIdentities::ExpectedKey, 320))
                    .col(text_null(UserIdentities::Data))
                    .col(timestamp(UserIdentities::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(UserIdentities::LinkedAt))
                    .col(timestamp_null(UserIdentities::LastLoginAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserIdentities::Table, UserIdentities::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Una persona del proveedor sólo puede estar en una cuenta (varias pendientes, con la
        // clave nula, sí), y una cuenta sólo tiene una identidad de cada proveedor.
        manager
            .create_index(
                Index::create()
                    .name("idx_user_identities_external")
                    .table(UserIdentities::Table)
                    .col(UserIdentities::Provider)
                    .col(UserIdentities::ExternalKey)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_identities_user")
                    .table(UserIdentities::Table)
                    .col(UserIdentities::Provider)
                    .col(UserIdentities::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_identities_user_id")
                    .table(UserIdentities::Table)
                    .col(UserIdentities::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserIdentities::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserIdentities {
    Table,
    Id,
    UserId,
    Provider,
    ExternalId,
    ExternalKey,
    Pending,
    ExpectedKey,
    Data,
    CreatedAt,
    LinkedAt,
    LastLoginAt,
}
