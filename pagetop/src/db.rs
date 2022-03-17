pub use url::Url as DbUri;

pub use sea_orm::{
    DbErr,
    DatabaseConnection as DbConn,
};

pub mod entity {
    pub use sea_orm::entity::prelude::*;
}

pub mod migration {
    pub use sea_schema::migration::prelude::*;
}

#[macro_export]
macro_rules! boxed_migration {
    ( $migration_module:ident ) => {{
        Box::new(migration::$migration_module::Migration)
    }};
}
