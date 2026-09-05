use crate::AutoDefault;
use crate::core::component::{Context, Contextual};

// **< Breakpoint >*********************************************************************************

/// Puntos de corte *responsive*, *mobile-first* (aplican "a partir de" el ancho indicado).
///
/// No define ningún valor en píxeles por sí mismo; cada tema decide a qué ancho corresponde cada
/// variante en su sistema de diseño (ver [`Theme::breakpoint_min_width()`]).
///
/// [`Theme::breakpoint_min_width()`]: crate::core::theme::Theme::breakpoint_min_width
#[derive(AutoDefault, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Breakpoint {
    /// Base *mobile-first*, equivale a "siempre".
    #[default]
    Xs,
    /// A partir del ancho donde un tema suele pasar de móvil a tableta.
    Sm,
    /// A partir del ancho donde un tema suele pasar a un escritorio pequeño.
    Md,
    /// A partir del ancho donde un tema suele pasar a un escritorio normal.
    Lg,
    /// A partir del ancho donde un tema suele considerar el escritorio ancho.
    Xl,
    /// A partir del ancho donde un tema suele considerar el escritorio muy ancho.
    Xxl,
}

impl Breakpoint {
    // Todas las variantes, en orden mobile-first (de Xs a Xxl).
    pub(crate) const ALL: [Breakpoint; 6] = [
        Breakpoint::Xs,
        Breakpoint::Sm,
        Breakpoint::Md,
        Breakpoint::Lg,
        Breakpoint::Xl,
        Breakpoint::Xxl,
    ];

    /// Ancho mínimo resuelto a través del tema activo del contexto actual, como valor CSS ya
    /// formateado (p. ej. `"768px"`), o `""` si la variante se aplica siempre, sin ancho real.
    ///
    /// Atajo de [`Theme::breakpoint_min_width()`](crate::core::theme::Theme::breakpoint_min_width)
    /// a través de [`Context::theme()`].
    pub fn min_width(&self, cx: &Context) -> &'static str {
        cx.theme().breakpoint_min_width(*self)
    }
}
