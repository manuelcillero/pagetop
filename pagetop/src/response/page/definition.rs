use super::{BeforeRenderPageHook, ResultPage, HOOK_BEFORE_RENDER_PAGE};

use crate::core::component::*;
use crate::core::hook::{action_ref, run_actions};
use crate::html::{html, Classes, ClassesOp, Favicon, Markup, DOCTYPE};
use crate::locale::{langid_for, LanguageIdentifier};
use crate::response::fatal_error::FatalError;
use crate::{fn_builder, server};

use unic_langid::CharacterDirection;

use std::collections::HashMap;

type PageTitle = OneComponent<L10n>;
type PageDescription = OneComponent<L10n>;

#[rustfmt::skip]
pub struct Page {
    title       : PageTitle,
    description : PageDescription,
    metadata    : Vec<(&'static str, &'static str)>,
    properties  : Vec<(&'static str, &'static str)>,
    favicon     : Option<Favicon>,
    context     : RenderContext,
    body_classes: Classes,
    regions     : HashMap<&'static str, ComponentsBundle>,
    template    : String,
}

impl Default for Page {
    #[rustfmt::skip]
    fn default() -> Self {
        Page {
            title       : PageTitle::new(),
            description : PageDescription::new(),
            metadata    : Vec::new(),
            properties  : Vec::new(),
            favicon     : None,
            context     : RenderContext::new(),
            body_classes: Classes::new().with_value(ClassesOp::SetDefault, "body"),
            regions     : common_components(),
            template    : "default".to_owned(),
        }
    }
}

impl Page {
    pub fn new(request: server::HttpRequest) -> Self {
        let mut page = Page::default();
        page.context.alter(ContextOp::Request(Some(request)));
        page
    }

    // Page BUILDER.

    #[fn_builder]
    pub fn alter_language(&mut self, language: &'static str) -> &mut Self {
        self.context.alter(ContextOp::LangId(langid_for(language)));
        self
    }

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
    pub fn alter_this_in(
        &mut self,
        region: &'static str,
        component: impl ComponentTrait,
    ) -> &mut Self {
        if let Some(regions) = self.regions.get_mut(region) {
            regions.add(component);
        } else {
            self.regions
                .insert(region, ComponentsBundle::new_with(component));
        }
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Page GETTERS.

    pub fn langid(&self) -> &LanguageIdentifier {
        self.context.langid()
    }

    pub fn title(&mut self) -> String {
        self.title.render(&mut self.context).into_string()
    }

    pub fn description(&mut self) -> String {
        self.description.render(&mut self.context).into_string()
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

    pub fn context(&mut self) -> &mut RenderContext {
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
        // Acciones de los módulos antes de renderizar la página.
        run_actions(HOOK_BEFORE_RENDER_PAGE, |hook| {
            action_ref::<BeforeRenderPageHook>(&**hook).run(self)
        });

        // Acciones del tema antes de renderizar la página.
        self.context.theme().before_render_page(self);

        // Primero, renderizar el cuerpo.
        let body = self.context.theme().render_page_body(self);

        // Luego, renderizar la cabecera.
        let head = self.context.theme().render_page_head(self);

        // Finalmente, renderizar la página.
        let lang = self.langid().language.as_str();
        let dir = match self.langid().character_direction() {
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

    pub fn render_region(&mut self, region: &str) -> Option<Markup> {
        match self.regions.get_mut(region) {
            Some(components) => Some(components.render(&mut self.context)),
            None => None,
        }
    }
}
