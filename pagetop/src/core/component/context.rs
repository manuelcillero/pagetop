use crate::core::theme::all::{theme_by_single_name, THEME};
use crate::core::theme::ThemeStaticRef;
use crate::html::{html, Assets, JavaScript, Markup, StyleSheet};
use crate::locale::{LanguageIdentifier, LANGID};
use crate::service::HttpRequest;
use crate::{concat_string, util};

use std::collections::HashMap;
use std::str::FromStr;

pub enum ContextOp {
    LangId(&'static LanguageIdentifier),
    Theme(&'static str),
    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),
    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
}

#[rustfmt::skip]
pub struct Context {
    request    : HttpRequest,
    langid     : &'static LanguageIdentifier,
    theme      : ThemeStaticRef,
    stylesheets: Assets<StyleSheet>,
    javascripts: Assets<JavaScript>,
    params     : HashMap<&'static str, String>,
    id_counter : usize,
}

impl Context {
    #[rustfmt::skip]
    pub(crate) fn new(request: HttpRequest) -> Self {
        Context {
            request,
            langid     : &LANGID,
            theme      : *THEME,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            params     : HashMap::<&str, String>::new(),
            id_counter : 0,
        }
    }

    pub fn alter(&mut self, op: ContextOp) -> &mut Self {
        match op {
            ContextOp::LangId(langid) => {
                self.langid = langid;
            }
            ContextOp::Theme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*THEME);
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

    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    pub(crate) fn langid(&self) -> &LanguageIdentifier {
        self.langid
    }

    pub(crate) fn theme(&self) -> ThemeStaticRef {
        self.theme
    }

    pub fn get_param<T: FromStr + ToString>(&mut self, key: &'static str) -> Option<T> {
        if let Some(value) = self.params.get(key) {
            if let Ok(value) = T::from_str(value) {
                return Some(value);
            }
        }
        None
    }

    /// Context PREPARE.

    pub fn prepare(&mut self) -> Markup {
        html! {
            (self.stylesheets.prepare())
            (self.javascripts.prepare())
        }
    }

    // Context EXTRAS.

    pub fn required_id<T>(&mut self, id: Option<String>) -> String {
        match id {
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
