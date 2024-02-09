mod error;
pub use error::ErrorPage;

pub use actix_web::Result as ResultPage;

use crate::base::action;
use crate::core::component::{AnyComponents, ArcAnyComponent, ComponentTrait};
use crate::core::component::{Context, ContextOp};
use crate::core::theme::ComponentsInRegions;
use crate::html::{html, Markup, DOCTYPE};
use crate::html::{ClassesOp, Favicon, OptionClasses, OptionId, OptionTranslated};
use crate::locale::L10n;
use crate::{fn_with, service};

use unic_langid::CharacterDirection;

#[rustfmt::skip]
pub struct Page {
    title       : OptionTranslated,
    description : OptionTranslated,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    favicon     : Option<Favicon>,
    context     : Context,
    body_classes: OptionClasses,
    skip_to     : OptionId,
    regions     : ComponentsInRegions,
    template    : String,
}

impl Page {
    #[rustfmt::skip]
    pub fn new(request: service::HttpRequest) -> Self {
        Page {
            title       : OptionTranslated::default(),
            description : OptionTranslated::default(),
            metadata    : Vec::default(),
            properties  : Vec::default(),
            favicon     : None,
            context     : Context::new(request),
            body_classes: OptionClasses::default(),
            skip_to     : OptionId::default(),
            regions     : ComponentsInRegions::default(),
            template    : "default".to_owned(),
        }
    }

    // Page BUILDER.

    #[fn_with]
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    #[fn_with]
    pub fn alter_description(&mut self, description: L10n) -> &mut Self {
        self.description.alter_value(description);
        self
    }

    #[fn_with]
    pub fn alter_metadata(&mut self, name: &'static str, content: &'static str) -> &mut Self {
        self.metadata.push((name, content));
        self
    }

    #[fn_with]
    pub fn alter_property(&mut self, property: &'static str, content: &'static str) -> &mut Self {
        self.metadata.push((property, content));
        self
    }

    #[fn_with]
    pub fn alter_favicon(&mut self, favicon: Option<Favicon>) -> &mut Self {
        self.favicon = favicon;
        self
    }

    #[fn_with]
    pub fn alter_context(&mut self, op: ContextOp) -> &mut Self {
        self.context.alter(op);
        self
    }

    #[fn_with]
    pub fn alter_body_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    #[fn_with]
    pub fn alter_skip_to(&mut self, id: impl Into<String>) -> &mut Self {
        self.skip_to.alter_value(id);
        self
    }

    #[fn_with]
    pub fn alter_component_in(
        &mut self,
        region: &'static str,
        component: impl ComponentTrait,
    ) -> &mut Self {
        self.regions
            .add_component_in(region, ArcAnyComponent::new(component));
        self
    }

    #[fn_with]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
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

    pub fn body_classes(&self) -> &OptionClasses {
        &self.body_classes
    }

    pub fn skip_to(&self) -> &OptionId {
        &self.skip_to
    }

    pub fn components_in(&self, region: &str) -> AnyComponents {
        self.regions.get_components(self.context.theme(), region)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Page RENDER.

    pub fn render(&mut self) -> ResultPage<Markup, ErrorPage> {
        // Theme actions before preparing the page body.
        self.context.theme().before_prepare_body(self);

        // Packages actions before preparing the page body.
        action::page::BeforePrepareBody::dispatch(self);

        // Prepare page body.
        let body = self.context.theme().prepare_body(self);

        // Theme actions after preparing the page body.
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
