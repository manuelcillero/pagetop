use crate::core::global;

pub use maud::{DOCTYPE, Markup, PreEscaped, html};

mod definition;
pub use definition::Theme;

pub fn register_theme(t: &'static (dyn Theme + 'static)) {
    global::THEMES.write().unwrap().push(t);
}

pub fn find_theme(name: &str) -> Option<&'static (dyn Theme + 'static)> {
    let themes = global::THEMES.write().unwrap();
    match themes.iter().find(|t| t.name() == name) {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
