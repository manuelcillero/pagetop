//! API para definir y ejecutar migraciones de base de datos.
//!
//! Cuando una extensión necesita persistir datos en una base de datos usando `pagetop_seaorm`,
//! define sus migraciones en un submódulo `migration/` y las aplica al arrancar con la macro
//! [`install_migrations!`](crate::install_migrations).
//!
//! Con una sola importación tienes todo lo necesario:
//!
//! ```rust
//! use pagetop_seaorm::migration::*;
//! ```
//!
//! # Convención de nombrado
//!
//! Cada migración es un módulo con el formato `m<YYYYMMDD>_<NNNNNN>_<nombre_descriptivo>`. El
//! prefijo numérico garantiza el orden cronológico de aplicación:
//!
//! ```text
//! src/
//! └── migration/
//!     ├── m20240101_000001_create_users.rs
//!     └── m20240115_000002_add_email_index.rs
//! ```
//!
//! # Estructura de una migración
//!
//! Cada archivo define un *struct* `Migration` que implementa [`MigrationTrait`]. El método `up`
//! aplica el cambio; `down` lo revierte. Si no se implementa, devuelve un error; es **obligatorio**
//! implementarlo si la extensión usa [`uninstall_migrations!`](crate::uninstall_migrations):
//!
//! ```rust,no_run
//! // src/migration/m20240101_000001_create_users.rs
//! use pagetop_seaorm::migration::*;
//!
//! pub struct Migration;
//!
//! #[async_trait::async_trait]
//! impl MigrationTrait for Migration {
//!     async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
//!         manager
//!             .create_table(
//!                 table_auto(Users::Table)
//!                     .col(pk_auto(Users::Id))
//!                     .col(string_uniq(Users::Email))
//!                     .col(string(Users::Name))
//!                     .to_owned(),
//!             )
//!             .await
//!     }
//! }
//!
//! #[derive(DeriveIden)]
//! enum Users {
//!     Table,
//!     Id,
//!     Email,
//!     Name,
//! }
//! ```
//!
//! # Seguimiento automático
//!
//! Las migraciones se mantienen en una tabla `seaql_migrations` de la base de datos. Cada migración
//! aplicada queda registrada con su nombre y su marca de tiempo. Las migraciones ya aplicadas se
//! omiten en ejecuciones posteriores.
//!
//! # Operaciones con `SchemaManager`
//!
//! El parámetro `manager` que recibe cada migración expone los métodos necesarios para
//! modificar el esquema. Estos son los más habituales:
//!
//! | Método                                        | Acción                          |
//! |-----------------------------------------------|---------------------------------|
//! | [`SchemaManager::create_table`]               | Crea una tabla                  |
//! | [`SchemaManager::drop_table`]                 | Elimina una tabla               |
//! | [`SchemaManager::alter_table`]                | Modifica una tabla existente    |
//! | [`SchemaManager::rename_table`]               | Renombra una tabla              |
//! | [`SchemaManager::truncate_table`]             | Vacía una tabla                 |
//! | [`SchemaManager::create_index`]               | Crea un índice                  |
//! | [`SchemaManager::drop_index`]                 | Elimina un índice               |
//! | [`SchemaManager::create_foreign_key`]         | Crea una clave foránea          |
//! | [`SchemaManager::drop_foreign_key`]           | Elimina una clave foránea       |
//! | [`SchemaManager::has_table`]                  | Comprueba si existe una tabla   |
//! | [`SchemaManager::has_column`]                 | Comprueba si existe una columna |
//! | [`SchemaManager::has_index`]                  | Comprueba si existe un índice   |
//! | [`SchemaManager::create_type`] *(PostgreSQL)* | Crea un tipo personalizado      |
//! | [`SchemaManager::alter_type`] *(PostgreSQL)*  | Modifica un tipo personalizado  |
//! | [`SchemaManager::drop_type`] *(PostgreSQL)*   | Elimina un tipo personalizado   |
//!
//! # Funciones de esquema
//!
//! Las funciones de esquema disponibles simplifican la definición de columnas. Siguen el patrón
//! `<tipo>(col)` (NOT NULL), `<tipo>_null(col)` (nulable) y `<tipo>_uniq(col)` (NOT NULL + UNIQUE).
//! Algunos ejemplos:
//!
//! | Función             | SQL equivalente                                     |
//! |---------------------|-----------------------------------------------------|
//! | `table_auto(tabla)` | `CREATE TABLE IF NOT EXISTS` + timestamps           |
//! | `pk_auto(col)`      | `INTEGER NOT NULL PRIMARY KEY` (con autoincremento) |
//! | `pk_uuid(col)`      | `UUID NOT NULL PRIMARY KEY`                         |
//! | `string(col)`       | `VARCHAR NOT NULL`                                  |
//! | `string_null(col)`  | `VARCHAR NULL`                                      |
//! | `string_uniq(col)`  | `VARCHAR NOT NULL UNIQUE`                           |
//! | `integer(col)`      | `INTEGER NOT NULL`                                  |
//! | `boolean(col)`      | `BOOLEAN NOT NULL`                                  |
//! | `timestamp(col)`    | `TIMESTAMP NOT NULL`                                |
//! | `uuid(col)`         | `UUID NOT NULL`                                     |
//!
//! Estas son sólo las funciones más habituales. El módulo [`schema`] define la lista completa, con
//! variantes para `decimal`, `date`, `time`, `json`, `blob`, `binary`, `array`, `enumeration`,
//! `char`, `interval` y sus formas `_null` y `_uniq`.

// **< Adaptación de `sea-orm-migration/lib.rs` (ver §Créditos en README.md) >**********************

//pub mod cli;
pub mod connection;
pub mod manager;
pub mod migrator;
//pub mod prelude;
pub mod schema;
pub mod seaql_migrations;
//pub mod util;

#[doc(inline)]
pub use connection::*;
#[doc(inline)]
pub use manager::*;
//pub use migrator::*;

/// Permite implementar *traits* con métodos `async`:
#[doc(inline)]
pub use async_trait;
//pub use sea_orm;
//pub use sea_orm::sea_query;
pub use sea_orm::DbErr;

pub trait MigrationName {
    fn name(&self) -> &str;
}

/// The migration definition
#[async_trait::async_trait]
pub trait MigrationTrait: MigrationName + Send + Sync {
    /// Define actions to perform when applying the migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>;

    /// Define actions to perform when rolling back the migration
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "Rollback not implemented for this migration".to_owned(),
        ))
    }
}

// *************************************************************************************************

#[doc(inline)]
pub use migrator::MigratorTrait;
#[doc(inline)]
pub use schema::*;
pub use sea_orm::DeriveIden;
pub use sea_orm::sea_query::*;

use pagetop::core::TypeInfo;

impl<M: MigrationTrait> MigrationName for M {
    fn name(&self) -> &str {
        // Extrae el módulo contenedor, descartando el segmento final "Migration".
        TypeInfo::NameTo(-2).of::<M>()
    }
}

/// Elemento de migración listo para incluir en la lista de un [`MigratorTrait`].
pub type MigrationItem = Box<dyn MigrationTrait>;

/// Interfaz síncrona para ejecutar migraciones desde código no asíncrono.
///
/// Todo tipo que implemente [`MigratorTrait`] obtiene esta interfaz automáticamente, incluidos los
/// tipos generados por los macros [`install_migrations!`](crate::install_migrations) y
/// [`uninstall_migrations!`](crate::uninstall_migrations).
pub trait MigratorBase {
    /// Ejecuta las migraciones pendientes en orden ascendente.
    ///
    /// Provoca un `panic!` si alguna migración falla, evitando que la aplicación arranque con un
    /// esquema de base de datos inconsistente.
    fn run_up();

    /// Revierte todas las migraciones en orden descendente.
    ///
    /// Provoca un `panic!` si alguna reversión falla.
    fn run_down();
}

impl<M: MigratorTrait> MigratorBase for M {
    fn run_up() {
        let conn = SchemaManagerConnection::Connection(&super::DBCONN);
        if let Err(e) = super::run_now(Self::up(conn, None)) {
            panic!("Migration upgrade failed: {e}");
        }
    }

    fn run_down() {
        let conn = SchemaManagerConnection::Connection(&super::DBCONN);
        if let Err(e) = super::run_now(Self::down(conn, None)) {
            panic!("Migration downgrade failed: {e}");
        }
    }
}

/// Aplica las migraciones pendientes al arrancar una extensión.
///
/// Recibe uno o más nombres de módulo de migración y ejecuta el método `up` de los que aún no estén
/// registrados en la tabla `seaql_migrations`. Se invoca habitualmente desde
/// [`Extension::initialize`](pagetop::core::extension::Extension::initialize).
///
/// **Requisito:** cada módulo de migración debe declararse como submódulo público bajo un módulo
/// `migration` accesible desde el punto de llamada, y exportar `pub struct Migration` que
/// implemente [`MigrationTrait`]. El macro genera rutas de la forma
/// `migration::<nombre>::Migration`. Estructura mínima:
///
/// En `src/migration.rs`:
/// ```rust,ignore
/// pub mod m20240101_000001_create_users;
/// pub mod m20240115_000002_add_email_index;
/// ```
///
/// En `src/lib.rs`:
/// ```rust,ignore
/// mod migration;
///
/// impl Extension for MyExt {
///     fn initialize(&self) {
///         install_migrations!(
///             m20240101_000001_create_users,
///             m20240115_000002_add_email_index,
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! install_migrations {
    ( $($migration_module:ident),+ $(,)? ) => {{
        use $crate::migration::{MigrationItem, MigratorBase, MigratorTrait};

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

/// Revierte las migraciones de una extensión en orden inverso al de su aplicación.
///
/// Ejecuta el método `down` de cada migración indicada. Si el método `down` de alguna migración
/// devuelve un error, provoca un `panic!`. Complementario a
/// [`install_migrations!`](crate::install_migrations).
///
/// **Requisito:** los módulos de migración deben declararse como en
/// [`install_migrations!`](crate::install_migrations). Todos los módulos indicados **deben
/// implementar `down`**; la implementación por defecto devuelve error, lo que provoca un pánico en
/// `run_down`.
///
/// En `src/lib.rs`:
/// ```rust,ignore
/// impl Extension for MyExt {
///     fn uninitialize(&self) {
///         uninstall_migrations!(
///             m20240101_000001_create_users,
///             m20240115_000002_add_email_index,
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! uninstall_migrations {
    ( $($migration_module:ident),+ $(,)? ) => {{
        use $crate::migration::{MigrationItem, MigratorBase, MigratorTrait};

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
