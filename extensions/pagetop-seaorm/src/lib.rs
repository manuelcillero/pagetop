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

**Añade la dependencia** a tu `Cargo.toml`:

- Si estás desarrollando una **aplicación**, activa el motor de base de datos que necesites:

  ```toml
  [features]
  default  = ["postgres"]
  mysql    = ["pagetop-seaorm/mysql"]
  postgres = ["pagetop-seaorm/postgres"]
  sqlite   = ["pagetop-seaorm/sqlite"]

  [dependencies]
  pagetop-seaorm = { version = "..." }
  ```

  Las *features* disponibles son `mysql`, `postgres` y `sqlite`. Gracias a la unificación de
  *features* de Cargo, activar el motor aquí lo propaga a todos los *crates* del árbol de
  dependencias que usen `pagetop-seaorm`; no es necesario que cada extensión lo reexporte.

- Si estás escribiendo una **extensión**, declara la dependencia sin *features*:

  ```toml
  [dependencies]
  pagetop-seaorm = { version = "..." }
  ```

  La selección del motor es responsabilidad de la aplicación que use la extensión, no de la
  extensión misma.

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

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_seaorm::SeaORM,
        ]
    }

    async fn initialize(&self) {
        install_migrations!(m20240101_000001_create_users);
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&MyApp).await.run().await
}
```

**Escribe las migraciones** usando la API de [`migration`]:

```rust,no_run
// src/migration/m20240101_000001_create_users.rs
use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
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

include_locales!(LOCALES_SEAORM);

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use url::Url;

use std::sync::OnceLock;

pub mod config;

pub mod db;

pub mod migration;

static DBCONN: OnceLock<DatabaseConnection> = OnceLock::new();

/// Implementa la extensión.
pub struct SeaORM;

#[async_trait]
impl Extension for SeaORM {
    fn name(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_SEAORM)
    }

    fn description(&self) -> Lc {
        Lc::t("extension_description", &LOCALES_SEAORM)
    }

    async fn initialize(&self) {
        trace::info!(
            "Connecting to database \"{}\" using a pool of {} connections",
            &config::SETTINGS.database.db_name,
            &config::SETTINGS.database.max_pool_size
        );

        let db_uri: String = match config::SETTINGS.database.db_type {
            config::DbType::Unset => panic!(
                "database.db_type is not configured: use \"mysql\", \"postgres\" or \"sqlite\""
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
                tmp_uri
                    // https://github.com/launchbadge/sqlx/issues/1624
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

        let conn = Database::connect::<ConnectOptions>({
            let mut db_opt = ConnectOptions::new(db_uri);
            db_opt.max_connections(config::SETTINGS.database.max_pool_size);
            db_opt
        })
        .await
        .expect("Failed to connect to database");

        DBCONN.set(conn).expect("DBCONN already initialized");
    }
}
