use pagetop::prelude::*;

pub fn setup(c: &mut form::select::Field) {
    if c.props().has_class("form-floating") {
        c.alter_multiple(false);
        c.alter_rows(None::<u16>);
    }
}

pub fn render(c: &form::select::Field, cx: &mut Context) -> Result<Markup, ComponentError> {
    let container_id = c.id();
    let select_id = container_id.as_deref().map(|id| util::join!(id, "-select"));
    let floating = c.props().has_class("form-floating");
    let label = match c.label().lookup(cx) {
        Some(text) => html! {
            label for=[select_id.as_deref()] class="form-label" {
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
            select
                id=[select_id.as_deref()]
                class="form-select"
                name=[c.name().get()]
                multiple[*c.multiple()]
                size=[c.rows().get()]
                autocomplete=[c.autocomplete().get()]
                autofocus[*c.autofocus()]
                required[*c.required()]
                disabled[*c.disabled()]
            {
                @for entry in c.entries() {
                    @match entry {
                        form::select::Entry::Item(opt) => {
                            option
                                value=(opt.value().as_str().unwrap_or(""))
                                selected[*opt.selected()]
                                disabled[*opt.disabled()]
                            {
                                (opt.label().using(cx))
                            }
                        }
                        form::select::Entry::Group(group) => {
                            optgroup
                                label=(group.label().using(cx))
                                disabled[*group.disabled()]
                            {
                                @for opt in group.items() {
                                    option
                                        value=(opt.value().as_str().unwrap_or(""))
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
            @if let Some(description) = c.help_text().lookup(cx) {
                div class="form-text" { (description) }
            }
        }
    })
}
