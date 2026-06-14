/*!
<div align="center">

<h1>PageTop SeaORM</h1>

<p>Proporciona a <strong>PageTop</strong> acceso basado en <a href="https://www.sea-ql.org/SeaORM">SeaORM</a> a bases de datos relacionales.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-seaorm?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-seaorm)
[![Crates.io](https://img.shields.io/crates/v/pagetop-seaorm.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-seaorm)
[![Descargas](https://img.shields.io/crates/d/pagetop-seaorm.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-seaorm)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-seaorm#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

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
use pagetop_seaorm::install_migrations;

mod migration;

struct MyApp;

impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_seaorm::SeaORM,
        ]
    }

    fn initialize(&self) {
        install_migrations!(m20240101_000001_create_users);
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&MyApp).run()?.await
}
```

**Escribe las migraciones** usando la API de [`migration`]:

```rust,no_run
// src/migration/m20240101_000001_create_users.rs
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
                    .col(string(Users::Name))
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
    Name,
}
```

**Define las entidades** en un módulo `entity/` usando las macros de derivación de [`db`]:

```rust,no_run
// src/entity/user.rs
use pagetop_seaorm::db::*;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

**Opera con la base de datos** pasando la conexión [`db::dbconn()`] a cada consulta:

```rust,ignore
use pagetop_seaorm::db::*;

// Asumiendo que existe un módulo `user` con la entidad definida arriba.
async fn example() -> Result<(), DbErr> {
    // Listar todos los registros:
    let users = user::Entity::find().all(dbconn()).await?;

    // Buscar por clave primaria:
    let found = user::Entity::find_by_id(1).one(dbconn()).await?;

    // Insertar un registro:
    let new_user = user::ActiveModel {
        email: Set("alice@example.com".to_owned()),
        name: Set("Alice".to_owned()),
        ..Default::default()
    };
    user::Entity::insert(new_user).exec(dbconn()).await?;
    Ok(())
}
```
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/assets/favicon.ico"
)]

use pagetop::prelude::*;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use url::Url;

use std::sync::LazyLock;

include_locales!(LOCALES_SEAORM);

pub mod config;

pub mod db;

pub mod migration;

// Ejecuta un *future* de forma síncrona dentro del runtime de Tokio.
//
// Usa [`tokio::task::block_in_place`] para ceder el hilo actual al código bloqueante sin detener el
// *pool* de trabajo de Tokio, y a continuación ejecuta el *future* con el *handle* del *runtime*
// activo. Requiere el *runtime* multi-hilo (predeterminado con `#[pagetop::main]`).
//
// En tests, `#[pagetop::test]` aplica `multi_thread` por defecto. Si se utiliza `#[tokio::test]`
// directamente, habría que añadir `(flavor = "multi_thread")` si el test invoca código que llame a
// esta función.
pub(crate) fn run_now<F: std::future::Future>(future: F) -> F::Output {
    tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(future))
}

pub(crate) static DBCONN: LazyLock<DatabaseConnection> = LazyLock::new(|| {
    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        &config::SETTINGS.database.db_name,
        &config::SETTINGS.database.max_pool_size
    );

    let db_uri: String = match config::SETTINGS.database.db_type {
        config::DbType::Unset => panic!(
            "database.db_type is not configured: set it to \"mysql\", \"postgres\" or \"sqlite\""
        ),
        config::DbType::Mysql | config::DbType::Postgres => {
            let scheme = if matches!(config::SETTINGS.database.db_type, config::DbType::Mysql) {
                "mysql"
            } else {
                "postgres"
            };
            let mut tmp_uri = Url::parse(&format!(
                "{}://{}/{}",
                scheme,
                &config::SETTINGS.database.db_host,
                &config::SETTINGS.database.db_name
            ))
            .expect("Invalid database URL: check db_host and db_name in config");
            tmp_uri
                .set_username(config::SETTINGS.database.db_user.as_str())
                .expect("Failed to set db_user in connection URL");
            // https://github.com/launchbadge/sqlx/issues/1624
            tmp_uri
                .set_password(Some(config::SETTINGS.database.db_pass.as_str()))
                .expect("Failed to set db_pass in connection URL");
            if let Some(port) = config::SETTINGS.database.db_port {
                tmp_uri
                    .set_port(Some(port))
                    .expect("Failed to set db_port in connection URL");
            }
            tmp_uri.to_string()
        }
        config::DbType::Sqlite => {
            format!("sqlite://{}", &config::SETTINGS.database.db_name)
        }
    };

    run_now(Database::connect::<ConnectOptions>({
        let mut db_opt = ConnectOptions::new(db_uri);
        db_opt.max_connections(config::SETTINGS.database.max_pool_size);
        db_opt
    }))
    .expect("Failed to connect to database")
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
