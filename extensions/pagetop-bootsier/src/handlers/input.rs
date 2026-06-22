use pagetop::prelude::*;

pub fn render(c: &form::input::Field, cx: &mut Context) -> Result<Markup, ComponentError> {
    let container_id = c.id();
    let input_id = container_id.as_deref().map(|id| util::join!(id, "-input"));
    let floating = c.props().has_class("form-floating");
    let input_class = if *c.plaintext() {
        "form-control-plaintext"
    } else {
        "form-control"
    };
    // La etiqueta flotante requiere `placeholder` para animar la etiqueta; si no está definido, se
    // fuerza `placeholder=""`.
    let placeholder = if floating {
        Some(c.placeholder().lookup(cx).unwrap_or_default())
    } else {
        c.placeholder().lookup(cx)
    };
    let label = match c.label().lookup(cx) {
        Some(text) => html! {
            label for=[input_id.as_deref()] class="form-label" {
                (text)
                @if *c.required() {
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
        div (c.props()) {
            @if !floating { (label) }
            input
                type=(c.kind())
                id=[input_id.as_deref()]
                class=(input_class)
                name=[c.name().get()]
                value=[c.value().get()]
                minlength=[c.minlength().get()]
                maxlength=[c.maxlength().get()]
                placeholder=[placeholder]
                inputmode=[c.inputmode().get()]
                autocomplete=[c.autocomplete().get()]
                autofocus[*c.autofocus()]
                readonly[*c.readonly() || *c.plaintext()]
                required[*c.required()]
                disabled[*c.disabled()];
            @if floating { (label) }
            @if let Some(description) = c.help_text().lookup(cx) {
                div class="form-text" { (description) }
            }
        }
    })
}
