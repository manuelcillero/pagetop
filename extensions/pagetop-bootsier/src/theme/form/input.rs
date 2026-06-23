//! Definiciones para crear campos de texto de una línea.

use pagetop::prelude::*;

pub use pagetop::base::component::form::input::{Field, Kind, Mode};

/// Extensión de Bootsier para [`form::input::Field`].
///
/// Proporciona soporte para **etiquetas flotantes** (*floating label*). La etiqueta flotante se
/// superpone al control mientras está vacío y permanece flotante cuando tiene contenido o está
/// enfocado.
///
/// ```rust,no_run
/// # use pagetop::locale::L10n;
/// use pagetop_bootsier::theme::*;
///
/// let nombre = form::input::Field::text()
///     .with_name("name")
///     .with_label(L10n::n("Name"))
///     .with_placeholder(L10n::n("Enter your name"))
///     .with_floating_label(true);
/// ```
pub trait InputBootsier {
    /// Establece si la etiqueta se muestra flotante sobre el campo.
    ///
    /// Cuando está activo, la etiqueta se superpone al campo y asciende al enfocarlo o cuando tiene
    /// contenido. Requiere que el campo tenga un atributo `placeholder` definido; si no se
    /// especifica, se fuerza `placeholder=""` antes del renderizado.
    fn with_floating_label(self, floating: bool) -> Self;
}

impl InputBootsier for Field {
    fn with_floating_label(self, floating: bool) -> Self {
        if floating {
            self.with_prop(PropsOp::add_classes("form-floating"))
        } else {
            self.with_prop(PropsOp::remove_classes("form-floating"))
        }
    }
}
