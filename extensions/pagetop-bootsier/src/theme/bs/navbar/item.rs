use pagetop::prelude::*;

use crate::theme::bs::nav;

// Idéntico a `navbar::Item::prepare()` de base salvo la variante `Nav`: en vez de reconstruir el
// `<ul>` a mano (lo que se salta la cadena de temas), clona el `Nav` embebido, lo marca como
// "dentro de una Navbar" (para que `theme::bs::nav::setup()` use `navbar-nav` en vez de `nav` como
// clase base) y lo renderiza con su ciclo de vida completo -así recibe también `kind`/`layout`
// traducidos a clases de Bootstrap, y cualquier otro tema que intercepte `Nav` en el futuro-.
pub(crate) async fn render(
    item: &navbar::Item,
    cx: &mut Context,
) -> Result<Markup, ComponentError> {
    match item {
        navbar::Item::Nav(embed) => {
            let Some(mut nav) = embed.get().cloned() else {
                return Ok(html! {});
            };
            nav.alter_prop(PropsOp::set_extra(nav::EXTRA_IN_NAVBAR, true));
            Ok(html! { (nav.render(cx).await) })
        }
        _ => item.prepare(cx).await,
    }
}
