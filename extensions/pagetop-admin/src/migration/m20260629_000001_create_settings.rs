use pagetop_seaorm::migration::*;

/// Tabla `settings`: almacén clave-valor JSON para configuración persistente.
pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(string_len(Settings::Key, 190).primary_key())
                    .col(string_len(Settings::Scope, 64))
                    .col(text(Settings::Value))
                    .col(timestamp(Settings::UpdatedAt).default(Expr::current_timestamp()))
                    .col(integer_null(Settings::UpdatedBy))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_settings_scope")
                    .table(Settings::Table)
                    .col(Settings::Scope)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Key,
    Scope,
    Value,
    UpdatedAt,
    UpdatedBy,
}
