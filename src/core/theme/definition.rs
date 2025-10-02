use crate::core::extension::Extension;
use crate::core::theme::Region;
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;

use std::sync::LazyLock;

/// Referencia estática a un tema.
///
/// Los temas son también extensiones. Por tanto, deben declararse como **instancias estáticas** que
/// implementen [`Theme`] y, a su vez, [`Extension`].
pub type ThemeRef = &'static dyn Theme;

/// Métodos predefinidos de renderizado para las páginas de un tema.
///
/// Contiene las implementaciones base de las **secciones** `<head>` y `<body>`. Se implementa
/// automáticamente para cualquier tipo que implemente [`Theme`], por lo que normalmente no requiere
/// implementación explícita.
///
/// Si un tema **sobrescribe** [`render_page_head()`](Theme::render_page_head) o
/// [`render_page_body()`](Theme::render_page_body), se puede volver al comportamiento por defecto
/// cuando se necesite usando FQS (*Fully Qualified Syntax*):
///
/// - `<Self as ThemePage>::render_body(self, page, self.page_regions())`
/// - `<Self as ThemePage>::render_head(self, page)`
pub trait ThemePage {
    /// Renderiza el **contenido interior** del `<body>` de la página.
    ///
    /// Recorre `regions` en el **orden declarado** y, para cada región con contenido, genera un
    /// contenedor con `role="region"` y un `aria-label` localizado.
    /// Se asume que cada identificador de región es **único** dentro de la página.
    ///
    /// La etiqueta `<body>` no se incluye aquí; únicamente renderiza su contenido.
    fn render_body(&self, page: &mut Page, regions: &[(Region, L10n)]) -> Markup {
        html! {
            @for (region, region_label) in regions {
                @let output = page.context().render_components_of(region.key());
                @if !output.is_empty() {
                    @let region_name = region.name();
                    div
                        id=(region_name)
                        class={ "region region--" (region_name) }
                        role="region"
                        aria-label=[region_label.lookup(page)]
                    {
                        (output)
                    }
                }
            }
        }
    }

    /// Renderiza el **contenido interior** del `<head>` de la página.
    ///
    /// Recorre y genera por defecto las etiquetas básicas (`charset`, `title`, `description`,
    /// `viewport`, `X-UA-Compatible`), los metadatos (`name/content`) y propiedades
    /// (`property/content`), además de los recursos CSS/JS de la página.
    ///
    /// La etiqueta `<head>` no se incluye aquí; únicamente renderiza su contenido.
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
    /// Devuelve una **lista estática** de pares `(Region, L10n)` que se usará para renderizar todas
    /// las regiones que componen una página en el orden indicado .
    ///
    /// Si un tema necesita un conjunto distinto de regiones, se puede **sobrescribir** este método
    /// con los siguientes requisitos y recomendaciones:
    ///
    /// - Los identificadores deben ser **estables** (p. ej. `"sidebar-left"`, `"content"`).
    /// - La región `"content"` es **obligatoria**. Se puede usar [`Region::default()`] para
    ///   declararla.
    /// - La etiqueta `L10n` se evalúa con el idioma activo de la página.
    ///
    /// Por defecto devuelve:
    ///
    /// - `"header"`: cabecera.
    /// - `"content"`: contenido principal (**obligatoria**).
    /// - `"footer"`: pie.
    fn page_regions(&self) -> &'static [(Region, L10n)] {
        static REGIONS: LazyLock<[(Region, L10n); 3]> = LazyLock::new(|| {
            [
                (Region::declare("header"), L10n::l("region_header")),
                (Region::default(), L10n::l("region_content")),
                (Region::declare("footer"), L10n::l("region_footer")),
            ]
        });
        &REGIONS[..]
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
