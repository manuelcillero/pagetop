use crate::html::{html, Markup, PreEscaped};

#[derive(Default)]
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
        self.0
            .push(format!("<meta name=\"theme-color\" content=\"{}\">", color));
        self
    }

    pub fn with_ms_tile_color(mut self, color: &str) -> Self {
        self.0.push(format!(
            "<meta name=\"msapplication-TileColor\" content=\"{}\">",
            color
        ));
        self
    }

    pub fn with_ms_tile_image(mut self, image: &str) -> Self {
        self.0.push(format!(
            "<meta name=\"msapplication-TileImage\" content=\"{}\">",
            image
        ));
        self
    }

    fn add_item(mut self, rel: &str, source: &str, sizes: &str, color: &str) -> Self {
        let mut link: String = format!("<link rel=\"{}\"", rel);
        if let Some(i) = source.rfind('.') {
            link = match source[i..].to_owned().to_lowercase().as_str() {
                ".gif" => format!("{} type=\"image/gif\"", link),
                ".ico" => format!("{} type=\"image/x-icon\"", link),
                ".jpg" => format!("{} type=\"image/jpg\"", link),
                ".png" => format!("{} type=\"image/png\"", link),
                ".svg" => format!("{} type=\"image/svg+xml\"", link),
                _ => link,
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

    pub(crate) fn render(&self) -> Markup {
        html! {
            @for item in &self.0 {
                (PreEscaped(item))
            }
        }
    }
}
