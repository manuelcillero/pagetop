use pagetop::core::TypeInfo;
use pagetop::trace;

pub use url::Url as DbUri;

pub use sea_orm::error::{DbErr, RuntimeErr};
pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

mod dbconn;
pub(crate) use dbconn::{run_now, DBCONN};

// Adaptación de `sea-orm-migration` (ver §Créditos en README.md).
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
