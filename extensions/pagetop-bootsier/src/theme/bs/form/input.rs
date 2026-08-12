//! Definiciones para crear campos de texto de una línea.

use pagetop::prelude::*;

pub use pagetop::base::component::form::input::{Field, Kind, Mode};

const EXTRA_FLOATING_LABEL: &str = "bootsier.form.input.floating_label";

/// Extensión de Bootsier para [`form::input::Field`].
///
/// Proporciona soporte para **etiquetas flotantes** (*floating label*). La etiqueta flotante se
/// superpone al control mientras está vacío y permanece flotante cuando tiene contenido o está
/// enfocado.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let nombre = bs::form::input::Field::text()
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
    #[builder_fn]
    fn with_floating_label(self, floating: bool) -> Self;
}

impl InputBootsier for Field {
    #[builder_fn]
    fn with_floating_label(mut self, floating: bool) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_FLOATING_LABEL, floating));
        self
    }
}

// **< Field SETUP >********************************************************************************

pub(crate) fn setup(field: &mut Field) {
    if field.props().extra_or(EXTRA_FLOATING_LABEL, false) {
        field.alter_prop(PropsOp::add_classes("form-floating"));
    } else {
        field.alter_prop(PropsOp::remove_classes("form-floating"));
    }
}

// **< Field RENDER >*******************************************************************************

pub(crate) fn render(field: &Field, cx: &mut Context) -> Result<Markup, ComponentError> {
    let container_id = field.id();
    let input_id = container_id.as_deref().map(|id| util::join!(id, "-input"));
    let input_class = if *field.plaintext() {
        "form-control-plaintext"
    } else {
        "form-control"
    };
    let strict = field.kind().is_strict();
    let masked = *field.kind() == Kind::StrictPassword;
    let autocomplete = if strict {
        Some(form::Autocomplete::Off)
    } else {
        field.autocomplete().get()
    };

    // La etiqueta flotante requiere `placeholder` para animar la etiqueta.
    let floating = field.props().extra_or(EXTRA_FLOATING_LABEL, false);
    // Si no está definido, se fuerza `placeholder=""`.
    let placeholder = if floating {
        Some(field.placeholder().lookup(cx).unwrap_or_default())
    } else {
        field.placeholder().lookup(cx)
    };
    let label = match field.label().lookup(cx) {
        Some(text) => html! {
            label for=[input_id.as_deref()] class="form-label" {
                (text)
                @if *field.required() {
                    span
                        class="form-required"
                        title=[L10n::l("field_required").lookup(cx)]
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
            input
                type=(field.kind())
                id=[input_id.as_deref()]
                class=(input_class)
                name=[field.name().get()]
                value=[field.value().get()]
                minlength=[field.minlength().get()]
                maxlength=[field.maxlength().get()]
                placeholder=[placeholder]
                inputmode=[field.inputmode().get()]
                autocomplete=[autocomplete]
                spellcheck=[strict.then_some("false")]
                autocorrect=[strict.then_some("off")]
                style=[masked.then_some("-webkit-text-security: disc; text-security: disc;")]
                autofocus[*field.autofocus()]
                readonly[*field.readonly() || *field.plaintext() || strict]
                onfocus=[strict.then_some("this.removeAttribute('readonly')")]
                required[*field.required()]
                disabled[*field.disabled()];
            @if floating { (label) }
            @if let Some(description) = field.help_text().lookup(cx) {
                div class="form-text" { (description) }
            }
        }
    })
}
