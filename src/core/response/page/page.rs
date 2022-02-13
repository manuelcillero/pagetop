use crate::config::SETTINGS;
use crate::core::server;
use crate::core::state::{COMPONENTS, THEME};
use crate::core::theme::{DOCTYPE, Markup, html};
use crate::core::response::page::{PageAssets, PageComponent, PageContainer};

use std::borrow::Cow;
use std::collections::HashMap;

pub enum TextDirection { LeftToRight, RightToLeft, Auto }

pub struct Page<'a> {
    language    : &'a str,
    title       : &'a str,
    direction   : &'a str,
    description : &'a str,
    body_classes: Cow<'a, str>,
    assets      : PageAssets,
    regions     : HashMap<&'a str, PageContainer>,
    template    : &'a str,
}

impl<'a> Page<'a> {

    pub fn prepare() -> Self {
        Page {
            language    : &SETTINGS.app.language[..2],
            title       : &SETTINGS.app.name,
            direction   : "ltr",
            description : "",
            body_classes: "body".into(),
            assets      : PageAssets::new(),
            regions     : COMPONENTS.read().unwrap().clone(),
            template    : "default",
        }
    }

    // Page BUILDER.

    pub fn with_language(&mut self, language: &'a str) -> &mut Self {
        self.language = language;
        self
    }

    pub fn with_title(&mut self, title: &'a str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn with_direction(&mut self, dir: TextDirection) -> &mut Self {
        self.direction = match dir {
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
            _ => "auto"
        };
        self
    }

    pub fn with_description(&mut self, description: &'a str) -> &mut Self {
        self.description = description;
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

    pub fn using_template(&mut self, template: &'a str) -> &mut Self {
        self.template = template;
        self
    }

    // Page GETTERS.

    pub fn language(&self) -> &str {
        self.language
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn direction(&self) -> TextDirection {
        match self.direction {
            "ltr" => TextDirection::LeftToRight,
            "rtl" => TextDirection::RightToLeft,
            _ => TextDirection::Auto
        }
    }

    pub fn description(&self) -> &str {
        self.description
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
        self.template
    }

    // Page RENDER.

    pub fn render(&mut self) -> server::Result<Markup> {
        // Acciones del tema antes de renderizar la página.
        THEME.before_render_page(self);

        // Primero, renderizar el cuerpo.
        let body = THEME.render_page_body(self);

        // Luego, renderizar la cabecera.
        let head = THEME.render_page_head(self);

        // Finalmente, renderizar la página.
        return Ok(html! {
            (DOCTYPE)
            html lang=(self.language) dir=(self.direction) {
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
}

pub fn render_component(
    component: &dyn PageComponent,
    assets: &mut PageAssets
) -> Markup {
    match component.is_renderable() {
        true => match THEME.render_component(component, assets) {
            Some(markup) => markup,
            None => component.default_render(assets)
        },
        false => html! {}
    }
}
