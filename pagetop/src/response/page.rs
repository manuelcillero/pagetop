mod error;
pub use error::ErrorPage;

pub use actix_web::Result as ResultPage;

use crate::base::action;
use crate::core::component::{AssetsOp, Context};
use crate::core::component::{ChildComponent, ChildOp, ComponentTrait};
use crate::fn_builder;
use crate::html::{html, Markup, DOCTYPE};
use crate::html::{ClassesOp, OptionClasses, OptionId, OptionTranslated};
use crate::locale::L10n;
use crate::service::HttpRequest;

use unic_langid::CharacterDirection;

#[rustfmt::skip]
pub struct Page {
    title       : OptionTranslated,
    description : OptionTranslated,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    context     : Context,
    body_id     : OptionId,
    body_classes: OptionClasses,
}

impl Page {
    #[rustfmt::skip]
    pub fn new(request: HttpRequest) -> Self {
        Page {
            title       : OptionTranslated::default(),
            description : OptionTranslated::default(),
            metadata    : Vec::default(),
            properties  : Vec::default(),
            context     : Context::new(request),
            body_id     : OptionId::default(),
            body_classes: OptionClasses::default(),
        }
    }

    // Page BUILDER.

    #[fn_builder]
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    #[fn_builder]
    pub fn alter_description(&mut self, description: L10n) -> &mut Self {
        self.description.alter_value(description);
        self
    }

    #[fn_builder]
    pub fn alter_metadata(&mut self, name: &'static str, content: &'static str) -> &mut Self {
        self.metadata.push((name, content));
        self
    }

    #[fn_builder]
    pub fn alter_property(&mut self, property: &'static str, content: &'static str) -> &mut Self {
        self.metadata.push((property, content));
        self
    }

    #[fn_builder]
    pub fn alter_assets(&mut self, op: AssetsOp) -> &mut Self {
        self.context.alter_assets(op);
        self
    }

    #[fn_builder]
    pub fn alter_body_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.body_id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_body_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_theme(&mut self, theme: &'static str) -> &mut Self {
        self.context.alter_assets(AssetsOp::Theme(theme));
        self
    }

    #[fn_builder]
    pub fn alter_layout(&mut self, layout: &'static str) -> &mut Self {
        self.context.alter_assets(AssetsOp::Layout(layout));
        self
    }

    #[fn_builder]
    pub fn alter_in_region(&mut self, region: &'static str, op: ChildOp) -> &mut Self {
        self.context.alter_in_region(region, op);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.context
            .alter_in_region("content", ChildOp::Add(ChildComponent::with(component)));
        self
    }

    pub fn with_component_in(
        mut self,
        region: &'static str,
        component: impl ComponentTrait,
    ) -> Self {
        self.context
            .alter_in_region(region, ChildOp::Add(ChildComponent::with(component)));
        self
    }

    // Page GETTERS.

    pub fn title(&mut self) -> Option<String> {
        self.title.using(self.context.langid())
    }

    pub fn description(&mut self) -> Option<String> {
        self.description.using(self.context.langid())
    }

    pub fn metadata(&self) -> &Vec<(&str, &str)> {
        &self.metadata
    }

    pub fn properties(&self) -> &Vec<(&str, &str)> {
        &self.properties
    }

    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn body_id(&self) -> &OptionId {
        &self.body_id
    }

    pub fn body_classes(&self) -> &OptionClasses {
        &self.body_classes
    }

    // Page RENDER.

    pub fn render(&mut self) -> ResultPage<Markup, ErrorPage> {
        // Acciones específicas del tema antes de renderizar el <body>.
        self.context.theme().before_render_page_body(self);

        // Acciones de los paquetes antes de renderizar el <body>.
        action::page::BeforeRenderBody::dispatch(self);

        // Renderiza el <body>.
        let body = self.context.theme().render_page_body(self);

        // Acciones específicas del tema después de renderizar el <body>.
        self.context.theme().after_render_page_body(self);

        // Acciones de los paquetes después de renderizar el <body>.
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
