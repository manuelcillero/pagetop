use super::{BeforeRenderPageHook, PageContext, PageOp, ResultPage, HOOK_BEFORE_RENDER_PAGE};

use crate::app::fatal_error::FatalError;
use crate::core::component::*;
use crate::core::hook::{action_ref, run_actions};
use crate::html::{html, AttributeValue, Classes, ClassesOp, Markup, DOCTYPE};
use crate::{global, trace, LazyStatic};

use std::collections::HashMap;

static DEFAULT_LANGUAGE: LazyStatic<Option<String>> = LazyStatic::new(|| {
    let language = global::SETTINGS.app.language[..2].to_lowercase();
    if !language.is_empty() {
        Some(language)
    } else {
        None
    }
});

static DEFAULT_DIRECTION: LazyStatic<Option<String>> = LazyStatic::new(|| {
    let direction = global::SETTINGS.app.direction.to_lowercase();
    match direction.as_str() {
        "auto" => Some("auto".to_owned()),
        "ltr" => Some("ltr".to_owned()),
        "rtl" => Some("rtl".to_owned()),
        "" => None,
        _ => {
            trace::warn!(
                "Text direction \"{}\" not valid, {}",
                global::SETTINGS.app.direction,
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
    context     : PageContext,
    language    : AttributeValue,
    direction   : AttributeValue,
    title       : AttributeValue,
    description : AttributeValue,
    body_classes: Classes,
    regions     : HashMap<&'static str, ComponentsBundle>,
    template    : String,
}

impl Default for Page {
    #[rustfmt::skip]
    fn default() -> Self {
        Page {
            context     : PageContext::new(),
            language    : match &*DEFAULT_LANGUAGE {
                Some(language) => AttributeValue::new().with_value(language),
                _ => AttributeValue::new(),
            },
            direction   : match &*DEFAULT_DIRECTION {
                Some(direction) => AttributeValue::new().with_value(direction),
                _ => AttributeValue::new(),
            },
            title       : AttributeValue::new(),
            description : AttributeValue::new(),
            body_classes: Classes::new().with_value(ClassesOp::SetDefault, "body"),
            regions     : common_components(),
            template    : "default".to_owned(),
        }
    }
}

impl Page {
    pub fn new() -> Self {
        Page::default()
    }

    // Page BUILDER.

    pub fn with_context(mut self, op: PageOp) -> Self {
        self.alter_context(op);
        self
    }

    pub fn with_language(mut self, language: &str) -> Self {
        self.alter_language(language);
        self
    }

    pub fn with_direction(mut self, dir: TextDirection) -> Self {
        self.alter_direction(dir);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.alter_title(title);
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.alter_description(description);
        self
    }

    pub fn with_body_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_body_classes(op, classes);
        self
    }

    pub fn add_to(mut self, region: &'static str, component: impl ComponentTrait) -> Self {
        if let Some(regions) = self.regions.get_mut(region) {
            regions.add(component);
        } else {
            self.regions
                .insert(region, ComponentsBundle::new_with(component));
        }
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Page ALTER.

    pub fn alter_context(&mut self, op: PageOp) -> &mut Self {
        self.context.alter(op);
        self
    }

    pub fn alter_language(&mut self, language: &str) -> &mut Self {
        self.language.alter_value(language);
        self
    }

    pub fn alter_direction(&mut self, dir: TextDirection) -> &mut Self {
        self.direction.alter_value(match dir {
            TextDirection::Auto => "auto",
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
        });
        self
    }

    pub fn alter_title(&mut self, title: &str) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    pub fn alter_description(&mut self, description: &str) -> &mut Self {
        self.description.alter_value(description);
        self
    }

    pub fn alter_body_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.body_classes.alter_value(op, classes);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Page GETTERS.

    pub fn context(&mut self) -> &mut PageContext {
        &mut self.context
    }

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
