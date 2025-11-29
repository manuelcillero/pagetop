use crate::core::component::Contextual;
use crate::core::extension::Extension;
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::prelude::{DefaultTemplate, TemplateRef};
use crate::response::page::Page;

/// Interfaz común que debe implementar cualquier tema de PageTop.
///
/// Un tema es una [`Extension`](crate::core::extension::Extension) que define el aspecto general de
/// las páginas: cómo se renderiza el `<head>`, cómo se presenta el `<body>` usando plantillas
/// ([`Template`](crate::core::theme::Template)) que maquetan regiones
/// ([`Region`](crate::core::theme::Region)) y qué contenido mostrar en las páginas de error. El
/// contenido de cada región depende del [`Context`](crate::core::component::Context) y de su nombre
/// lógico.
///
/// Todos los métodos de este *trait* tienen una implementación por defecto, por lo que pueden
/// sobrescribirse selectivamente para crear nuevos temas con comportamientos distintos a los
/// predeterminados.
///
/// El único método **obligatorio** de `Extension` para un tema es [`theme()`](Extension::theme),
/// que debe devolver una referencia al propio tema:
///
/// ```rust
/// # use pagetop::prelude::*;
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
    /// Devuelve la plantilla ([`Template`](crate::core::theme::Template)) que el propio tema
    /// propone como predeterminada.
    ///
    /// Se utiliza al inicializar un [`Context`](crate::core::component::Context) o una página
    /// ([`Page`](crate::response::page::Page)) por si no se elige ninguna otra plantilla con
    /// [`Contextual::with_template()`](crate::core::component::Contextual::with_template).
    ///
    /// La implementación por defecto devuelve la plantilla estándar ([`DefaultTemplate::Standard`])
    /// con una estructura básica para la página. Los temas pueden sobrescribir este método para
    /// seleccionar otra plantilla predeterminada o una plantilla propia.
    #[inline]
    fn default_template(&self) -> TemplateRef {
        &DefaultTemplate::Standard
    }

    /// Acciones específicas del tema antes de renderizar el `<body>` de la página.
    ///
    /// Es un buen lugar para inicializar o ajustar recursos en función del contexto de la página,
    /// por ejemplo:
    ///
    /// - Añadir metadatos o propiedades a la cabecera de la página.
    /// - Preparar atributos compartidos.
    /// - Registrar *assets* condicionales en el contexto.
    ///
    /// La implementación por defecto no realiza ninguna acción.
    #[allow(unused_variables)]
    fn before_render_page_body(&self, page: &mut Page) {}

    /// Renderiza el contenido del `<body>` de la página.
    ///
    /// La implementación predeterminada delega en la plantilla asociada a la página, obtenida desde
    /// su [`Context`](crate::core::component::Context), y llama a
    /// [`Template::render()`](crate::core::theme::Template::render) para componer el `<body>` a
    /// partir de las regiones.
    ///
    /// Con la configuración por defecto, la plantilla estándar utiliza las regiones
    /// [`DefaultRegion::Header`](crate::core::theme::DefaultRegion::Header),
    /// [`DefaultRegion::Content`](crate::core::theme::DefaultRegion::Content) y
    /// [`DefaultRegion::Footer`](crate::core::theme::DefaultRegion::Footer) en ese orden.
    ///
    /// Los temas pueden sobrescribir este método para:
    ///
    /// - Forzar una plantilla concreta en determinadas páginas.
    /// - Consultar la plantilla de la página y variar la composición según su nombre.
    /// - Envolver el contenido en contenedores adicionales.
    /// - Implementar lógicas de composición alternativas.
    #[inline]
    fn render_page_body(&self, page: &mut Page) -> Markup {
        page.template().render(page.context())
    }

    /// Acciones específicas del tema después de renderizar el `<body>` de la página.
    ///
    /// Se invoca tras la generación del contenido del `<body>`. Es útil para:
    ///
    /// - Ajustar o registrar recursos en función de lo que se haya renderizado.
    /// - Realizar *tracing* o recopilar métricas.
    /// - Aplicar ajustes finales al estado de la página antes de producir el `<head>` o la
    ///   respuesta final.
    ///
    /// La implementación por defecto no realiza ninguna acción.
    #[allow(unused_variables)]
    fn after_render_page_body(&self, page: &mut Page) {}

    /// Renderiza el contenido del `<head>` de la página.
    ///
    /// Aunque en una página el `<head>` se encuentra antes del `<body>`, internamente se renderiza
    /// después para contar con los ajustes que hayan ido acumulando los componentes. Por ejemplo,
    /// permitiría añadir un archivo de iconos sólo si se ha incluido un icono en la página.
    ///
    /// Por defecto incluye:
    ///
    /// - La codificación (`charset="utf-8"`).
    /// - El título, usando el título de la página si existe y, en caso contrario, sólo el nombre de
    ///   la aplicación.
    /// - La descripción (`<meta name="description">`), si está definida.
    /// - La etiqueta `viewport` básica para diseño adaptable.
    /// - Los metadatos (`name`/`content`) y propiedades (`property`/`content`) declarados en la
    ///   página.
    /// - Los *assets* registrados en el contexto de la página.
    ///
    /// Los temas pueden sobrescribir este método para añadir etiquetas adicionales (por ejemplo,
    /// *favicons* personalizados, manifest, etiquetas de analítica, etc.).
    fn render_page_head(&self, page: &mut Page) -> Markup {
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

    /// Contenido predeterminado para la página de error "*403 - Forbidden*".
    ///
    /// Los temas pueden sobrescribir este método para personalizar el diseño y el contenido de la
    /// página de error, manteniendo o no el mensaje de los textos localizados.
    fn error403(&self, page: &mut Page) -> Markup {
        html! { div { h1 { (L10n::l("error403_notice").using(page)) } } }
    }

    /// Contenido predeterminado para la página de error "*404 - Not Found*".
    ///
    /// Los temas pueden sobrescribir este método para personalizar el diseño y el contenido de la
    /// página de error, manteniendo o no el mensaje de los textos localizados.
    fn error404(&self, page: &mut Page) -> Markup {
        html! { div { h1 { (L10n::l("error404_notice").using(page)) } } }
    }
}

/// Referencia estática a un tema.
pub type ThemeRef = &'static dyn Theme;
