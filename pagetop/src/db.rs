use crate::{config, run_now, trace, LazyStatic, ResultExt};

pub use url::Url as DbUri;

pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, Statement};

pub(crate) static DBCONN: LazyStatic<DbConn> = LazyStatic::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        &config::SETTINGS.database.db_name,
        &config::SETTINGS.database.max_pool_size
    );

    let db_uri = match config::SETTINGS.database.db_type.as_str() {
        "mysql" | "postgres" => {
            let mut tmp_uri = DbUri::parse(
                format!(
                    "{}://{}/{}",
                    &config::SETTINGS.database.db_type,
                    &config::SETTINGS.database.db_host,
                    &config::SETTINGS.database.db_name
                )
                .as_str(),
            )
            .unwrap();
            tmp_uri
                .set_username(config::SETTINGS.database.db_user.as_str())
                .unwrap();
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri
                .set_password(Some(config::SETTINGS.database.db_pass.as_str()))
                .unwrap();
            if config::SETTINGS.database.db_port != 0 {
                tmp_uri
                    .set_port(Some(config::SETTINGS.database.db_port))
                    .unwrap();
            }
            tmp_uri
        }
        "sqlite" => DbUri::parse(
            format!(
                "{}://{}",
                &config::SETTINGS.database.db_type,
                &config::SETTINGS.database.db_name
            )
            .as_str(),
        )
        .unwrap(),
        _ => {
            trace::error!(
                "Unrecognized database type \"{}\"",
                &config::SETTINGS.database.db_type
            );
            DbUri::parse("").unwrap()
        }
    };

    run_now(Database::connect::<ConnectOptions>({
        let mut db_opt = ConnectOptions::new(db_uri.to_string());
        db_opt.max_connections(config::SETTINGS.database.max_pool_size);
        db_opt
    }))
    .expect_or_log("Failed to connect to database")
});

static DBBACKEND: LazyStatic<DatabaseBackend> = LazyStatic::new(|| DBCONN.get_database_backend());

pub async fn query<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Vec<QueryResult>, DbErr> {
    DBCONN
        .query_all(Statement::from_string(
            *DBBACKEND,
            match *DBBACKEND {
                DatabaseBackend::MySql => stmt.to_string(MysqlQueryBuilder),
                DatabaseBackend::Postgres => stmt.to_string(PostgresQueryBuilder),
                DatabaseBackend::Sqlite => stmt.to_string(SqliteQueryBuilder),
            },
        ))
        .await
}

pub async fn exec<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Option<QueryResult>, DbErr> {
    DBCONN
        .query_one(Statement::from_string(
            *DBBACKEND,
            match *DBBACKEND {
                DatabaseBackend::MySql => stmt.to_string(MysqlQueryBuilder),
                DatabaseBackend::Postgres => stmt.to_string(PostgresQueryBuilder),
                DatabaseBackend::Sqlite => stmt.to_string(SqliteQueryBuilder),
            },
        ))
        .await
}

pub async fn exec_raw(stmt: String) -> Result<ExecResult, DbErr> {
    DBCONN
        .execute(Statement::from_string(*DBBACKEND, stmt))
        .await
}

// El siguiente m??dulo migration es una versi??n simplificada del m??dulo sea_orm_migration (v0.9.1)
// https://github.com/SeaQL/sea-orm/tree/0.9.1/sea-orm-migration para evitar los errores generados
// por el paradigma modular de PageTop. Se copian los siguientes archivos del original:
//
//    lib.rs => db/migration.rs       (descartando el uso de algunos m??dulos y exportaciones)
//    manager.rs => db/migration/manager.rs
//    migrator.rs => db/migration/migrator.rs         (suprimiendo la gesti??n de los errores)
//    prelude.rs =>  db/migration/prelude.rs                                   (evitando cli)
//    seaql_migrations.rs =>  db/migration/seaql_migrations.rs
//
mod migration;
pub use migration::prelude::*;

pub type MigrationItem = Box<dyn MigrationTrait>;

#[macro_export]
macro_rules! pub_migration {
    ( $migration:ident ) => {
        pub struct $migration;

        impl MigrationName for $migration {
            fn name(&self) -> &str {
                $crate::util::partial_type_name(module_path!(), 1)
            }
        }
    };
}

#[macro_export]
macro_rules! migration_item {
    ( $migration_module:ident ) => {{
        Box::new(migration::$migration_module::Migration)
    }};
}
