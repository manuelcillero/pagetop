use crate::{Lazy, base};
use crate::config::SETTINGS;
use crate::core::all;
use crate::core::theme::{Markup, PreEscaped, Theme, find_theme, html};

pub static DEFAULT_THEME: Lazy<&dyn Theme> = Lazy::new(|| {
    for t in all::THEMES.read().unwrap().iter() {
        if t.name().to_lowercase() == SETTINGS.app.theme.to_lowercase() {
            return *t;
        }
    }
    &base::theme::bootsier::BootsierTheme
});

// -----------------------------------------------------------------------------
// Favicon.
// -----------------------------------------------------------------------------

pub struct Favicon(Vec<String>);

impl Favicon {
    pub fn new() -> Self {
        Favicon(Vec::new())
    }

    pub fn with_icon(self, image: &str) -> Self {
        self.add_item("icon", image, "", "")
    }

    pub fn with_icon_for_sizes(self, image: &str, sizes: &str) -> Self {
        self.add_item("icon", image, sizes, "")
    }

    pub fn with_apple_touch_icon(self, image: &str, sizes: &str) -> Self {
        self.add_item("apple-touch-icon", image, sizes, "")
    }

    pub fn with_mask_icon(self, image: &str, color: &str) -> Self {
        self.add_item("mask-icon", image, "", color)
    }

    pub fn with_manifest(self, file: &str) -> Self {
        self.add_item("manifest", file, "", "")
    }

    pub fn with_theme_color(mut self, color: &str) -> Self {
        self.0.push(format!(
            "<meta name=\"theme-color\" content=\"{}\">", color
        ));
        self
    }

    pub fn with_ms_tile_color(mut self, color: &str) -> Self {
        self.0.push(format!(
            "<meta name=\"msapplication-TileColor\" content=\"{}\">", color
        ));
        self
    }

    pub fn with_ms_tile_image(mut self, image: &str) -> Self {
        self.0.push(format!(
            "<meta name=\"msapplication-TileImage\" content=\"{}\">", image
        ));
        self
    }

    fn add_item(
        mut self,
        rel   : &str,
        source: &str,
        sizes : &str,
        color : &str
    ) -> Self {
        let mut link: String = format!("<link rel=\"{}\"", rel);
        if let Some(i) = source.rfind('.') {
            link = match source[i..].to_string().to_lowercase().as_str() {
                ".gif" => format!("{} type=\"image/gif\"", link),
                ".ico" => format!("{} type=\"image/x-icon\"", link),
                ".jpg" => format!("{} type=\"image/jpg\"", link),
                ".png" => format!("{} type=\"image/png\"", link),
                ".svg" => format!("{} type=\"image/svg+xml\"", link),
                _ => link
            };
        }
        if !sizes.is_empty() {
            link = format!("{} sizes=\"{}\"", link, sizes);
        }
        if !color.is_empty() {
            link = format!("{} color=\"{}\"", link, color);
        }
        self.0.push(format!("{} href=\"{}\">", link, source));
        self
    }

    fn render(&self) -> Markup {
        html! {
            @for item in &self.0 {
                (PreEscaped(item))
            }
        }
    }
}

// -----------------------------------------------------------------------------
// StyleSheet.
// -----------------------------------------------------------------------------

pub struct StyleSheet {
    source: &'static str,
    weight: i8,
}
impl StyleSheet {
    pub fn source(s: &'static str) -> Self {
        StyleSheet {
            source: s,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn weight(self) -> i8 {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" href=(self.source);
        }
    }
}

// -----------------------------------------------------------------------------
// JavaScript.
// -----------------------------------------------------------------------------

#[derive(PartialEq)]
pub enum JSMode { Async, Defer, Normal }

pub struct JavaScript {
    source: &'static str,
    weight: i8,
    mode  : JSMode,
}
impl JavaScript {
    pub fn source(s: &'static str) -> Self {
        JavaScript {
            source: s,
            weight: 0,
            mode  : JSMode::Defer,
        }
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_mode(mut self, mode: JSMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn weight(self) -> i8 {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            script type="text/javascript"
                src=(self.source)
                async[self.mode == JSMode::Async]
                defer[self.mode == JSMode::Defer]
                {};
        }
    }
}

// -----------------------------------------------------------------------------
// Page assets.
// -----------------------------------------------------------------------------

pub struct Assets {
    theme      : &'static dyn Theme,
    favicon    : Option<Favicon>,
    metadata   : Vec<(String, String)>,
    stylesheets: Vec<StyleSheet>,
    javascripts: Vec<JavaScript>,
    with_jquery: bool,
    id_counter : u32,
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            theme      : *DEFAULT_THEME,
            favicon    : None,
            metadata   : Vec::new(),
            stylesheets: Vec::new(),
            javascripts: Vec::new(),
            with_jquery: false,
            id_counter : 0,
        }
    }

    pub fn using_theme(&mut self, theme_id: &str) -> &mut Self {
        self.theme = find_theme(theme_id).unwrap_or(*DEFAULT_THEME);
        self
    }

    pub fn with_favicon(&mut self, favicon: Favicon) -> &mut Self {
        self.favicon = Some(favicon);
        self
    }

    pub fn add_metadata(&mut self, name: String, content: String) -> &mut Self {
        self.metadata.push((name, content));
        self
    }

    pub fn add_stylesheet(&mut self, css: StyleSheet) -> &mut Self {
        match self.stylesheets.iter().position(|x| x.source == css.source) {
            Some(index) => if self.stylesheets[index].weight > css.weight {
                self.stylesheets.remove(index);
                self.stylesheets.push(css);
            },
            _ => self.stylesheets.push(css)
        }
        self
    }

    pub fn add_javascript(&mut self, js: JavaScript) -> &mut Self {
        match self.javascripts.iter().position(|x| x.source == js.source) {
            Some(index) => if self.javascripts[index].weight > js.weight {
                self.javascripts.remove(index);
                self.javascripts.push(js);
            },
            _ => self.javascripts.push(js)
        }
        self
    }

    pub fn add_jquery(&mut self) -> &mut Self {
        if !self.with_jquery {
            self.add_javascript(
                JavaScript::source(
                    "/theme/js/jquery.min.js?ver=3.6.0"
                )
                .with_weight(i8::MIN)
                .with_mode(JSMode::Normal)
            );
            self.with_jquery = true;
        }
        self
    }

    /// Assets GETTERS.

    pub fn theme(&mut self) -> &'static dyn Theme {
        self.theme
    }

    /// Assets RENDER.

    pub fn render(&mut self) -> Markup {
        let ordered_css = &mut self.stylesheets;
        ordered_css.sort_by_key(|o| o.weight);

        let ordered_js = &mut self.javascripts;
        ordered_js.sort_by_key(|o| o.weight);

        html! {
            @match &self.favicon {
                Some(favicon) => (favicon.render()),
                None => "",
            }
            @for (name, content) in &self.metadata {
                meta name=(name) content=(content) {}
            }
            @for css in ordered_css {
                (css.render())
            }
            @for js in ordered_js {
                (js.render())
            }
        }
    }

    // Assets EXTRAS.

    pub fn required_id(&mut self, prefix: &str, id: &str) -> String {
        if id.is_empty() {
            let prefix = prefix.trim().replace(" ", "_").to_lowercase();
            let prefix = if prefix.is_empty() {
                "prefix".to_string()
            } else {
                prefix
            };
            self.id_counter += 1;
            [prefix, self.id_counter.to_string()].join("-")
        } else {
            id.to_string()
        }
    }
}
