mod error;
pub use error::ErrorPage;

pub use actix_web::Result as ResultPage;

use crate::base::action;
use crate::core::component::{Child, ChildOp, Component};
use crate::core::theme::{ChildrenInRegions, ThemeRef, REGION_CONTENT};
use crate::html::{html, Markup, DOCTYPE};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::html::{AssetsOp, Context, Contextual};
use crate::html::{AttrClasses, ClassesOp};
use crate::html::{AttrId, AttrL10n};
use crate::locale::{CharacterDirection, L10n, LangId, LanguageIdentifier};
use crate::service::HttpRequest;
use crate::{builder_fn, AutoDefault};

/// Representa una página HTML completa lista para renderizar.
///
/// Una instancia de `Page` se compone dinámicamente permitiendo establecer título, descripción,
/// regiones donde disponer los componentes, atributos de `<body>` y otros aspectos del contexto de
/// renderizado.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Page {
    title       : AttrL10n,
    description : AttrL10n,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    body_id     : AttrId,
    body_classes: AttrClasses,
    context     : Context,
    regions     : ChildrenInRegions,
}

impl Page {
    /// Crea una nueva instancia de página.
    ///
    /// La solicitud HTTP se guardará en el contexto de renderizado de la página para poder ser
    /// recuperada por los componentes si es necesario.
    #[rustfmt::skip]
    pub fn new(request: HttpRequest) -> Self {
        Page {
            title       : AttrL10n::default(),
            description : AttrL10n::default(),
            metadata    : Vec::default(),
            properties  : Vec::default(),
            body_id     : AttrId::default(),
            body_classes: AttrClasses::default(),
            context     : Context::new(Some(request)),
            regions     : ChildrenInRegions::default(),
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
        self.metadata.push((property, content));
        self
    }

    /// Establece el atributo `id` del elemento `<body>`.
    #[builder_fn]
    pub fn with_body_id(mut self, id: impl AsRef<str>) -> Self {
        self.body_id.alter_value(id);
        self
    }

    /// Modifica las clases CSS del elemento `<body>` con una operación sobre [`AttrClasses`].
    #[builder_fn]
    pub fn with_body_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    /// **Obsoleto desde la versión 0.4.0**: usar [`add_component()`](Self::add_component) en su
    /// lugar.
    #[deprecated(since = "0.4.0", note = "Use `add_component()` instead")]
    pub fn with_component(self, component: impl Component) -> Self {
        self.add_component(component)
    }

    /// **Obsoleto desde la versión 0.4.0**: usar [`add_component_in()`](Self::add_component_in) en
    /// su lugar.
    #[deprecated(since = "0.4.0", note = "Use `add_component_in()` instead")]
    pub fn with_component_in(self, region_name: &'static str, component: impl Component) -> Self {
        self.add_component_in(region_name, component)
    }

    /// Añade un componente a la región de contenido por defecto.
    pub fn add_component(mut self, component: impl Component) -> Self {
        self.regions
            .alter_child_in(REGION_CONTENT, ChildOp::Add(Child::with(component)));
        self
    }

    /// Añade un componente en una región (`region_name`) de la página.
    pub fn add_component_in(
        mut self,
        region_name: &'static str,
        component: impl Component,
    ) -> Self {
        self.regions
            .alter_child_in(region_name, ChildOp::Add(Child::with(component)));
        self
    }

    /// **Obsoleto desde la versión 0.4.0**: usar [`with_child_in()`](Self::with_child_in) en su
    /// lugar.
    #[deprecated(since = "0.4.0", note = "Use `with_child_in()` instead")]
    pub fn with_child_in_region(mut self, region_name: &'static str, op: ChildOp) -> Self {
        self.alter_child_in(region_name, op);
        self
    }

    /// **Obsoleto desde la versión 0.4.0**: usar [`alter_child_in()`](Self::alter_child_in) en su
    /// lugar.
    #[deprecated(since = "0.4.0", note = "Use `alter_child_in()` instead")]
    pub fn alter_child_in_region(&mut self, region_name: &'static str, op: ChildOp) -> &mut Self {
        self.alter_child_in(region_name, op);
        self
    }

    /// Opera con [`ChildOp`] en una región (`region_name`) de la página.
    #[builder_fn]
    pub fn with_child_in(mut self, region_name: &'static str, op: ChildOp) -> Self {
        self.regions.alter_child_in(region_name, op);
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

    /// Devuelve el identificador del elemento `<body>`.
    pub fn body_id(&self) -> &AttrId {
        &self.body_id
    }

    /// Devuelve las clases CSS del elemento `<body>`.
    pub fn body_classes(&self) -> &AttrClasses {
        &self.body_classes
    }

    /// Devuelve una referencia mutable al [`Context`] de la página.
    ///
    /// El [`Context`] actúa como intermediario para muchos métodos de `Page` (idioma, tema,
    /// *layout*, recursos, solicitud HTTP, etc.). Resulta especialmente útil cuando un componente
    /// o un tema necesita recibir el contexto como parámetro.
    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    // **< Page RENDER >****************************************************************************

    /// Renderiza los componentes de una región (`region_name`) de la página.
    pub fn render_region(&mut self, region_name: &'static str) -> Markup {
        self.regions
            .merge_all_components(self.context.theme(), region_name)
            .render(&mut self.context)
    }

    /// Renderiza los recursos de la página.
    pub fn render_assets(&mut self) -> Markup {
        self.context.render_assets()
    }

    /// Renderiza la página completa en formato HTML.
    ///
    /// Ejecuta las acciones correspondientes antes y después de renderizar el `<body>`,
    /// así como del `<head>`, e inserta los atributos `lang` y `dir` en la etiqueta `<html>`.
    pub fn render(&mut self) -> ResultPage<Markup, ErrorPage> {
        // Acciones específicas del tema antes de renderizar el <body>.
        self.context.theme().before_render_page_body(self);

        // Acciones de las extensiones antes de renderizar el <body>.
        action::page::BeforeRenderBody::dispatch(self);

        // Renderiza el <body>.
        let body = self.context.theme().render_page_body(self);

        // Acciones específicas del tema después de renderizar el <body>.
        self.context.theme().after_render_page_body(self);

        // Acciones de las extensiones después de renderizar el <body>.
        action::page::AfterRenderBody::dispatch(self);

        // Renderiza el <head>.
        let head = self.context.theme().render_page_head(self);

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
                (head)
                (body)
            }
        })
    }
}

impl LangId for Page {
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
    fn with_theme(mut self, theme_name: &'static str) -> Self {
        self.context.alter_theme(theme_name);
        self
    }

    #[builder_fn]
    fn with_layout(mut self, layout_name: &'static str) -> Self {
        self.context.alter_layout(layout_name);
        self
    }

    #[builder_fn]
    fn with_param<T: 'static>(mut self, key: &'static str, value: T) -> Self {
        self.context.alter_param(key, value);
        self
    }

    #[builder_fn]
    fn with_assets(mut self, op: AssetsOp) -> Self {
        self.context.alter_assets(op);
        self
    }

    // **< Contextual GETTERS >*********************************************************************

    fn request(&self) -> Option<&HttpRequest> {
        self.context.request()
    }

    fn theme(&self) -> ThemeRef {
        self.context.theme()
    }

    fn layout(&self) -> &str {
        self.context.layout()
    }

    fn param<T: 'static>(&self, key: &'static str) -> Option<&T> {
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

    // **< Contextual HELPERS >*********************************************************************

    fn required_id<T>(&mut self, id: Option<String>) -> String {
        self.context.required_id::<T>(id)
    }
}
