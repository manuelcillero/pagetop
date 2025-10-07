use crate::core::extension::Extension;
use crate::core::theme::{Region, RegionRef, REGION_CONTENT};
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;
use crate::{global, join};

use std::sync::LazyLock;

/// Referencia estática a un tema.
///
/// Los temas son también extensiones. Por tanto, deben declararse como **instancias estáticas** que
/// implementen [`Theme`] y, a su vez, [`Extension`].
pub type ThemeRef = &'static dyn Theme;

/// Conjunto de regiones que los temas pueden exponer para el renderizado.
///
/// `ThemeRegion` define un conjunto de regiones predefinidas para estructurar un documento HTML.
/// Proporciona **identificadores estables** (vía [`Region::key()`]) y **etiquetas localizables**
/// (vía [`Region::label()`]) a las regiones donde se añadirán los componentes.
///
/// Se usa por defecto en [`Theme::page_regions()`](crate::core::theme::Theme::page_regions) y sus
/// variantes representan el conjunto mínimo recomendado para cualquier tema. Sin embargo, cada tema
/// podría exponer su propio conjunto de regiones.
pub enum ThemeRegion {
    /// Cabecera de la página.
    ///
    /// Clave: `"header"`. Suele contener *branding*, navegación principal o avisos globales.
    Header,

    /// Contenido principal de la página (**obligatoria**).
    ///
    /// Clave: `"content"`. Es el destino por defecto para insertar componentes a nivel de página.
    Content,

    /// Pie de página.
    ///
    /// Clave: `"footer"`. Suele contener enlaces legales, créditos o navegación secundaria.
    Footer,
}

impl Region for ThemeRegion {
    fn key(&self) -> &str {
        match self {
            ThemeRegion::Header => "header",
            ThemeRegion::Content => REGION_CONTENT,
            ThemeRegion::Footer => "footer",
        }
    }

    fn label(&self) -> L10n {
        L10n::l(join!("region_", self.key()))
    }
}

/// Métodos predefinidos de renderizado para las páginas de un tema.
///
/// Contiene las implementaciones base para renderizar las **secciones** `<head>` y `<body>`. Se
/// implementa automáticamente para cualquier tipo que implemente [`Theme`], por lo que normalmente
/// no requiere implementación explícita.
///
/// Si un tema **sobrescribe** uno o más de estos métodos de [`Theme`]:
///
/// - [`render_page_region()`](Theme::render_page_region),
/// - [`render_page_head()`](Theme::render_page_head), o
/// - [`render_page_body()`](Theme::render_page_body);
///
/// es posible volver al comportamiento por defecto usando FQS (*Fully Qualified Syntax*):
///
/// - `<Self as ThemePage>::render_body(self, page, self.page_regions())`
/// - `<Self as ThemePage>::render_head(self, page)`
pub trait ThemePage {
    /// Renderiza el **contenedor** de una región concreta del `<body>` de la página.
    ///
    /// Obtiene los componentes asociados a `region.key()` desde el contexto de la página y, si hay
    /// salida, envuelve el contenido en un contenedor `<div>` predefinido.
    ///
    /// Si la región **no produce contenido**, devuelve un `Markup` vacío.
    #[inline]
    fn render_region(&self, page: &mut Page, region: RegionRef) -> Markup {
        html! {
            @let key = region.key();
            @let output = page.context().render_components_of(key);
            @if !output.is_empty() {
                div
                    id=(key)
                    class={ "region region--" (key) }
                    role="region"
                    aria-label=[region.label().lookup(page)]
                {
                    (output)
                }
            }
        }
    }

    /// Renderiza el **contenido interior** del `<body>` de la página.
    ///
    /// Recorre `regions` en el **orden declarado** y, para cada región con contenido, delega en
    /// [`render_region()`](Self::render_region) la generación del contenedor. Las regiones sin
    /// contenido **no** producen salida. Se asume que cada identificador de región es **único**
    /// dentro de la página.
    ///
    /// La etiqueta `<body>` no se incluye aquí; únicamente renderiza su contenido.
    #[inline]
    fn render_body(&self, page: &mut Page, regions: &[RegionRef]) -> Markup {
        html! {
            @for region in regions {
                (self.render_region(page, *region))
            }
        }
    }

    /// Renderiza el **contenido interior** del `<head>` de la página.
    ///
    /// Incluye por defecto las etiquetas básicas (`charset`, `title`, `description`, `viewport`,
    /// `X-UA-Compatible`), los metadatos (`name/content`) y propiedades (`property/content`),
    /// además de los recursos CSS/JS de la página.
    ///
    /// La etiqueta `<head>` no se incluye aquí; únicamente se renderiza su contenido.
    #[inline]
    fn render_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            meta charset="utf-8";

            @if let Some(title) = page.title() {
                title { (global::SETTINGS.app.name) (" | ") (title) }
            } @else {
                title { (global::SETTINGS.app.name) }
            }

            @if let Some(description) = page.description() {
                meta name="description" content=(description);
            }

            meta name="viewport" content=(viewport);
            @for (name, content) in page.metadata() {
                meta name=(name) content=(content) {}
            }

            meta http-equiv="X-UA-Compatible" content="IE=edge";
            @for (property, content) in page.properties() {
                meta property=(property) content=(content) {}
            }

            (page.context().render_assets())
        }
    }
}

/// Interfaz común que debe implementar cualquier tema de PageTop.
///
/// Un tema implementa [`Theme`] y los métodos necesarios de [`Extension`]. El único método
/// **obligatorio** de `Extension` para un tema es [`theme()`](Extension::theme).
///
/// ```rust
/// use pagetop::prelude::*;
///
/// pub struct MyTheme;
///
/// impl Extension for MyTheme {
///     fn name(&self) -> L10n {
///         L10n::n("My theme")
///     }
///
///     fn description(&self) -> L10n {
///         L10n::n("A personal theme")
///     }
///
///     fn theme(&self) -> Option<ThemeRef> {
///         Some(&Self)
///     }
/// }
///
/// impl Theme for MyTheme {}
/// ```
pub trait Theme: Extension + ThemePage + Send + Sync {
    /// **Obsoleto desde la versión 0.4.0**: usar [`page_regions()`](Self::page_regions) en su
    /// lugar.
    #[deprecated(since = "0.4.0", note = "Use `page_regions()` instead")]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![("content", L10n::l("content"))]
    }

    /// Declaración ordenada de las regiones disponibles en la página.
    ///
    /// Retorna una **lista estática** de referencias ([`RegionRef`](crate::core::theme::RegionRef))
    /// que representan las regiones que el tema admite dentro del `<body>`.
    ///
    /// Cada referencia apunta a una instancia que implementa [`Region`](crate::core::theme::Region)
    /// para definir cada región de forma segura y estable. Y si un tema necesita un conjunto
    /// distinto de regiones, puede **sobrescribir** este método siguiendo estas recomendaciones:
    ///
    /// - Los identificadores devueltos por [`Region::key()`](crate::core::theme::Region::key)
    ///   deben ser **estables** (p. ej. `"sidebar-left"`, `"content"`).
    /// - La región `"content"` es **obligatoria**, ya que se usa como destino por defecto para
    ///   insertar componentes y renderizarlos.
    /// - El orden de la lista podría tener relevancia como **orden de renderizado** dentro del
    ///   `<body>` segun la implementación de [`render_page_body()`](Self::render_page_body).
    /// - Las etiquetas (`L10n`) de cada región se evaluarán con el idioma activo de la página.
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// fn page_regions(&self) -> &'static [RegionRef] {
    ///     static REGIONS: LazyLock<[RegionRef; 4]> = LazyLock::new(|| {
    ///         [
    ///             &ThemeRegion::Header,
    ///             &ThemeRegion::Content,
    ///             &ThemeRegion::Footer,
    ///         ]
    ///     });
    ///     &*REGIONS
    /// }
    /// ```
    fn page_regions(&self) -> &'static [RegionRef] {
        static REGIONS: LazyLock<[RegionRef; 3]> = LazyLock::new(|| {
            [
                &ThemeRegion::Header,
                &ThemeRegion::Content,
                &ThemeRegion::Footer,
            ]
        });
        &*REGIONS
    }

    /// Renderiza una región de la página.
    ///
    /// Si se sobrescribe este método, se puede volver al comportamiento base con:
    /// `<Self as ThemePage>::render_region(self, page, region)`.
    #[inline]
    fn render_page_region(&self, page: &mut Page, region: RegionRef) -> Markup {
        <Self as ThemePage>::render_region(self, page, region)
    }

    /// Acciones específicas del tema antes de renderizar el `<body>` de la página.
    ///
    /// Útil para preparar clases, inyectar recursos o ajustar metadatos.
    #[allow(unused_variables)]
    fn before_render_page_body(&self, page: &mut Page) {}

    /// Renderiza el contenido del `<body>` de la página.
    ///
    /// Si se sobrescribe este método, se puede volver al comportamiento base con:
    /// `<Self as ThemePage>::render_body(self, page, self.page_regions())`.
    #[inline]
    fn render_page_body(&self, page: &mut Page) -> Markup {
        <Self as ThemePage>::render_body(self, page, self.page_regions())
    }

    /// Acciones específicas del tema después de renderizar el `<body>` de la página.
    ///
    /// Útil para *tracing*, métricas o ajustes finales del estado de la página.
    #[allow(unused_variables)]
    fn after_render_page_body(&self, page: &mut Page) {}

    /// Renderiza el contenido del `<head>` de la página.
    ///
    /// Si se sobrescribe este método, se puede volver al comportamiento base con:
    /// `<Self as ThemePage>::render_head(self, page)`.
    #[inline]
    fn render_page_head(&self, page: &mut Page) -> Markup {
        <Self as ThemePage>::render_head(self, page)
    }

    /// Contenido predeterminado para la página de error "*403 – Forbidden*".
    ///
    /// Se puede sobrescribir este método para personalizar y adaptar este contenido al tema.
    fn error403(&self, page: &mut Page) -> Markup {
        html! { div { h1 { (L10n::l("error403_notice").using(page)) } } }
    }

    /// Contenido predeterminado para la página de error "*404 – Not Found*".
    ///
    /// Se puede sobrescribir este método para personalizar y adaptar este contenido al tema.
    fn error404(&self, page: &mut Page) -> Markup {
        html! { div { h1 { (L10n::l("error404_notice").using(page)) } } }
    }
}

/// Se implementa automáticamente `ThemePage` para cualquier tema.
impl<T: Theme> ThemePage for T {}
