//! Handler HTTP para el perfil del propio usuario autenticado.

use pagetop::base::component::table::Row;
use pagetop::prelude::*;

use crate::account::UserStatus;
use crate::component::admin::status_key;
use crate::entity::{role, user};
use crate::service::user_admin;
use crate::{LOCALES_USER, LOGIN_PATH, PROFILE_PATH};

// **< profile_get >********************************************************************************

/// GET /user - Perfil del usuario autenticado. Redirige al formulario de inicio de sesión si no hay
/// sesión activa, conservando la URL de retorno.
pub(crate) async fn profile_get(request: HttpRequest) -> Response {
    let cx = Context::new(request.clone());
    let Some(id) = cx.current_user().id() else {
        let target = cx.route(LOGIN_PATH).with_param("next", PROFILE_PATH);
        return Redirect::see_other(target).into_response();
    };

    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return ErrorPage::NotFound(Some(request)).into_response(),
    };
    let roles = match user_admin::user_roles(id).await {
        Ok(roles) => roles,
        Err(_) => return ErrorPage::InternalError(Some(request)).into_response(),
    };
    let status = UserStatus::from_i16(user.status);

    let mut page = Page::new(request);
    let details_block = profile_details(&user, status, page.context()).await;
    let roles_block = profile_roles(&roles, page.context()).await;

    page.with_title(Lc::t("title-profile", &LOCALES_USER))
        .with_child(details_block)
        .with_child(roles_block)
        .render()
        .await
        .into_response()
}

// Bloque de sólo lectura con los datos de perfil del usuario autenticado.
async fn profile_details(user: &user::Model, status: UserStatus, cx: &mut Context) -> Block {
    let mut table = Table::new()
        .with_prop(PropsOp::add_classes("user-admin-table"))
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-username-admin", &LOCALES_USER))
                .with_cell(user.username.as_str()),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-email", &LOCALES_USER))
                .with_cell(user.email.as_str()),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-display-name", &LOCALES_USER))
                .with_cell(user.display_name.as_deref().unwrap_or("-")),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-language", &LOCALES_USER))
                .with_cell(user.language.as_deref().unwrap_or("-")),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-timezone", &LOCALES_USER))
                .with_cell(user.timezone.as_deref().unwrap_or("-")),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("col-status", &LOCALES_USER))
                .with_cell(Lc::t(status_key(status), &LOCALES_USER)),
        );

    if user.is_admin {
        let badge = Badge::labeled(Lc::t("badge-admin", &LOCALES_USER))
            .with_prop(PropsOp::add_classes("user-admin-badge-admin"))
            .render(cx)
            .await;
        table = table.with_row(
            Row::new()
                .with_cell("")
                .with_cell(Html::with(move |_| badge.clone())),
        );
    }

    Block::new()
        .with_title(Lc::t("title-user-details", &LOCALES_USER))
        .with_child(table)
}

// Bloque de sólo lectura con los roles del usuario autenticado. A diferencia de la vista de
// administración, no enlaza cada rol a su pantalla de detalle: un usuario sin permisos de
// administración no puede acceder a ella.
async fn profile_roles(roles: &[role::Model], cx: &mut Context) -> Block {
    let mut items: Vec<(String, Option<Markup>)> = Vec::with_capacity(roles.len());
    for r in roles {
        let system_badge = if r.locked {
            Some(
                Badge::labeled(Lc::t("badge-system-role", &LOCALES_USER))
                    .with_prop(PropsOp::add_classes("user-admin-badge-system"))
                    .render(cx)
                    .await,
            )
        } else {
            None
        };
        items.push((r.label.clone(), system_badge));
    }

    Block::new()
        .with_title(Lc::t("field-roles", &LOCALES_USER))
        .with_child(Html::with(move |_cx| {
            html! {
                @if items.is_empty() {
                    "-"
                } @else {
                    ul.user-profile-roles {
                        @for (label, system_badge) in &items {
                            li {
                                (label.as_str())
                                @if let Some(badge) = system_badge {
                                    " "
                                    (badge)
                                }
                            }
                        }
                    }
                }
            }
        }))
}
