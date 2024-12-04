use crate::core::layout::LayoutRef;
use crate::global;

use std::sync::{LazyLock, RwLock};

// THEMES ******************************************************************************************

pub static LAYOUTS: LazyLock<RwLock<Vec<LayoutRef>>> = LazyLock::new(|| RwLock::new(Vec::new()));

// DEFAULT THEME ***********************************************************************************

pub static DEFAULT_LAYOUT: LazyLock<LayoutRef> =
    LazyLock::new(
        || match layout_by_short_name(&global::SETTINGS.app.layout) {
            Some(layout) => layout,
            None => &crate::base::layout::Basic,
        },
    );

// THEME BY NAME ***********************************************************************************

pub fn layout_by_short_name(short_name: &str) -> Option<LayoutRef> {
    let short_name = short_name.to_lowercase();
    match LAYOUTS
        .read()
        .unwrap()
        .iter()
        .find(|t| t.short_name().to_lowercase() == short_name)
    {
        Some(layout) => Some(*layout),
        _ => None,
    }
}
