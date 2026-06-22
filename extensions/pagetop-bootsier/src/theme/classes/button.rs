use pagetop::prelude::*;

use crate::theme::attrs::Color;

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
/// let save = Button::submit(L10n::n("Save"))
///     .with_prop(PropsOp::add_classes(classes::ButtonColor::solid(Color::Primary)));
///
/// // Botón con contorno.
/// let cancel = Button::plain(L10n::n("Cancel"))
///     .with_prop(PropsOp::add_classes(classes::ButtonColor::outline(Color::Secondary)));
///
/// // Botón tipo enlace.
/// let back = Button::plain(L10n::n("Back"))
///     .with_prop(PropsOp::add_classes(classes::ButtonColor::link()));
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

    /// Devuelve la clase `btn-*` correspondiente al color del botón.
    ///
    /// Si no se ha definido ningún estilo, devuelve `""`.
    pub fn to_class(self) -> String {
        match self.style {
            ButtonColorStyle::None => String::new(),
            ButtonColorStyle::Solid => format!("btn-{}", self.color.as_str()),
            ButtonColorStyle::Outline => format!("btn-outline-{}", self.color.as_str()),
            ButtonColorStyle::Link => "btn-link".to_owned(),
        }
    }
}

impl Into<CowStr> for ButtonColor {
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
/// let small = Button::submit(L10n::n("Save"))
///     .with_prop(PropsOp::add_classes(classes::ButtonSize::small()));
///
/// let large = Button::submit(L10n::n("Save"))
///     .with_prop(PropsOp::add_classes(classes::ButtonSize::large()));
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

    /// Devuelve la clase `btn-sm` o `btn-lg` correspondiente al tamaño del botón.
    ///
    /// Si no se ha definido ningún tamaño, devuelve `""`.
    pub fn to_class(self) -> String {
        match self.size {
            ButtonSizeVariant::None => String::new(),
            ButtonSizeVariant::Small => "btn-sm".to_owned(),
            ButtonSizeVariant::Large => "btn-lg".to_owned(),
        }
    }
}

impl Into<CowStr> for ButtonSize {
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
