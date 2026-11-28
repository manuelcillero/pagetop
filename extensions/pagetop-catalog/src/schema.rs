//! Columnas comunes de las tablas de los catálogos.
//!
//! Toda tabla de un catálogo tiene estas columnas, con estos nombres:
//!
//! | Columna        | Tipo                  | Para qué                                              |
//! |----------------|-----------------------|-------------------------------------------------------|
//! | `id`           | `INTEGER`             | Clave técnica, autoincremental.                       |
//! | `code`         | `VARCHAR(n)`, único   | Código, visible para el usuario; no cambia una vez creado. Sólo si el catálogo tiene código. |
//! | `name`         | `VARCHAR(255)`        | Nombre, en el idioma predeterminado de la aplicación. |
//! | `name_i18n`    | `JSON`, opcional      | Traducciones del nombre (`{"en-US": "..."}`), sólo si el catálogo las admite. |
//! | `is_active`    | `BOOLEAN`             | Uno inactivo no se ofrece para registros nuevos, pero sigue valiendo en los que ya lo usan. |
//! | `weight`       | `INTEGER`             | Orden de presentación (menor = antes).                |
//! | `created_at`   | fecha y hora, UTC     | Alta.                                                 |
//! | `updated_at`   | fecha y hora, UTC     | Último cambio.                                        |
//! | `updated_by`   | `INTEGER`, opcional   | Usuario del último cambio.                            |
//! | `row_version`  | `INTEGER`             | Versión del registro: dos ediciones simultáneas no se pisan. |
//!
//! [`catalog_table()`] las crea en la migración del catálogo, y las columnas propias se añaden a
//! continuación. Las fechas son `DATETIME` (sin zona horaria) en UTC, y no `TIMESTAMP`, que en
//! MySQL termina en 2038.
//!
//! # Ejemplo
//!
//! ```rust,no_run
//! use pagetop_catalog::schema::{CommonColumns, catalog_table};
//! use pagetop_seaorm::migration::*;
//!
//! pub struct Migration;
//!
//! #[pagetop::async_trait]
//! impl MigrationTrait for Migration {
//!     async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
//!         manager
//!             .create_table(
//!                 catalog_table(Colors::Table, CommonColumns::default())
//!                     .col(string_len_null(Colors::Hex, 7))
//!                     .to_owned(),
//!             )
//!             .await
//!     }
//! }
//!
//! #[derive(DeriveIden)]
//! enum Colors {
//!     Table,
//!     Hex,
//! }
//! ```
//!
//! La columna `updated_by` no tiene clave foránea, porque `pagetop-catalog` no sabe qué extensión
//! gestiona los usuarios; la aplicación puede añadirla en la misma migración.

use pagetop_seaorm::migration::{
    Alias, ColumnDef, Expr, IntoIden, Table, TableCreateStatement, boolean, date_time, integer,
    integer_null, json_binary_null, pk_auto, string_len, string_len_uniq,
};

/// Longitud máxima del nombre de un registro.
pub const NAME_MAX_LEN: usize = 255;

// Nombres de las columnas comunes.
pub(crate) const ID: &str = "id";
pub(crate) const CODE: &str = "code";
pub(crate) const NAME: &str = "name";
pub(crate) const NAME_I18N: &str = "name_i18n";
pub(crate) const IS_ACTIVE: &str = "is_active";
pub(crate) const WEIGHT: &str = "weight";
pub(crate) const CREATED_AT: &str = "created_at";
pub(crate) const UPDATED_AT: &str = "updated_at";
pub(crate) const UPDATED_BY: &str = "updated_by";
pub(crate) const ROW_VERSION: &str = "row_version";

/// Columnas comunes que varían de un catálogo a otro. Ver [`catalog_table()`].
#[derive(Clone, Copy, Debug)]
pub struct CommonColumns {
    /// Longitud máxima del código, o `None` si el catálogo no tiene código.
    pub code_max_len: Option<u32>,
    /// Si el nombre se traduce a otros idiomas (columna `name_i18n`).
    pub translations: bool,
}

impl Default for CommonColumns {
    /// Código de hasta 16 caracteres y sin traducciones.
    fn default() -> Self {
        CommonColumns {
            code_max_len: Some(16),
            translations: false,
        }
    }
}

/// Sentencia que crea la tabla `table` con las columnas comunes de un catálogo.
///
/// Se le añaden después las columnas propias (`.col(...)`) y, si hace falta, las claves foráneas y
/// los índices únicos de [`Catalog::with_unique()`](crate::Catalog::with_unique).
pub fn catalog_table(
    table: impl IntoIden + 'static,
    columns: CommonColumns,
) -> TableCreateStatement {
    let mut stmt = Table::create();
    stmt.table(table).col(pk_auto(Alias::new(ID)));
    if let Some(len) = columns.code_max_len {
        stmt.col(string_len_uniq(Alias::new(CODE), len));
    }
    stmt.col(string_len(Alias::new(NAME), NAME_MAX_LEN as u32));
    if columns.translations {
        stmt.col(json_binary_null(Alias::new(NAME_I18N)));
    }
    stmt.col(boolean(Alias::new(IS_ACTIVE)).default(true))
        .col(integer(Alias::new(WEIGHT)).default(0))
        .col(date_time(Alias::new(CREATED_AT)))
        .col(date_time(Alias::new(UPDATED_AT)))
        .col(integer_null(Alias::new(UPDATED_BY)))
        .col(row_version());
    stmt
}

fn row_version() -> ColumnDef {
    let mut col = integer(Alias::new(ROW_VERSION));
    col.default(Expr::val(1));
    col
}
