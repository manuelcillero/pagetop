//! Handlers de administración de usuarios.

use serde::Deserialize;

use pagetop::base::component::table::Row;
use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

use crate::ADMIN_ROLES_PATH;
use crate::ADMIN_USERS_PATH;
use crate::ANONYMOUS_ROLE_ID;
use crate::AUTHENTICATED_ROLE_ID;
use crate::LOCALES_USER;
use crate::account::{Account, UserStatus};
use crate::component::admin::{
    AdminPasswordForm, USER_ADMIN_FORM_ID, UserForm, UserFormMode, UserRolesForm, UserTable,
    status_key,
};
use crate::config::SETTINGS;
use crate::entity::{role, user};
use crate::error::AuthError;
use crate::handlers::admin::{back_link, frame, map_auth_error};
use crate::password;
use crate::permission::UserPermission;
use crate::service::role_admin;
use crate::service::user_admin::{self, UserListParams, UserSortField};

#[derive(Deserialize)]
pub(crate) struct UsersQuery {
    #[serde(default)]
    q: Option<String>,
    #[serde(default)]
    sort: Option<String>,
    #[serde(default)]
    dir: Option<String>,
    #[serde(default)]
    page: Option<u64>,
}

// **< list_get >***********************************************************************************

/// GET /admin/user/users - Listado de usuarios (búsqueda, orden y paginación vía HTMX).
pub(crate) async fn list_get(
    request: HttpRequest,
    web::Query(query): web::Query<UsersQuery>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;

    let params = UserListParams {
        query: query.q.clone(),
        sort: UserSortField::from_query(query.sort.as_deref()),
        dir: SortDir::from_query(query.dir.as_deref()),
        page: query.page.unwrap_or(1).max(1),
        per_page: SETTINGS.admin.list_page_size,
    };

    let result = match user_admin::list_users(&params).await {
        Ok(result) => result,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };

    let mut table = UserTable::new()
        .with_items(result.items)
        .with_sort(params.sort)
        .with_dir(params.dir)
        .with_query(query.q.clone())
        .with_page(result.page)
        .with_per_page(result.per_page)
        .with_total(result.total);

    if request.is_htmx() {
        let mut cx = Context::admin(request);
        Ok(HtmxResponse::new(table.render(&mut cx).await).into_response())
    } else {
        let title = Lc::t("title-admin-users", &LOCALES_USER);
        Ok(Page::admin(request)
            .with_title(title.clone())
            .with_child(
                frame(title)
                    .with_child(search_bar(query.q))
                    .with_child(table),
            )
            .render()
            .await
            .into_response())
    }
}

// **< search_bar >*********************************************************************************

fn search_bar(current_query: Option<String>) -> Html {
    let value = current_query.unwrap_or_default();
    Html::with(move |cx| {
        html! {
            div.user-admin-search {
                input
                    type="search"
                    id="user-admin-search-input"
                    name="q"
                    value=(value.as_str())
                    placeholder=[Lc::t("field-search-users", &LOCALES_USER).lookup(cx)]
                    hx-get=(cx.route(ADMIN_USERS_PATH).to_string())
                    hx-trigger="keyup changed delay:400ms, search"
                    hx-target="#user-table-wrapper"
                    hx-swap=(hx::swap::OUTER_HTML_SCROLL_TOP)
                    hx-push-url="true";
            }
        }
    })
}

// **< available_roles >****************************************************************************

// Roles asignables desde la UI de usuarios: excluye "anonymous" (nunca se asigna explícitamente)
// y "authenticated" (se envía siempre fijo vía campo oculto).
async fn available_roles(selected: &[i32]) -> Result<Vec<(i32, String, bool)>, AuthError> {
    let items = role_admin::list_roles(&role_admin::RoleListParams {
        sort: role_admin::RoleSortField::Weight,
        dir: SortDir::Asc,
    })
    .await?;
    Ok(items
        .into_iter()
        .filter(|r| r.id != ANONYMOUS_ROLE_ID && r.id != AUTHENTICATED_ROLE_ID)
        .map(|r| {
            let checked = selected.contains(&r.id);
            (r.id, r.label, checked)
        })
        .collect())
}

// **< new_get / new_post >*************************************************************************

/// GET /admin/user/users/new - Formulario de alta de usuario.
pub(crate) async fn new_get(
    request: HttpRequest,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    let roles = match available_roles(&[]).await {
        Ok(roles) => roles,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };
    // El campo "administrador" sólo se ofrece si quien da de alta ya es administrador: no es un
    // permiso del catálogo (igual que conceder/revocar en la edición, ver `set_user_admin`).
    let allow_admin_field = request.extension::<Account>().is_some_and(|a| a.is_admin);
    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));
    let title = Lc::t("title-admin-user-new", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    UserForm::new()
                        .with_mode(UserFormMode::New)
                        .with_roles(roles)
                        .with_allow_admin_field(allow_admin_field)
                        .with_waypoint(waypoint),
                )
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct NewUserFormData {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    timezone: String,
    #[serde(default)]
    role_ids: Vec<String>,
    #[serde(default)]
    is_admin: bool,
}

/// POST /admin/user/users/new - Crea un usuario.
///
/// Usa `RawForm` + `serde_qs` en lugar de `axum::extract::Form` (basado en `serde_urlencoded`,
/// que no deserializa claves repetidas como `role_ids=2&role_ids=3` en un `Vec<T>`).
pub(crate) async fn new_post(
    request: HttpRequest,
    web::Query(waypoint): web::Query<Waypoint>,
    raw: web::RawForm,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    let Ok(form) = serde_qs::from_bytes::<NewUserFormData>(&raw.0) else {
        return Err(ErrorPage::BadRequest(Some(request)));
    };
    let allow_admin_field = request.extension::<Account>().is_some_and(|a| a.is_admin);
    // Nunca fiarse sólo de que el campo esté presente en el formulario: sólo se concede si quien
    // envía la petición ya es administrador, aunque alguien manipulase la petición a mano.
    let is_admin = form.is_admin && allow_admin_field;

    let role_ids: Vec<i32> = form
        .role_ids
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    let result = user_admin::create_user(user_admin::NewUserData {
        username: &form.username,
        email: &form.email,
        password: &form.password,
        confirm_password: &form.confirm_password,
        display_name: util::non_blank(&form.display_name),
        language: util::non_blank(&form.language),
        timezone: util::non_blank(&form.timezone),
        initial_role_ids: &role_ids,
        is_admin,
    })
    .await;

    match result {
        Ok(_) => {
            let cx = Context::admin(request);
            let target = waypoint.or(cx.route(ADMIN_USERS_PATH));
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let roles = available_roles(&role_ids).await.unwrap_or_default();
            let mut page = Page::admin(request);
            let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));
            let form_component = UserForm::new()
                .with_mode(UserFormMode::New)
                .with_username(form.username)
                .with_email(form.email)
                .with_display_name(form.display_name)
                .with_language(form.language)
                .with_timezone(form.timezone)
                .with_roles(roles)
                .with_allow_admin_field(allow_admin_field)
                .with_is_admin(is_admin)
                .with_error(map_auth_error(&err))
                .with_waypoint(waypoint);
            let title = Lc::t("title-admin-user-new", &LOCALES_USER);
            Ok(page
                .with_title(title.clone())
                .with_child(
                    frame(title)
                        .with_child(form_component)
                        .with_child(back_link(back_href)),
                )
                .render()
                .await
                .into_response())
        }
    }
}

// **< edit_get / edit_post >************************************************************************

async fn render_user_edit(
    request: HttpRequest,
    id: i32,
    error: Option<Lc>,
    waypoint: Waypoint,
) -> Response {
    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return ErrorPage::NotFound(Some(request)).into_response(),
    };
    let status = UserStatus::from_i16(user.status);
    // El botón de conceder/revocar sólo se muestra si quien lo ve ya es administrador y no está
    // viendo su propio perfil: no es un permiso del catálogo, y nadie puede automodificarse el
    // flag (ver `set_user_admin`).
    let can_toggle_admin = request
        .extension::<Account>()
        .is_some_and(|a| a.is_admin && a.id != id);
    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));
    let title = Lc::t("title-admin-user-edit", &LOCALES_USER);
    let actions = edit_actions(
        id,
        status,
        user.is_admin,
        can_toggle_admin,
        &waypoint,
        page.context(),
    );

    page.with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    UserForm::new()
                        .with_mode(UserFormMode::Edit)
                        .with_user_id(Some(id))
                        .with_username(user.username)
                        .with_email(user.email)
                        .with_display_name(user.display_name.unwrap_or_default())
                        .with_language(user.language.unwrap_or_default())
                        .with_timezone(user.timezone.unwrap_or_default())
                        .with_error(error)
                        .with_waypoint(waypoint.clone()),
                )
                .with_child(actions)
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response()
}

// Enlaces a las pantallas dedicadas (roles, restablecer contraseña) y botones de bloqueo/activación
// y de concesión/revocación de acceso irrestricto (este último sólo si `can_toggle_admin`). El
// listado de origen (`waypoint`) se arrastra a todas ellas para que, al volver aquí, esta misma
// pantalla siga sabiendo devolver al listado en el estado en que se dejó.
//
// "Guardar" (envía el `<form>` de `UserForm` vía el atributo `form`, ver `USER_ADMIN_FORM_ID`),
// "Gestionar roles" y "Restablecer contraseña" son botones sueltos; bloqueo/activación y
// concesión/revocación de admin (este último sólo si `can_toggle_admin`) van cada uno en su propio
// `Form`, con un campo `Hidden` para el nuevo valor, para conservar el envío nativo sin JavaScript,
// mejorado con `hx-post`/`hx-confirm`. Todos son hijos directos del mismo `Container` con Flex, que
// los alinea en fila con espaciado uniforme sin que ninguno tenga que ser forzosamente un `Button`
// suelto (ver PAGETOP.md, "Preferir componentes a `html!` en bruto").
fn edit_actions(
    user_id: i32,
    status: UserStatus,
    target_is_admin: bool,
    can_toggle_admin: bool,
    waypoint: &Waypoint,
    cx: &mut Context,
) -> Container {
    let (next_status, label_key) = match status {
        UserStatus::Blocked => ("active", "btn-activate"),
        _ => ("blocked", "btn-block"),
    };
    let (next_is_admin, admin_label_key, admin_confirm_key) = if target_is_admin {
        ("false", "btn-revoke-admin", "confirm-revoke-admin")
    } else {
        ("true", "btn-grant-admin", "confirm-grant-admin")
    };

    let roles_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{user_id}/roles")));
    let password_href =
        waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{user_id}/password")));
    let status_action =
        waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{user_id}/status")));
    let admin_action = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{user_id}/admin")));

    let mut status_form = Form::new()
        .with_action(status_action.clone())
        .with_method(form::Method::Post)
        .with_prop(PropsOp::set(hx::POST, status_action.to_string()))
        .with_child(form::Hidden::field("status", next_status))
        .with_child(
            Button::submit(Lc::t(label_key, &LOCALES_USER))
                .with_style(button::Style::Solid(Intent::Warning)),
        );
    if let Some(confirm) = Lc::t("confirm-change-status", &LOCALES_USER).lookup(cx) {
        status_form = status_form.with_prop(PropsOp::set(hx::CONFIRM, confirm));
    }

    let mut container = Container::new()
        .with_flex(
            Flex::row()
                .with_wrap(flex::Behavior::Wrap)
                .with_align(flex::Align::Center)
                .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
        )
        .with_child(
            Button::submit(Lc::t("btn-save", &LOCALES_USER))
                .with_style(button::Style::Solid(Intent::Primary))
                .with_prop(PropsOp::set("form", USER_ADMIN_FORM_ID)),
        )
        .with_child(
            Button::anchor(Lc::t("btn-manage-roles", &LOCALES_USER), roles_href)
                .with_style(button::Style::Solid(Intent::Neutral)),
        )
        .with_child(
            Button::anchor(Lc::t("btn-reset-password", &LOCALES_USER), password_href)
                .with_style(button::Style::Solid(Intent::Neutral)),
        )
        .with_child(status_form);

    if can_toggle_admin {
        let mut admin_form = Form::new()
            .with_action(admin_action.clone())
            .with_method(form::Method::Post)
            .with_prop(PropsOp::set(hx::POST, admin_action.to_string()))
            .with_child(form::Hidden::field("is_admin", next_is_admin))
            .with_child(Button::submit(Lc::t(admin_label_key, &LOCALES_USER)));
        if let Some(confirm) = Lc::t(admin_confirm_key, &LOCALES_USER).lookup(cx) {
            admin_form = admin_form.with_prop(PropsOp::set(hx::CONFIRM, confirm));
        }
        container = container.with_child(admin_form);
    }

    container
}

/// GET /admin/user/users/{id}/edit - Formulario de edición de usuario.
pub(crate) async fn edit_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    Ok(render_user_edit(request, id, None, waypoint).await)
}

#[derive(Deserialize)]
pub(crate) struct EditUserFormData {
    username: String,
    email: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    timezone: String,
}

/// POST /admin/user/users/{id}/edit - Actualiza los datos de perfil de un usuario.
pub(crate) async fn edit_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<EditUserFormData>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;

    let result = user_admin::update_user(
        id,
        user_admin::UserUpdateData {
            username: &form.username,
            email: &form.email,
            display_name: util::non_blank(&form.display_name),
            language: util::non_blank(&form.language),
            timezone: util::non_blank(&form.timezone),
        },
    )
    .await;

    match result {
        Ok(()) => {
            let cx = Context::admin(request);
            let target = waypoint.or(cx.route(ADMIN_USERS_PATH));
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let mut page = Page::admin(request);
            let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));
            let form_component = UserForm::new()
                .with_mode(UserFormMode::Edit)
                .with_user_id(Some(id))
                .with_username(form.username)
                .with_email(form.email)
                .with_display_name(form.display_name)
                .with_language(form.language)
                .with_timezone(form.timezone)
                .with_error(map_auth_error(&err))
                .with_waypoint(waypoint);
            let title = Lc::t("title-admin-user-edit", &LOCALES_USER);
            Ok(page
                .with_title(title.clone())
                .with_child(
                    frame(title)
                        .with_child(form_component)
                        .with_child(back_link(back_href)),
                )
                .render()
                .await
                .into_response())
        }
    }
}

// **< view_get >***********************************************************************************

/// GET /admin/user/users/{id}/view - Pantalla de sólo lectura con todos los datos del usuario,
/// incluidos los roles asignados.
pub(crate) async fn view_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;

    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return Err(ErrorPage::NotFound(Some(request))),
    };
    let roles = match user_admin::user_roles(id).await {
        Ok(roles) => roles,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };
    let status = UserStatus::from_i16(user.status);

    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));
    let details_block = user_view_details(&user, status, page.context()).await;
    let roles_block = user_view_roles(&roles, page.context()).await;

    let title = Lc::t("title-admin-user-view", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(details_block)
                .with_child(roles_block)
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response())
}

// Bloque de sólo lectura con los datos de perfil del usuario.
async fn user_view_details(user: &user::Model, status: UserStatus, cx: &mut Context) -> Block {
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

// Bloque de sólo lectura con los roles asignados al usuario, cada uno enlazado a su propia
// pantalla de vista.
async fn user_view_roles(roles: &[role::Model], cx: &mut Context) -> Block {
    let mut items: Vec<(i32, String, String, Option<Markup>)> = Vec::with_capacity(roles.len());
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
        items.push((r.id, r.machine_name.clone(), r.label.clone(), system_badge));
    }

    Block::new()
        .with_title(Lc::t("field-roles", &LOCALES_USER))
        .with_child(Html::with(move |cx| {
            html! {
                @if items.is_empty() {
                    "-"
                } @else {
                    table.user-admin-table {
                        tbody {
                            @for (id, machine_name, label, system_badge) in &items {
                                tr {
                                    td {
                                        a href=(cx.route(format!("{ADMIN_ROLES_PATH}/{id}/view")).to_string()) {
                                            (label.as_str())
                                        }
                                    }
                                    td { (machine_name.as_str()) }
                                    td {
                                        @if let Some(badge) = system_badge { (badge) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }))
}

// **< roles_get / roles_post >**********************************************************************

/// GET /admin/user/users/{id}/roles - Formulario de asignación de roles de un usuario.
pub(crate) async fn roles_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    require_permission(&request, &UserPermission::AssignRoles)?;

    if user_admin::find_user(id).await.is_err() {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    let current = match user_admin::user_role_ids(id).await {
        Ok(ids) => ids,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };
    let roles = match available_roles(&current).await {
        Ok(roles) => roles,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };

    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));

    let title = Lc::t("title-admin-user-roles", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    UserRolesForm::new()
                        .with_user_id(id)
                        .with_roles(roles)
                        .with_waypoint(waypoint),
                )
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct UserRolesFormData {
    #[serde(default)]
    role_ids: Vec<String>,
}

/// POST /admin/user/users/{id}/roles - Reemplaza el conjunto de roles asignados a un usuario.
pub(crate) async fn roles_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    raw: web::RawForm,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    require_permission(&request, &UserPermission::AssignRoles)?;

    let Ok(form) = serde_qs::from_bytes::<UserRolesFormData>(&raw.0) else {
        return Err(ErrorPage::BadRequest(Some(request)));
    };

    let role_ids: Vec<i32> = form
        .role_ids
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    match user_admin::set_user_roles(id, &role_ids).await {
        Ok(()) => {
            let cx = Context::admin(request);
            let target = waypoint.or(cx.route(ADMIN_USERS_PATH));
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let roles = available_roles(&role_ids).await.unwrap_or_default();
            let mut page = Page::admin(request);
            let back_href = waypoint.or(page.context().route(ADMIN_USERS_PATH));
            let form_component = UserRolesForm::new()
                .with_user_id(id)
                .with_roles(roles)
                .with_error(map_auth_error(&err))
                .with_waypoint(waypoint);
            let title = Lc::t("title-admin-user-roles", &LOCALES_USER);
            Ok(page
                .with_title(title.clone())
                .with_child(
                    frame(title)
                        .with_child(form_component)
                        .with_child(back_link(back_href)),
                )
                .render()
                .await
                .into_response())
        }
    }
}

// **< status_post >********************************************************************************

#[derive(Deserialize)]
pub(crate) struct StatusFormData {
    status: String,
}

/// POST /admin/user/users/{id}/status - Bloquea o activa una cuenta.
pub(crate) async fn status_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<StatusFormData>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    require_permission(&request, &UserPermission::BlockAccounts)?;
    let Some(account) = request.extension::<Account>().cloned() else {
        return Err(ErrorPage::AccessDenied(Some(request)));
    };

    let new_status = match form.status.as_str() {
        "blocked" => UserStatus::Blocked,
        _ => UserStatus::Active,
    };
    // Ver comentario equivalente en `admin_post`: el botón envía la petición vía `hx-post` (para el
    // diálogo `hx-confirm`) sin `hx-target`, así que un `Redirect` normal terminaría anidado dentro
    // del propio `<form>`. `HtmxResponse::redirect()` fuerza una navegación real en el cliente.
    let is_htmx = request.is_htmx();

    match user_admin::set_user_status(id, new_status, account.id).await {
        Ok(()) => {
            let cx = Context::admin(request);
            let edit_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{id}/edit")));
            if is_htmx {
                Ok(HtmxResponse::empty().redirect(edit_href).into_response())
            } else {
                Ok(Redirect::see_other(edit_href).into_response())
            }
        }
        Err(err) => Ok(render_user_edit(request, id, Some(map_auth_error(&err)), waypoint).await),
    }
}

// **< admin_post >*********************************************************************************

#[derive(Deserialize)]
pub(crate) struct AdminFormData {
    is_admin: String,
}

/// POST /admin/user/users/{id}/admin - Concede o revoca el acceso irrestricto (`is_admin`).
///
/// No pasa por `require_permission`: conceder o revocar este flag no es un permiso del catálogo,
/// se comprueba directamente contra `account.is_admin` para que sólo un administrador pueda
/// tocarlo (un permiso concedido vía rol nunca basta).
pub(crate) async fn admin_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<AdminFormData>,
) -> Result<Response, ErrorPage> {
    let Some(account) = request.extension::<Account>().cloned() else {
        return Err(ErrorPage::AccessDenied(Some(request)));
    };
    if !account.is_admin {
        return Err(ErrorPage::AccessDenied(Some(request)));
    }

    let new_is_admin = form.is_admin == "true";
    // El botón envía la petición vía `hx-post` (para el diálogo `hx-confirm`), sin `hx-target`: si
    // se responde con un `Redirect` normal, HTMX sigue la redirección con `fetch` y sustituye el
    // propio `<form>` (destino por defecto sin `hx-target`) por la página completa que devuelve,
    // anidándola dentro de sí misma. `HtmxResponse::redirect()` evita eso: instruye al cliente
    // (cabecera `HX-Redirect`) para que navegue de verdad a la URL, en vez de intentar un `swap`.
    let is_htmx = request.is_htmx();

    match user_admin::set_user_admin(id, new_is_admin, account.id).await {
        Ok(()) => {
            let cx = Context::admin(request);
            let edit_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{id}/edit")));
            if is_htmx {
                Ok(HtmxResponse::empty().redirect(edit_href).into_response())
            } else {
                Ok(Redirect::see_other(edit_href).into_response())
            }
        }
        Err(err) => Ok(render_user_edit(request, id, Some(map_auth_error(&err)), waypoint).await),
    }
}

// **< password_get / password_post >****************************************************************

/// GET /admin/user/users/{id}/password - Formulario de restablecimiento de contraseña por un
/// administrador.
pub(crate) async fn password_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;
    if user_admin::find_user(id).await.is_err() {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    let mut page = Page::admin(request);
    let edit_href = waypoint.append_to(
        page.context()
            .route(format!("{ADMIN_USERS_PATH}/{id}/edit")),
    );
    let title = Lc::t("title-admin-user-password", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    AdminPasswordForm::new()
                        .with_user_id(id)
                        .with_waypoint(waypoint),
                )
                .with_child(back_link(edit_href)),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct AdminPasswordFormData {
    password: String,
    confirm_password: String,
}

/// POST /admin/user/users/{id}/password - Aplica la nueva contraseña e invalida las sesiones
/// activas del usuario.
pub(crate) async fn password_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<AdminPasswordFormData>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminUsers)?;

    let result = match password::passwords_match(&form.password, &form.confirm_password) {
        Ok(()) => user_admin::admin_reset_password(id, &form.password).await,
        Err(err) => Err(err),
    };

    match result {
        Ok(()) => {
            let cx = Context::admin(request);
            let edit_href = waypoint.append_to(cx.route(format!("{ADMIN_USERS_PATH}/{id}/edit")));
            Ok(Redirect::see_other(edit_href).into_response())
        }
        Err(err) => {
            let mut page = Page::admin(request);
            let edit_href = waypoint.append_to(
                page.context()
                    .route(format!("{ADMIN_USERS_PATH}/{id}/edit")),
            );
            let title = Lc::t("title-admin-user-password", &LOCALES_USER);
            Ok(page
                .with_title(title.clone())
                .with_child(
                    frame(title)
                        .with_child(
                            AdminPasswordForm::new()
                                .with_user_id(id)
                                .with_error(map_auth_error(&err))
                                .with_waypoint(waypoint),
                        )
                        .with_child(back_link(edit_href)),
                )
                .render()
                .await
                .into_response())
        }
    }
}
