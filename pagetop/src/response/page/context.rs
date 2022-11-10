use super::PageOp;

use crate::core::theme::{all::theme_by_single_name, ThemeStaticRef};
use crate::html::{html, Assets, Favicon, IdentifierValue, JavaScript, Markup, ModeJS, StyleSheet};
use crate::{base, concat_string, config, util, LazyStatic};

static DEFAULT_THEME: LazyStatic<ThemeStaticRef> =
    LazyStatic::new(|| match theme_by_single_name(&config::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &base::theme::bootsier::Bootsier,
    });

#[rustfmt::skip]
pub struct PageContext {
    theme      : ThemeStaticRef,
    favicon    : Option<Favicon>,
    metadata   : Vec<(&'static str, &'static str)>,
    properties : Vec<(&'static str, &'static str)>,
    stylesheets: Assets<StyleSheet>,
    javascripts: Assets<JavaScript>,
    with_jquery: bool,
    id_counter : usize,
}

impl Default for PageContext {
    #[rustfmt::skip]
    fn default() -> Self {
        PageContext {
            theme      : *DEFAULT_THEME,
            favicon    : None,
            metadata   : Vec::new(),
            properties : Vec::new(),
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            with_jquery: false,
            id_counter : 0,
        }
    }
}

impl PageContext {
    pub fn new() -> Self {
        PageContext::default()
    }

    pub fn alter(&mut self, op: PageOp) -> &mut Self {
        match op {
            PageOp::SetTheme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*DEFAULT_THEME);
            }

            PageOp::AddFavicon(favicon) => {
                self.favicon = Some(favicon);
            }
            PageOp::RemoveFavicon => {
                self.favicon = None;
            }

            PageOp::AddMetadata(name, content) => {
                self.metadata.push((name, content));
            }
            PageOp::AddProperty(property, content) => {
                self.properties.push((property, content));
            }

            PageOp::AddStyleSheet(css) => {
                self.stylesheets.add(css);
            }
            PageOp::RemoveStyleSheet(source) => {
                self.stylesheets.remove(source);
            }

            PageOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            PageOp::RemoveJavaScript(source) => {
                self.javascripts.remove(source);
            }
            PageOp::AddJQuery => {
                if !self.with_jquery {
                    self.javascripts.add(
                        JavaScript::located("/theme/js/jquery.min.js")
                            .with_version("3.6.0")
                            .with_weight(isize::MIN)
                            .with_mode(ModeJS::Normal),
                    );
                    self.with_jquery = true;
                }
            }
        }
        self
    }

    /// PageContext GETTERS.

    pub(crate) fn theme(&mut self) -> ThemeStaticRef {
        self.theme
    }

    /// PageContext RENDER.

    pub fn render(&mut self) -> Markup {
        html! {
            @match &self.favicon {
                Some(favicon) => (favicon.render()),
                None => "",
            }
            @for (name, content) in &self.metadata {
                meta name=(name) content=(content) {}
            }
            @for (property, content) in &self.properties {
                meta property=(property) content=(content) {}
            }
            (self.stylesheets.render())
            (self.javascripts.render())
        }
    }

    // PageContext EXTRAS.

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
