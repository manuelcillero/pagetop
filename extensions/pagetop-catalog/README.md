<div align="center">

<h1>PageTop Catalog</h1>

<p>Catálogos administrables para aplicaciones <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-catalog?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-catalog)
[![Crates.io](https://img.shields.io/crates/v/pagetop-catalog.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-catalog)
[![Descargas](https://img.shields.io/crates/d/pagetop-catalog.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-catalog)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-catalog#licencia)

<br>
</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Descripción general

Casi toda aplicación tiene **catálogos**: listas que administran sus usuarios y que el resto de la
aplicación usa (tipos de cliente, motivos de devolución, almacenes, colores...). Cada uno necesita
lo mismo: un listado con búsqueda, filtro y orden, un formulario de alta y edición, poder retirar
un registro sin perder los datos que lo usan, no poder borrar uno que esté en uso, quién cambió
qué, y permisos para consultarlo y para gestionarlo.

Con `pagetop-catalog` todo eso se escribe una sola vez. Cada catálogo tiene **su propia tabla**,
con sus tipos, restricciones y claves foráneas, y se declara con un `Catalog`: el nombre, la
sección del panel en la que aparece, el título y sus columnas propias (`Field`). La extensión
genera el resto:

- **Listado** paginado en el panel de administración, con búsqueda por código o nombre (`*` como
  comodín), filtro por estado (activos, inactivos o todos) y por los campos que se indiquen, y
  orden por código, nombre o peso; se actualiza con HTMX sin recargar la página.
- **Alta y edición** con un formulario generado a partir de las columnas, con su ayuda y su
  validación. El código no cambia una vez creado. Dos usuarios que editan a la vez no se pisan: la
  versión del registro (`row_version`) rechaza el cambio que llega en segundo lugar.
- **Desactivar y reactivar**: un registro inactivo no se ofrece para registros nuevos, pero sigue
  valiendo en los que ya lo usan: su formulario lo ofrece marcado como inactivo y lo acepta sin
  cambiar; una vez cambiado, ya no se puede volver a elegir.
- **Borrado protegido**: sólo se borra un registro que nadie usa, lo que garantizan las claves
  foráneas de la base de datos también frente a extensiones que el catálogo no conoce.
- **Salvaguardas** (`DeclareDeactivationGuards`): reglas de otras extensiones que impiden
  desactivar o borrar un registro mientras se usa.
- **Registro de los cambios** (`DeclareChangeRecorders`): cada cambio, con quién lo hizo y qué
  columnas cambiaron, se entrega a la aplicación dentro de la misma transacción, para su pista de
  auditoría, y el historial de cada registro se muestra en el panel.
- **Permisos**: uno para consultar el catálogo y otro para gestionarlo, declarados en el catálogo
  de permisos de PageTop (`<prefijo>:view_<catálogo>` y `<prefijo>:manage_<catálogo>`).
- **Traducciones** del nombre a los idiomas de la aplicación, si el catálogo las admite.
- **Exportación e importación en CSV**: la importación muestra antes qué hará cada fila y se aplica
  entera o no se aplica.

Cada catálogo se ajusta a su caso: puede no tener código (`Catalog::without_code()`) y
identificarse por su nombre o por un conjunto de columnas únicas (`Catalog::with_unique()`),
calificar el nombre con el de otro registro donde no basta (`Catalog::with_qualifier()`, como
«ubicación - sede»), usar los permisos de la aplicación en lugar de los suyos
(`Catalog::with_permissions()`) o un mensaje propio para las restricciones únicas de otras
tablas (`Catalog::with_constraint_message()`). Las columnas propias pueden ser texto, texto
largo, sí o no, enteros, decimales, opciones fijas o referencias a otro catálogo (`FieldKind`).

## Uso

Añade `pagetop-catalog` a tu proyecto:

```toml
[dependencies]
pagetop-catalog = "..."
```

Crea la tabla en una migración con `schema::catalog_table()`, que añade las columnas comunes, y
añade las propias:

```rust
use pagetop_catalog::schema::{CommonColumns, catalog_table};
use pagetop_seaorm::migration::*;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                catalog_table(
                    Colors::Table,
                    CommonColumns {
                        translations: true,
                        ..Default::default()
                    },
                )
                .col(string_len_null(Colors::Hex, 7))
                .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Colors {
    Table,
    Hex,
}
```

Y declara el catálogo, con la sección del panel en la que aparece:

```rust
use pagetop::prelude::*;
use pagetop_catalog::{Catalog, CatalogBag, DeclareCatalogs, Field};

fn declare_sections(bag: &mut AdminSectionBag) {
    bag.add(AdminSection::new("shop", Lc::n("Shop")));
}

fn declare_catalogs(bag: &mut CatalogBag) {
    bag.add(
        Catalog::new("colors", "shop", Lc::n("Colors"))
            .with_permission_prefix("shop")
            .with_translations(true)
            .with_field(
                Field::text("hex", Lc::n("Hex code"))
                    .with_help(Lc::n("Optional. Six hexadecimal digits after #."))
                    .with_max_len(7)
                    .with_in_list(true),
            ),
    );
}

struct Shop;

#[async_trait]
impl Extension for Shop {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_catalog::Catalogs]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::admin::DeclareAdminSections::new(declare_sections),
            DeclareCatalogs::new(declare_catalogs),
        ]
    }
}
```

El listado queda en `/admin/shop/colors`. Para leer los registros desde el resto de la aplicación,
cada extensión usa su propia entidad de SeaORM sobre la misma tabla.

## Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.

## Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
