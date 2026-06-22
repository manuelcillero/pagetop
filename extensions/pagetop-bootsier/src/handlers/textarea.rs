use pagetop::prelude::*;

pub fn setup(c: &mut form::Textarea) {
    if c.props().has_class("form-floating") {
        c.alter_rows(None::<u16>);
    }
}

pub fn render(c: &form::Textarea, cx: &mut Context) -> Result<Markup, ComponentError> {
    let container_id = c.id();
    let textarea_id = container_id
        .as_deref()
        .map(|id| util::join!(id, "-textarea"));
    let floating = c.props().has_class("form-floating");
    // La etiqueta flotante requiere `placeholder` para animar la etiqueta; si no está definido se
    // fuerza `placeholder=""`.
    let placeholder = if floating {
        Some(c.placeholder().lookup(cx).unwrap_or_default())
    } else {
        c.placeholder().lookup(cx)
    };
    let label = match c.label().lookup(cx) {
        Some(text) => html! {
            label for=[textarea_id.as_deref()] class="form-label" {
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
            textarea
                id=[textarea_id.as_deref()]
                class="form-control"
                name=[c.name().get()]
                rows=[c.rows().get()]
                minlength=[c.minlength().get()]
                maxlength=[c.maxlength().get()]
                placeholder=[placeholder]
                autocomplete=[c.autocomplete().get()]
                autofocus[*c.autofocus()]
                readonly[*c.readonly()]
                required[*c.required()]
                disabled[*c.disabled()]
            {
                @if let Some(value) = c.value().get() { (value) }
            }
            @if floating { (label) }
            @if let Some(description) = c.help_text().lookup(cx) {
                div class="form-text" { (description) }
            }
        }
    })
}
