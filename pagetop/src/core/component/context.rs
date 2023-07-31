use crate::core::theme::all::{theme_by_single_name, THEME};
use crate::core::theme::ThemeRef;
use crate::html::{html, Assets, HeadScript, HeadStyles, JavaScript, Markup, StyleSheet};
use crate::locale::{LanguageIdentifier, LANGID};
use crate::service::HttpRequest;
use crate::{concat_string, util};

use std::collections::HashMap;
use std::str::FromStr;

pub enum ContextOp {
    LangId(&'static LanguageIdentifier),
    Theme(&'static str),
    // Stylesheets.
    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),
    // Styles in head.
    AddHeadStyles(HeadStyles),
    RemoveHeadStyles(&'static str),
    // JavaScripts.
    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
    // Scripts in head.
    AddHeadScript(HeadScript),
    RemoveHeadScript(&'static str),
}

#[rustfmt::skip]
pub struct Context {
    request   : HttpRequest,
    langid    : &'static LanguageIdentifier,
    theme     : ThemeRef,
    stylesheet: Assets<StyleSheet>,                     // Stylesheets.
    headstyles: Assets<HeadStyles>,                     // Styles in head.
    javascript: Assets<JavaScript>,                     // JavaScripts.
    headscript: Assets<HeadScript>,                     // Scripts in head.
    params    : HashMap<&'static str, String>,
    id_counter: usize,
}

impl Context {
    #[rustfmt::skip]
    pub(crate) fn new(request: HttpRequest) -> Self {
        Context {
            request,
            langid    : &LANGID,
            theme     : *THEME,
            stylesheet: Assets::<StyleSheet>::new(),    // Stylesheets.
            headstyles: Assets::<HeadStyles>::new(),    // Styles in head.
            javascript: Assets::<JavaScript>::new(),    // JavaScripts.
            headscript: Assets::<HeadScript>::new(),    // Scripts in head.
            params    : HashMap::<&str, String>::new(),
            id_counter: 0,
        }
    }

    #[rustfmt::skip]
    pub fn alter(&mut self, op: ContextOp) -> &mut Self {
        match op {
            ContextOp::LangId(langid) => {
                self.langid = langid;
            }
            ContextOp::Theme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*THEME);
            }
            // Stylesheets.
            ContextOp::AddStyleSheet(css)     => { self.stylesheet.add(css);     }
            ContextOp::RemoveStyleSheet(path) => { self.stylesheet.remove(path); }
            // Styles in head.
            ContextOp::AddHeadStyles(styles)  => { self.headstyles.add(styles);  }
            ContextOp::RemoveHeadStyles(path) => { self.headstyles.remove(path); }
            // JavaScripts.
            ContextOp::AddJavaScript(js)      => { self.javascript.add(js);      }
            ContextOp::RemoveJavaScript(path) => { self.javascript.remove(path); }
            // Scripts in head.
            ContextOp::AddHeadScript(script)  => { self.headscript.add(script);  }
            ContextOp::RemoveHeadScript(path) => { self.headscript.remove(path); }
        }
        self
    }

    pub fn set_param<T: FromStr + ToString>(&mut self, key: &'static str, value: T) -> &mut Self {
        self.params.insert(key, value.to_string());
        self
    }

    pub fn remove_param(&mut self, key: &'static str) -> &mut Self {
        self.params.remove(key);
        self
    }

    /// Context GETTERS.

    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    pub fn langid(&self) -> &LanguageIdentifier {
        self.langid
    }

    pub fn theme(&self) -> ThemeRef {
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
            (self.stylesheet.prepare())                 // Stylesheets.
            (self.headstyles.prepare())                 // Styles in head.
            (self.javascript.prepare())                 // JavaScripts.
            (self.headscript.prepare())                 // Scripts in head.
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
