use crate::prelude::*;

/// Navegación del panel de administración para el usuario de la petición actual.
///
/// Es una [`Navbar`] con un enlace al inicio del panel (*dashboard*) seguido de un desplegable por
/// sección, con las páginas de cada una a las que el usuario puede acceder; las secciones sin
/// ninguna no se muestran. Lo construye en cada petición a partir de [`AdminRegistry`].
///
/// La plantilla [`CoreTemplates::Admin`] lo pinta tras la región [`CoreRegions::Header`]. Se
/// construye sólo con componentes del núcleo (`Navbar`, `Nav` y `Dropdown`), así que un tema le da
/// su aspecto capturando esos componentes o éste mismo en [`Theme::render_component()`].
///
/// Sin acceso a ninguna página del panel ([`can_access_admin()`]) no se renderiza: no se despachan
/// sus acciones ni intervienen los temas.
///
/// [`Theme::render_component()`]: crate::core::theme::Theme::render_component
#[derive(AutoDefault, Clone, Debug)]
pub struct AdminMenu;

#[async_trait]
impl Component for AdminMenu {
    fn new() -> Self {
        Self
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        can_access_admin(cx)
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let registry = AdminRegistry::global();

        let mut nav =
            Nav::new().with_item(nav::Item::link(Lc::l("admin_dashboard"), admin_path("")));
        for section in registry.sections() {
            let pages: Vec<_> = registry
                .pages_for_section(section.key())
                .iter()
                .filter(|p| p.is_accessible(cx))
                .collect();
            if pages.is_empty() {
                continue;
            }
            let mut menu = Dropdown::new().with_title(section.title().clone());
            for page in pages {
                menu = menu.with_item(dropdown::Item::link(
                    page.title().clone(),
                    admin_path(page.path()),
                ));
            }
            nav = nav.with_item(nav::Item::dropdown(menu));
        }

        Ok(Navbar::simple()
            .with_prop(PropsOp::add_classes("admin-menu"))
            .with_item(navbar::Item::nav(nav))
            .render(cx)
            .await)
    }
}
