use pagetop::core::TypeInfo;
use pagetop::trace;

pub(crate) use url::Url as DbUri;

pub use sea_orm::error::{DbErr, RuntimeErr};
pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

mod dbconn;
pub(crate) use dbconn::{run_now, DBCONN};

// Adaptación de `sea-orm-migration` (ver §Créditos en README.md).
mod migration;
pub use migration::prelude::*;
pub use migration::schema::*;

/// Ejecuta una consulta para devolver todas las filas resultantes.
///
/// Acepta cualquier tipo que implemente [`QueryStatementWriter`] (p. ej. [`SelectStatement`]) y
/// serializa la sentencia al dialecto de la base de datos configurada antes de ejecutarla. Cada
/// fila se devuelve como un [`QueryResult`] sin tipar; extrae los valores con
/// [`QueryResult::try_get`].
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// async fn example() -> Result<(), DbErr> {
///     let mut stmt = Query::select()
///         .column(Asterisk)
///         .from(Alias::new("users"))
///         .to_owned();
///     let rows = fetch_all(&mut stmt).await?;
///     for row in rows {
///         let name: String = row.try_get("", "name")?;
///         println!("{name}");
///     }
///     Ok(())
/// }
/// ```
pub async fn fetch_all<Q: QueryStatementWriter>(stmt: &mut Q) -> Result<Vec<QueryResult>, DbErr> {
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

/// Ejecuta una consulta y devuelve sólo la primera fila, si existe.
///
/// Funciona igual que [`fetch_all`] pero detiene la ejecución tras la primera fila y devuelve
/// `None` si la consulta no produce resultados.
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// async fn example() -> Result<(), DbErr> {
///     let mut stmt = Query::select()
///         .column(Asterisk)
///         .from(Alias::new("users"))
///         .and_where(Expr::col(Alias::new("id")).eq(1))
///         .to_owned();
///     if let Some(row) = fetch_one(&mut stmt).await? {
///         let name: String = row.try_get("", "name")?;
///         println!("{name}");
///     }
///     Ok(())
/// }
/// ```
pub async fn fetch_one<Q: QueryStatementWriter>(
    stmt: &mut Q,
) -> Result<Option<QueryResult>, DbErr> {
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

/// Ejecuta una sentencia SQL en crudo (INSERT, UPDATE, DELETE…) y devuelve el resultado de
/// la operación.
///
/// A diferencia de [`fetch_all`] y [`fetch_one`], no construye la consulta, sino que la recibe como
/// cadena ya formada. Útil para sentencias avanzadas o para migraciones puntuales. El
/// [`ExecResult`] devuelto permite consultar las filas afectadas o el último ID insertado.
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// async fn example() -> Result<(), DbErr> {
///     let result = execute("DELETE FROM sessions WHERE expired = 1").await?;
///     println!("Filas eliminadas: {}", result.rows_affected());
///     Ok(())
/// }
/// ```
pub async fn execute(stmt: impl Into<String>) -> Result<ExecResult, DbErr> {
    let dbconn = &*DBCONN;
    let dbbackend = dbconn.get_database_backend();
    dbconn
        .execute(Statement::from_string(dbbackend, stmt.into()))
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
