use crate::core::extension::Extension;
use crate::core::theme::Region;
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;

use std::sync::LazyLock;

/// Representa una referencia a un tema.
///
/// Los temas son también extensiones. Por tanto se deben definir igual, es decir, como instancias
/// estáticas globales que implementan [`Theme`], pero también [`Extension`].
pub type ThemeRef = &'static dyn Theme;

/// Interfaz común que debe implementar cualquier tema de `PageTop`.
///
/// Un tema implementará [`Theme`] y los métodos que sean necesarios de [`Extension`], aunque el
/// único obligatorio será [`theme()`](Extension::theme).
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
pub trait Theme: Extension + Send + Sync {
    /// **Obsoleto desde la versión 0.4.0**: usar [`declared_regions()`](Self::declared_regions) en
    /// su lugar.
    #[deprecated(since = "0.4.0", note = "Use `declared_regions()` instead")]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![("content", L10n::l("content"))]
    }

    /// Declaración ordenada de las regiones disponibles en la página.
    ///
    /// Devuelve una lista estática de pares `(Region, L10n)` que se usará para renderizar en el
    /// orden indicado todas las regiones que componen una página. Los identificadores deben ser
    /// **estables** como `"sidebar-left"` o `"content"`. La etiqueta `L10n` devuelve el nombre de la
    /// región en el idioma activo de la página.
    ///
    /// Si el tema requiere un conjunto distinto de regiones, se puede sobrescribir este método para
    /// devolver una lista diferente. Si no, se usará la lista predeterminada:
    ///
    /// - `"header"`: cabecera.
    /// - `"content"`: contenido principal (**obligatoria**).
    /// - `"footer"`: pie.
    ///
    /// Sólo la región `"content"` es obligatoria, usa [`Region::default()`] para declararla.
    #[inline]
    fn declared_regions(&self) -> &'static [(Region, L10n)] {
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
    /// Por defecto, recorre [`declared_regions()`](Self::declared_regions) **en el orden que se han
    /// declarado** y, para cada región con contenido, genera un contenedor con `role="region"` y
    /// `aria-label` localizado.
    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @for (region, region_label) in self.declared_regions() {
                    @let output = page.render_region(region.key());
                    @if !output.is_empty() {
                        @let region_name = region.name();
                        div
                            id=(region_name)
                            class={ "region region--" (region_name) }
                            role="region"
                            aria-label=[region_label.using(page)]
                        {
                            (output)
                        }
                    }
                }
            }
        }
    }

    /// Acciones específicas del tema después de renderizar el `<body>` de la página.
    ///
    /// Útil para *tracing*, métricas o ajustes finales del estado de la página.
    #[allow(unused_variables)]
    fn after_render_page_body(&self, page: &mut Page) {}

    /// Renderiza el contenido del `<head>` de la página.
    ///
    /// Por defecto, genera las etiquetas básicas (`charset`, `title`, `description`, `viewport`,
    /// `X-UA-Compatible`), los metadatos y propiedades de la página y los recursos (CSS/JS).
    fn render_page_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
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

                (page.render_assets())
            }
        }
    }

    /// Página de error "*403 – Forbidden*" predeterminada.
    ///
    /// Se puede sobrescribir este método para personalizar y adaptar este contenido al tema.
    fn error403(&self, page: &mut Page) -> Markup {
        html! { div { h1 { (L10n::l("error403_notice").to_markup(page)) } } }
    }

    /// Página de error "*404 – Not Found*" predeterminada.
    ///
    /// Se puede sobrescribir este método para personalizar y adaptar este contenido al tema.
    fn error404(&self, page: &mut Page) -> Markup {
        html! { div { h1 { (L10n::l("error404_notice").to_markup(page)) } } }
    }
}
