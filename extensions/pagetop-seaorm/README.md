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

Para MySQL o PostgreSQL añade también `db_user`, `db_pass`, `db_host` y `db_port`.

**Declara la extensión** en tu aplicación o en la extensión que la requiera:

```rust,no_run
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
use pagetop_seaorm::db::*;

#[derive(DeriveMigrationName)]
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


## 📚 Créditos

Este *crate* se apoya en bibliotecas del ecosistema [SeaQL](https://github.com/SeaQL) como:

* [SeaORM](https://www.sea-ql.org/SeaORM), ORM asíncrono que usa internamente
  [SQLx](https://github.com/launchbadge/sqlx) para el acceso y la ejecución de consultas a la base
  de datos.

* [SeaQuery](https://github.com/SeaQL/sea-query), generador de consultas SQL sobre el que se
  construye el motor de migraciones y los *helpers* de esquema.

* [sea-schema](https://github.com/SeaQL/sea-schema), librería de introspección de esquemas SQL,
  usada por el módulo de migraciones para interrogar la estructura real de la base de datos (tablas,
  columnas, índices y claves externas).

También incorpora código adaptado de las siguientes fuentes:

* [**sea-orm-migration (v1.0.0)**](https://github.com/SeaQL/sea-orm/tree/1.0.0/sea-orm-migration):
  El módulo de migraciones (`src/db/migration/`) es una versión personalizada de
  [sea-orm-migration](https://crates.io/crates/sea-orm-migration). Se integra directamente en lugar
  de usarlo como dependencia porque su paradigma de CLI no es compatible con el ciclo de vida de las
  extensiones de PageTop, donde las migraciones deben ejecutarse durante la inicialización de cada
  extensión. Los ficheros adaptados del original son:

  | Original en `sea-orm-migration` | Observaciones                           |
  |---------------------------------|-----------------------------------------|
  | `lib.rs`                        | Excluye módulos y exportaciones del CLI |
  | `connection.rs`                 | Integración completa                    |
  | `manager.rs`                    | Integración completa                    |
  | `migrator.rs`                   | Omite la gestión de errores del CLI     |
  | `prelude.rs`                    | Excluye exportaciones del CLI           |
  | `seaql_migrations.rs`           | Integración completa                    |

* [**loco-rs/loco**](https://github.com/loco-rs/loco/blob/master/src/schema.rs): El módulo
  `src/db/migration/schema.rs`, con funciones de ayuda para definir columnas de tablas de forma
  ergonómica, está adaptado del fichero `src/schema.rs` del proyecto [Loco](https://loco.rs/).


## 🚧 Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.


## 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
