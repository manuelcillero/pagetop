use pagetop::prelude::*;

use crate::config;
use crate::theme::bs;

// Regiones de Bootsier: se renderiza sin el `<div role="region">` envolvente que aplica
// `layout::Template::prepare()` por defecto. Devuelve `None` si `component` no envuelve una
// `CoreTemplates`, dejando que el resto de la cadena de temas (o el propio componente) resuelva el
// renderizado por defecto.
//
// `CoreTemplates::Admin` delega hoy en el mismo maquetado que `Standard`: la diferencia entre
// página "normal" y de administración no vive en la plantilla, sino en qué se registra en cada
// región -- p. ej. el menú de `pagetop-admin` en `CoreRegions::Aside`, que se autolimita a
// `CoreTemplates::Admin` (ver `pagetop_admin::component::AdminMenu`). Mantener la rama separada
// deja abierta la posibilidad de que un maquetado distinto lo necesite en el futuro sin tocar el
// núcleo.
pub(crate) async fn render(
    component: &layout::Template,
    cx: &mut Context,
) -> Option<Result<Markup, ComponentError>> {
    match component.template().downcast_ref::<CoreTemplates>()? {
        CoreTemplates::Standard => Some(Ok(render_standard(cx).await)),
        CoreTemplates::Admin => Some(Ok(render_standard(cx).await)),
    }
}

// `CoreRegions::Header`, `CoreRegions::Aside`, `CoreRegions::Content` y `CoreRegions::Footer`
// envueltos en un contenedor de ancho configurable.
async fn render_standard(cx: &mut Context) -> Markup {
    bs::Container::new()
        .with_prop(PropsOp::add_classes("container-wrapper"))
        .with_width(bs::container::Width::FluidMax(
            config::SETTINGS.bootsier.max_width,
        ))
        .with_child(layout::Region::header())
        .with_child(layout::Region::aside())
        .with_child(layout::Region::default())
        .with_child(layout::Region::footer())
        .render(cx)
        .await
}
