use crate::{Lazy, base, concat_string, util};
use crate::config::SETTINGS;
use crate::html::*;
use crate::core::theme::ThemeTrait;
use crate::core::theme::all::theme_by_single_name;

static DEFAULT_THEME: Lazy<&dyn ThemeTrait> = Lazy::new(|| {
    match theme_by_single_name(&SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &base::theme::bootsier::Bootsier,
    }
});

pub enum InContextOp {
    SetTheme(&'static str),
    AddMetadata(&'static str, &'static str),
    Favicon(Option<Favicon>),
    StyleSheet(AssetsOp<StyleSheet>),
    JavaScript(AssetsOp<JavaScript>),
    AddJQuery,
}

pub struct InContext {
    theme      : &'static dyn ThemeTrait,
    favicon    : Option<Favicon>,
    metadata   : Vec<(String, String)>,
    stylesheets: Assets<StyleSheet>,
    javascripts: Assets<JavaScript>,
    with_jquery: bool,
    id_counter : usize,
}

impl InContext {
    pub fn new() -> Self {
        InContext {
            theme      : *DEFAULT_THEME,
            favicon    : None,
            metadata   : Vec::new(),
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            with_jquery: false,
            id_counter : 0,
        }
    }

    pub fn alter(&mut self, op: InContextOp) -> &mut Self {
        match op {
            InContextOp::SetTheme(theme_name) => {
                self.theme = theme_by_single_name(theme_name).unwrap_or(*DEFAULT_THEME);
            },
            InContextOp::AddMetadata(name, content) => {
                self.metadata.push((name.to_owned(), content.to_owned()));
            },
            InContextOp::Favicon(favicon) => {
                self.favicon = favicon;
            },
            InContextOp::StyleSheet(css) => {
                self.stylesheets.alter(css);
            },
            InContextOp::JavaScript(js) => {
                self.javascripts.alter(js);
            },
            InContextOp::AddJQuery => if !self.with_jquery {
                self.javascripts.alter(AssetsOp::Add(
                    JavaScript::located("/theme/js/jquery.min.js")
                        .with_version("3.6.0")
                        .with_weight(isize::MIN)
                        .with_mode(JSMode::Normal)
                ));
                self.with_jquery = true;
            },
        }
        self
    }

    /// InContext GETTERS.

    pub(crate) fn theme(&mut self) -> &'static dyn ThemeTrait {
        self.theme
    }

    /// InContext RENDER.

    pub fn render(&mut self) -> Markup {
        html! {
            @match &self.favicon {
                Some(favicon) => (favicon.render()),
                None => "",
            }
            @for (name, content) in &self.metadata {
                meta name=(name) content=(content) {}
            }
            (self.stylesheets.render())
            (self.javascripts.render())
        }
    }

    // InContext EXTRAS.

    pub fn required_id<T>(&mut self, id: &IdentifierValue) -> String {
        match id.get() {
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
