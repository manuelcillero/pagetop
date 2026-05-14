/*!
<div align="center">

<h1>PageTop SeaORM</h1>

<p>Proporciona a <strong>PageTop</strong> acceso basado en <a href="https://www.sea-ql.org/SeaORM">SeaORM</a> a bases de datos relacionales.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-seaorm?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-seaorm)
[![Crates.io](https://img.shields.io/crates/v/pagetop-seaorm.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-seaorm)
[![Descargas](https://img.shields.io/crates/d/pagetop-seaorm.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-seaorm)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-seaorm#licencia)

</div>

## 🧭 Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.


## ⚡️ Guía rápida

**Añade la dependencia** a tu `Cargo.toml` activando el motor de base de datos que necesites:

```toml
[dependencies]
pagetop-seaorm = { version = "...", features = ["sqlite"] }
```

Las *features* disponibles son `mysql`, `postgres` y `sqlite`.

**Configura la conexión** en el archivo de configuración de la aplicación:

```toml
[database]
db_type = "sqlite"
db_name = "my_app.db"
max_pool_size = 5
```

Para MySQL o PostgreSQL añade también `db_user`, `db_pass` y `db_host`. El campo `db_port` es
opcional; si se omite se usa el puerto predeterminado del motor.

**Declara la extensión** en tu aplicación o en la extensión que la requiera:

```rust,ignore
use pagetop::prelude::*;

struct MyApp;

impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_seaorm::SeaORM,
        ]
    }

    fn initialize(&self) {
        install_migrations!(m20240101_000001_create_users_table);
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&MyApp).run()?.await
}
```

**Escribe las migraciones** usando la API de SeaORM:

```rust,no_run
use pagetop_seaorm::migration::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Users::Table)
                    .col(pk_auto(Users::Id))
                    .col(string_uniq(Users::Email))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
}
```
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

use pagetop::prelude::*;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use url::Url;

use std::sync::LazyLock;

include_locales!(LOCALES_SEAORM);

pub mod config;

pub mod db;

pub mod migration;

pub(crate) use futures::executor::block_on as run_now;

pub(crate) static DBCONN: LazyLock<DatabaseConnection> = LazyLock::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        &config::SETTINGS.database.db_name,
        &config::SETTINGS.database.max_pool_size
    );

    let db_uri = match config::SETTINGS.database.db_type.as_str() {
        "mysql" | "postgres" => {
            let mut tmp_uri = Url::parse(
                format!(
                    "{}://{}/{}",
                    &config::SETTINGS.database.db_type,
                    &config::SETTINGS.database.db_host,
                    &config::SETTINGS.database.db_name
                )
                .as_str(),
            )
            .unwrap();
            tmp_uri
                .set_username(config::SETTINGS.database.db_user.as_str())
                .unwrap();
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri
                .set_password(Some(config::SETTINGS.database.db_pass.as_str()))
                .unwrap();
            if let Some(port) = config::SETTINGS.database.db_port {
                tmp_uri.set_port(Some(port)).unwrap();
            }
            tmp_uri
        }
        "sqlite" => Url::parse(
            format!(
                "{}://{}",
                &config::SETTINGS.database.db_type,
                &config::SETTINGS.database.db_name
            )
            .as_str(),
        )
        .unwrap(),
        _ => panic!(
            "Unrecognized database type \"{}\"",
            config::SETTINGS.database.db_type
        ),
    };

    run_now(Database::connect::<ConnectOptions>({
        let mut db_opt = ConnectOptions::new(db_uri.to_string());
        db_opt.max_connections(config::SETTINGS.database.max_pool_size);
        db_opt
    }))
    .unwrap_or_else(|_| panic!("Failed to connect to database"))
});

/// Implementa la extensión.
pub struct SeaORM;

impl Extension for SeaORM {
    fn name(&self) -> L10n {
        L10n::t("extension_name", &LOCALES_SEAORM)
    }

    fn description(&self) -> L10n {
        L10n::t("extension_description", &LOCALES_SEAORM)
    }

    fn initialize(&self) {
        std::sync::LazyLock::force(&DBCONN);
    }
}
