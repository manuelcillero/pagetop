use crate::core::component::ChildOp;
use crate::core::theme::all::{theme_by_short_name, DEFAULT_THEME};
use crate::core::theme::{ChildrenInRegions, ThemeRef};
use crate::html::{html, Markup};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::join_string;
use crate::locale::{LanguageIdentifier, DEFAULT_LANGID};
use crate::service::HttpRequest;
use crate::util::TypeInfo;

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
}

#[derive(Debug)]
pub enum ErrorParam {
    NotFound,
    ParseError(String),
}

impl fmt::Display for ErrorParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorParam::NotFound => write!(f, "Parameter not found"),
            ErrorParam::ParseError(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl Error for ErrorParam {}

#[rustfmt::skip]
pub struct Context {
    request   : HttpRequest,
    langid    : &'static LanguageIdentifier,
    theme     : ThemeRef,
    layout    : &'static str,
    favicon   : Option<Favicon>,
    stylesheet: Assets<StyleSheet>,
    javascript: Assets<JavaScript>,
    regions   : ChildrenInRegions,
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
            regions   : ChildrenInRegions::default(),
            params    : HashMap::<&str, String>::new(),
            id_counter: 0,
        }
    }

    pub fn alter_assets(&mut self, op: AssetsOp) -> &mut Self {
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
        }
        self
    }

    pub fn alter_in_region(&mut self, region_id: &'static str, op: ChildOp) -> &mut Self {
        self.regions.alter_in_region(region_id, op);
        self
    }

    pub fn alter_param<T: FromStr + ToString>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> &mut Self {
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

    pub fn regions(&self) -> &ChildrenInRegions {
        &self.regions
    }

    pub fn get_param<T: FromStr + ToString>(&self, key: &'static str) -> Result<T, ErrorParam> {
        self.params
            .get(key)
            .ok_or(ErrorParam::NotFound)
            .and_then(|v| T::from_str(v).map_err(|_| ErrorParam::ParseError(v.clone())))
    }

    // Context RENDER.

    pub fn render_assets(&mut self) -> Markup {
        html! {
            @if let Some(favicon) = &self.favicon {
                (favicon.render())
            }
            (self.stylesheet.render())
            (self.javascript.render())
        }
    }

    pub fn render_region(&mut self, region_id: impl Into<String>) -> Markup {
        self.regions
            .all_in_region(self.theme, &region_id.into())
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
            join_string!(prefix, "-", self.id_counter.to_string())
        }
    }
}
