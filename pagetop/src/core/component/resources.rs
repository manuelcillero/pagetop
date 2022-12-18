use crate::core::theme::{all::theme_by_single_name, ThemeStaticRef};
use crate::html::{html, Assets, IdentifierValue, JavaScript, Markup, ModeJS, StyleSheet};
use crate::{base, concat_string, config, util, LazyStatic};

static DEFAULT_THEME: LazyStatic<ThemeStaticRef> =
    LazyStatic::new(|| match theme_by_single_name(&config::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &base::theme::bootsier::Bootsier,
    });

pub enum ResourceOp {
    SetTheme(&'static str),
    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),
    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
    AddJQuery,
}

#[rustfmt::skip]
pub struct RenderResources {
    theme      : ThemeStaticRef,
    stylesheets: Assets<StyleSheet>,
    javascripts: Assets<JavaScript>,
    with_jquery: bool,
    id_counter : usize,
}

impl Default for RenderResources {
    #[rustfmt::skip]
    fn default() -> Self {
        RenderResources {
            theme      : *DEFAULT_THEME,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            with_jquery: false,
            id_counter : 0,
        }
    }
}

impl RenderResources {
    pub fn new() -> Self {
        RenderResources::default()
    }

    pub fn alter(&mut self, op: ResourceOp) -> &mut Self {
        match op {
            ResourceOp::SetTheme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*DEFAULT_THEME);
            }
            ResourceOp::AddStyleSheet(css) => {
                self.stylesheets.add(css);
            }
            ResourceOp::RemoveStyleSheet(source) => {
                self.stylesheets.remove(source);
            }
            ResourceOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            ResourceOp::RemoveJavaScript(source) => {
                self.javascripts.remove(source);
            }
            ResourceOp::AddJQuery => {
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

    /// Resources GETTERS.

    pub(crate) fn theme(&mut self) -> ThemeStaticRef {
        self.theme
    }

    /// Resources RENDER.

    pub fn render(&mut self) -> Markup {
        html! {
            (self.stylesheets.render())
            (self.javascripts.render())
        }
    }

    // Resources EXTRAS.

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
