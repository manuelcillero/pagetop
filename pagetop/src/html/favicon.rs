use crate::html::{html, Markup};

#[derive(Default)]
pub struct Favicon(Vec<Markup>);

impl Favicon {
    pub fn new() -> Self {
        Favicon::default()
    }

    // Favicon BUILDER.

    pub fn with_icon(self, image: &str) -> Self {
        self.add_icon_item("icon", image, None, None)
    }

    pub fn with_icon_for_sizes(self, image: &str, sizes: &str) -> Self {
        self.add_icon_item("icon", image, Some(sizes), None)
    }

    pub fn with_apple_touch_icon(self, image: &str, sizes: &str) -> Self {
        self.add_icon_item("apple-touch-icon", image, Some(sizes), None)
    }

    pub fn with_mask_icon(self, image: &str, color: &str) -> Self {
        self.add_icon_item("mask-icon", image, None, Some(color))
    }

    pub fn with_manifest(self, file: &str) -> Self {
        self.add_icon_item("manifest", file, None, None)
    }

    pub fn with_theme_color(mut self, color: &str) -> Self {
        self.0.push(html! { meta name="theme-color" content=(color); });
        self
    }

    pub fn with_ms_tile_color(mut self, color: &str) -> Self {
        self.0.push(html! { meta name="msapplication-TileColor" content=(color); });
        self
    }

    pub fn with_ms_tile_image(mut self, image: &str) -> Self {
        self.0.push(html! { meta name="msapplication-TileImage" content=(image); });
        self
    }

    fn add_icon_item(
        mut self,
        icon_rel: &str,
        icon_source: &str,
        icon_sizes: Option<&str>,
        icon_color: Option<&str>,
    ) -> Self {
        let icon_type = match icon_source.rfind('.') {
            Some(i) => match icon_source[i..].to_owned().to_lowercase().as_str() {
                ".gif" => Some("image/gif"),
                ".ico" => Some("image/x-icon"),
                ".jpg" => Some("image/jpg"),
                ".png" => Some("image/png"),
                ".svg" => Some("image/svg+xml"),
                _ => None,
            },
            _ => None,
        };
        self.0.push(html! {
            link
                rel=(icon_rel)
                type=[(icon_type)]
                sizes=[(icon_sizes)]
                color=[(icon_color)]
                href=(icon_source);
        });
        self
    }

    // Favicon RENDER.

    pub(crate) fn render(&self) -> Markup {
        html! {
            @for item in &self.0 {
                (item)
            }
        }
    }
}
