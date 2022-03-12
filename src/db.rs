pub use url::Url as Uri;

#[cfg(feature = "mysql")]
pub use {
    barrel::backend::MySql as Database,
    sqlx::MySqlPool as Conn,
};

#[cfg(feature = "postgres")]
pub use {
    barrel::backend::Pg as Database,
    sqlx::PgPool as Conn,
};

pub use barrel::{Migration, types};
pub use refinery::embed_migrations;
pub use refinery::Runner as Migrations;
