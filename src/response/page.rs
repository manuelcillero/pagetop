mod error;
pub use error::ErrorPage;

pub use actix_web::Result as ResultPage;

use crate::base::action;
use crate::builder_fn;
use crate::core::component::{Child, ChildOp, Component};
use crate::core::theme::{ChildrenInRegions, ThemeRef, CONTENT_REGION_NAME};
use crate::html::{html, AssetsOp, Context, Markup, DOCTYPE};
use crate::html::{ClassesOp, OptionClasses, OptionId, OptionTranslated};
use crate::locale::{CharacterDirection, L10n, LangId, LanguageIdentifier};
use crate::service::HttpRequest;

/// Representa una página HTML completa lista para renderizar.
///
/// Una instancia de `Page` se compone dinámicamente permitiendo establecer título, descripción,
/// regiones donde disponer los componentes, atributos de `<body>` y otros aspectos del contexto de
/// renderizado.
#[rustfmt::skip]
pub struct Page {
    title       : OptionTranslated,
    description : OptionTranslated,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    context     : Context,
    body_id     : OptionId,
    body_classes: OptionClasses,
    regions     : ChildrenInRegions,
}

impl Page {
    /// Crea una nueva instancia de página.
    ///
    /// Si se proporciona la solicitud HTTP, se guardará en el contexto de renderizado de la página
    /// para poder ser recuperada por los componentes si es necesario.
    #[rustfmt::skip]
    pub fn new(request: Option<HttpRequest>) -> Self {
        Page {
            title       : OptionTranslated::default(),
            description : OptionTranslated::default(),
            metadata    : Vec::default(),
            properties  : Vec::default(),
            context     : Context::new(request),
            body_id     : OptionId::default(),
            body_classes: OptionClasses::default(),
            regions     : ChildrenInRegions::default(),
        }
    }

    // Page BUILDER ********************************************************************************

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

    /// Modifica la fuente de idioma de la página ([`Context::with_langid()`]).
    #[builder_fn]
    pub fn with_langid(mut self, language: &impl LangId) -> Self {
        self.context.alter_langid(language);
        self
    }

    /// Modifica el tema que se usará para renderizar la página ([`Context::with_theme()`]).
    #[builder_fn]
    pub fn with_theme(mut self, theme_name: &'static str) -> Self {
        self.context.alter_theme(theme_name);
        self
    }

    /// Modifica la composición para renderizar la página ([`Context::with_layout()`]).
    #[builder_fn]
    pub fn with_layout(mut self, layout_name: &'static str) -> Self {
        self.context.alter_layout(layout_name);
        self
    }

    /// Define los recursos de la página usando [`AssetsOp`].
    #[builder_fn]
    pub fn with_assets(mut self, op: AssetsOp) -> Self {
        self.context.alter_assets(op);
        self
    }

    /// Establece el atributo `id` del elemento `<body>`.
    #[builder_fn]
    pub fn with_body_id(mut self, id: impl AsRef<str>) -> Self {
        self.body_id.alter_value(id);
        self
    }

    /// Modifica las clases CSS del elemento `<body>` con una operación sobre [`OptionClasses`].
    #[builder_fn]
    pub fn with_body_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    /// Añade un componente a la región de contenido por defecto.
    pub fn with_component(mut self, component: impl Component) -> Self {
        self.regions
            .alter_child_in_region(CONTENT_REGION_NAME, ChildOp::Add(Child::with(component)));
        self
    }

    /// Añade un componente en una región (`region_name`) de la página.
    pub fn with_component_in(
        mut self,
        region_name: &'static str,
        component: impl Component,
    ) -> Self {
        self.regions
            .alter_child_in_region(region_name, ChildOp::Add(Child::with(component)));
        self
    }

    /// Opera con [`ChildOp`] en una región (`region_name`) de la página.
    #[builder_fn]
    pub fn with_child_in_region(mut self, region_name: &'static str, op: ChildOp) -> Self {
        self.regions.alter_child_in_region(region_name, op);
        self
    }

    // Page GETTERS ********************************************************************************

    /// Devuelve el título traducido para el idioma de la página, si existe.
    pub fn title(&mut self) -> Option<String> {
        self.title.using(&self.context)
    }

    /// Devuelve la descripción traducida para el idioma de la página, si existe.
    pub fn description(&mut self) -> Option<String> {
        self.description.using(&self.context)
    }

    /// Devuelve la lista de metadatos `<meta name=...>`.
    pub fn metadata(&self) -> &Vec<(&str, &str)> {
        &self.metadata
    }

    /// Devuelve la lista de propiedades `<meta property=...>`.
    pub fn properties(&self) -> &Vec<(&str, &str)> {
        &self.properties
    }

    /// Devuelve la solicitud HTTP asociada.
    pub fn request(&self) -> Option<&HttpRequest> {
        self.context.request()
    }

    /// Devuelve el identificador de idioma asociado.
    pub fn langid(&self) -> &LanguageIdentifier {
        self.context.langid()
    }

    /// Devuelve el tema que se usará para renderizar la página.
    pub fn theme(&self) -> ThemeRef {
        self.context.theme()
    }

    /// Devuelve la composición para renderizar la página. Por defecto es `"default"`.
    pub fn layout(&self) -> &str {
        self.context.layout()
    }

    /// Devuelve el identificador del elemento `<body>`.
    pub fn body_id(&self) -> &OptionId {
        &self.body_id
    }

    /// Devuelve las clases CSS del elemento `<body>`.
    pub fn body_classes(&self) -> &OptionClasses {
        &self.body_classes
    }

    // Page RENDER *********************************************************************************

    /// Renderiza los componentes de una región (`regiona_name`) de la página.
    pub fn render_region(&mut self, region_name: &'static str) -> Markup {
        self.regions
            .merge_all_components(self.context.theme(), region_name)
            .render(&mut self.context)
    }

    /// Renderiza los recursos de la página.
    pub fn render_assets(&self) -> Markup {
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
