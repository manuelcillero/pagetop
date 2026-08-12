use pagetop::prelude::*;

use crate::theme::ThemeColor;

// **< ButtonColor >********************************************************************************

/// Estilo visual aplicado al color de un botón ([`ButtonColor`]).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonColorStyle {
    /// Sin clase de color (estilo por defecto del tema).
    #[default]
    None,
    /// Botón sólido: genera la clase `btn-{color}`.
    Solid,
    /// Botón con contorno: genera la clase `btn-outline-{color}`.
    Outline,
    /// Botón tipo enlace: genera la clase `btn-link`.
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
/// let save = bs::Button::submit(Lc::n("Save"))
///     .with_prop(PropsOp::add_classes(class::ButtonColor::solid(ThemeColor::Primary)));
///
/// // Botón con contorno.
/// let cancel = bs::Button::plain(Lc::n("Cancel"))
///     .with_prop(PropsOp::add_classes(class::ButtonColor::outline(ThemeColor::Secondary)));
///
/// // Botón tipo enlace.
/// let back = bs::Button::plain(Lc::n("Back"))
///     .with_prop(PropsOp::add_classes(class::ButtonColor::link()));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct ButtonColor {
    style: ButtonColorStyle,
    color: ThemeColor,
}

impl ButtonColor {
    /// Sin clase de color (estilo por defecto del tema).
    pub fn new() -> Self {
        Self::default()
    }

    /// Botón sólido: genera la clase `btn-{color}`.
    pub fn solid(color: ThemeColor) -> Self {
        Self {
            style: ButtonColorStyle::Solid,
            color,
            ..Default::default()
        }
    }

    /// Botón con contorno: genera la clase `btn-outline-{color}`.
    pub fn outline(color: ThemeColor) -> Self {
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
    pub fn with_color(mut self, color: ThemeColor) -> Self {
        self.color = color;
        self
    }

    /// Cambia el estilo aplicado al botón (sólido, contorno o enlace).
    pub fn with_style(mut self, style: ButtonColorStyle) -> Self {
        self.style = style;
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

/// Tamaño aplicado a un botón ([`ButtonSize`]).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonSizeKind {
    /// Sin clase de tamaño (tamaño por defecto del tema).
    #[default]
    None,
    /// Botón compacto: genera la clase `btn-sm`.
    Small,
    /// Botón grande: genera la clase `btn-lg`.
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
/// let small = bs::Button::submit(Lc::n("Save"))
///     .with_prop(PropsOp::add_classes(class::ButtonSize::small()));
///
/// let large = bs::Button::submit(Lc::n("Save"))
///     .with_prop(PropsOp::add_classes(class::ButtonSize::large()));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct ButtonSize {
    size: ButtonSizeKind,
}

impl ButtonSize {
    /// Sin clase de tamaño (tamaño por defecto del tema).
    pub fn new() -> Self {
        Self::default()
    }

    /// Botón compacto: genera la clase `btn-sm`.
    pub fn small() -> Self {
        Self {
            size: ButtonSizeKind::Small,
        }
    }

    /// Botón grande: genera la clase `btn-lg`.
    pub fn large() -> Self {
        Self {
            size: ButtonSizeKind::Large,
        }
    }

    // **< ButtonSize BUILDER >*********************************************************************

    /// Cambia el tamaño aplicado al botón.
    pub fn with_size(mut self, size: ButtonSizeKind) -> Self {
        self.size = size;
        self
    }

    // **< ButtonSize HELPERS >*********************************************************************

    /// Añade la clase `btn-sm` o `btn-lg` a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        let class = match self.size {
            ButtonSizeKind::None => return,
            ButtonSizeKind::Small => "btn-sm",
            ButtonSizeKind::Large => "btn-lg",
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
