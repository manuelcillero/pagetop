use crate::base::component::{Html, Intro, IntroOpening};
use crate::core::component::{Child, ChildOp, Component, Contextual};
use crate::core::extension::Extension;
use crate::core::theme::{DefaultRegion, DefaultTemplate, TemplateRef};
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;
use crate::service::http::StatusCode;

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

    /// Contenido predefinido para la página de error "*403 - Forbidden*" (acceso denegado).
    ///
    /// Los temas pueden sobrescribir este método para personalizar el diseño y el contenido de la
    /// página de error.
    fn error_403(&self, page: &mut Page) {
        page.alter_title(L10n::l("error403_title"))
            .alter_template(&DefaultTemplate::Error)
            .alter_child_in(
                &DefaultRegion::Content,
                ChildOp::Prepend(Child::with(Html::with(move |cx| {
                    html! {
                        div {
                            h1 { (L10n::l("error403_alert").using(cx)) }
                            p { (L10n::l("error403_help").using(cx)) }
                        }
                    }
                }))),
            );
    }

    /// Contenido predefinido para la página de error "*404 - Not Found*" (recurso no encontrado).
    ///
    /// Los temas pueden sobrescribir este método para personalizar el diseño y el contenido de la
    /// página de error.
    fn error_404(&self, page: &mut Page) {
        page.alter_title(L10n::l("error404_title"))
            .alter_template(&DefaultTemplate::Error)
            .alter_child_in(
                &DefaultRegion::Content,
                ChildOp::Prepend(Child::with(Html::with(move |cx| {
                    html! {
                        div {
                            h1 { (L10n::l("error404_alert").using(cx)) }
                            p { (L10n::l("error404_help").using(cx)) }
                        }
                    }
                }))),
            );
    }

    /// Permite al tema preparar y componer una página de error fatal.
    ///
    /// Por defecto, asigna el título al documento (`title`) y muestra un componente [`Intro`] con
    /// el código HTTP del error (`code`) y los mensajes proporcionados (`alert` y `help`) como
    /// descripción del error.
    ///
    /// Este método no se utiliza en las implementaciones predefinidas de [`Self::error_403()`] ni
    /// [`Self::error_404()`], que definen su propio contenido específico.
    ///
    /// Los temas pueden sobrescribir este método para personalizar el diseño y el contenido de la
    /// página de error.
    fn error_fatal(&self, page: &mut Page, code: StatusCode, title: L10n, alert: L10n, help: L10n) {
        page.alter_title(title)
            .alter_template(&DefaultTemplate::Error)
            .alter_child_in(
                &DefaultRegion::Content,
                ChildOp::Prepend(Child::with(
                    Intro::new()
                        .with_title(L10n::l("error_code").with_arg("code", code.to_string()))
                        .with_slogan(L10n::n(code.to_string()))
                        .with_button(None)
                        .with_opening(IntroOpening::Custom)
                        .add_child(Html::with(move |cx| {
                            html! {
                                h1 { (alert.using(cx)) }
                                p { (help.using(cx)) }
                            }
                        })),
                )),
            );
    }
}

/// Referencia estática a un tema.
pub type ThemeRef = &'static dyn Theme;
