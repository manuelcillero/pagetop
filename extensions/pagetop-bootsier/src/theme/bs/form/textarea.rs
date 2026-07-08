//! Definiciones para crear áreas de texto en formularios.

use pagetop::prelude::*;

pub use pagetop::base::component::form::Textarea;

const EXTRA_FLOATING_LABEL: &str = "bootsier.form.textarea.floating_label";

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
        self.with_prop(PropsOp::set_extra(EXTRA_FLOATING_LABEL, floating))
    }
}

// **< Textarea SETUP >*****************************************************************************

pub(crate) fn setup(field: &mut Textarea) {
    if field.props().extra_or(EXTRA_FLOATING_LABEL, false) {
        field.alter_prop(PropsOp::add_classes("form-floating"));
        field.alter_rows(None::<u16>);
    } else {
        field.alter_prop(PropsOp::remove_classes("form-floating"));
    }
}

// **< Textarea RENDER >****************************************************************************

pub(crate) fn render(field: &Textarea, cx: &mut Context) -> Result<Markup, ComponentError> {
    let container_id = field.id();
    let textarea_id = container_id
        .as_deref()
        .map(|id| util::join!(id, "-textarea"));
    let floating = field.props().extra_or(EXTRA_FLOATING_LABEL, false);
    // La etiqueta flotante requiere `placeholder` para animar la etiqueta; si no está definido se
    // fuerza `placeholder=""`.
    let placeholder = if floating {
        Some(field.placeholder().lookup(cx).unwrap_or_default())
    } else {
        field.placeholder().lookup(cx)
    };
    let label = match field.label().lookup(cx) {
        Some(text) => html! {
            label for=[textarea_id.as_deref()] class="form-label" {
                (text)
                @if *field.required() {
                    span
                        class="form-required"
                        title=(L10n::l("field_required").using(cx))
                    {
                        "*"
                    }
                }
            }
        },
        None => html! {},
    };
    Ok(html! {
        div (field.props()) {
            @if !floating { (label) }
            textarea
                id=[textarea_id.as_deref()]
                class="form-control"
                name=[field.name().get()]
                rows=[field.rows().get()]
                minlength=[field.minlength().get()]
                maxlength=[field.maxlength().get()]
                placeholder=[placeholder]
                autocomplete=[field.autocomplete().get()]
                autofocus[*field.autofocus()]
                readonly[*field.readonly()]
                required[*field.required()]
                disabled[*field.disabled()]
            {
                @if let Some(value) = field.value().get() { (value) }
            }
            @if floating { (label) }
            @if let Some(description) = field.help_text().lookup(cx) {
                div class="form-text" { (description) }
            }
        }
    })
}
