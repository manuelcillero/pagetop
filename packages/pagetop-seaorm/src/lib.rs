use pagetop::prelude::*;

use std::sync::LazyLock;

pub mod config;
pub mod db;

/// The package Prelude.
pub mod prelude {
    pub use crate::db;
    pub use crate::install_migrations;
}

include_locales!(LOCALES_SEAORM);

/// Implements [`PackageTrait`] and specific package API.
pub struct SeaORM;

impl PackageTrait for SeaORM {
    fn name(&self) -> L10n {
        L10n::t("package_name", &LOCALES_SEAORM)
    }

    fn description(&self) -> L10n {
        L10n::t("package_description", &LOCALES_SEAORM)
    }

    fn init(&self) {
        LazyLock::force(&db::DBCONN);
    }
}
