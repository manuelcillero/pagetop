use crate::base::action;
use crate::core::component::{ArcComponent, ArcComponents as RegionComponents, ComponentTrait};
use crate::core::component::{Context, ContextOp};
use crate::core::theme::ComponentsRegions;
use crate::html::{html, Markup, DOCTYPE};
use crate::html::{ClassesOp, Favicon, OptionClasses, OptionId, OptionTranslated};
use crate::locale::L10n;
use crate::response::fatal_error::FatalError;
use crate::{fn_builder, service};

use unic_langid::CharacterDirection;

pub use actix_web::Result as ResultPage;

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
    regions     : ComponentsRegions,
    template    : String,
}

impl Page {
    #[rustfmt::skip]
    pub fn new(request: service::HttpRequest) -> Self {
        Page {
            title       : OptionTranslated::new(),
            description : OptionTranslated::new(),
            metadata    : Vec::new(),
            properties  : Vec::new(),
            favicon     : None,
            context     : Context::new(request),
            body_classes: OptionClasses::new(),
            skip_to     : OptionId::new(),
            regions     : ComponentsRegions::new(),
            template    : "default".to_owned(),
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
    pub fn alter_context(&mut self, op: ContextOp) -> &mut Self {
        self.context.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_body_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_skip_to(&mut self, id: impl Into<String>) -> &mut Self {
        self.skip_to.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_in(&mut self, region: &'static str, component: impl ComponentTrait) -> &mut Self {
        self.regions.add_in(region, ArcComponent::with(component));
        self
    }

    #[fn_builder]
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

    pub fn components_in(&self, region: &str) -> RegionComponents {
        self.regions.get_components(self.context.theme(), region)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Page RENDER.

    pub fn render(&mut self) -> ResultPage<Markup, FatalError> {
        // Theme actions before preparing the page body.
        self.context.theme().before_prepare_body(self);

        // Module actions before preparing the page body.
        action::page::BeforePrepareBody::dispatch(self);

        // Prepare page body.
        let body = self.context.theme().prepare_body(self);

        // Theme actions after preparing the page body.
        self.context.theme().after_prepare_body(self);

        // Module actions after preparing the page body.
        action::page::AfterPrepareBody::dispatch(self);

        // Prepare page head.
        let head = self.context.theme().prepare_head(self);

        // Render the page.
        let lang = self.context.langid().language.as_str();
        let dir = match self.context.langid().character_direction() {
            CharacterDirection::LTR => "ltr",
            CharacterDirection::RTL => "rtl",
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
