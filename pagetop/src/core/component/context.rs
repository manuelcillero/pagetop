use crate::core::module::{all::theme_by_single_name, ThemeStaticRef};
use crate::html::{html, Assets, IdentifierValue, JavaScript, Markup, StyleSheet};
use crate::locale::{LanguageIdentifier, DEFAULT_LANGID};
use crate::server::HttpRequest;
use crate::{concat_string, config, util, LazyStatic};

use std::collections::HashMap;
use std::str::FromStr;

static DEFAULT_THEME: LazyStatic<ThemeStaticRef> =
    LazyStatic::new(|| match theme_by_single_name(&config::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &crate::core::basic::Basic,
    });

pub enum ContextOp {
    LangId(&'static LanguageIdentifier),
    Theme(&'static str),
    Request(Option<HttpRequest>),
    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),
    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
}

#[rustfmt::skip]
pub struct RenderContext {
    langid     : &'static LanguageIdentifier,
    theme      : ThemeStaticRef,
    request    : Option<HttpRequest>,
    stylesheets: Assets<StyleSheet>,
    javascripts: Assets<JavaScript>,
    params     : HashMap<&'static str, String>,
    id_counter : usize,
}

impl Default for RenderContext {
    #[rustfmt::skip]
    fn default() -> Self {
        RenderContext {
            langid     : &DEFAULT_LANGID,
            theme      : *DEFAULT_THEME,
            request    : None,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            params     : HashMap::<&str, String>::new(),
            id_counter : 0,
        }
    }
}

impl RenderContext {
    pub(crate) fn new() -> Self {
        RenderContext::default()
    }

    pub fn alter(&mut self, op: ContextOp) -> &mut Self {
        match op {
            ContextOp::LangId(langid) => {
                self.langid = langid;
            }
            ContextOp::Theme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*DEFAULT_THEME);
            }
            ContextOp::Request(request) => {
                self.request = request;
            }
            ContextOp::AddStyleSheet(css) => {
                self.stylesheets.add(css);
            }
            ContextOp::RemoveStyleSheet(source) => {
                self.stylesheets.remove(source);
            }
            ContextOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            ContextOp::RemoveJavaScript(source) => {
                self.javascripts.remove(source);
            }
        }
        self
    }

    pub fn set_param<T: FromStr + ToString>(&mut self, key: &'static str, value: T) -> &mut Self {
        self.params.insert(key, value.to_string());
        self
    }

    /// Context GETTERS.

    pub(crate) fn langid(&self) -> &LanguageIdentifier {
        self.langid
    }

    pub(crate) fn theme(&self) -> ThemeStaticRef {
        self.theme
    }

    pub fn request(&self) -> &Option<HttpRequest> {
        &self.request
    }

    pub fn get_param<T: FromStr + ToString>(&mut self, key: &'static str) -> Option<T> {
        if let Some(value) = self.params.get(key) {
            if let Ok(value) = T::from_str(value) {
                return Some(value);
            }
        }
        None
    }

    /// Context RENDER.

    pub fn render(&mut self) -> Markup {
        html! {
            (self.stylesheets.render())
            (self.javascripts.render())
        }
    }

    // Context EXTRAS.

    pub fn required_id<T>(&mut self, id: &IdentifierValue) -> String {
        match id.get() {
            Some(id) => id,
            None => {
                let prefix = util::single_type_name::<T>()
                    .trim()
                    .replace(' ', "_")
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
