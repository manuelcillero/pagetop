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
//! También define las regiones internas reservadas ([`ReservedRegion`]) que actúan como puntos de
//! anclaje globales al inicio y al final del `<body>`, fuera de las regiones que maqueta la
//! plantilla activa.

mod error;
pub use error::ErrorPage;
pub(crate) use error::{render_error_pages, response_for_panic, route_not_found};

use crate::auth::CurrentUser;
use crate::base::action;
use crate::base::component::layout;
use crate::core::component::{AssetsOp, ChildOp, ComponentRender};
use crate::core::component::{Context, ContextError, Contextual};
use crate::core::theme::{CoreRegion, CoreTemplate, RegionName, RegionRef, TemplateRef, ThemeRef};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::html::{Attr, Props, PropsOp};
use crate::html::{DOCTYPE, Markup, html};
use crate::locale::{CharacterDirection, L10n, LangId, LanguageIdentifier};
use crate::web::HttpRequest;
use crate::{AutoDefault, builder_fn};

// **< ReservedRegion >*****************************************************************************

/// Regiones internas reservadas como puntos de anclaje globales.
///
/// Representan contenedores especiales situados al inicio y al final del `<body>`, fuera de las
/// regiones que maqueta la plantilla activa. Las renderiza directamente [`Page::render()`],
/// envolviendo el resultado de
/// [`Theme::render_page_body()`](crate::core::theme::Theme::render_page_body). **No suelen usarse
/// como regiones "visibles" en los temas**, sino para inyectar contenido global o técnico.
#[derive(AutoDefault)]
pub enum ReservedRegion {
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

impl RegionName for ReservedRegion {
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::PageTop => "page-top",
            Self::PageBottom => "page-bottom",
        }
    }

    #[inline]
    fn label(&self) -> L10n {
        L10n::default()
    }
}

// **< Page >***************************************************************************************

/// Representa una página HTML completa lista para renderizar.
///
/// Una instancia de `Page` se compone dinámicamente permitiendo establecer título, descripción,
/// regiones donde disponer los componentes, atributos de `<body>` y otros aspectos del contexto de
/// renderizado.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Page {
    title       : Attr<L10n>,
    description : Attr<L10n>,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    context     : Context,
}

impl Page {
    /// Crea una nueva instancia de página.
    ///
    /// La petición HTTP se guarda en el contexto de renderizado, que extrae automáticamente el
    /// [`CurrentUser`] inyectado por middleware en sus extensiones (ver
    /// [`Context::new`](crate::core::component::Context::new)). Cualquier handler tiene acceso al
    /// usuario actual desde el momento en que se crea la página, sin llamadas adicionales.
    pub fn new(request: HttpRequest) -> Self {
        Page {
            context: Context::new(Some(request)),
            ..Default::default()
        }
    }

    /// Crea una nueva instancia de página con la plantilla [`CoreTemplate::Admin`].
    ///
    /// Cada tema puede maquetarla de forma distinta capturando
    /// [`Template`](crate::base::component::layout::Template) en `handle_component()`, pero la
    /// plantilla en sí es la misma constante para cualquier tema.
    pub fn admin(request: HttpRequest) -> Self {
        Page {
            context: Context::new(Some(request)).with_template(&CoreTemplate::Admin),
            ..Default::default()
        }
    }

    // **< Page BUILDER >***************************************************************************

    /// Establece el título de la página como un valor traducible.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title.alter_value(title);
        self
    }

    /// Establece la descripción de la página como un valor traducible.
    #[builder_fn]
    pub fn with_description(mut self, description: L10n) -> Self {
        self.description.alter_value(description);
        self
    }

    /// Añade una entrada `<meta name="..." content="...">` al `<head>`.
    #[builder_fn]
    pub fn with_metadata(mut self, name: &'static str, content: &'static str) -> Self {
        self.metadata.push((name, content));
        self
    }

    /// Añade una entrada `<meta property="..." content="...">` al `<head>`.
    #[builder_fn]
    pub fn with_property(mut self, property: &'static str, content: &'static str) -> Self {
        self.properties.push((property, content));
        self
    }

    // **< Page GETTERS >***************************************************************************

    /// Devuelve el título traducido para el idioma de la página, si existe.
    pub fn title(&mut self) -> Option<String> {
        self.title.lookup(&self.context)
    }

    /// Devuelve la descripción traducida para el idioma de la página, si existe.
    pub fn description(&mut self) -> Option<String> {
        self.description.lookup(&self.context)
    }

    /// Devuelve la lista de metadatos `<meta name=...>`.
    pub fn metadata(&self) -> &Vec<(&str, &str)> {
        &self.metadata
    }

    /// Devuelve la lista de propiedades `<meta property=...>`.
    pub fn properties(&self) -> &Vec<(&str, &str)> {
        &self.properties
    }

    /// Devuelve una referencia mutable al [`Context`] de la página.
    ///
    /// El [`Context`] actúa como intermediario para muchos métodos de `Page` (idioma, tema,
    /// *layout*, recursos, petición HTTP, etc.). Resulta especialmente útil cuando un componente
    /// o un tema necesita recibir el contexto como parámetro.
    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    // **< Page RENDER >****************************************************************************

    /// Renderiza la página completa en formato HTML.
    ///
    /// El proceso de renderizado de la página sigue esta secuencia:
    ///
    /// 1. Ejecuta
    ///    [`Theme::before_render_page_body()`](crate::core::theme::Theme::before_render_page_body)
    ///    para que el tema pueda ejecutar acciones específicas antes de renderizar el `<body>`.
    /// 2. Despacha [`action::page::BeforeRenderBody`] para que otras extensiones puedan realizar
    ///    ajustes previos sobre la página.
    /// 3. **Construye el contenido del `<body>`**:
    ///    - Renderiza la región reservada superior ([`ReservedRegion::PageTop`]).
    ///    - Llama a [`Theme::render_page_body()`](crate::core::theme::Theme::render_page_body) para
    ///      renderizar las regiones del cuerpo principal de la página.
    ///    - Renderiza la región reservada inferior ([`ReservedRegion::PageBottom`]).
    /// 4. Ejecuta
    ///    [`Theme::after_render_page_body()`](crate::core::theme::Theme::after_render_page_body)
    ///    para que el tema pueda aplicar ajustes finales.
    /// 5. Despacha [`action::page::AfterRenderBody`] para permitir que otras extensiones realicen
    ///    sus últimos ajustes tras generar el `<body>`.
    /// 6. Renderiza el `<head>` llamando a
    ///    [`Theme::render_page_head()`](crate::core::theme::Theme::render_page_head).
    /// 7. Obtiene el idioma y la dirección del texto a partir de
    ///    [`Context::langid()`](crate::core::component::Context::langid) e inserta los atributos
    ///    `lang` y `dir` en la etiqueta `<html>`.
    /// 8. Compone el documento HTML completo (`<!DOCTYPE html>`, `<html>`, `<head>`, `<body>`) y
    ///    devuelve un [`Result`] con el [`Markup`] final.
    pub async fn render(&mut self) -> Result<Markup, ErrorPage> {
        // Acciones específicas del tema antes de renderizar el <body>.
        self.context.theme().before_render_page_body(self);

        // Acciones de las extensiones antes de renderizar el <body>.
        action::page::BeforeRenderBody::dispatch(self);

        // Renderiza el <body>.
        let body = html! {
            (layout::Region::of(&ReservedRegion::PageTop).render(&mut self.context).await)
            (self.context.theme().render_page_body(self).await)
            (layout::Region::of(&ReservedRegion::PageBottom).render(&mut self.context).await)
        };

        // Acciones específicas del tema después de renderizar el <body>.
        self.context.theme().after_render_page_body(self);

        // Acciones de las extensiones después de renderizar el <body>.
        action::page::AfterRenderBody::dispatch(self);

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
                body (self.body_props()) {
                    (body)
                }
            }
        })
    }
}

/// Permite a [`Page`] actuar como proveedor de idioma usando el [`Context`] de la página.
///
/// Resulta útil para usar [`Page`] directamente como fuente de traducción en [`L10n::lookup()`] o
/// [`L10n::using()`].
impl LangId for Page {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        self.context.langid()
    }
}

impl Contextual for Page {
    // **< Contextual BUILDER >*********************************************************************

    #[builder_fn]
    fn with_request(mut self, request: Option<HttpRequest>) -> Self {
        self.context.alter_request(request);
        self
    }

    #[builder_fn]
    fn with_langid(mut self, language: &impl LangId) -> Self {
        self.context.alter_langid(language);
        self
    }

    #[builder_fn]
    fn with_theme(mut self, theme: ThemeRef) -> Self {
        self.context.alter_theme(theme);
        self
    }

    #[builder_fn]
    fn with_template(mut self, template: TemplateRef) -> Self {
        self.context.alter_template(template);
        self
    }

    #[builder_fn]
    fn with_param<T: Send + Sync + 'static>(mut self, key: &'static str, value: T) -> Self {
        self.context.alter_param(key, value);
        self
    }

    #[builder_fn]
    fn with_assets(mut self, op: AssetsOp) -> Self {
        self.context.alter_assets(op);
        self
    }

    #[builder_fn]
    fn with_body_props(mut self, op: PropsOp) -> Self {
        self.context.alter_body_props(op);
        self
    }

    #[builder_fn]
    fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.context.alter_child_in(&CoreRegion::Content, op.into());
        self
    }

    #[builder_fn]
    fn with_child_in(mut self, region: RegionRef, op: impl Into<ChildOp>) -> Self {
        self.context.alter_child_in(region, op.into());
        self
    }

    // **< Contextual GETTERS >*********************************************************************

    fn request(&self) -> Option<&HttpRequest> {
        self.context.request()
    }

    fn current_user(&self) -> &CurrentUser {
        self.context.current_user()
    }

    fn theme(&self) -> ThemeRef {
        self.context.theme()
    }

    fn template(&self) -> TemplateRef {
        self.context.template()
    }

    fn param<T: 'static>(&self, key: &'static str) -> Result<&T, ContextError> {
        self.context.param(key)
    }

    fn favicon(&self) -> Option<&Favicon> {
        self.context.favicon()
    }

    fn stylesheets(&self) -> &Assets<StyleSheet> {
        self.context.stylesheets()
    }

    fn javascripts(&self) -> &Assets<JavaScript> {
        self.context.javascripts()
    }

    fn body_props(&self) -> &Props {
        self.context.body_props()
    }

    // **< Contextual HELPERS >*********************************************************************

    fn remove_param(&mut self, key: &'static str) -> bool {
        self.context.remove_param(key)
    }
}
