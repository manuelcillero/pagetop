use crate::core::theme::{all::theme_by_single_name, ThemeStaticRef};
use crate::html::{html, Assets, IdentifierValue, JavaScript, Markup, ModeJS, StyleSheet};
use crate::{base, concat_string, config, util, LazyStatic};

static DEFAULT_THEME: LazyStatic<ThemeStaticRef> =
    LazyStatic::new(|| match theme_by_single_name(&config::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &base::theme::bootsier::Bootsier,
    });

pub enum ContextOp {
    SetTheme(&'static str),
    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),
    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
    AddJQuery,
}

#[rustfmt::skip]
pub struct RenderContext {
    theme      : ThemeStaticRef,
    stylesheets: Assets<StyleSheet>,
    javascripts: Assets<JavaScript>,
    with_jquery: bool,
    id_counter : usize,
}

impl Default for RenderContext {
    #[rustfmt::skip]
    fn default() -> Self {
        RenderContext {
            theme      : *DEFAULT_THEME,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            with_jquery: false,
            id_counter : 0,
        }
    }
}

impl RenderContext {
    pub fn new() -> Self {
        RenderContext::default()
    }

    pub fn alter(&mut self, op: ContextOp) -> &mut Self {
        match op {
            ContextOp::SetTheme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*DEFAULT_THEME);
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
            ContextOp::AddJQuery => {
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

    /// Context GETTERS.

    pub(crate) fn theme(&mut self) -> ThemeStaticRef {
        self.theme
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
