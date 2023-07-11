mod action;
pub use action::*;

use crate::core::component::l10n::L10n;
use crate::core::component::{ComponentTrait, Context, ContextOp, OneComponent};
use crate::core::theme::ComponentsRegions;
use crate::html::{html, Classes, ClassesOp, Favicon, Markup, DOCTYPE};
use crate::response::fatal_error::FatalError;
use crate::{fn_builder, service};

use unic_langid::CharacterDirection;

pub use actix_web::Result as ResultPage;

type PageTitle = OneComponent<L10n>;
type PageDescription = OneComponent<L10n>;

#[rustfmt::skip]
pub struct Page {
    title       : PageTitle,
    description : PageDescription,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    favicon     : Option<Favicon>,
    context     : Context,
    body_classes: Classes,
    regions     : ComponentsRegions,
    template    : String,
}

impl Page {
    #[rustfmt::skip]
    pub fn new(request: service::HttpRequest) -> Self {
        Page {
            title       : PageTitle::new(),
            description : PageDescription::new(),
            metadata    : Vec::new(),
            properties  : Vec::new(),
            favicon     : None,
            context     : Context::new(request),
            body_classes: Classes::new().with_value(ClassesOp::SetDefault, "body"),
            regions     : ComponentsRegions::new(),
            template    : "default".to_owned(),
        }
    }

    // Page BUILDER.

    #[fn_builder]
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.set(title);
        self
    }

    #[fn_builder]
    pub fn alter_description(&mut self, description: L10n) -> &mut Self {
        self.description.set(description);
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
    pub fn alter_body_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_in(&mut self, region: &'static str, component: impl ComponentTrait) -> &mut Self {
        self.regions.add_to(region, component);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Page GETTERS.

    pub fn title(&mut self) -> String {
        self.title.prepare(&mut self.context).into_string()
    }

    pub fn description(&mut self) -> String {
        self.description.prepare(&mut self.context).into_string()
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

    pub fn body_classes(&self) -> &Classes {
        &self.body_classes
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Page RENDER.

    pub fn render(&mut self) -> ResultPage<Markup, FatalError> {
        // Module actions before preparing the page body.
        run_actions_before_prepare_body(self);

        // Theme actions before preparing the page body.
        self.context.theme().before_prepare_body(self);

        // Prepare page body.
        let body = self.context.theme().prepare_body(self);

        // Module actions after preparing the page body.
        run_actions_after_prepare_body(self);

        // Theme actions after preparing the page body.
        self.context.theme().after_prepare_body(self);

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

    pub fn prepare_region(&mut self, region: &str) -> Option<Markup> {
        let render = self
            .regions
            .get_extended_pack(self.context.theme().single_name(), region)
            .prepare(self.context());
        if render.is_empty() {
            None
        } else {
            Some(render)
        }
    }
}
