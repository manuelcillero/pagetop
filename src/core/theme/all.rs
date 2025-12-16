use crate::core::theme::ThemeRef;
use crate::global;

use parking_lot::RwLock;

use std::sync::LazyLock;

// **< TEMAS >**************************************************************************************

pub static THEMES: LazyLock<RwLock<Vec<ThemeRef>>> = LazyLock::new(|| RwLock::new(Vec::new()));

// **< TEMA PREDETERMINADO >************************************************************************

pub static DEFAULT_THEME: LazyLock<ThemeRef> =
    LazyLock::new(|| match theme_by_short_name(&global::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &crate::base::theme::Basic,
    });

// **< TEMA POR NOMBRE >****************************************************************************

/// Devuelve el tema identificado por su [`short_name()`](AnyInfo::short_name).
pub fn theme_by_short_name(short_name: &'static str) -> Option<ThemeRef> {
    let short_name = short_name.to_lowercase();
    match THEMES
        .read()
        .iter()
        .find(|t| t.short_name().to_lowercase() == short_name)
    {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
