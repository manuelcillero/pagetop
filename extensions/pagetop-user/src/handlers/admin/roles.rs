//! Handlers de administración de roles.

use serde::Deserialize;

use pagetop::base::component::table::Row;
use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

use crate::ADMIN_ROLES_PATH;
use crate::LOCALES_USER;
use crate::component::admin::{
    PermissionGroups, RoleForm, RoleFormMode, RolePermissionsForm, RoleTable,
};
use crate::config::SETTINGS;
use crate::entity::role;
use crate::handlers::admin::{back_link, frame, map_auth_error};
use crate::permission::{self, UserPermission};
use crate::service::role_admin::{self, RolePageParams, RoleSortField};

#[derive(Deserialize)]
pub(crate) struct RolesQuery {
    #[serde(default)]
    sort: Option<String>,
    #[serde(default)]
    dir: Option<String>,
    #[serde(default)]
    page: Option<u64>,
}

// **< list_get >***********************************************************************************

/// GET /admin/user/roles - Listado de roles (orden y paginación vía HTMX).
pub(crate) async fn list_get(
    request: HttpRequest,
    web::Query(query): web::Query<RolesQuery>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let params = RolePageParams {
        sort: RoleSortField::from_query(query.sort.as_deref()),
        dir: SortDir::from_query(query.dir.as_deref()),
        page: query.page.unwrap_or(1).max(1),
        per_page: SETTINGS.admin.list_page_size,
    };

    let result = match role_admin::list_roles_page(&params).await {
        Ok(result) => result,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };

    let mut table = RoleTable::new()
        .with_items(result.items)
        .with_sort(params.sort)
        .with_dir(params.dir)
        .with_page(result.page)
        .with_per_page(result.per_page)
        .with_total(result.total);

    if request.is_htmx() {
        let mut cx = Context::admin(request);
        Ok(HtmxResponse::new(table.render(&mut cx).await).into_response())
    } else {
        let title = Lc::t("title-admin-roles", &LOCALES_USER);
        Ok(Page::admin(request)
            .with_title(title.clone())
            .with_child(frame(title).with_child(table))
            .render()
            .await
            .into_response())
    }
}

// **< new_get / new_post >*************************************************************************

/// GET /admin/user/roles/new - Formulario de alta de rol.
pub(crate) async fn new_get(
    request: HttpRequest,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;
    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));
    let title = Lc::t("title-admin-role-new", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    RoleForm::new()
                        .with_mode(RoleFormMode::New)
                        .with_waypoint(waypoint),
                )
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct NewRoleFormData {
    machine_name: String,
    label: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    weight: String,
}

/// POST /admin/user/roles/new - Crea un rol.
pub(crate) async fn new_post(
    request: HttpRequest,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<NewRoleFormData>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let weight: i32 = form.weight.trim().parse().unwrap_or(0);

    let result = role_admin::create_role(role_admin::NewRoleData {
        machine_name: form.machine_name.trim(),
        label: &form.label,
        description: util::non_blank(&form.description),
        weight,
    })
    .await;

    match result {
        Ok(_) => {
            let cx = Context::admin(request);
            let target = waypoint.or(cx.route(ADMIN_ROLES_PATH));
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let mut page = Page::admin(request);
            let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));
            let form_component = RoleForm::new()
                .with_mode(RoleFormMode::New)
                .with_machine_name(form.machine_name)
                .with_label(form.label)
                .with_description(form.description)
                .with_weight(weight)
                .with_error(map_auth_error(&err))
                .with_waypoint(waypoint);
            let title = Lc::t("title-admin-role-new", &LOCALES_USER);
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

/// GET /admin/user/roles/{id}/edit - Formulario de edición de rol.
pub(crate) async fn edit_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let role = match role_admin::find_role(id).await {
        Ok(role) => role,
        Err(_) => return Err(ErrorPage::NotFound(Some(request))),
    };
    if role.locked {
        return Err(ErrorPage::AccessDenied(Some(request)));
    }

    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));

    let title = Lc::t("title-admin-role-edit", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    RoleForm::new()
                        .with_mode(RoleFormMode::Edit)
                        .with_role_id(Some(id))
                        .with_machine_name(role.machine_name)
                        .with_label(role.label)
                        .with_description(role.description.unwrap_or_default())
                        .with_weight(role.weight)
                        .with_waypoint(waypoint),
                )
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct EditRoleFormData {
    label: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    weight: String,
}

/// POST /admin/user/roles/{id}/edit - Actualiza un rol (rechazado si está bloqueado).
pub(crate) async fn edit_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<EditRoleFormData>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let weight: i32 = form.weight.trim().parse().unwrap_or(0);

    let result = role_admin::update_role(
        id,
        role_admin::RoleUpdateData {
            label: &form.label,
            description: util::non_blank(&form.description),
            weight,
        },
    )
    .await;

    match result {
        Ok(()) => {
            let cx = Context::admin(request);
            let target = waypoint.or(cx.route(ADMIN_ROLES_PATH));
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let machine_name = role_admin::find_role(id)
                .await
                .map(|role| role.machine_name)
                .unwrap_or_default();
            let mut page = Page::admin(request);
            let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));
            let form_component = RoleForm::new()
                .with_mode(RoleFormMode::Edit)
                .with_role_id(Some(id))
                .with_machine_name(machine_name)
                .with_label(form.label)
                .with_description(form.description)
                .with_weight(weight)
                .with_error(map_auth_error(&err))
                .with_waypoint(waypoint);
            let title = Lc::t("title-admin-role-edit", &LOCALES_USER);
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

/// GET /admin/user/roles/{id}/view - Pantalla de sólo lectura con los datos del rol y todos los
/// permisos del catálogo, marcando los que tiene concedidos.
pub(crate) async fn view_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let role = match role_admin::find_role(id).await {
        Ok(role) => role,
        Err(_) => return Err(ErrorPage::NotFound(Some(request))),
    };
    let selected = match role_admin::role_permission_keys(id).await {
        Ok(keys) => keys,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };
    let groups = build_permission_groups(&selected);

    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));
    let details_block = role_view_details(&role, page.context()).await;

    let title = Lc::t("title-admin-role-view", &LOCALES_USER);
    let mut content = frame(title.clone()).with_child(details_block);
    for group_block in role_view_permissions(&groups) {
        content = content.with_child(group_block);
    }
    content = content.with_child(back_link(back_href));

    Ok(page
        .with_title(title)
        .with_child(content)
        .render()
        .await
        .into_response())
}

// Bloque de sólo lectura con los datos del rol.
async fn role_view_details(role: &role::Model, cx: &mut Context) -> Block {
    let mut table = Table::new()
        .with_prop(PropsOp::add_classes("user-admin-table"))
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-machine-name", &LOCALES_USER))
                .with_cell(role.machine_name.as_str()),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-label", &LOCALES_USER))
                .with_cell(role.label.as_str()),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-description", &LOCALES_USER))
                .with_cell(role.description.as_deref().unwrap_or("-")),
        )
        .with_row(
            Row::new()
                .with_cell(Lc::t("field-weight", &LOCALES_USER))
                .with_cell(role.weight.to_string()),
        );

    if role.locked {
        let badge = Badge::labeled(Lc::t("badge-system-role", &LOCALES_USER))
            .with_prop(PropsOp::add_classes("user-admin-badge-system"))
            .render(cx)
            .await;
        table = table.with_row(
            Row::new()
                .with_cell("")
                .with_cell(Html::with(move |_| badge.clone())),
        );
    }

    Block::new()
        .with_title(Lc::t("title-role-details", &LOCALES_USER))
        .with_child(table)
}

// Un bloque por grupo del catálogo de permisos: cada permiso concedido se marca con la clase
// `user-admin-permission-granted` (negrita, vía CSS del tema); el resto con
// `user-admin-permission-missing` (gris claro, vía CSS del tema).
fn role_view_permissions(groups: &PermissionGroups) -> Vec<Block> {
    groups
        .iter()
        .map(|(group_label, perms)| {
            let perms = perms.clone();
            Block::new()
                .with_title(group_label.clone())
                .with_child(Html::with(move |cx| {
                    html! {
                        ul.user-admin-permission-list {
                            @for (_key, label, granted) in &perms {
                                @if *granted {
                                    li.user-admin-permission-granted { (label.using(cx)) }
                                } @else {
                                    li.user-admin-permission-missing { (label.using(cx)) }
                                }
                            }
                        }
                    }
                }))
        })
        .collect()
}

// **< delete_post >********************************************************************************

/// POST /admin/user/roles/{id}/delete - Elimina un rol (rechazado si está bloqueado o en uso).
///
/// Vuelve a mostrar la página/orden indicados en `sort`/`dir`/`page` (propagados desde la fila que
/// inició el borrado, ver `delete_confirm_get`), tanto si el borrado tiene éxito como si falla, en
/// vez de reiniciar siempre a la primera página.
pub(crate) async fn delete_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(query): web::Query<RolesQuery>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let result = role_admin::delete_role(id).await;

    if request.is_htmx() {
        let params = RolePageParams {
            sort: RoleSortField::from_query(query.sort.as_deref()),
            dir: SortDir::from_query(query.dir.as_deref()),
            page: query.page.unwrap_or(1).max(1),
            per_page: SETTINGS.admin.list_page_size,
        };
        let page_result = role_admin::list_roles_page(&params)
            .await
            .unwrap_or_default();
        let message = result.err().as_ref().map(map_auth_error);
        let mut table = RoleTable::new()
            .with_items(page_result.items)
            .with_sort(params.sort)
            .with_dir(params.dir)
            .with_page(page_result.page)
            .with_per_page(page_result.per_page)
            .with_total(page_result.total)
            .with_message(message);
        let mut cx = Context::new(request);
        Ok(HtmxResponse::new(table.render(&mut cx).await).into_response())
    } else {
        let cx = Context::new(request);
        match result {
            Ok(()) => {
                let target = cx
                    .route(ADMIN_ROLES_PATH)
                    .alter_param(
                        "sort",
                        RoleSortField::from_query(query.sort.as_deref()).as_str(),
                    )
                    .alter_param("dir", SortDir::from_query(query.dir.as_deref()))
                    .alter_param("page", query.page.unwrap_or(1).max(1).to_string())
                    .to_string();
                Ok(Redirect::see_other(target).into_response())
            }
            Err(_) => Err(ErrorPage::BadRequest(cx.request().cloned())),
        }
    }
}

// **< delete_confirm_get >*************************************************************************

/// GET /admin/user/roles/{id}/delete/confirm - Botón de confirmación de borrado para esta fila,
/// insertado vía htmx en el diálogo de confirmación compartido por toda la tabla (ver
/// `RoleTable`), con la URL de borrado de este rol ya incrustada.
///
/// Propaga `sort`/`dir`/`page` (recibidos como query string desde la propia fila) hacia la URL de
/// borrado, para que `delete_post` pueda volver a mostrar la misma página tras el borrado, en vez
/// de reiniciar siempre al listado por defecto.
pub(crate) async fn delete_confirm_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(query): web::Query<RolesQuery>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;

    let mut cx = Context::admin(request);
    let delete_href = cx
        .route(format!("{ADMIN_ROLES_PATH}/{id}/delete"))
        .alter_param(
            "sort",
            RoleSortField::from_query(query.sort.as_deref()).as_str(),
        )
        .alter_param("dir", SortDir::from_query(query.dir.as_deref()))
        .alter_param("page", query.page.unwrap_or(1).max(1).to_string())
        .to_string();

    let mut button = Button::plain(Lc::t("btn-delete", &LOCALES_USER))
        .with_prop(PropsOp::set(hx::POST, delete_href))
        .with_prop(PropsOp::set(hx::TARGET, "#role-table-wrapper"))
        .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP))
        .with_prop(PropsOp::set("data-dialog-dismiss", "modal"));

    Ok(HtmxResponse::new(button.render(&mut cx).await).into_response())
}

// **< permissions_get / permissions_post >**********************************************************

fn build_permission_groups(selected: &[String]) -> PermissionGroups {
    let registry = permission::registry();
    registry
        .groups()
        .iter()
        .map(|(group, group_label)| {
            let perms = registry
                .by_group(group)
                .map(|permission| {
                    let key = permission.key();
                    let checked = selected.iter().any(|k| k.as_str() == key);
                    (key, permission.label(), checked)
                })
                .collect();
            (group_label.clone(), perms)
        })
        .collect()
}

/// GET /admin/user/roles/{id}/permissions - Formulario de asignación de permisos de un rol.
/// Permitido aunque el rol esté bloqueado: los roles de sistema también necesitan permisos.
pub(crate) async fn permissions_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;
    require_permission(&request, &UserPermission::AdminPermissions)?;

    if role_admin::find_role(id).await.is_err() {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    let selected = match role_admin::role_permission_keys(id).await {
        Ok(keys) => keys,
        Err(_) => return Err(ErrorPage::InternalError(Some(request))),
    };
    let groups = build_permission_groups(&selected);

    let mut page = Page::admin(request);
    let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));

    let title = Lc::t("title-admin-role-permissions", &LOCALES_USER);
    Ok(page
        .with_title(title.clone())
        .with_child(
            frame(title)
                .with_child(
                    RolePermissionsForm::new()
                        .with_role_id(id)
                        .with_groups(groups)
                        .with_waypoint(waypoint),
                )
                .with_child(back_link(back_href)),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct RolePermissionsFormData {
    #[serde(default)]
    permission_keys: Vec<String>,
}

/// POST /admin/user/roles/{id}/permissions - Reemplaza el conjunto de permisos de un rol.
///
/// Usa `RawForm` + `serde_qs` en lugar de `axum::extract::Form` (basado en `serde_urlencoded`,
/// que no deserializa claves repetidas como `permission_keys=a&permission_keys=b` en un `Vec<T>`).
pub(crate) async fn permissions_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Query(waypoint): web::Query<Waypoint>,
    raw: web::RawForm,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::AdminRoles)?;
    require_permission(&request, &UserPermission::AdminPermissions)?;

    let Ok(form) = serde_qs::from_bytes::<RolePermissionsFormData>(&raw.0) else {
        return Err(ErrorPage::BadRequest(Some(request)));
    };

    match role_admin::set_role_permissions(id, &form.permission_keys).await {
        Ok(()) => {
            let cx = Context::admin(request);
            let target = waypoint.or(cx.route(ADMIN_ROLES_PATH));
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let groups = build_permission_groups(&form.permission_keys);
            let mut page = Page::admin(request);
            let back_href = waypoint.or(page.context().route(ADMIN_ROLES_PATH));
            let form_component = RolePermissionsForm::new()
                .with_role_id(id)
                .with_groups(groups)
                .with_error(map_auth_error(&err))
                .with_waypoint(waypoint);
            let title = Lc::t("title-admin-role-permissions", &LOCALES_USER);
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
