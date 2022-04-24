pub use url::Url as DbUri;

pub use sea_orm::{DatabaseConnection as DbConn, ExecResult, QueryResult};

pub use sea_schema::migration::prelude::*;

#[macro_export]
macro_rules! boxed_migration {
    ( $migration_module:ident ) => {{
        Box::new(migration::$migration_module::Migration)
    }};
}
