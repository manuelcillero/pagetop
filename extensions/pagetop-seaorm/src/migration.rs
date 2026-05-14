//! API para definir y ejecutar migraciones de base de datos.
//!
//! Re-exporta los tipos de SeaORM necesarios para escribir migraciones y ofrece las macros
//! [`crate::install_migrations`] y [`crate::uninstall_migrations`] para aplicarlas o revertirlas al
//! arrancar la extensión.
//!
//! ```rust,ignore
//! use pagetop_seaorm::db::*;
//! use pagetop_seaorm::migration::*;
//! ```

// **< Adaptación de `sea-orm-migration` (ver §Créditos en README.md) >*****************************

//pub mod cli;
pub mod connection;
pub mod manager;
pub mod migrator;
//pub mod prelude;
pub mod schema;
pub mod seaql_migrations;
//pub mod util;

pub use connection::*;
pub use manager::*;
//pub use migrator::*;

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

pub use migrator::MigratorTrait;
pub use schema::*;
pub use sea_orm::sea_query::*;
pub use sea_orm::DeriveIden;

use pagetop::core::TypeInfo;
use pagetop::trace;

impl<M: MigrationTrait> MigrationName for M {
    fn name(&self) -> &str {
        TypeInfo::NameTo(-2).of::<M>()
    }
}

pub type MigrationItem = Box<dyn MigrationTrait>;

pub trait MigratorBase {
    fn run_up();

    fn run_down();
}

#[rustfmt::skip]
impl<M: MigratorTrait> MigratorBase for M {
    fn run_up() {
        if let Err(e) = super::run_now(Self::up(SchemaManagerConnection::Connection(&super::DBCONN), None)) {
            trace::error!("Migration upgrade failed ({})", e);
        };
    }

    fn run_down() {
        if let Err(e) = super::run_now(Self::down(SchemaManagerConnection::Connection(&super::DBCONN), None)) {
            trace::error!("Migration downgrade failed ({})", e);
        };
    }
}

/// Aplica las migraciones pendientes al arrancar una extensión.
///
/// Recibe uno o más módulos de migración y ejecuta el método `up` de los que aún no estén
/// registrados en la tabla `seaql_migrations`. Se invoca habitualmente desde
/// [`Extension::initialize`](pagetop::core::extension::Extension::initialize).
///
/// ```rust,ignore
/// impl Extension for MyExt {
///     fn initialize(&self) {
///         install_migrations!(
///             m20240101_000001_create_users_table,
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
/// Ejecuta el método `down` de cada migración indicada. Si alguna no implementa `down`,
/// detiene el proceso con un error. Complementario a [`crate::install_migrations`].
///
/// ```rust,ignore
/// impl Extension for MyExt {
///     fn uninitialize(&self) {
///         uninstall_migrations!(
///             m20240101_000001_create_users_table,
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
