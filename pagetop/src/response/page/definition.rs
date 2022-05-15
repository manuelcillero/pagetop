use crate::{Lazy, app, trace};
use crate::config::SETTINGS;
use crate::html::*;
use crate::core::hook::{hook_ref, run_hooks};
use crate::core::component::*;
use super::{BEFORE_RENDER_PAGE_HOOK, BeforeRenderPageHook};

use std::collections::HashMap;

static DEFAULT_LANGUAGE: Lazy<Option<String>> = Lazy::new(|| {
    let language = SETTINGS.app.language[..2].to_lowercase();
    if !language.is_empty() {
        Some(language)
    } else {
        None
    }
});

static DEFAULT_DIRECTION: Lazy<Option<String>> = Lazy::new(|| {
    let direction = SETTINGS.app.direction.to_lowercase();
    match direction.as_str() {
        "auto" => Some("auto".to_owned()),
        "ltr" => Some("ltr".to_owned()),
        "rtl" => Some("rtl".to_owned()),
        "" => None,
        _ => {
            trace::warn!(
                "Text direction \"{}\" not valid, {}",
                SETTINGS.app.direction,
                "check the settings file"
            );
            None
        }
    }
});

pub enum TextDirection { Auto, LeftToRight, RightToLeft }

pub struct Page<'a> {
    language    : OptAttr,
    direction   : OptAttr,
    title       : OptAttr,
    description : OptAttr,
    context     : InContext,
    regions     : HashMap<&'a str, ComponentsBundle>,
    body_classes: Classes,
    template    : String,
}

impl<'a> Page<'a> {

    pub fn new() -> Self {
        Page {
            language    : match &*DEFAULT_LANGUAGE {
                Some(language) => OptAttr::new_with_value(language),
                _ => OptAttr::new(),
            },
            direction   : match &*DEFAULT_DIRECTION {
                Some(direction) => OptAttr::new_with_value(direction),
                _ => OptAttr::new(),
            },
            title       : OptAttr::new(),
            description : OptAttr::new(),
            context     : InContext::new(),
            regions     : common_components(),
            body_classes: Classes::new_with_default("body"),
            template    : "default".to_owned(),
        }
    }

    // Page BUILDER.

    pub fn with_language(&mut self, language: &str) -> &mut Self {
        self.language.with_value(language);
        self
    }

    pub fn with_direction(&mut self, dir: TextDirection) -> &mut Self {
        self.direction.with_value(match dir {
            TextDirection::Auto => "auto",
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
        });
        self
    }

    pub fn with_title(&mut self, title: &str) -> &mut Self {
        self.title.with_value(title);
        self
    }

    pub fn with_description(&mut self, description: &str) -> &mut Self {
        self.description.with_value(description);
        self
    }

    pub fn add_to(
        &mut self,
        region: &'a str,
        component: impl ComponentTrait
    ) -> &mut Self {
        if let Some(regions) = self.regions.get_mut(region) {
            regions.add(component);
        } else {
            self.regions.insert(region, ComponentsBundle::new_with(component));
        }
        self
    }

    pub fn alter_body_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.body_classes.alter(classes, op);
        self
    }

    pub fn using_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Page GETTERS.

    pub fn language(&self) -> &Option<String> {
        self.language.option()
    }

    pub fn direction(&self) -> &Option<String> {
        self.direction.option()
    }

    pub fn title(&self) -> &Option<String> {
        self.title.option()
    }

    pub fn description(&self) -> &Option<String> {
        self.description.option()
    }

    pub fn context(&mut self) -> &mut InContext {
        &mut self.context
    }

    pub fn body_classes(&self) -> &Option<String> {
        self.body_classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Page RENDER.

    pub fn render(&mut self) -> app::Result<Markup> {
        // Acciones de los módulos antes de renderizar la página.
        run_hooks(
            BEFORE_RENDER_PAGE_HOOK,
            |a| hook_ref::<BeforeRenderPageHook>(&**a).run(self)
        );

        // Acciones del tema antes de renderizar la página.
        self.context.theme().before_render_page(self);

        // Primero, renderizar el cuerpo.
        let body = self.context.theme().render_page_body(self);

        // Luego, renderizar la cabecera.
        let head = self.context.theme().render_page_head(self);

        // Finalmente, renderizar la página.
        return Ok(html! {
            (DOCTYPE)
            html lang=[self.language()] dir=[self.direction()] {
                (head)
                (body)
            }
        })
    }

    pub fn render_region(&mut self, region: &str) -> Markup {
        match self.regions.get_mut(region) {
            Some(components) => components.render(&mut self.context),
            None => html! {}
        }
    }

    // Page EXTRAS.

    pub fn using_theme(&mut self, theme_name: &str) -> &mut Self {
        self.context.using_theme(theme_name);
        self
    }
}
