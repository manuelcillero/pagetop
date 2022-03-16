use crate::core::all;

mod definition;
pub use definition::ThemeTrait;

pub fn register_theme(t: &'static (dyn ThemeTrait + 'static)) {
    all::THEMES.write().unwrap().push(t);
}

pub fn find_theme(name: &str) -> Option<&'static (dyn ThemeTrait + 'static)> {
    let themes = all::THEMES.write().unwrap();
    match themes.iter().find(|t| t.name() == name) {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
