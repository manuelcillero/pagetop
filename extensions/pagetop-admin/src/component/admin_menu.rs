use pagetop::prelude::*;

use crate::registry;

/// Componente que renderiza el menú de secciones visibles del panel de administración.
///
/// Construye el [`Nav`] en cada petición a partir de [`registry::admin_menu()`], ya filtrado por
/// el usuario de la petición actual. Pensado para que un tema lo registre en su propia región de
/// navegación (p. ej. `pagetop-bootsier` lo añade a su sidebar) e intercepte `Nav`/`nav::Item` en
/// [`Theme::handle_component()`](pagetop::core::theme::Theme::handle_component) si quiere darle un
/// aspecto propio; sin intercepción, se renderiza con el marcado por defecto de [`Nav`].
///
/// Sólo se renderiza en páginas creadas con
/// [`Page::admin()`](pagetop::response::Page::admin) (plantilla
/// [`CoreTemplates::Admin`](pagetop::core::theme::CoreTemplates::Admin)). Es necesario comprobarlo
/// explícitamente porque, si se registra en una región de propósito general como
/// [`CoreRegions::Aside`](pagetop::core::theme::CoreRegions::Aside), se renderizaría también en
/// páginas `Standard` si no se autolimitara.
#[derive(AutoDefault, Clone, Debug)]
pub struct AdminMenu;

#[async_trait]
impl Component for AdminMenu {
    fn new() -> Self {
        Self
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        if !matches!(
            cx.template().downcast_ref::<CoreTemplates>(),
            Some(CoreTemplates::Admin)
        ) {
            return Ok(html! {});
        }
        Ok(registry::admin_menu(cx).render(cx).await)
    }
}
