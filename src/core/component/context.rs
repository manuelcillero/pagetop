use crate::base::component::add_base_assets;
use crate::core::component::AnyOp;
use crate::core::theme::all::{theme_by_short_name, THEME_DEFAULT};
use crate::core::theme::{ComponentsInRegions, ThemeRef};
use crate::html::{html, Markup};
use crate::html::{Assets, HeadScript, HeadStyles, JavaScript, StyleSheet};
use crate::locale::{LanguageIdentifier, LANGID_DEFAULT};
use crate::service::HttpRequest;
use crate::util::TypeInfo;
use crate::{concat_string, trace};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json as json;

use std::collections::HashMap;
use std::error::Error;

use std::fmt;

pub enum AssetsOp {
    LangId(&'static LanguageIdentifier),
    Theme(&'static str),
    Layout(&'static str),
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
    // Add assets to properly use base components.
    AddBaseAssets,
}

#[derive(Debug)]
pub enum ParamError {
    NotFound,
    ParseError(json::Error),
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamError::NotFound => write!(f, "Parameter not found"),
            ParamError::ParseError(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl Error for ParamError {}

#[rustfmt::skip]
pub struct Context {
    request   : HttpRequest,
    langid    : &'static LanguageIdentifier,
    theme     : ThemeRef,
    layout    : &'static str,
    stylesheet: Assets<StyleSheet>,                     // Stylesheets.
    headstyles: Assets<HeadStyles>,                     // Styles in head.
    javascript: Assets<JavaScript>,                     // JavaScripts.
    headscript: Assets<HeadScript>,                     // Scripts in head.
    regions   : ComponentsInRegions,
    params    : HashMap<&'static str, json::Value>,
    id_counter: usize,
}

impl Context {
    #[rustfmt::skip]
    pub(crate) fn new(request: HttpRequest) -> Self {
        Context {
            request,
            langid    : &LANGID_DEFAULT,
            theme     : *THEME_DEFAULT,
            layout    : "default",
            stylesheet: Assets::<StyleSheet>::new(),    // Stylesheets.
            headstyles: Assets::<HeadStyles>::new(),    // Styles in head.
            javascript: Assets::<JavaScript>::new(),    // JavaScripts.
            headscript: Assets::<HeadScript>::new(),    // Scripts in head.
            regions   : ComponentsInRegions::default(),
            params    : HashMap::<&str, json::Value>::new(),
            id_counter: 0,
        }
    }

    #[rustfmt::skip]
    pub fn set_assets(&mut self, op: AssetsOp) -> &mut Self {
        match op {
            AssetsOp::LangId(langid) => {
                self.langid = langid;
            }
            AssetsOp::Theme(theme_name) => {
                self.theme = theme_by_short_name(theme_name).unwrap_or(*THEME_DEFAULT);
            }
            AssetsOp::Layout(layout) => {
                self.layout = layout;
            }

            // Stylesheets.
            AssetsOp::AddStyleSheet(css)     => { self.stylesheet.add(css);     }
            AssetsOp::RemoveStyleSheet(path) => { self.stylesheet.remove(path); }
            // Styles in head.
            AssetsOp::AddHeadStyles(styles)  => { self.headstyles.add(styles);  }
            AssetsOp::RemoveHeadStyles(path) => { self.headstyles.remove(path); }
            // JavaScripts.
            AssetsOp::AddJavaScript(js)      => { self.javascript.add(js);      }
            AssetsOp::RemoveJavaScript(path) => { self.javascript.remove(path); }
            // Scripts in head.
            AssetsOp::AddHeadScript(script)  => { self.headscript.add(script);  }
            AssetsOp::RemoveHeadScript(path) => { self.headscript.remove(path); }

            // Add assets to properly use base components.
            AssetsOp::AddBaseAssets => { add_base_assets(self); }
        }
        self
    }

    pub fn set_regions(&mut self, region: &'static str, op: AnyOp) -> &mut Self {
        self.regions.set_components(region, op);
        self
    }

    pub fn set_param<T: Serialize>(&mut self, key: &'static str, value: &T) -> &mut Self {
        json::to_value(value).map_or_else(
            |e| trace::error!("Serialization failed for param {key}: {e}"),
            |v| {
                self.params.insert(key, v);
            },
        );
        self
    }

    // Context GETTERS.

    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    pub fn langid(&self) -> &LanguageIdentifier {
        self.langid
    }

    pub fn theme(&self) -> ThemeRef {
        self.theme
    }

    pub fn layout(&self) -> &str {
        self.layout
    }

    pub fn regions(&self) -> &ComponentsInRegions {
        &self.regions
    }

    pub fn get_param<T: DeserializeOwned>(&self, key: &'static str) -> Result<T, ParamError> {
        self.params
            .get(key)
            .ok_or(ParamError::NotFound)
            .and_then(|v| json::from_value(v.clone()).map_err(ParamError::ParseError))
    }

    // Context PREPARE.

    pub(crate) fn prepare_assets(&mut self) -> Markup {
        html! {
            (self.stylesheet.prepare())                 // Stylesheets.
            (self.headstyles.prepare())                 // Styles in head.
            (self.javascript.prepare())                 // JavaScripts.
            (self.headscript.prepare())                 // Scripts in head.
        }
    }

    pub(crate) fn prepare_region(&mut self, region: impl Into<String>) -> Markup {
        self.regions
            .all_components(self.theme, region.into().as_str())
            .render(self)
    }

    // Context EXTRAS.

    pub fn remove_param(&mut self, key: &'static str) -> bool {
        self.params.remove(key).is_some()
    }

    pub fn required_id<T>(&mut self, id: Option<String>) -> String {
        if let Some(id) = id {
            id
        } else {
            let prefix = TypeInfo::ShortName
                .of::<T>()
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
