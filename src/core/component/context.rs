use crate::base::component::add_base_assets;
use crate::concat_string;
use crate::core::component::AnyOp;
use crate::core::theme::all::{theme_by_short_name, DEFAULT_THEME};
use crate::core::theme::{ComponentsInRegions, ThemeRef};
use crate::global::TypeInfo;
use crate::html::{html, Markup};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::locale::{LanguageIdentifier, DEFAULT_LANGID};
use crate::service::HttpRequest;

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use std::fmt;

pub enum AssetsOp {
    LangId(&'static LanguageIdentifier),
    Theme(&'static str),
    Layout(&'static str),
    // Favicon.
    SetFavicon(Option<Favicon>),
    SetFaviconIfNone(Favicon),
    // Stylesheets.
    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),
    // JavaScripts.
    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
    // Add assets to properly use base components.
    AddBaseAssets,
}

#[derive(Debug)]
pub enum ParamError {
    NotFound,
    ParseError(String),
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
    favicon   : Option<Favicon>,
    stylesheet: Assets<StyleSheet>,
    javascript: Assets<JavaScript>,
    regions   : ComponentsInRegions,
    params    : HashMap<&'static str, String>,
    id_counter: usize,
}

impl Context {
    #[rustfmt::skip]
    pub(crate) fn new(request: HttpRequest) -> Self {
        Context {
            request,
            langid    : &DEFAULT_LANGID,
            theme     : *DEFAULT_THEME,
            layout    : "default",
            favicon   : None,
            stylesheet: Assets::<StyleSheet>::new(),
            javascript: Assets::<JavaScript>::new(),
            regions   : ComponentsInRegions::default(),
            params    : HashMap::<&str, String>::new(),
            id_counter: 0,
        }
    }

    pub fn set_assets(&mut self, op: AssetsOp) -> &mut Self {
        match op {
            AssetsOp::LangId(langid) => {
                self.langid = langid;
            }
            AssetsOp::Theme(theme_name) => {
                self.theme = theme_by_short_name(theme_name).unwrap_or(*DEFAULT_THEME);
            }
            AssetsOp::Layout(layout) => {
                self.layout = layout;
            }
            // Favicon.
            AssetsOp::SetFavicon(favicon) => {
                self.favicon = favicon;
            }
            AssetsOp::SetFaviconIfNone(icon) => {
                if self.favicon.is_none() {
                    self.favicon = Some(icon);
                }
            }
            // Stylesheets.
            AssetsOp::AddStyleSheet(css) => {
                self.stylesheet.add(css);
            }
            AssetsOp::RemoveStyleSheet(path) => {
                self.stylesheet.remove(path);
            }
            // JavaScripts.
            AssetsOp::AddJavaScript(js) => {
                self.javascript.add(js);
            }
            AssetsOp::RemoveJavaScript(path) => {
                self.javascript.remove(path);
            }
            // Add assets to properly use base components.
            AssetsOp::AddBaseAssets => {
                add_base_assets(self);
            }
        }
        self
    }

    pub fn set_regions(&mut self, region: &'static str, op: AnyOp) -> &mut Self {
        self.regions.set_components(region, op);
        self
    }

    pub fn set_param<T: FromStr + ToString>(&mut self, key: &'static str, value: &T) -> &mut Self {
        self.params.insert(key, value.to_string());
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

    pub fn get_param<T: FromStr + ToString>(&self, key: &'static str) -> Result<T, ParamError> {
        self.params
            .get(key)
            .ok_or(ParamError::NotFound)
            .and_then(|v| T::from_str(v).map_err(|_| ParamError::ParseError(v.clone())))
    }

    // Context PREPARE.

    pub(crate) fn prepare_assets(&mut self) -> Markup {
        html! {
            @if let Some(favicon) = &self.favicon {
                (favicon.prepare())
            }
            (self.stylesheet.prepare())
            (self.javascript.prepare())
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
