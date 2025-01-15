use pagetop::trace;
use pagetop::util::TypeInfo;

pub use url::Url as DbUri;

pub use sea_orm::error::{DbErr, RuntimeErr};
pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

mod dbconn;
pub(crate) use dbconn::{run_now, DBCONN};

// The migration module is a customized version of the sea_orm_migration module (v1.0.0)
// https://github.com/SeaQL/sea-orm/tree/1.0.0/sea-orm-migration to avoid errors caused by the
// extensions paradigm of PageTop. Files integrated from original:
//
//    lib.rs => db/migration.rs . . . . . . . . . . . . . . (excluding some modules and exports)
//    connection.rs => db/migration/connection.rs . . . . . . . . . . . . . . (full integration)
//    manager.rs => db/migration/manager.rs . . . . . . . . . . . . . . . . . (full integration)
//    migrator.rs => db/migration/migrator.rs . . . . . . . . . . . .(omitting error management)
//    prelude.rs => db/migration/prelude.rs . . . . . . . . . . . . . . . . . . . (avoiding CLI)
//    seaql_migrations.rs => db/migration/seaql_migrations.rs . . . . . . . . (full integration)
//
mod migration;
pub use migration::prelude::*;
pub use migration::schema::*;

pub async fn query<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Vec<QueryResult>, DbErr> {
    let dbconn = &*DBCONN;
    let dbbackend = dbconn.get_database_backend();
    dbconn
        .query_all(Statement::from_string(
            dbbackend,
            match dbbackend {
                DatabaseBackend::MySql => stmt.to_string(MysqlQueryBuilder),
                DatabaseBackend::Postgres => stmt.to_string(PostgresQueryBuilder),
                DatabaseBackend::Sqlite => stmt.to_string(SqliteQueryBuilder),
            },
        ))
        .await
}

pub async fn exec<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Option<QueryResult>, DbErr> {
    let dbconn = &*DBCONN;
    let dbbackend = dbconn.get_database_backend();
    dbconn
        .query_one(Statement::from_string(
            dbbackend,
            match dbbackend {
                DatabaseBackend::MySql => stmt.to_string(MysqlQueryBuilder),
                DatabaseBackend::Postgres => stmt.to_string(PostgresQueryBuilder),
                DatabaseBackend::Sqlite => stmt.to_string(SqliteQueryBuilder),
            },
        ))
        .await
}

pub async fn exec_raw(stmt: String) -> Result<ExecResult, DbErr> {
    let dbconn = &*DBCONN;
    let dbbackend = dbconn.get_database_backend();
    dbconn
        .execute(Statement::from_string(dbbackend, stmt))
        .await
}

pub trait MigratorBase {
    fn run_up();

    fn run_down();
}

#[rustfmt::skip]
impl<M: MigratorTrait> MigratorBase for M {
    fn run_up() {
        if let Err(e) = run_now(Self::up(SchemaManagerConnection::Connection(&DBCONN), None)) {
            trace::error!("Migration upgrade failed ({})", e);
        };
    }

    fn run_down() {
        if let Err(e) = run_now(Self::down(SchemaManagerConnection::Connection(&DBCONN), None)) {
            trace::error!("Migration downgrade failed ({})", e);
        };
    }
}

impl<M: MigrationTrait> MigrationName for M {
    fn name(&self) -> &str {
        TypeInfo::NameTo(-2).of::<M>()
    }
}

pub type MigrationItem = Box<dyn MigrationTrait>;

#[macro_export]
macro_rules! install_migrations {
    ( $($migration_module:ident),+ $(,)? ) => {{
        use $crate::db::{MigrationItem, MigratorBase, MigratorTrait};

        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<MigrationItem> {
                let mut m = Vec::<MigrationItem>::new();
                $(
                    m.push(Box::new(migration::$migration_module::Migration));
                )*
                m
            }
        }
        Migrator::run_up();
    }};
}

#[macro_export]
macro_rules! uninstall_migrations {
    ( $($migration_module:ident),+ $(,)? ) => {{
        use $crate::db::{MigrationItem, MigratorBase, MigratorTrait};

        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<MigrationItem> {
                let mut m = Vec::<MigrationItem>::new();
                $(
                    m.push(Box::new(migration::$migration_module::Migration));
                )*
                m
            }
        }
        Migrator::run_down();
    }};
}
