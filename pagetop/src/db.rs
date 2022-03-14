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
macro_rules! db_migrations {
    ( $DBCONN:ident ) => {{
        $crate::run_now({
            use $crate::db::migration::MigratorTrait;

            migration::Migrator::up($DBCONN, None)
        })
    }};
}
