//! Responde a una petición web generando una página HTML completa.
//!
//! Este módulo define [`Page`], que representa una página HTML lista para renderizar. Cada página
//! se construye a partir de un [`Context`] propio, donde se registran el tema activo, la plantilla
//! ([`TemplateName`](crate::core::theme::TemplateName)) que define la disposición de las regiones
//! ([`RegionName`]), los componentes asociados y los recursos adicionales (hojas de estilo,
//! scripts, *favicon*, etc.).
//!
//! El renderizado ([`Page::render()`]) delega en el tema ([`Theme`](crate::core::theme::Theme)) la
//! composición del `<head>` y del `<body>`, y se ejecutan las acciones registradas por las
//! extensiones antes y después de generar los contenidos.
//!
//! También define las regiones internas reservadas ([`ReservedRegions`]) que actúan como puntos de
//! anclaje globales al inicio y al final del `<body>`, fuera de las regiones que maqueta la
//! plantilla activa.

mod error;
pub use error::ErrorPage;
pub(crate) use error::{render_error_pages, response_for_panic, route_not_found};

use crate::AutoDefault;
use crate::base::action;
use crate::base::component::layout;
use crate::core::component::{ComponentRender, Context, Contextual};
use crate::core::theme::RegionName;
use crate::html::{DOCTYPE, Markup, html};
use crate::locale::{CharacterDirection, LangId, Lc};
use crate::web::HttpRequest;

// **< ReservedRegions >****************************************************************************

/// Regiones internas reservadas como puntos de anclaje globales.
///
/// Representan contenedores especiales situados al inicio y al final del `<body>`, fuera de las
/// regiones que maqueta la plantilla activa. Las renderiza directamente [`Page::render()`],
/// envolviendo el resultado de
/// [`Theme::render_page_body()`](crate::core::theme::Theme::render_page_body). **No suelen usarse
/// como regiones "visibles" en los temas**, sino para inyectar contenido global o técnico.
#[derive(AutoDefault)]
pub enum ReservedRegions {
    /// Región interna situada al **inicio del `<body>`**, de nombre `"page-top"`.
    ///
    /// Proporciona un contenedor donde las extensiones puedan inyectar elementos auxiliares antes
    /// del resto de regiones (cabecera, contenido, etc.), como marcadores técnicos, inicializadores
    /// o contenido de depuración.
    ///
    /// Se considera una región **reservada** para este tipo de usos globales.
    #[default]
    PageTop,

    /// Región interna situada al **final del `<body>`**, de nombre `"page-bottom"`.
    ///
    /// Proporciona un contenedor donde las extensiones puedan inyectar contenido global después del
    /// resto de regiones (cabecera, contenido, etc.), como elementos auxiliares asociados a
    /// comportamientos dinámicos.
    ///
    /// Igual que [`Self::PageTop`], se considera una región **reservada** para este tipo de usos
    /// globales.
    PageBottom,
}

impl RegionName for ReservedRegions {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::PageTop => "page-top",
            Self::PageBottom => "page-bottom",
        }
    }

    #[inline]
    fn label(&self) -> Lc {
        Lc::default()
    }
}

// **< Page >***************************************************************************************

/// Representa una página HTML completa lista para renderizar.
///
/// Envuelve el [`Context`] de renderizado de la página e implementa [`Contextual`] sobre él, así
/// que una página se mantiene con los mismos métodos que un contexto: título, descripción, regiones
/// donde disponer los componentes, atributos de `<body>`, recursos, etc. Lo que añade `Page` es el
/// renderizado del documento completo ([`Page::render()`]).
#[derive(AutoDefault)]
pub struct Page {
    context: Context,
}

impl Page {
    /// Crea una nueva instancia de página.
    ///
    /// La petición HTTP se guarda en el contexto de renderizado, que extrae automáticamente el
    /// [`CurrentUser`](crate::auth::CurrentUser) inyectado por middleware en sus extensiones (ver
    /// [`Context::new`](crate::core::component::Context::new)). Cualquier handler tiene acceso al
    /// usuario actual desde el momento en que se crea la página, sin llamadas adicionales.
    pub fn new(request: HttpRequest) -> Self {
        Page {
            context: Context::new(request),
        }
    }

    /// Crea una nueva instancia de página con la plantilla [`CoreTemplates::Admin`].
    ///
    /// Es la forma de crear una página del panel de administración: la plantilla añade la
    /// navegación del panel y, alrededor del contenido, las migas de pan, el título
    /// ([`with_title()`](Contextual::with_title)) y las pestañas y botones declarados para su ruta
    /// (ver el módulo [`admin`](crate::admin)).
    ///
    /// Cada tema puede maquetarla de forma distinta capturando [`Template`] vía
    /// [`Theme::render_component()`], pero la plantilla en sí es la misma constante para cualquier
    /// tema.
    ///
    /// [`CoreTemplates::Admin`]: crate::core::theme::CoreTemplates::Admin
    /// [`Template`]: crate::base::component::layout::Template
    /// [`Theme::render_component()`]: crate::core::theme::Theme::render_component
    pub fn admin(request: HttpRequest) -> Self {
        Page {
            context: Context::admin(request),
        }
    }

    /// Renderiza la página completa en formato HTML.
    ///
    /// El proceso de renderizado de la página sigue esta secuencia:
    ///
    /// 1. Ejecuta [`Theme::before_render_page_body()`] para que el tema pueda ejecutar acciones
    ///    específicas antes de renderizar el `<body>`.
    /// 2. Despacha [`action::page::BeforeRenderBody`] para que otras extensiones puedan realizar
    ///    ajustes previos sobre la página.
    /// 3. **Construye el contenido del `<body>`**:
    ///    - Renderiza la región reservada superior ([`ReservedRegions::PageTop`]).
    ///    - Llama a [`Theme::render_page_body()`] para renderizar las regiones del cuerpo principal
    ///      de la página.
    ///    - Renderiza la región reservada inferior ([`ReservedRegions::PageBottom`]).
    ///
    ///    Los componentes que se aplazan ([`Component::is_deferred()`]), como el componente
    ///    [`Messages`] que encabeza la región [`CoreRegions::Content`], dejan de momento una marca
    ///    en su lugar.
    /// 4. Ejecuta [`Theme::after_render_page_body()`] para que el tema pueda aplicar ajustes
    ///    finales.
    /// 5. Despacha [`action::page::AfterRenderBody`] para permitir que otras extensiones realicen
    ///    sus últimos ajustes tras generar el `<body>`.
    /// 6. Renderiza los componentes aplazados y sustituye sus marcas por el resultado; si alguno no
    ///    genera nada, su marca se queda. Así muestran también lo que se haya acumulado en el
    ///    contexto en los pasos anteriores (por ejemplo, los mensajes para el usuario).
    /// 7. Renderiza el `<head>` llamando a [`Theme::render_page_head()`].
    /// 8. Obtiene el idioma y la dirección del texto a partir de [`Context::langid()`] e inserta
    ///    los atributos `lang` y `dir` en la etiqueta `<html>`.
    /// 9. Compone el documento HTML completo (`<!DOCTYPE html>`, `<html>`, `<head>`, `<body>`) y
    ///    devuelve un [`Result`] con el [`Markup`] final.
    ///
    /// [`Theme::before_render_page_body()`]: crate::core::theme::Theme::before_render_page_body
    /// [`Theme::render_page_body()`]: crate::core::theme::Theme::render_page_body
    /// [`Theme::after_render_page_body()`]: crate::core::theme::Theme::after_render_page_body
    /// [`Theme::render_page_head()`]: crate::core::theme::Theme::render_page_head
    /// [`CoreRegions::Content`]: crate::core::theme::CoreRegions::Content
    /// [`Component::is_deferred()`]: crate::core::component::Component::is_deferred
    /// [`Context::langid()`]: crate::locale::LangId::langid
    /// [`Messages`]: crate::base::component::Messages
    pub async fn render(&mut self) -> Result<Markup, ErrorPage> {
        // Acciones específicas del tema antes de renderizar el <body>.
        self.context.theme().before_render_page_body(self);

        // Acciones de las extensiones antes de renderizar el <body>.
        action::page::BeforeRenderBody::dispatch(self);

        // Renderiza el <body>, aplazando los componentes que lo pidan.
        self.context.start_deferring();
        let body = html! {
            (layout::Region::of(&ReservedRegions::PageTop).render(&mut self.context).await)
            (self.context.theme().render_page_body(self).await)
            (layout::Region::of(&ReservedRegions::PageBottom).render(&mut self.context).await)
        };

        // Acciones específicas del tema después de renderizar el <body>.
        self.context.theme().after_render_page_body(self);

        // Acciones de las extensiones después de renderizar el <body>.
        action::page::AfterRenderBody::dispatch(self);

        // Renderiza los componentes aplazados, antes del <head> para que lleguen sus recursos.
        let body = self.context.render_deferred(body).await;

        // Renderiza el <head>.
        let head = self.context.theme().render_page_head(self).await;

        // Compone la página incluyendo los atributos de idioma y dirección del texto.
        let lang = &self.context.langid().language;
        let dir = match self.context.langid().character_direction() {
            CharacterDirection::LTR => "ltr",
            CharacterDirection::RTL => "rtl",
            CharacterDirection::TTB => "auto",
        };
        Ok(html! {
            (DOCTYPE)
            html lang=(lang) dir=(dir) {
                head {
                    (head)
                }
                body (self.body_props().clone().unpack(&mut self.context)) {
                    (body)
                }
            }
        })
    }
}

impl Contextual for Page {
    #[inline]
    fn context(&self) -> &Context {
        &self.context
    }

    #[inline]
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}
