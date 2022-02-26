use crate::core::all::THEMES;

pub use maud::{DOCTYPE, Markup, PreEscaped, html};

mod api;
pub use api::Theme;

pub fn register_theme(t: &'static (dyn Theme + 'static)) {
    THEMES.write().unwrap().push(t);
}

pub fn find_theme(name: &str) -> Option<&'static (dyn Theme + 'static)> {
    let themes = THEMES.write().unwrap();
    match themes.iter().find(|t| t.name() == name) {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
