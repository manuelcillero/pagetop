use crate::{Lazy, base, concat_string, util};
use crate::config::SETTINGS;
use crate::html::{Markup, html};
use crate::core::theme::*;

mod favicon;
pub use favicon::Favicon;

mod javascript;
pub use javascript::{JavaScript, JSMode};

mod stylesheet;
pub use stylesheet::StyleSheet;

static DEFAULT_THEME: Lazy<&dyn ThemeTrait> = Lazy::new(|| {
    match all::theme_by_single_name(&SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &base::theme::bootsier::Bootsier,
    }
});

pub struct Context {
    theme      : &'static dyn ThemeTrait,
    favicon    : Option<Favicon>,
    metadata   : Vec<(String, String)>,
    stylesheets: Vec<StyleSheet>,
    javascripts: Vec<JavaScript>,
    with_jquery: bool,
    id_counter : usize,
}

impl Context {
    pub fn new() -> Self {
        Context {
            theme      : *DEFAULT_THEME,
            favicon    : None,
            metadata   : Vec::new(),
            stylesheets: Vec::new(),
            javascripts: Vec::new(),
            with_jquery: false,
            id_counter : 0,
        }
    }

    pub fn using_theme(&mut self, theme_name: &str) -> &mut Self {
        self.theme = all::theme_by_single_name(theme_name).unwrap_or(*DEFAULT_THEME);
        self
    }

    pub fn with_favicon(&mut self, favicon: Favicon) -> &mut Self {
        self.favicon = Some(favicon);
        self
    }

    pub fn add_metadata(&mut self, name: String, content: String) -> &mut Self {
        self.metadata.push((name, content));
        self
    }

    pub fn add_stylesheet(&mut self, css: StyleSheet) -> &mut Self {
        match self.stylesheets.iter().position(|x| x.source == css.source) {
            Some(index) => if self.stylesheets[index].weight > css.weight {
                self.stylesheets.remove(index);
                self.stylesheets.push(css);
            },
            _ => self.stylesheets.push(css)
        }
        self
    }

    pub fn add_javascript(&mut self, js: JavaScript) -> &mut Self {
        match self.javascripts.iter().position(|x| x.source == js.source) {
            Some(index) => if self.javascripts[index].weight > js.weight {
                self.javascripts.remove(index);
                self.javascripts.push(js);
            },
            _ => self.javascripts.push(js)
        }
        self
    }

    pub fn add_jquery(&mut self) -> &mut Self {
        if !self.with_jquery {
            self.add_javascript(
                JavaScript::source(
                    "/theme/js/jquery.min.js?ver=3.6.0"
                )
                .with_weight(isize::MIN)
                .with_mode(JSMode::Normal)
            );
            self.with_jquery = true;
        }
        self
    }

    /// Context GETTERS.

    pub(crate) fn theme(&mut self) -> &'static dyn ThemeTrait {
        self.theme
    }

    /// Context RENDER.

    pub fn render(&mut self) -> Markup {
        let ordered_css = &mut self.stylesheets;
        ordered_css.sort_by_key(|o| o.weight);

        let ordered_js = &mut self.javascripts;
        ordered_js.sort_by_key(|o| o.weight);

        html! {
            @match &self.favicon {
                Some(favicon) => (favicon.render()),
                None => "",
            }
            @for (name, content) in &self.metadata {
                meta name=(name) content=(content) {}
            }
            @for css in ordered_css {
                (css.render())
            }
            @for js in ordered_js {
                (js.render())
            }
        }
    }

    // Context EXTRAS.

    pub fn required_id<T>(&mut self, id: &Option<String>) -> String {
        match id {
            Some(id) => id.to_string(),
            None => {
                let prefix = util::single_type_name::<T>()
                    .trim()
                    .replace(" ", "_")
                    .to_lowercase();
                let prefix = if prefix.is_empty() {
                    "prefix".to_owned()
                } else {
                    prefix
                };
                self.id_counter += 1;
                concat_string!(prefix, "-", self.id_counter.to_string())
            }
        }
    }
}
