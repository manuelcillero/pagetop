//! Menú de cuenta (iniciar y cerrar sesión).

use pagetop::prelude::*;
use pagetop_admin::registry::can_access_admin;
use pagetop_htmx::prelude::*;

use crate::config::SETTINGS;
use crate::{LOCALES_USER, LOGIN_PATH, LOGOUT_PATH, PROFILE_PATH, REGISTER_PATH};

/// Construye el menú de cuenta como un [`Nav`] justificado al final de su contenedor.
///
/// Para un usuario anónimo ofrece un enlace de inicio de sesión y, si el registro está permitido,
/// otro para crear una cuenta. Para un usuario autenticado, un desplegable titulado con su nombre
/// visible, con su perfil, la administración (si tiene acceso a ella) y la opción de cerrar
/// sesión. Se construye con componentes del núcleo, sin depender de ningún tema, para
/// usarlo dentro de una [`Navbar`].
///
/// El cierre de sesión se envía con una petición htmx que requiere JavaScript en el navegador.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_user::prelude::*;
///
/// # fn build(cx: &Context) -> Navbar {
/// Navbar::simple().with_item(navbar::Item::nav(account_menu(cx)))
/// # }
/// ```
pub fn account_menu(cx: &Context) -> Nav {
    let nav = Nav::new().with_prop(FlexItem::push_end());
    let user = cx.current_user();
    if user.is_authenticated() {
        let title = match user.display_name() {
            Some(name) => Lc::n(name.to_string()),
            None => Lc::t("menu-account", &LOCALES_USER),
        };
        // El cierre de sesión requiere POST; el atributo `hx-post` viaja en el `<li>` del elemento
        // (ver `logout_post`, que responde con `HX-Redirect` a las peticiones htmx).
        let logout = dropdown::Item::button(Lc::t("btn-logout", &LOCALES_USER))
            .with_prop(PropsOp::set(hx::POST, cx.route(LOGOUT_PATH).to_string()));
        let mut menu = Dropdown::new()
            .with_title(title)
            .with_menu_end(true)
            .with_item(dropdown::Item::link(
                Lc::t("menu-profile", &LOCALES_USER),
                cx.route(PROFILE_PATH),
            ));
        if can_access_admin(cx) {
            menu = menu.with_item(dropdown::Item::link(
                Lc::t("menu-admin", &LOCALES_USER),
                cx.route(pagetop_admin::ADMIN_BASE_PATH),
            ));
        }
        nav.with_item(nav::Item::dropdown(
            menu.with_item(dropdown::Item::divider()).with_item(logout),
        ))
    } else {
        let nav = nav.with_item(nav::Item::link(
            Lc::t("menu-login", &LOCALES_USER),
            cx.route(LOGIN_PATH),
        ));
        if SETTINGS.allow_registration {
            nav.with_item(nav::Item::link(
                Lc::t("link-register", &LOCALES_USER),
                cx.route(REGISTER_PATH),
            ))
        } else {
            nav
        }
    }
}

/// Componente que renderiza el menú de cuenta, ver [`account_menu()`].
#[derive(AutoDefault, Clone, Debug)]
pub struct AccountMenu;

#[async_trait]
impl Component for AccountMenu {
    fn new() -> Self {
        Self
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(account_menu(cx).render(cx).await)
    }
}
