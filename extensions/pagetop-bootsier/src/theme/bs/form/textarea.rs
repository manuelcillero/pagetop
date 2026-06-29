//! Definiciones para crear áreas de texto en formularios.

use pagetop::prelude::*;

pub use pagetop::base::component::form::Textarea;

/// Extensión de Bootsier para [`form::Textarea`].
///
/// Proporciona soporte para **etiquetas flotantes** (*floating label*). La etiqueta flotante se
/// superpone al control mientras no hay ninguna opción seleccionada y permanece flotante cuando hay
/// una selección activa.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let comentario = bs::form::Textarea::new()
///     .with_name("comment")
///     .with_label(L10n::n("Comment"))
///     .with_placeholder(L10n::n("Write here..."))
///     .with_floating_label(true);
/// ```
pub trait TextareaBootsier {
    /// Establece si la etiqueta se muestra flotante sobre el campo.
    ///
    /// Cuando está activo, la etiqueta se superpone al área de texto y asciende al enfocarlo o
    /// cuando tiene contenido. Requiere que el campo tenga un atributo `placeholder` definido;
    /// si no se especifica, se fuerza `placeholder=""` antes del renderizado.
    ///
    /// Si se usa la etiqueta flotante, se anula el valor establecido con
    /// [`with_rows()`](form::Textarea::with_rows) antes del renderizado.
    fn with_floating_label(self, floating: bool) -> Self;
}

impl TextareaBootsier for Textarea {
    fn with_floating_label(self, floating: bool) -> Self {
        if floating {
            self.with_prop(PropsOp::add_classes("form-floating"))
        } else {
            self.with_prop(PropsOp::remove_classes("form-floating"))
        }
    }
}
