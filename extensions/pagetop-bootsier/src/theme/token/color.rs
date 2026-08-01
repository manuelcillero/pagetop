use pagetop::prelude::*;

// **< ThemeColor >*********************************************************************************

/// Paleta de colores temáticos.
///
/// Equivalen a los nombres estándar definidos por Bootstrap (`primary`, `secondary`, `success`,
/// etc.). Se utiliza para componer las clases de color de [`Bg`], [`Border`] o [`Text`].
///
/// [`Bg`]: crate::theme::class::Bg
/// [`Border`]: crate::theme::class::Border
/// [`Text`]: crate::theme::class::Text
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ThemeColor {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
}

impl ThemeColor {
    /// Devuelve el nombre del color Bootstrap (`"primary"`, `"danger"`, etc.).
    #[rustfmt::skip]
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary   => "primary",
            Self::Secondary => "secondary",
            Self::Success   => "success",
            Self::Info      => "info",
            Self::Warning   => "warning",
            Self::Danger    => "danger",
            Self::Light     => "light",
            Self::Dark      => "dark",
        }
    }
}

// **< OpacityLevel >*******************************************************************************

/// Niveles de opacidad (`opacity-*`).
///
/// Se utiliza para las clases que definen la transparencia de [`Bg`] (`bg-opacity-*`), [`Border`]
/// (`border-opacity-*`) o [`Text`] (`text-opacity-*`).
///
/// [`Bg`]: crate::theme::class::Bg
/// [`Border`]: crate::theme::class::Border
/// [`Text`]: crate::theme::class::Text
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum OpacityLevel {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Permite generar clases `*-opacity-100` (100% de opacidad).
    Opaque,
    /// Permite generar clases `*-opacity-75` (75%).
    SemiOpaque,
    /// Permite generar clases `*-opacity-50` (50%).
    Half,
    /// Permite generar clases `*-opacity-25` (25%).
    SemiTransparent,
    /// Permite generar clases `*-opacity-10` (10%).
    AlmostTransparent,
    /// Permite generar clases `*-opacity-0` (0%, totalmente transparente).
    Transparent,
}

impl OpacityLevel {
    // Devuelve el sufijo para `*opacity-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default           => None,
            Self::Opaque            => Some("-100"),
            Self::SemiOpaque        => Some("-75"),
            Self::Half              => Some("-50"),
            Self::SemiTransparent   => Some("-25"),
            Self::AlmostTransparent => Some("-10"),
            Self::Transparent       => Some("-0"),
        }
    }

    // Añade la opacidad a la cadena de clases usando el prefijo dado (`bg`, `border`, `text`, o
    // vacío para `opacity-*`).
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String, prefix: &str) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            if prefix.is_empty() {
                classes.push_str("opacity");
            } else {
                classes.push_str(prefix);
                classes.push_str("-opacity");
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase de opacidad `opacity-*`.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// assert_eq!(OpacityLevel::Opaque.to_class(), "opacity-100");
    /// assert_eq!(OpacityLevel::Half.to_class(), "opacity-50");
    /// assert_eq!(OpacityLevel::Default.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class, "");
        class
    }
}
