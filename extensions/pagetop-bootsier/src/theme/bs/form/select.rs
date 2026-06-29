//! Definiciones para crear listas de selección.

use pagetop::prelude::*;

pub use pagetop::base::component::form::select::{Entry, Field, Group, Item};

/// Extensión de Bootsier para [`form::select::Field`].
///
/// Proporciona soporte para **etiquetas flotantes** (*floating label*). La etiqueta flotante se
/// superpone al control mientras no hay ninguna opción seleccionada y permanece flotante cuando hay
/// una selección activa.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let language = bs::form::select::Field::new()
///     .with_name("language")
///     .with_label(L10n::n("Language"))
///     .with_floating_label(true)
///     .with_item(bs::form::select::Item::new("", L10n::n("— Choose —")).with_selected(true))
///     .with_item(bs::form::select::Item::new("es", L10n::n("Spanish")))
///     .with_item(bs::form::select::Item::new("en", L10n::n("English")));
/// ```
pub trait SelectBootsier {
    /// Establece si la etiqueta se muestra flotante sobre el campo.
    ///
    /// Cuando está activo, la etiqueta se superpone al control y permanece flotante siempre que
    /// haya una opción visible.
    ///
    /// Si se usa la etiqueta flotante, se anulan los valores establecidos con
    /// [`with_multiple()`](form::select::Field::with_multiple) y
    /// [`with_rows()`](form::select::Field::with_rows) antes del renderizado.
    fn with_floating_label(self, floating: bool) -> Self;
}

impl SelectBootsier for Field {
    fn with_floating_label(self, floating: bool) -> Self {
        if floating {
            self.with_prop(PropsOp::add_classes("form-floating"))
        } else {
            self.with_prop(PropsOp::remove_classes("form-floating"))
        }
    }
}
