use crate::{Lazy, app, trace};
use crate::config::SETTINGS;
use crate::html::{Classes, DOCTYPE, Markup, OptAttr, html};
use crate::response::page::{PageAssets, PageComponent, PageContainer};

use std::sync::RwLock;
use std::collections::HashMap;

static COMPONENTS: Lazy<RwLock<HashMap<&str, PageContainer>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

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
    assets      : PageAssets,
    body_classes: Classes,
    regions     : HashMap<&'a str, PageContainer>,
    template    : String,
}

impl<'a> Page<'a> {

    pub fn new() -> Self {
        Page {
            language    : match &*DEFAULT_LANGUAGE {
                Some(language) => OptAttr::some(language),
                _ => OptAttr::none(),
            },
            direction   : match &*DEFAULT_DIRECTION {
                Some(direction) => OptAttr::some(direction),
                _ => OptAttr::none(),
            },
            title       : OptAttr::none(),
            description : OptAttr::none(),
            body_classes: Classes::some_class("body"),
            assets      : PageAssets::new(),
            regions     : COMPONENTS.read().unwrap().clone(),
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

    pub fn add_body_class(&mut self, class: &str) -> &mut Self {
        self.body_classes.add_class(class);
        self
    }

    pub fn add_body_classes(&mut self, classes: Vec<String>) -> &mut Self {
        self.body_classes.add_classes(classes);
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
        self.template = template.to_owned();
        self
    }

    // Page GETTERS.

    pub fn language(&self) -> &str {
        self.language.value()
    }

    pub fn direction(&self) -> &str {
        self.direction.value()
    }

    pub fn title(&self) -> &str {
        self.title.value()
    }

    pub fn description(&self) -> &str {
        self.description.value()
    }

    pub fn body_classes(&mut self) -> &str {
        self.body_classes.classes()
    }

    pub fn assets(&mut self) -> &mut PageAssets {
        &mut self.assets
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Page RENDER.

    pub fn render(&mut self) -> app::Result<Markup> {
        // Acciones del tema antes de renderizar la página.
        self.assets.theme().before_render_page(self);

        // Primero, renderizar el cuerpo.
        let body = self.assets.theme().render_page_body(self);

        // Luego, renderizar la cabecera.
        let head = self.assets.theme().render_page_head(self);

        // Finalmente, renderizar la página.
        return Ok(html! {
            (DOCTYPE)
            html lang=[&self.language.option()] dir=[&self.direction.option()] {
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

    pub fn using_theme(&mut self, theme_name: &str) -> &mut Self {
        self.assets.using_theme(theme_name);
        self
    }
}

pub fn render_component(
    component: &dyn PageComponent,
    assets: &mut PageAssets
) -> Markup {
    match component.is_renderable() {
        true => match assets.theme().render_component(component, assets) {
            Some(html) => html,
            None => component.default_render(assets)
        },
        false => html! {}
    }
}

pub fn add_component_to(region: &'static str, component: impl PageComponent) {
    let mut hmap = COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, PageContainer::new_with(component));
    }
}
