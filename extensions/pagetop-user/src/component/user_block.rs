//! Componente de bloque de usuario (login/logout en la cabecera).

use pagetop::prelude::*;

use crate::config::SETTINGS;
use crate::{LOCALES_USER, LOGIN_PATH, LOGOUT_PATH, REGISTER_PATH};

#[derive(AutoDefault, Clone, Debug)]
pub struct UserBlock;

#[async_trait]
impl Component for UserBlock {
    fn new() -> Self {
        Self
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let user = cx.current_user();
        Ok(if user.is_authenticated() {
            let display = user.display_name().unwrap_or("?");
            html! {
                nav.user-block {
                    span.user-name { (display) }
                    " · "
                    form.user-logout-inline method="post" action=(cx.route(LOGOUT_PATH)) {
                        button type="submit" {
                            (Lc::t("btn-logout", &LOCALES_USER).using(cx))
                        }
                    }
                }
            }
        } else {
            html! {
                nav.user-block {
                    a href=(cx.route(LOGIN_PATH)) {
                        (Lc::t("btn-login", &LOCALES_USER).using(cx))
                    }
                    @if SETTINGS.allow_registration {
                        " · "
                        a href=(cx.route(REGISTER_PATH)) {
                            (Lc::t("link-register", &LOCALES_USER).using(cx))
                        }
                    }
                }
            }
        })
    }
}
