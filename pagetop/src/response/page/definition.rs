use super::{BeforeRenderPageHook, ResultPage, HOOK_BEFORE_RENDER_PAGE};

use crate::core::component::*;
use crate::core::hook::{action_ref, run_actions};
use crate::html::{
    html, AttributeValue, Classes, ClassesOp, ContextOp, Favicon, Markup, RenderContext, DOCTYPE,
};
use crate::response::FatalError;
use crate::{config, fn_builder, locale, server, trace, LazyStatic};

use std::collections::HashMap;

static DEFAULT_DIRECTION: LazyStatic<Option<String>> = LazyStatic::new(|| {
    let direction = config::SETTINGS.app.direction.to_lowercase();
    match direction.as_str() {
        "auto" => Some("auto".to_owned()),
        "ltr" => Some("ltr".to_owned()),
        "rtl" => Some("rtl".to_owned()),
        "" => None,
        _ => {
            trace::warn!(
                "Text direction \"{}\" not valid, {}",
                config::SETTINGS.app.direction,
                "check the settings file"
            );
            None
        }
    }
});

pub enum TextDirection {
    Auto,
    LeftToRight,
    RightToLeft,
}

#[rustfmt::skip]
pub struct Page {
    language    : AttributeValue,
    direction   : AttributeValue,
    title       : AttributeValue,
    description : AttributeValue,
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
            language    : AttributeValue::new().with_value(locale::LANGID.language.as_str()),
            direction   : match &*DEFAULT_DIRECTION {
                Some(direction) => AttributeValue::new().with_value(direction),
                _ => AttributeValue::new(),
            },
            title       : AttributeValue::new(),
            description : AttributeValue::new(),
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
    pub fn alter_language(&mut self, language: &str) -> &mut Self {
        self.language.alter_value(language);
        self
    }

    #[fn_builder]
    pub fn alter_direction(&mut self, dir: TextDirection) -> &mut Self {
        self.direction.alter_value(match dir {
            TextDirection::Auto => "auto",
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
        });
        self
    }

    #[fn_builder]
    pub fn alter_title(&mut self, title: &str) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    #[fn_builder]
    pub fn alter_description(&mut self, description: &str) -> &mut Self {
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

    pub fn language(&self) -> &AttributeValue {
        &self.language
    }

    pub fn direction(&self) -> &AttributeValue {
        &self.direction
    }

    pub fn title(&self) -> &AttributeValue {
        &self.title
    }

    pub fn description(&self) -> &AttributeValue {
        &self.description
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
        Ok(html! {
            (DOCTYPE)
            html lang=[self.language().get()] dir=[self.direction().get()] {
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
