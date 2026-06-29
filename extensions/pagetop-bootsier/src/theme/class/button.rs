use pagetop::prelude::*;

use crate::theme::token::Color;

// **< ButtonColor >********************************************************************************

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
enum ButtonColorStyle {
    #[default]
    None,
    Solid,
    Outline,
    Link,
}

/// Clases para establecer el **color y estilo** de los botones.
///
/// # Ejemplos
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// // Botón sólido.
/// let save = bs::Button::submit(L10n::n("Save"))
///     .with_prop(PropsOp::add_classes(class::ButtonColor::solid(token::Color::Primary)));
///
/// // Botón con contorno.
/// let cancel = bs::Button::plain(L10n::n("Cancel"))
///     .with_prop(PropsOp::add_classes(class::ButtonColor::outline(token::Color::Secondary)));
///
/// // Botón tipo enlace.
/// let back = bs::Button::plain(L10n::n("Back"))
///     .with_prop(PropsOp::add_classes(class::ButtonColor::link()));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct ButtonColor {
    style: ButtonColorStyle,
    color: Color,
}

impl ButtonColor {
    /// Sin clase de color (estilo por defecto del tema).
    pub fn new() -> Self {
        Self::default()
    }

    /// Botón sólido: genera la clase `btn-{color}`.
    pub fn solid(color: Color) -> Self {
        Self {
            style: ButtonColorStyle::Solid,
            color,
            ..Default::default()
        }
    }

    /// Botón con contorno: genera la clase `btn-outline-{color}`.
    pub fn outline(color: Color) -> Self {
        Self {
            style: ButtonColorStyle::Outline,
            color,
            ..Default::default()
        }
    }

    /// Botón tipo enlace: genera la clase `btn-link`.
    pub fn link() -> Self {
        Self {
            style: ButtonColorStyle::Link,
            ..Default::default()
        }
    }

    // **< ButtonColor BUILDER >********************************************************************

    /// Cambia el color aplicado al botón (`btn-*` o `btn-outline-*`).
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    // **< ButtonColor HELPERS >********************************************************************

    /// Añade la clase `btn-*` a la cadena de clases.
    #[rustfmt::skip]
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        let (prefix, suffix) = match self.style {
            ButtonColorStyle::None    => return,
            ButtonColorStyle::Solid   => ("btn-",         self.color.as_str()),
            ButtonColorStyle::Outline => ("btn-outline-", self.color.as_str()),
            ButtonColorStyle::Link    => ("btn-link",      ""),
        };
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(prefix);
        classes.push_str(suffix);
    }

    /// Devuelve la clase `btn-*` correspondiente al color del botón.
    ///
    /// Si no se ha definido ningún estilo, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl Into<CowStr> for ButtonColor {
    /// Permite pasar [`ButtonColor`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}

// **< ButtonSize >*********************************************************************************

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
enum ButtonSizeVariant {
    #[default]
    None,
    Small,
    Large,
}

/// Clases para establecer el **tamaño** de los botones.
///
/// # Ejemplos
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let small = bs::Button::submit(L10n::n("Save"))
///     .with_prop(PropsOp::add_classes(class::ButtonSize::small()));
///
/// let large = bs::Button::submit(L10n::n("Save"))
///     .with_prop(PropsOp::add_classes(class::ButtonSize::large()));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct ButtonSize {
    size: ButtonSizeVariant,
}

impl ButtonSize {
    /// Sin clase de tamaño (tamaño por defecto del tema).
    pub fn new() -> Self {
        Self::default()
    }

    /// Botón compacto: genera la clase `btn-sm`.
    pub fn small() -> Self {
        Self {
            size: ButtonSizeVariant::Small,
        }
    }

    /// Botón grande: genera la clase `btn-lg`.
    pub fn large() -> Self {
        Self {
            size: ButtonSizeVariant::Large,
        }
    }

    // **< ButtonSize HELPERS >*********************************************************************

    /// Añade la clase `btn-sm` o `btn-lg` a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        let class = match self.size {
            ButtonSizeVariant::None => return,
            ButtonSizeVariant::Small => "btn-sm",
            ButtonSizeVariant::Large => "btn-lg",
        };
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(class);
    }

    /// Devuelve la clase `btn-sm` o `btn-lg` correspondiente al tamaño del botón.
    ///
    /// Si no se ha definido ningún tamaño, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl Into<CowStr> for ButtonSize {
    /// Permite pasar [`ButtonSize`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
