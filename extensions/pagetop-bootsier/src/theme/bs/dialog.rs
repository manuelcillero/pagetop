//! Definiciones para crear diálogos modales ([`Dialog`]).

use pagetop::prelude::*;

pub use pagetop::base::component::Dialog;

// **< Dialog SETUP >*******************************************************************************

pub(crate) fn setup(dialog: &mut Dialog) {
    dialog.alter_prop(PropsOp::prepend_classes("modal fade"));
}

// **< Dialog RENDER >******************************************************************************

pub(crate) async fn render(dialog: &Dialog, cx: &mut Context) -> Result<Markup, ComponentError> {
    let body = dialog.body().render(cx).await;
    let footer = dialog.footer().render(cx).await;
    if body.is_empty() && footer.is_empty() {
        return Ok(html! {});
    }

    let title = dialog.title().using(cx);
    // `setup()` del componente garantiza que habrá un `id` antes de renderizar. Sin título no hay
    // elemento que etiquete el diálogo, así que `aria-labelledby` se omite en vez de apuntar a un
    // `id` inexistente.
    let id_label = (!title.is_empty()).then(|| util::join!(dialog.id().unwrap(), "-label"));

    Ok(html! {
        div
            (dialog.props())
            tabindex="-1"
            aria-hidden="true"
            aria-labelledby=[id_label.as_deref()]
        {
            div class="modal-dialog" {
                div class="modal-content" {
                    div class="modal-header" {
                        @if let Some(id_label) = &id_label {
                            h5 id=(id_label) class="modal-title" { (title) }
                        }
                        button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label=[Lc::l("dialog_close").lookup(cx)]
                        {}
                    }
                    div class="modal-body" { (body) }
                    @if !footer.is_empty() {
                        div class="modal-footer" { (footer) }
                    }
                }
            }
        }
    })
}
