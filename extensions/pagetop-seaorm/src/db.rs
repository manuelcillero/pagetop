//! Definición de entidades y acceso a la base de datos.
//!
//! Agrupa los *traits*, macros y tipos del sistema de entidades de SeaORM, junto con las funciones
//! [`dbconn`], [`execute`], [`fetch_all`], [`fetch_one`] y [`paginate`], en una sola importación:
//!
//! ```rust,no_run
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
//! se necesita SQL en crudo con parámetros reales, se puede construir un [`sea_orm::Statement`]
//! directamente con [`sea_orm::Statement::from_sql_and_values`].
//!
//! # Tipos esenciales
//!
//! Destacan los siguientes elementos de uso más frecuente:
//!
//! - **Acceso**: [`DatabaseConnection`], [`dbconn`] (para obtener el pool de conexiones).
//! - **Consultas**: [`EntityTrait`], [`QueryFilter`], [`QueryOrder`], [`QuerySelect`].
//! - **Transacciones**: [`TransactionTrait`], [`DatabaseTransaction`], [`flatten_txn_err`]
//!   (para convertir [`TransactionError<E>`] a `E`).
//! - **Modelos activos**: [`ActiveModelTrait`], [`ActiveValue`] ([`ActiveValue::Set`],
//!   [`ActiveValue::Unchanged`], [`ActiveValue::NotSet`]).
//! - **Macros de derivación**: [`DeriveEntityModel`], [`DeriveColumn`], [`DerivePrimaryKey`],
//!   [`DeriveRelation`], [`EnumIter`].
//! - **Errores**: [`DbErr`], [`TransactionError`].
//! - **Resultados**: [`QueryResult`] (filas sin tipar), [`ExecResult`] (INSERT/UPDATE/DELETE),
//!   [`Paginated`] (página de resultados).
//!
//! # Definir una entidad
//!
//! ```rust,no_run
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
//! # Operaciones CRUD
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
//! # Paginación
//!
//! [`paginate`] ejecuta una consulta paginada sobre una entidad y devuelve un [`Paginated`] con los
//! elementos de la página junto con su metadata (`total`, `page`, `per_page`, `total_pages`). Es el
//! camino habitual para listados administrables (usuarios, roles...).
//!
//! Cuando cada elemento necesita enriquecerse con datos de otra tabla que no vienen incluidos en
//! la propia consulta paginada (una colección asociada, un conteo relacionado...),
//! [`Paginated::map_items`] aplica esa transformación de forma asíncrona y falible sin perder la
//! metadata de paginación ya calculada.
//!
//! # Acceso completo a SeaORM
//!
//! Este módulo re-exporta el crate `sea_orm` íntegro. Úsalo cuando necesites un tipo o función que
//! no esté expuesto directamente en `db::*`:
//!
//! ```rust,no_run
//! use pagetop_seaorm::db::sea_orm;
//!
//! // Tipos o utilidades no incluidos en db::*:
//! let _: sea_orm::DatabaseBackend = sea_orm::DatabaseBackend::Sqlite;
//! ```
//!
//! # Construcción de consultas en tiempo de ejecución
//!
//! El módulo [`query`] re-exporta `sea_query` para construir las sentencias SQL que se pasan a
//! [`fetch_all`] y [`fetch_one`]. Es el compañero natural de esas funciones dentro del módulo `db`:
//!
//! ```rust,no_run
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

// Re-exporta el crate `sea_orm` íntegro como puerta de acceso a su API completa.
//
// Útil para tipos o utilidades que no están expuestos directamente en `db::*`; la inmensa mayoría
// de operaciones no necesitan este módulo, `db::*` cubre los casos habituales.
//
// Por otro lado, este re-export es funcionalmente necesario para que los derive de `sea-orm-macros`
// (`DeriveEntityModel`, `DeriveRelation`...) compilen sin declarar `sea-orm` como dependencia
// directa en el `Cargo.toml` de la extensión o aplicación. Los derive generan rutas `sea_orm::...`
// (vía `quote!`, sin `Span::mixed_site()`), que resuelven contra cualquier item `sea_orm` visible
// en el módulo que invoca la macro. Basta con declarar `use pagetop_seaorm::db::*;` en cada entidad
// para compilar sin errores.
#[doc(hidden)]
pub use sea_orm;

pub use sea_orm::{
    ActiveValue, Condition, DatabaseTransaction, DbBackend, ExecResult, NotSet, Order, QueryOrder,
    QuerySelect, Set, TransactionError, TransactionTrait, Unchanged,
};

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
    super::DBCONN
        // La inicialización requiere async (`Database::connect().await`). OnceLock asigna el valor
        // con `.set()` en `Extension::initialize()` (contra LazyLock que sólo admite uso síncrono).
        .get()
        // Este texto no se multiplica al hacer `#[inline]`, es un `&'static str` que el compilador
        // almacena una vez. Sólo se duplica una comprobación de nulo y un salto condicional.
        .expect("Database not initialized: SeaORM extension must be listed as a dependency")
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
    let conn = dbconn();
    let backend = conn.get_database_backend();
    conn.execute(sea_orm::Statement::from_string(backend, stmt.into()))
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
/// ```rust,no_run
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
    conn.query_all(sea_orm::Statement::from_string(
        backend,
        match backend {
            sea_orm::DatabaseBackend::MySql => stmt.to_string(query::MysqlQueryBuilder),
            sea_orm::DatabaseBackend::Postgres => stmt.to_string(query::PostgresQueryBuilder),
            sea_orm::DatabaseBackend::Sqlite => stmt.to_string(query::SqliteQueryBuilder),
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
/// ```rust,no_run
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
    conn.query_one(sea_orm::Statement::from_string(
        backend,
        match backend {
            sea_orm::DatabaseBackend::MySql => stmt.to_string(query::MysqlQueryBuilder),
            sea_orm::DatabaseBackend::Postgres => stmt.to_string(query::PostgresQueryBuilder),
            sea_orm::DatabaseBackend::Sqlite => stmt.to_string(query::SqliteQueryBuilder),
        },
    ))
    .await
}

/// Convierte el error de una transacción (`TransactionError<E>`) al propio tipo `E`.
///
/// [`TransactionTrait::transaction`] puede fallar de dos formas distintas:
/// [`TransactionError::Connection`] si el fallo ocurre en la propia transacción (conexión,
/// `BEGIN`/`COMMIT`/`ROLLBACK`...), o [`TransactionError::Transaction`] si el fallo es el error que
/// devolvió la clausura. `flatten_txn_err()` convierte ambos casos al mismo tipo `E` (usando
/// `From<DbErr>` para el primero), de modo que el resultado se pueda propagar con
/// `.map_err(flatten_txn_err)?` sin distinguir el origen del error.
///
/// Requiere que el tipo de error propio de la aplicación implemente `From<DbErr>`, lo habitual con
/// `#[derive(thiserror::Error)]` y `#[from]`.
///
/// Uso directo, en el punto donde se resuelve la transacción, sin nada más que declarar:
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// async fn example() -> Result<(), DbErr> {
///     dbconn()
///         .transaction::<_, (), DbErr>(|_txn| Box::pin(async move { Ok(()) }))
///         .await
///         .map_err(flatten_txn_err)
/// }
/// ```
///
/// Si una aplicación hace transacciones en muchos puntos con su propio tipo de error, puede delegar
/// en `flatten_txn_err()` una sola vez mediante `impl From<TransactionError<E>> for E` (legal
/// porque `E` es un tipo local de la aplicación, no genérico) y despreocuparse de `.map_err()` en
/// el resto de llamadas, propagando el error con `?` directamente:
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// #[derive(Debug)]
/// struct MyError(DbErr);
///
/// impl std::fmt::Display for MyError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "database error: {}", self.0)
///     }
/// }
///
/// impl From<DbErr> for MyError {
///     fn from(err: DbErr) -> Self {
///         MyError(err)
///     }
/// }
///
/// impl From<TransactionError<MyError>> for MyError {
///     fn from(err: TransactionError<MyError>) -> Self {
///         flatten_txn_err(err)
///     }
/// }
///
/// async fn example() -> Result<(), MyError> {
///     dbconn()
///         .transaction::<_, (), MyError>(|_txn| Box::pin(async move { Ok(()) }))
///         .await?;
///     Ok(())
/// }
/// ```
pub fn flatten_txn_err<E: From<DbErr>>(err: TransactionError<E>) -> E {
    match err {
        TransactionError::Connection(db_err) => db_err.into(),
        TransactionError::Transaction(err) => err,
    }
}

// **< Paginated / paginate >***********************************************************************

/// Página de resultados de una consulta paginada.
pub struct Paginated<T> {
    /// Elementos de esta página.
    pub items: Vec<T>,
    /// Número total de registros que cumplen la consulta, sin paginar.
    pub total: u64,
    /// Página actual, empezando en `1`.
    pub page: u64,
    /// Número de elementos por página.
    pub per_page: u64,
    /// Número total de páginas.
    pub total_pages: u64,
}

// Implementación manual en lugar de `#[derive(Default)]`, que exigiría un `T: Default` innecesario,
// ya que una página vacía no requiere que el tipo de elemento lo sea.
impl<T> Default for Paginated<T> {
    fn default() -> Self {
        Paginated {
            items: Vec::new(),
            total: 0,
            page: 1,
            per_page: 1,
            total_pages: 1,
        }
    }
}

impl<T> Paginated<T> {
    /// Transforma los elementos de la página con una función asíncrona y falible, conservando el
    /// resto de la metadata de paginación (`total`, `page`, `per_page`, `total_pages`).
    ///
    /// * `f` - función que recibe los elementos actuales (`Vec<T>`) y devuelve, de forma
    ///   asíncrona, el resultado de la transformación (`Result<Vec<U>, E>`).
    pub async fn map_items<U, E, Fut>(
        self,
        f: impl FnOnce(Vec<T>) -> Fut,
    ) -> Result<Paginated<U>, E>
    where
        Fut: std::future::Future<Output = Result<Vec<U>, E>>,
    {
        let items = f(self.items).await?;
        Ok(Paginated {
            items,
            total: self.total,
            page: self.page,
            per_page: self.per_page,
            total_pages: self.total_pages,
        })
    }
}

/// Ejecuta una consulta paginada con el sistema de entidades y retorna la página solicitada.
///
/// Añade la metadata de paginación (`total`, `total_pages`); `page` y `per_page` se ajustan a un
/// mínimo de `1`, ya que no existe la página `0` ni un tamaño de página vacío.
///
/// ```rust,no_run
/// use pagetop_seaorm::db::*;
///
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
/// #[sea_orm(table_name = "users")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: i32,
///     pub email: String,
/// }
///
/// #[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
/// pub enum Relation {}
///
/// impl ActiveModelBehavior for ActiveModel {}
///
/// async fn example() -> Result<(), DbErr> {
///     let page = paginate(Entity::find(), 1, 20).await?;
///     println!("{} usuarios en {} páginas", page.total, page.total_pages);
///     Ok(())
/// }
/// ```
pub async fn paginate<E>(
    select: Select<E>,
    page: u64,
    per_page: u64,
) -> Result<Paginated<E::Model>, DbErr>
where
    E: EntityTrait,
    E::Model: sea_orm::FromQueryResult + Send + Sync,
{
    let per_page = per_page.max(1);
    let page = page.max(1);
    let paginator = select.paginate(dbconn(), per_page);
    let sea_orm::ItemsAndPagesNumber {
        number_of_items: total,
        number_of_pages: total_pages,
    } = paginator.num_items_and_pages().await?;
    let items = paginator.fetch_page(page.saturating_sub(1)).await?;
    Ok(Paginated {
        items,
        total,
        page,
        per_page,
        total_pages,
    })
}
