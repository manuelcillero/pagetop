use pagetop::prelude::*;

pub mod config;
pub mod db;

/// Preludio de la extensión.
pub mod prelude {
    pub use crate::db::*;
    pub use crate::install_migrations;
}

include_locales!(LOCALES_SEAORM);

/// Extensión que integra SeaORM como framework de base de datos para aplicaciones PageTop.
pub struct SeaORM;

impl Extension for SeaORM {
    fn name(&self) -> L10n {
        L10n::t("extension_name", &LOCALES_SEAORM)
    }

    fn description(&self) -> L10n {
        L10n::t("extension_description", &LOCALES_SEAORM)
    }

    fn initialize(&self) {
        std::sync::LazyLock::force(&db::DBCONN);
    }
}
