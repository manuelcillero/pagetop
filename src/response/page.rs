mod error;
pub use error::ErrorPage;

pub use actix_web::Result as ResultPage;

use crate::base::action;
use crate::core::component::{AnyComponent, AnyOp, ComponentTrait};
use crate::core::component::{AssetsOp, Context};
use crate::fn_builder;
use crate::html::{html, ClassesOp, Favicon, Markup, OptionTranslated, DOCTYPE};
use crate::locale::L10n;
use crate::service::HttpRequest;

use unic_langid::CharacterDirection;

#[rustfmt::skip]
pub struct Page {
    title      : OptionTranslated,
    description: OptionTranslated,
    metadata   : Vec<(&'static str, &'static str)>,
    properties : Vec<(&'static str, &'static str)>,
    favicon    : Option<Favicon>,
    context    : Context,
}

impl Page {
    #[rustfmt::skip]
    pub fn new(request: HttpRequest) -> Self {
        Page {
            title      : OptionTranslated::default(),
            description: OptionTranslated::default(),
            metadata   : Vec::default(),
            properties : Vec::default(),
            favicon    : None,
            context    : Context::new(request),
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
    pub fn alter_favicon(&mut self, favicon: Option<Favicon>) -> &mut Self {
        self.favicon = favicon;
        self
    }

    #[fn_builder]
    pub fn alter_assets(&mut self, op: AssetsOp) -> &mut Self {
        self.context.alter_assets(op);
        self
    }

    #[fn_builder]
    pub fn alter_body_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.context.alter_body_id(id);
        self
    }

    #[fn_builder]
    pub fn alter_body_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.context.alter_body_classes(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_body_skip_to(&mut self, id: impl Into<String>) -> &mut Self {
        self.context.alter_body_skip_to(id);
        self
    }

    #[fn_builder]
    pub fn alter_layout(&mut self, layout: &'static str) -> &mut Self {
        self.context.alter_assets(AssetsOp::Layout(layout));
        self
    }

    #[fn_builder]
    pub fn alter_regions(&mut self, region: &'static str, op: AnyOp) -> &mut Self {
        self.context.alter_regions(region, op);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.context
            .alter_regions("content", AnyOp::Add(AnyComponent::with(component)));
        self
    }

    pub fn with_component_in(
        mut self,
        region: &'static str,
        component: impl ComponentTrait,
    ) -> Self {
        self.context
            .alter_regions(region, AnyOp::Add(AnyComponent::with(component)));
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

    pub fn favicon(&self) -> &Option<Favicon> {
        &self.favicon
    }

    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    // Page RENDER.

    pub fn render(&mut self) -> ResultPage<Markup, ErrorPage> {
        // Theme operations before preparing the page body.
        self.context.theme().before_prepare_body(self);

        // Packages actions before preparing the page body.
        action::page::BeforePrepareBody::dispatch(self);

        // Prepare page body.
        let body = self.context.theme().prepare_body(self);

        // Theme operations after preparing the page body.
        self.context.theme().after_prepare_body(self);

        // Packages actions after preparing the page body.
        action::page::AfterPrepareBody::dispatch(self);

        // Prepare page head.
        let head = self.context.theme().prepare_head(self);

        // Render the page.
        let lang = self.context.langid().language.as_str();
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
