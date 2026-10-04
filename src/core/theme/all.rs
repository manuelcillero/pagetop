use crate::core::theme::ThemeRef;
use crate::global;

use parking_lot::RwLock;

use std::sync::LazyLock;

// **< TEMAS >**************************************************************************************

static THEMES: LazyLock<RwLock<Vec<ThemeRef>>> = LazyLock::new(|| RwLock::new(Vec::new()));

// Registra el tema si no lo estaba ya, para evitar duplicados. Devuelve `true` si lo ha añadido.
pub(crate) fn register_theme(theme: ThemeRef) -> bool {
    let mut themes = THEMES.write();
    if themes.iter().any(|t| t.type_id() == theme.type_id()) {
        return false;
    }
    themes.push(theme);
    true
}

/// Devuelve los temas habilitados en la aplicación, en el orden en que se registraron.
pub fn enabled_themes() -> Vec<ThemeRef> {
    THEMES.read().clone()
}

/// Devuelve el tema identificado por su [`short_name()`](crate::core::AnyInfo::short_name), si está
/// habilitado, sin distinguir mayúsculas y minúsculas.
pub fn theme_by_short_name(short_name: &str) -> Option<ThemeRef> {
    THEMES
        .read()
        .iter()
        .find(|t| t.short_name().eq_ignore_ascii_case(short_name))
        .copied()
}

// **< TEMA PREDETERMINADO >************************************************************************

static DEFAULT_THEME: LazyLock<ThemeRef> =
    LazyLock::new(|| match theme_by_short_name(&global::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &crate::base::theme::Basic,
    });

/// Devuelve el tema predeterminado de la aplicación: el configurado en `app.theme` si está
/// habilitado o, en otro caso, [`Basic`](crate::base::theme::Basic).
///
/// Es el tema del sitio, no necesariamente el que se usa en una petición concreta: para renderizar,
/// el tema efectivo es el de [`Contextual::theme()`], que tiene en cuenta el tema preferido del
/// usuario ([`CurrentUser::theme()`]).
///
/// [`Contextual::theme()`]: crate::core::component::Contextual::theme
/// [`CurrentUser::theme()`]: crate::auth::CurrentUser::theme
pub fn default_theme() -> ThemeRef {
    *DEFAULT_THEME
}
