use pagetop_seaorm::migration::*;

use sea_orm::{ConnectionTrait, DbBackend};

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Roles::Table)
                    .col(pk_auto(Roles::Id))
                    .col(string_len_uniq(Roles::MachineName, 64))
                    .col(string_len(Roles::Label, 128))
                    .col(text_null(Roles::Description))
                    .col(integer(Roles::Weight).default(0))
                    // Los roles del sistema (anonymous, authenticated) no se borran.
                    .col(boolean(Roles::Locked).default(false))
                    .to_owned(),
            )
            .await?;

        // Filas de sistema: anonymous (1), authenticated (2).
        let insert = Query::insert()
            .into_table(Roles::Table)
            .columns([
                Roles::Id,
                Roles::MachineName,
                Roles::Label,
                Roles::Weight,
                Roles::Locked,
            ])
            .values_panic([
                1.into(),
                "anonymous".into(),
                "Anonymous".into(),
                0.into(),
                true.into(),
            ])
            .values_panic([
                2.into(),
                "authenticated".into(),
                "Authenticated".into(),
                1.into(),
                true.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;

        // Los IDs anteriores se insertan explícitamente; en PostgreSQL la secuencia del `serial`
        // no avanza con inserciones explícitas, así que el próximo alta chocaría con estas filas.
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared(
                    "SELECT setval(pg_get_serial_sequence('roles', 'id'), \
                     (SELECT MAX(id) FROM roles))",
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    MachineName,
    Label,
    Description,
    Weight,
    Locked,
}
