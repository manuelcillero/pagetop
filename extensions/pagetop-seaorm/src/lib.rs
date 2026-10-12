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

    async fn initialize(&self) -> Result<(), BoxError> {
        install_migrations!(m20240101_000001_create_users)?;
        Ok(())
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

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use url::Url;

use std::sync::OnceLock;

pub mod config;

pub mod db;

pub mod migration;

static DBCONN: OnceLock<DatabaseConnection> = OnceLock::new();

/// Errores que detienen el arranque de la aplicación al conectar con la base de datos o al aplicar
/// las migraciones.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    /// No se ha configurado el motor de base de datos (`database.db_type`).
    #[error("database.db_type is not configured: use \"mysql\", \"postgres\" or \"sqlite\"")]
    TypeNotConfigured,
    /// No se puede construir la URL de conexión con los ajustes de la sección `[database]`.
    #[error("invalid database URL: check db_host and db_name in config")]
    InvalidUrl,
    /// No se puede conectar con la base de datos.
    #[error("cannot connect to database \"{name}\"")]
    Connection {
        /// Nombre de la base de datos (`database.db_name`).
        name: String,
        /// Error devuelto al conectar.
        #[source]
        source: DbErr,
    },
    /// Una migración, o su reversión, ha fallado.
    #[error("migration failed")]
    Migration(#[source] DbErr),
}

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

    fn settings(&self) -> Option<&'static dyn LoadSettings> {
        Some(&config::SETTINGS)
    }

    async fn initialize(&self) -> Result<(), BoxError> {
        // Preparar la aplicación varias veces en el mismo proceso (las pruebas lo hacen) reutiliza
        // la conexión ya abierta.
        if DBCONN.get().is_none() {
            let conn = connect().await?;
            // Si otra preparación simultánea se adelanta, se queda la suya y se descarta ésta.
            let _ = DBCONN.set(conn);
        }
        Ok(())
    }
}

// Abre el pool de conexiones con los ajustes de la sección `[database]`.
async fn connect() -> Result<DatabaseConnection, DbError> {
    let settings = &*config::SETTINGS;
    let db_uri: String = match settings.db_type {
        config::DbType::Unset => return Err(DbError::TypeNotConfigured),
        config::DbType::Mysql | config::DbType::Postgres => {
            let scheme = if matches!(settings.db_type, config::DbType::Mysql) {
                "mysql"
            } else {
                "postgres"
            };
            let mut uri = Url::parse(&format!(
                "{}://{}/{}",
                scheme, settings.db_host, settings.db_name
            ))
            .map_err(|_| DbError::InvalidUrl)?;
            // Sin servidor en la URL (p. ej. con `db_host` vacío) no se pueden fijar el usuario,
            // la contraseña ni el puerto.
            uri.set_username(&settings.db_user)
                .map_err(|()| DbError::InvalidUrl)?;
            // https://github.com/launchbadge/sqlx/issues/1624
            uri.set_password(Some(&settings.db_pass))
                .map_err(|()| DbError::InvalidUrl)?;
            if let Some(port) = settings.db_port {
                uri.set_port(Some(port)).map_err(|()| DbError::InvalidUrl)?;
            }
            uri.to_string()
        }
        config::DbType::Sqlite => format!("sqlite://{}", settings.db_name),
    };

    trace::info!(
        "Connecting to database \"{}\" using a pool of {} connections",
        settings.db_name,
        settings.max_pool_size
    );
    let mut options = ConnectOptions::new(db_uri);
    options.max_connections(settings.max_pool_size);
    Database::connect(options)
        .await
        .map_err(|source| DbError::Connection {
            name: settings.db_name.clone(),
            source,
        })
}
