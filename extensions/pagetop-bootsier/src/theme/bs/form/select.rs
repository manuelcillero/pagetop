//! Definiciones para crear listas de selección.

use pagetop::prelude::*;

pub use pagetop::base::component::form::select::{Entry, Field, Group, Item};

const EXTRA_FLOATING_LABEL: &str = "bootsier.form.select.floating_label";

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
///     .with_label(Lc::n("Language"))
///     .with_floating_label(true)
///     .with_item(bs::form::select::Item::new("", Lc::n("— Choose —")).with_selected(true))
///     .with_item(bs::form::select::Item::new("es", Lc::n("Spanish")))
///     .with_item(bs::form::select::Item::new("en", Lc::n("English")));
/// ```
#[builder_impl]
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

#[builder_impl]
impl SelectBootsier for Field {
    fn with_floating_label(mut self, floating: bool) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_FLOATING_LABEL, floating));
        self
    }
}

// **< Field SETUP >********************************************************************************

pub(crate) fn setup(field: &mut Field) {
    if field.props().extra_or(EXTRA_FLOATING_LABEL, false) {
        field.alter_prop(PropsOp::add_classes("form-floating"));
        field.alter_multiple(false);
        field.alter_rows(None::<u16>);
    } else {
        field.alter_prop(PropsOp::remove_classes("form-floating"));
    }
}

// **< Field RENDER >*******************************************************************************

pub(crate) fn render(field: &Field, cx: &mut Context) -> Result<Markup, ComponentError> {
    let container_id = field.id();
    let select_id = container_id.as_deref().map(|id| util::join!(id, "-select"));
    let floating = field.props().extra_or(EXTRA_FLOATING_LABEL, false);
    let label = match field.label().lookup(cx) {
        Some(text) => html! {
            label for=[select_id.as_deref()] class="form-label" {
                (text)
                @if *field.required() {
                    span
                        class="form-required"
                        title=[Lc::l("field_required").lookup(cx)]
                    {
                        "*"
                    }
                }
            }
        },
        None => html! {},
    };
    Ok(html! {
        div (field.props().unpack(cx)) {
            @if !floating { (label) }
            select
                id=[select_id.as_deref()]
                class="form-select"
                name=[field.name().as_deref()]
                multiple[*field.multiple()]
                size=[field.rows()]
                autocomplete=[field.autocomplete()]
                autofocus[*field.autofocus()]
                required[*field.required()]
                disabled[*field.disabled()]
            {
                @for entry in field.entries() {
                    @match entry {
                        form::select::Entry::Item(opt) => {
                            option
                                value=(opt.value().as_deref().unwrap_or(""))
                                selected[*opt.selected()]
                                disabled[*opt.disabled()]
                            {
                                (opt.label().using(cx))
                            }
                        }
                        form::select::Entry::Group(group) => {
                            optgroup
                                label=[group.label().lookup(cx)]
                                disabled[*group.disabled()]
                            {
                                @for opt in group.items() {
                                    option
                                        value=(opt.value().as_deref().unwrap_or(""))
                                        selected[*opt.selected()]
                                        disabled[*opt.disabled()]
                                    {
                                        (opt.label().using(cx))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            @if floating { (label) }
            @if let Some(description) = field.help_text().lookup(cx) {
                div class="form-text" { (description) }
            }
        }
    })
}
