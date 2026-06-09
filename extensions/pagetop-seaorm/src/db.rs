//! Definición de entidades y acceso a la base de datos.
//!
//! Agrupa los *traits*, macros y tipos del sistema de entidades de SeaORM, junto con las funciones
//! [`dbconn`], [`execute`], [`fetch_all`] y [`fetch_one`], en una sola importación:
//!
//! ```rust
//! use pagetop_seaorm::db::*;
//! ```
//!
//! El sistema de entidades (`Entity::find()`, `Entity::insert()`, transacciones) es el camino
//! recomendado para la mayoría de operaciones. Las funciones [`execute`], [`fetch_all`] y
//! [`fetch_one`] ofrecen vías alternativas para cuando ese sistema no es suficiente, como consultas
//! sin entidad concreta, SQL específico para el motor de base de datos utilizado o sentencias
//! puntuales.
//!
//! Estas funciones integran los valores como literales escapados, no como parámetros de base de
//! datos. Para consultas con datos del usuario, el sistema de entidades es más robusto. Si aun así
//! se necesita SQL en crudo con parámetros reales, se puede construir un [`api::Statement`]
//! directamente con [`api::Statement::from_sql_and_values`].
//!
//! ## Tipos esenciales
//!
//! Destacan los siguientes elementos de uso más frecuente:
//!
//! - **Acceso**: [`DatabaseConnection`], [`dbconn`] (para obtener el pool de conexiones).
//! - **Consultas**: [`EntityTrait`], [`QueryFilter`], [`QueryOrder`], [`QuerySelect`].
//! - **Transacciones**: [`TransactionTrait`], [`DatabaseTransaction`].
//! - **Modelos activos**: [`ActiveModelTrait`], [`ActiveValue`] ([`ActiveValue::Set`],
//!   [`ActiveValue::Unchanged`], [`ActiveValue::NotSet`]).
//! - **Macros de derivación**: [`DeriveEntityModel`], [`DeriveColumn`], [`DerivePrimaryKey`],
//!   [`DeriveRelation`], [`EnumIter`].
//! - **Errores**: [`DbErr`].
//! - **Resultados**: [`QueryResult`] (filas sin tipar), [`ExecResult`] (INSERT/UPDATE/DELETE).
//!
//! ## Definir una entidad
//!
//! ```rust
//! use pagetop_seaorm::db::*;
//!
//! #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
//! #[sea_orm(table_name = "users")]
//! // El struct debe llamarse `Model`: es un requisito de `DeriveEntityModel`.
//! pub struct Model {
//!     #[sea_orm(primary_key)]
//!     pub id: i32,
//!     pub email: String,
//!     pub name: String,
//! }
//!
//! #[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
//! pub enum Relation {}
//!
//! // `DeriveEntityModel` genera también `ActiveModel`, `Entity`, `Column` y `PrimaryKey`.
//! impl ActiveModelBehavior for ActiveModel {}
//! ```
//!
//! ## Operaciones CRUD
//!
//! ```rust,ignore
//! use pagetop_seaorm::db::*;
//!
//! // El código asume que existe un módulo `user` con una entidad SeaORM definida.
//!
//! async fn example() -> Result<(), DbErr> {
//!     // Buscar todos los registros.
//!     let users = user::Entity::find().all(dbconn()).await?;
//!
//!     // Buscar con filtro.
//!     let alices = user::Entity::find()
//!         .filter(user::Column::Name.eq("Alice"))
//!         .all(dbconn())
//!         .await?;
//!
//!     // Buscar por clave primaria.
//!     let found = user::Entity::find_by_id(1).one(dbconn()).await?;
//!
//!     // Insertar.
//!     let model = user::ActiveModel {
//!         name: ActiveValue::Set("Alice".into()),
//!         ..Default::default()
//!     };
//!     user::Entity::insert(model).exec(dbconn()).await?;
//!
//!     // Actualizar: campos con `ActiveValue::Set`, clave primaria con `ActiveValue::Unchanged`.
//!     let patch = user::ActiveModel {
//!         id: ActiveValue::Unchanged(1),
//!         name: ActiveValue::Set("Bob".into()),
//!         ..Default::default()
//!     };
//!     patch.update(dbconn()).await?;
//!
//!     // Eliminar por clave primaria.
//!     user::Entity::delete_by_id(1).exec(dbconn()).await?;
//!
//!     // Transacción. `Box::pin` es necesario: `TransactionTrait` exige `Pin<Box<dyn Future>>`.
//!     dbconn().transaction::<_, DbErr, _>(|txn| Box::pin(async move {
//!         user::Entity::insert(
//!             user::ActiveModel { name: ActiveValue::Set("Carol".into()), ..Default::default() }
//!         ).exec(txn).await?;
//!         user::Entity::delete_by_id(2).exec(txn).await?;
//!         Ok(())
//!     })).await?;
//!     Ok(())
//! }
//! ```
//!
//! Para migraciones y definición de esquemas usa [`migration`](crate::migration).
//!
//! ## Acceso completo a SeaORM
//!
//! El módulo [`api`] re-exporta el crate `sea_orm` íntegro bajo ese alias. Úsalo cuando necesites
//! un tipo o función que no esté expuesto directamente en `db::*`:
//!
//! ```rust
//! use pagetop_seaorm::db::api;
//!
//! // Tipos o utilidades no incluidos en db::*:
//! let _: api::DatabaseBackend = api::DatabaseBackend::Sqlite;
//! ```
//!
//! ## Construcción de consultas en tiempo de ejecución
//!
//! El módulo [`query`] re-exporta `sea_query` para construir las sentencias SQL que se pasan a
//! [`fetch_all`] y [`fetch_one`]. Es el compañero natural de esas funciones dentro del módulo `db`:
//!
//! ```rust
//! use pagetop_seaorm::db::*;
//! use pagetop_seaorm::db::query::*;
//!
//! async fn example() -> Result<(), DbErr> {
//!     let stmt = Query::select()
//!         .column(Asterisk)
//!         .from(Alias::new("users"))
//!         .to_owned();
//!     let rows = fetch_all(&stmt).await?;
//!     Ok(())
//! }
//! ```

pub use sea_orm::prelude::*;

pub use sea_orm::{
    ActiveValue, DatabaseTransaction, ExecResult, QueryOrder, QuerySelect, TransactionTrait,
};

/// Permite implementar *traits* con métodos `async`:
#[doc(inline)]
pub use async_trait;

/// Re-exporta el crate `sea_orm` íntegro como puerta de acceso a su API completa.
///
/// Útil para tipos o utilidades que no están expuestos directamente en [`db::*`](self). La inmensa
/// mayoría de operaciones no necesitan este módulo; `db::*` cubre los casos habituales.
#[doc(inline)]
pub use sea_orm as api;

/// Re-exporta `sea_query` para construir sentencias SQL en tiempo de ejecución.
///
/// Proporciona los constructores de consultas (`Query`, `Expr`, `Alias`, ...) que se pasan a
/// [`fetch_all`] y [`fetch_one`]. Aunque [`migration`](crate::migration) expone las mismas
/// herramientas en el contexto de la definición de esquemas, `query` las sitúa donde corresponde
/// cuando se trata del acceso a la base de datos en tiempo de ejecución.
#[doc(inline)]
pub use sea_orm::sea_query as query;

/// Devuelve una referencia estática al pool de conexiones.
///
/// El pool se inicializa una sola vez al arrancar la aplicación; las llamadas posteriores devuelven
/// la misma referencia sin coste apreciable. Se puede invocar tantas veces como sea necesario sin
/// penalización.
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// let _conn: &DatabaseConnection = dbconn();
/// ```
#[inline]
pub fn dbconn() -> &'static DatabaseConnection {
    &super::DBCONN
}

/// Ejecuta una sentencia SQL en crudo y devuelve su resultado.
///
/// No construye la sentencia (INSERT, UPDATE, DELETE), sino que la recibe como una cadena ya
/// formada. Útil para SQL que el sistema de entidades no cubre. El [`ExecResult`] devuelto expone
/// [`rows_affected`](ExecResult::rows_affected) y [`last_insert_id`](ExecResult::last_insert_id)
/// (fiable en MySQL y SQLite; en PostgreSQL devuelve `0`, usa `RETURNING` con el sistema de
/// entidades si necesitas el id insertado).
///
/// > **Nota:** no sirve para SELECT porque no devuelve filas. Para leer datos usa [`fetch_all`] o
/// > [`fetch_one`].
///
/// > **Advertencia:** nunca interpoles valores externos en la cadena SQL directamente. Para
/// > sentencias con parámetros de usuario usa el sistema de entidades.
///
/// ```rust
/// use pagetop_seaorm::db::*;
///
/// async fn example() -> Result<(), DbErr> {
///     let result = execute("DELETE FROM sessions WHERE expired = 1").await?;
///     println!("Filas eliminadas: {}", result.rows_affected());
///     Ok(())
/// }
/// ```
pub async fn execute(stmt: impl Into<String>) -> Result<ExecResult, DbErr> {
    let conn = dbconn();
    let backend = conn.get_database_backend();
    conn.execute(api::Statement::from_string(backend, stmt.into()))
        .await
}

/// Ejecuta una consulta para devolver todas las filas resultantes.
///
/// Acepta cualquier tipo que implemente [`query::QueryStatementWriter`] (p. ej.
/// [`query::SelectStatement`]) y serializa la sentencia para el motor de base de datos usado antes
/// de ejecutarla. Cada fila se devuelve como un [`QueryResult`] sin tipar; extrae los valores con
/// [`QueryResult::try_get`].
///
/// Usa esta función cuando la consulta SELECT no mapea una entidad concreta (JOINs, agregaciones,
/// proyecciones parciales) o cuando necesitas control total sobre el SQL generado. Para sentencias
/// que modifican datos (INSERT, UPDATE, DELETE), usa [`execute`]. Para consultas que sí mapean a
/// una entidad, es preferible `Entity::find().all(dbconn())`.
///
/// Los valores se integran como literales escapados, no como parámetros de base de datos. Para
/// datos procedentes del usuario, el sistema de entidades es más robusto.
///
/// ```rust
/// use pagetop_seaorm::db::*;
/// use pagetop_seaorm::db::query::*;
///
/// async fn example() -> Result<(), DbErr> {
///     let stmt = Query::select()
///         .column(Asterisk)
///         .from(Alias::new("users"))
///         .to_owned();
///     let rows = fetch_all(&stmt).await?;
///     for row in rows {
///         let name: String = row.try_get("", "name")?;
///         println!("{name}");
///     }
///     Ok(())
/// }
/// ```
pub async fn fetch_all<Q: query::QueryStatementWriter>(
    stmt: &Q,
) -> Result<Vec<QueryResult>, DbErr> {
    let conn = dbconn();
    let backend = conn.get_database_backend();
    conn.query_all(api::Statement::from_string(
        backend,
        match backend {
            api::DatabaseBackend::MySql => stmt.to_string(query::MysqlQueryBuilder),
            api::DatabaseBackend::Postgres => stmt.to_string(query::PostgresQueryBuilder),
            api::DatabaseBackend::Sqlite => stmt.to_string(query::SqliteQueryBuilder),
        },
    ))
    .await
}

/// Ejecuta una consulta y devuelve sólo la primera fila, si existe.
///
/// Funciona igual que [`fetch_all`] pero devuelve la primera fila si existe, o `None` si la
/// consulta no produce resultados. Está diseñada para sentencias SELECT; para modificar datos sin
/// entidad mapeada, usa [`execute`].
///
/// Si la consulta puede devolver varias filas, se recomienda incluir `LIMIT 1` en la sentencia
/// para que el motor detenga la búsqueda en cuanto encuentre la primera fila y no recupere
/// resultados que se descartarán de todas formas.
///
/// Usa esta función cuando la consulta SELECT no mapea una entidad concreta (JOINs, agregaciones,
/// proyecciones parciales) o cuando necesitas control total sobre el SQL generado. Para consultas
/// que sí mapean a una entidad, es preferible `Entity::find().one(dbconn())`.
///
/// Los valores se integran como literales escapados, no como parámetros de base de datos. Para
/// datos procedentes del usuario, el sistema de entidades es más robusto.
///
/// ```rust
/// use pagetop_seaorm::db::*;
/// use pagetop_seaorm::db::query::*;
///
/// async fn example() -> Result<(), DbErr> {
///     let stmt = Query::select()
///         .column(Asterisk)
///         .from(Alias::new("users"))
///         .and_where(Expr::col(Alias::new("id")).eq(1))
///         .to_owned();
///     if let Some(row) = fetch_one(&stmt).await? {
///         let name: String = row.try_get("", "name")?;
///         println!("{name}");
///     }
///     Ok(())
/// }
/// ```
pub async fn fetch_one<Q: query::QueryStatementWriter>(
    stmt: &Q,
) -> Result<Option<QueryResult>, DbErr> {
    let conn = dbconn();
    let backend = conn.get_database_backend();
    conn.query_one(api::Statement::from_string(
        backend,
        match backend {
            api::DatabaseBackend::MySql => stmt.to_string(query::MysqlQueryBuilder),
            api::DatabaseBackend::Postgres => stmt.to_string(query::PostgresQueryBuilder),
            api::DatabaseBackend::Sqlite => stmt.to_string(query::SqliteQueryBuilder),
        },
    ))
    .await
}
