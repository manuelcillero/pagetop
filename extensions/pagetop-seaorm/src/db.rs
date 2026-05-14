//! API completa de SeaORM para operaciones con la base de datos.
//!
//! Re-exporta el *prelude* de SeaORM (entidades, traits, tipos de valor, macros de derivación…)
//! y expone tres funciones de consulta propias. Con una sola importación tienes todo lo necesario
//! para definir entidades y realizar operaciones CRUD:
//!
//! ```rust,ignore
//! use pagetop_seaorm::db::*;
//! ```
//!
//! Para definir el esquema de la base de datos o escribir migraciones usa además
//! [`crate::migration`].

pub use sea_orm::prelude::*;

use sea_orm::sea_query::{
    MysqlQueryBuilder, PostgresQueryBuilder, QueryStatementWriter, SqliteQueryBuilder,
};
use sea_orm::{DatabaseBackend, ExecResult, Statement};

/// Devuelve una referencia al pool de conexiones para usarla con el sistema de entidades.
///
/// Permite pasar la conexión a los métodos `all`, `one`, `exec`, etc. del sistema de entidades
/// de SeaORM. El coste de esta llamada es prácticamente nulo: sólo devuelve una referencia a un
/// valor inicializado una sola vez al arrancar la aplicación.
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// // Consultas tipadas con el sistema de entidades de SeaORM:
/// //   let users = User::find().all(connection()).await?;
/// //   let user  = User::find_by_id(1).one(connection()).await?;
/// //   User::insert(model).exec(connection()).await?;
/// let _conn = connection();
/// ```
pub fn connection() -> &'static DatabaseConnection {
    &super::DBCONN
}

/// Ejecuta una consulta para devolver todas las filas resultantes.
///
/// Acepta cualquier tipo que implemente [`crate::migration::QueryStatementWriter`] (p. ej. [`crate::migration::SelectStatement`]) y
/// serializa la sentencia al dialecto de la base de datos configurada antes de ejecutarla. Cada
/// fila se devuelve como un [`QueryResult`] sin tipar; extrae los valores con
/// [`QueryResult::try_get`].
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
/// use pagetop_seaorm::migration::*;
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
    let dbconn = &*super::DBCONN;
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
/// use pagetop_seaorm::migration::*;
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
    let dbconn = &*super::DBCONN;
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
/// A diferencia de [`fetch_all`] y [`fetch_one`], no construye la consulta, sino que la recibe
/// como cadena ya formada. Útil para sentencias avanzadas o para migraciones puntuales. El
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
    let dbconn = &*super::DBCONN;
    let dbbackend = dbconn.get_database_backend();
    dbconn
        .execute(Statement::from_string(dbbackend, stmt.into()))
        .await
}
