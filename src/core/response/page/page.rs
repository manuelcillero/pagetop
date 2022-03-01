use crate::{Lazy, trace, util};
use crate::config::SETTINGS;
use crate::core::{all, server};
use crate::core::theme::{DOCTYPE, Markup, html};
use crate::core::response::page::{PageAssets, PageComponent, PageContainer};

use std::borrow::Cow;
use std::collections::HashMap;

pub static DEFAULT_LANGUAGE: Lazy<Option<String>> = Lazy::new(|| {
    let language = SETTINGS.app.language[..2].to_lowercase();
    if !language.is_empty() {
        Some(language)
    } else {
        None
    }
});

pub static DEFAULT_DIRECTION: Lazy<Option<String>> = Lazy::new(|| {
    let direction = SETTINGS.app.direction.to_lowercase();
    match direction.as_str() {
        "auto" => Some("auto".to_string()),
        "ltr" => Some("ltr".to_string()),
        "rtl" => Some("rtl".to_string()),
        "" => None,
        _ => {
            trace::warn!(
                "Text direction \"{}\" not valid. {}.",
                SETTINGS.app.direction,
                "Check the settings file"
            );
            None
        }
    }
});

pub enum TextDirection { Auto, LeftToRight, RightToLeft }

pub struct Page<'a> {
    language    : Option<String>,
    direction   : Option<String>,
    title       : Option<String>,
    description : Option<String>,
    assets      : PageAssets,
    body_classes: Cow<'a, str>,
    regions     : HashMap<&'a str, PageContainer>,
    template    : Option<String>,
}

impl<'a> Page<'a> {

    pub fn prepare() -> Self {
        Page {
            language    : match &*DEFAULT_LANGUAGE {
                Some(language) => Some(language.to_string()),
                _ => None,
            },
            direction   : match &*DEFAULT_DIRECTION {
                Some(direction) => Some(direction.to_string()),
                _ => None,
            },
            title       : None,
            description : None,
            body_classes: "body".into(),
            assets      : PageAssets::new(),
            regions     : all::COMPONENTS.read().unwrap().clone(),
            template    : Some("default".to_string()),
        }
    }

    // Page BUILDER.

    pub fn with_language(&mut self, language: &str) -> &mut Self {
        self.language = util::optional_value(language);
        self
    }

    pub fn with_direction(&mut self, dir: TextDirection) -> &mut Self {
        self.direction = match dir {
            TextDirection::Auto => Some("auto".to_string()),
            TextDirection::LeftToRight => Some("ltr".to_string()),
            TextDirection::RightToLeft => Some("rtl".to_string()),
        };
        self
    }

    pub fn with_title(&mut self, title: &str) -> &mut Self {
        self.title = util::optional_value(title);
        self
    }

    pub fn with_description(&mut self, description: &str) -> &mut Self {
        self.description = util::optional_value(description);
        self
    }

    pub fn with_body_classes(&mut self, body_classes: &'a str) -> &mut Self {
        self.body_classes = body_classes.into();
        self
    }

    pub fn add_body_classes(&mut self, body_classes: &'a str) -> &mut Self {
        self.body_classes = String::from(
            format!("{} {}", self.body_classes, body_classes).trim()
        ).into();
        self
    }

    pub fn add_to(
        &mut self,
        region: &'a str,
        component: impl PageComponent
    ) -> &mut Self {
        if let Some(regions) = self.regions.get_mut(region) {
            regions.add(component);
        } else {
            self.regions.insert(region, PageContainer::new_with(component));
        }
        self
    }

    pub fn using_template(&mut self, template: &str) -> &mut Self {
        self.template = util::optional_value(template);
        self
    }

    // Page GETTERS.

    pub fn language(&self) -> &str {
        util::assigned_value(&self.language)
    }

    pub fn direction(&self) -> &str {
        util::assigned_value(&self.direction)
    }

    pub fn title(&self) -> &str {
        util::assigned_value(&self.title)
    }

    pub fn description(&self) -> &str {
        util::assigned_value(&self.description)
    }

    pub fn body_classes(&self) -> &str {
        if self.body_classes.is_empty() {
            return "body";
        }
        &self.body_classes
    }

    pub fn assets(&mut self) -> &mut PageAssets {
        &mut self.assets
    }

    pub fn template(&self) -> &str {
        util::assigned_value(&self.template)
    }

    // Page RENDER.

    pub fn render(&mut self) -> server::Result<Markup> {
        // Acciones del tema antes de renderizar la página.
        self.assets.theme().before_render_page(self);

        // Primero, renderizar el cuerpo.
        let body = self.assets.theme().render_page_body(self);

        // Luego, renderizar la cabecera.
        let head = self.assets.theme().render_page_head(self);

        // Finalmente, renderizar la página.
        return Ok(html! {
            (DOCTYPE)
            html lang=[&self.language] dir=[&self.direction] {
                (head)
                (body)
            }
        })
    }

    pub fn render_region(&mut self, region: &str) -> Markup {
        match self.regions.get_mut(region) {
            Some(components) => components.render(&mut self.assets),
            None => html! {}
        }
    }

    // Page EXTRAS.

    pub fn using_theme(&mut self, theme_id: &str) -> &mut Self {
        self.assets.using_theme(theme_id);
        self
    }
}

pub fn render_component(
    component: &dyn PageComponent,
    assets: &mut PageAssets
) -> Markup {
    match component.is_renderable() {
        true => match assets.theme().render_component(component, assets) {
            Some(markup) => markup,
            None => component.default_render(assets)
        },
        false => html! {}
    }
}

pub fn add_component_to(region: &'static str, component: impl PageComponent) {
    let mut hmap = all::COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, PageContainer::new_with(component));
    }
}
