<div align="center">

<h1>PageTop Bootsier</h1>

<p>Tema de <strong>PageTop</strong> basado en Bootstrap para aplicar su catálogo de estilos y componentes flexibles.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-bootsier?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-bootsier)
[![Crates.io](https://img.shields.io/crates/v/pagetop-bootsier.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-bootsier)
[![Descargas](https://img.shields.io/crates/d/pagetop-bootsier.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-bootsier)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-bootsier#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

Igual que con otras extensiones, **añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-bootsier = { ... }
```

**Declara la extensión** en tu aplicación (o extensión que la requiera). Recuerda que el orden en
`dependencies()` determina la prioridad relativa frente a las otras extensiones:

```rust,no_run
use pagetop::prelude::*;

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            // ...
            &pagetop_bootsier::Bootsier,
            // ...
        ]
    }
}
```

Y **selecciona el tema en la configuración** de la aplicación:

```toml
[app]
theme = "Bootsier"
```

o **fuerza el tema por código** en una página concreta:

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;

async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_theme(&Bootsier)
        .with_child(
            Block::new()
                .with_title(Lc::l("sample_title"))
                .with_child(Html::with(|cx| html! {
                    p { (Lc::l("sample_content").using(cx)) }
                })),
        )
        .render().await
}
```

## Plantillas

Bootsier maqueta las dos plantillas de PageTop (`CoreTemplates`) con las mismas regiones (cabecera,
lateral, contenido y pie): `Standard`, la plantilla por defecto de cualquier página; y `Admin`, que
se activa creando la página con `Page::admin()` en lugar de `Page::new()` y añade el panel de
administración (ver más abajo).

```rust,no_run
use pagetop::prelude::*;

async fn about(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! {
            h1 { "Sobre nosotros" }
            p { "Texto de presentación." }
        }))
        .render().await
}
```

## Panel de administración

En la plantilla `Admin`, Bootsier sigue la composición del núcleo: en lugar del contenido pinta el
marco de la página (`AdminFrame`: migas, título, botones y pestañas). La navegación del panel
(`Nav::admin()`) la coloca la aplicación, en una barra aparte o en su barra principal, donde se
pliega con su botón en pantallas pequeñas. Todo se construye con componentes del núcleo
(`Navbar`/`Nav`/`Dropdown`, `Breadcrumb`), que Bootsier ya pinta con su aspecto.

## Selector de modo de color

`bs::theme_toggle()` crea un ítem de menú con el selector de modo de color (claro / oscuro /
automático), pensado para añadirse a cualquier `Nav` de la aplicación, no sólo a las páginas de
administración:

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

let navbar = bs::Navbar::simple().with_item(bs::navbar::Item::nav(
    bs::Nav::new()
        .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
        .with_item(bs::theme_toggle()),
));
```

## Temas hijo

Un tema hijo hereda todo de Bootsier (plantillas, componentes, estilos, *scripts* y fuentes) y
cambia su aspecto con una hoja de estilos propia que se carga detrás de la de Bootsier. Esa hoja
sólo contiene lo que cambia: Bootstrap define casi todo su aspecto con variables CSS (`--bs-*`), y
basta con redefinirlas:

```css
:root {
  --bs-border-radius: .25rem;
  --bs-link-color: #6f42c1;
  --bs-link-color-rgb: 111, 66, 193;
  --bs-link-hover-color: #59359a;
  --bs-link-hover-color-rgb: 89, 53, 154;
}
```

Las variantes de color de los componentes (`.btn-primary`, `.alert-primary`...) llevan sus propios
valores, así que se cambian redefiniendo las variables de cada componente:

```css
.btn-primary {
  --bs-btn-bg: #6f42c1;
  --bs-btn-border-color: #6f42c1;
  --bs-btn-hover-bg: #59359a;
  --bs-btn-hover-border-color: #59359a;
  --bs-btn-active-bg: #4b2d83;
  --bs-btn-active-border-color: #4b2d83;
}
```

Lo redefinido en `:root` vale también en el modo oscuro, porque gana a los valores oscuros de
Bootstrap; lo que deba ser distinto en él va dentro de `[data-bs-theme=dark] { ... }`.

El `build.rs` del tema hijo incluye la hoja en el binario con `pagetop-build`:

```rust,ignore
use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./assets/css", None)
        .with_name("my_theme_css")
        .build()
}
```

El tema la sirve con `serve_static_files!(files, [my_theme_css] => "/my-theme/css")` en
`Extension::configure_static_files()`, declara a Bootsier como padre y añade su hoja tras la de
Bootsier:

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;

pub struct MyTheme;

#[async_trait]
impl Extension for MyTheme {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }

    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&Bootsier]
    }
}

#[async_trait]
impl Theme for MyTheme {
    fn parent(&self) -> Option<ThemeRef> {
        Some(&Bootsier)
    }

    fn before_render_page_body(&self, page: &mut Page) {
        Bootsier.before_render_page_body(page);
        // Justo detrás de la hoja de Bootsier (peso -99).
        page.alter_assets(StyleSheet::from("/my-theme/css/my-theme.css").with_weight(-98));
    }
}
```

## Créditos

Este *crate* integra la biblioteca de estilos [Bootstrap 5.3.8](https://getbootstrap.com/) para
definir el comportamiento, la apariencia y los componentes de la interfaz. Bootstrap se distribuye
bajo licencia [MIT](https://github.com/twbs/bootstrap/blob/main/LICENSE).

Los iconos son los de [Bootstrap Icons 1.13.1](https://icons.getbootstrap.com/), bajo licencia
[MIT](https://github.com/twbs/icons/blob/main/LICENSE), y la tipografía es
[Source Sans 3](https://github.com/adobe-fonts/source-sans), de Adobe, bajo licencia
[SIL Open Font License 1.1](https://github.com/adobe-fonts/source-sans/blob/release/LICENSE.md).

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
