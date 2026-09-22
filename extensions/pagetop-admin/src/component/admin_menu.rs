use pagetop::prelude::*;

use crate::registry;

/// Componente que renderiza la barra de navegación del panel de administración.
///
/// Construye la [`Navbar`] en cada petición a partir de [`registry::admin_navbar()`], ya filtrada
/// por el usuario de la petición actual. Pensado para que un tema la registre en su propia región
/// de navegación e intercepte `Navbar`/`Nav`/`Dropdown` en
/// [`Theme::render_component()`](pagetop::core::theme::Theme::render_component) si quiere darle un
/// aspecto propio (p. ej. `pagetop-bootsier` la reconoce automáticamente al ser componentes del
/// núcleo); sin intercepción, se renderiza con el marcado por defecto de cada componente.
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
        Ok(registry::admin_navbar(cx).render(cx).await)
    }
}
