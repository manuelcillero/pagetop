mod error;
pub use error::ErrorPage;

mod context;
pub use context::{AssetsOp, ContextPage /*, ParamError*/};
/*
pub type FnContextualPath = fn(cx: &Context) -> &str;
*/

use crate::fn_builder;
use crate::html::{html, Markup, PrepareMarkup, DOCTYPE};
use crate::html::{ClassesOp, OptionClasses, OptionId, OptionTranslated};
use crate::locale::L10n;
use crate::service::HttpRequest;

pub use actix_web::Result as ResultPage;

use unic_langid::CharacterDirection;

#[rustfmt::skip]
pub struct Page {
    title       : OptionTranslated,
    description : OptionTranslated,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    context     : ContextPage,
    body_id     : OptionId,
    body_classes: OptionClasses,
    body_content: PrepareMarkup,
}

impl Page {
    #[rustfmt::skip]
    pub fn new(request: HttpRequest) -> Self {
        Page {
            title       : OptionTranslated::default(),
            description : OptionTranslated::default(),
            metadata    : Vec::default(),
            properties  : Vec::default(),
            context     : ContextPage::new(request),
            body_id     : OptionId::default(),
            body_classes: OptionClasses::default(),
            body_content: PrepareMarkup::default(),
        }
    }

    // Page BUILDER.

    #[fn_builder]
    pub fn set_title(&mut self, title: L10n) -> &mut Self {
        self.title.set_value(title);
        self
    }

    #[fn_builder]
    pub fn set_description(&mut self, description: L10n) -> &mut Self {
        self.description.set_value(description);
        self
    }

    #[fn_builder]
    pub fn set_metadata(&mut self, name: &'static str, content: &'static str) -> &mut Self {
        self.metadata.push((name, content));
        self
    }

    #[fn_builder]
    pub fn set_property(&mut self, property: &'static str, content: &'static str) -> &mut Self {
        self.metadata.push((property, content));
        self
    }

    #[fn_builder]
    pub fn set_assets(&mut self, op: AssetsOp) -> &mut Self {
        self.context.set_assets(op);
        self
    }

    #[fn_builder]
    pub fn set_body_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.body_id.set_value(id);
        self
    }

    #[fn_builder]
    pub fn set_body_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.body_classes.set_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn set_body(&mut self, content: PrepareMarkup) -> &mut Self {
        self.body_content = content;
        self
    }
    /*
        #[fn_builder]
        pub fn set_layout(&mut self, layout: &'static str) -> &mut Self {
            self.context.set_assets(AssetsOp::Layout(layout));
            self
        }

        #[fn_builder]
        pub fn set_regions(&mut self, region: &'static str, op: AnyOp) -> &mut Self {
            self.context.set_regions(region, op);
            self
        }

        pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
            self.context
                .set_regions("content", AnyOp::Add(AnyComponent::with(component)));
            self
        }

        pub fn with_component_in(
            mut self,
            region: &'static str,
            component: impl ComponentTrait,
        ) -> Self {
            self.context
                .set_regions(region, AnyOp::Add(AnyComponent::with(component)));
            self
        }
    */
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

    pub fn context(&mut self) -> &mut ContextPage {
        &mut self.context
    }

    pub fn body_id(&self) -> &OptionId {
        &self.body_id
    }

    pub fn body_classes(&self) -> &OptionClasses {
        &self.body_classes
    }

    pub fn body_content(&self) -> &PrepareMarkup {
        &self.body_content
    }

    // Page RENDER.

    pub fn render(&mut self) -> ResultPage<Markup, ErrorPage> {
        // Theme operations before preparing the page body.
        //self.context.theme().before_prepare_body(self);

        // Packages actions before preparing the page body.
        //action::page::BeforePrepareBody::dispatch(self);

        // Prepare page body.
        let body = self.context.theme().prepare_page_body(self);

        // Theme operations after preparing the page body.
        //self.context.theme().after_prepare_body(self);

        // Packages actions after preparing the page body.
        //action::page::AfterPrepareBody::dispatch(self);

        // Prepare page head.
        let head = self.context.theme().prepare_page_head(self);

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
                (head.render())
                (body.render())
            }
        })
    }
}
