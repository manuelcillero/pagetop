pub use url::Url as DbUri;

pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};

pub use sea_orm_migration::prelude::*;

pub type MigrationItem = Box<dyn MigrationTrait>;

#[macro_export]
macro_rules! pub_migration {
    ( $migration:ident ) => {
        pub struct $migration;

        impl MigrationName for $migration {
            fn name(&self) -> &str {
                crate::util::partial_type_name(module_path!(), 1)
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
