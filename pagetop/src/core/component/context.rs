use crate::config::SETTINGS;
use crate::core::theme::all::theme_by_single_name;
use crate::core::theme::ThemeTrait;
use crate::html::*;
use crate::{base, concat_string, util, LazyStatic};

static DEFAULT_THEME: LazyStatic<&dyn ThemeTrait> = LazyStatic::new(||
    match theme_by_single_name(&SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &base::theme::bootsier::Bootsier,
    }
);

pub enum InContextOp {
    SetTheme(&'static str),

    AddFavicon(Favicon),
    RemoveFavicon,

    AddMetadata(&'static str, &'static str),
    AddProperty(&'static str, &'static str),

    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),

    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
    AddJQuery,
}

pub struct InContext {
    theme      : &'static dyn ThemeTrait,
    favicon    : Option<Favicon>,
    metadata   : Vec<(&'static str, &'static str)>,
    properties : Vec<(&'static str, &'static str)>,
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
            properties : Vec::new(),
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
            }

            InContextOp::AddFavicon(favicon) => {
                self.favicon = Some(favicon);
            }
            InContextOp::RemoveFavicon => {
                self.favicon = None;
            }

            InContextOp::AddMetadata(name, content) => {
                self.metadata.push((name, content));
            }
            InContextOp::AddProperty(property, content) => {
                self.properties.push((property, content));
            }

            InContextOp::AddStyleSheet(css) => {
                self.stylesheets.add(css);
            }
            InContextOp::RemoveStyleSheet(source) => {
                self.stylesheets.remove(source);
            }

            InContextOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            InContextOp::RemoveJavaScript(source) => {
                self.javascripts.remove(source);
            }
            InContextOp::AddJQuery => {
                if !self.with_jquery {
                    self.javascripts.add(
                        JavaScript::located("/theme/js/jquery.min.js")
                            .with_version("3.6.0")
                            .with_weight(isize::MIN)
                            .with_mode(JSMode::Normal),
                    );
                    self.with_jquery = true;
                }
            }
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
            @for (property, content) in &self.properties {
                meta property=(property) content=(content) {}
            }
            (self.stylesheets.render())
            (self.javascripts.render())
        }
    }

    // InContext EXTRAS.

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
