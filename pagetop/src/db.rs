pub use url::Url as DbUri;

pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};


// El siguiente módulo migration es una versión simplificada del módulo sea_orm_migration (v0.9.1)
// https://github.com/SeaQL/sea-orm/tree/0.9.1/sea-orm-migration para evitar los errores generados
// por el paradigma modular de PageTop. Se copian los siguientes archivos del original:
//
//    lib.rs => db/migration.rs       (descartando el uso de algunos módulos y exportaciones)
//    manager.rs => db/migration/manager.rs
//    migrator.rs => db/migration/migrator.rs         (suprimiendo la gestión de los errores)
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
