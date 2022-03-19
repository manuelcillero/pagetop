use crate::all;

mod definition;
pub use definition::ThemeTrait;

pub mod aliner;
pub mod minimal;
pub mod bootsier;

pub fn register_theme(t: &'static dyn ThemeTrait) {
    all::THEMES.write().unwrap().push(t);
}

pub fn find_theme(name: &str) -> Option<&'static dyn ThemeTrait> {
    let themes = all::THEMES.write().unwrap();
    match themes.iter().find(|t| t.name() == name) {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
