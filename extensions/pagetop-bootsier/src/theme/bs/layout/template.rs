use pagetop::prelude::*;

use crate::config;
use crate::theme::bs;

// Regiones de Bootsier: se renderiza sin el `<div role="region">` envolvente que aplica
// `layout::Template::prepare()` por defecto. Devuelve `None` si `component` no envuelve una
// `CoreTemplates`, dejando que el resto de la cadena de temas (o el propio componente) resuelva el
// renderizado por defecto.
//
// `CoreTemplates::Admin` sigue la composición del núcleo: tras la cabecera, la navegación del
// panel (`AdminMenu`) y, en lugar del contenido, el marco de la página (`AdminFrame`), que
// contiene la región `Content`.
pub(crate) async fn render(
    component: &layout::Template,
    cx: &mut Context,
) -> Option<Result<Markup, ComponentError>> {
    match component.template().downcast_ref::<CoreTemplates>()? {
        CoreTemplates::Standard => Some(Ok(render_standard(cx).await)),
        CoreTemplates::Admin => Some(Ok(render_admin(cx).await)),
    }
}

// `CoreRegions::Header`, `CoreRegions::Aside`, `CoreRegions::Content` y `CoreRegions::Footer`
// envueltos en un contenedor de ancho configurable.
async fn render_standard(cx: &mut Context) -> Markup {
    wrapper()
        .with_child(layout::Region::header())
        .with_child(layout::Region::aside())
        .with_child(layout::Region::default())
        .with_child(layout::Region::footer())
        .render(cx)
        .await
}

// Como `render_standard()`, con la navegación del panel tras la cabecera y el marco de la página
// en lugar de la región `Content`.
async fn render_admin(cx: &mut Context) -> Markup {
    wrapper()
        .with_prop(PropsOp::add_classes("container-wrapper-admin"))
        .with_child(layout::Region::header())
        .with_child(AdminMenu::new())
        .with_child(layout::Region::aside())
        .with_child(AdminFrame::new())
        .with_child(layout::Region::footer())
        .render(cx)
        .await
}

fn wrapper() -> bs::Container {
    bs::Container::new()
        .with_prop(PropsOp::add_classes("container-wrapper"))
        .with_width(bs::container::Width::FluidMax(config::SETTINGS.max_width))
}
