//! Handlers de administración de usuarios.

use serde::Deserialize;

use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

use crate::ADMIN_USERS_PATH;
use crate::LOCALES_USER;
use crate::account::{Account, UserStatus};
use crate::actions::AdminUserFormExtras;
use crate::component::admin::{
    AdminPasswordForm, USER_ADMIN_FORM_ID, UserForm, UserFormMode, UserTable,
};
use crate::component::{cancel_link, user_details};
use crate::config::SETTINGS;
use crate::entity::{role, user};
use crate::error::AuthError;
use crate::handlers::account::managed_or;
use crate::handlers::admin::{back_link, map_auth_error};
use crate::mail::{Notice, send_notice};
use crate::password;
use crate::permission::UserPermission;
use crate::provider::{self, UserField};
use crate::role_path;
use crate::service::role_admin;
use crate::service::user_admin::{self, UserListParams, UserSortField};
use crate::user_path;

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
    let params = UserListParams {
        query: query.q.clone(),
        sort: UserSortField::from_query(query.sort.as_deref()),
        dir: SortDir::from_query(query.dir.as_deref()),
        page: query.page.unwrap_or(1).max(1),
        per_page: SETTINGS.admin.list_page_size,
    };

    let result = match user_admin::list_users(&params).await {
        Ok(result) => result,
        Err(_) => return Err(ErrorPage::InternalError(request)),
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
            .with_title(title)
            .with_child(search_filter(query.q))
            .with_child(table)
            .render()
            .await
            .into_response())
    }
}

// **< search_filter >******************************************************************************

fn search_filter(current_query: Option<String>) -> ListFilter {
    hx_filter::boost(
        ListFilter::new()
            .with_filter_action(ADMIN_USERS_PATH.as_str())
            .with_field(
                form::input::Field::search()
                    .with_id("user-admin-search")
                    .with_name("q")
                    .with_value(current_query.unwrap_or_default())
                    .with_placeholder(Lc::t("field-search-users", &LOCALES_USER)),
            ),
        "#user-table-wrapper",
    )
}

// **< available_roles >****************************************************************************

// Roles asignables desde la UI de usuarios: todos salvo los de sistema, que nunca se asignan.
async fn available_roles(selected: &[i32]) -> Result<Vec<(i32, String, bool)>, AuthError> {
    let items = role_admin::list_roles(&role_admin::RoleListParams {
        sort: role_admin::RoleSortField::Weight,
        dir: SortDir::Asc,
    })
    .await?;
    Ok(items
        .into_iter()
        .filter(|r| !crate::is_system_role(r.id))
        .map(|r| {
            let checked = selected.contains(&r.id);
            (r.id, r.label, checked)
        })
        .collect())
}

// Identificadores de rol enviados en el formulario de alta o edición.
fn parse_role_ids(role_ids: &[String]) -> Vec<i32> {
    role_ids.iter().filter_map(|s| s.parse().ok()).collect()
}

// **< require_manageable >*************************************************************************

// Carga el usuario y comprueba que quien hace la petición puede gestionarlo. La cuenta de un
// administrador sólo la gestiona otro administrador: si no, quien administra usuarios podría
// apoderarse de ella (p. ej. restableciendo su contraseña) y obtener acceso total, que sólo
// un administrador puede conceder.
async fn require_manageable(request: &HttpRequest, id: i32) -> Result<user::Model, ErrorPage> {
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(request.clone()));
    };
    if user.is_admin && !request.extension::<Account>().is_some_and(|a| a.is_admin()) {
        return Err(ErrorPage::AccessDenied(request.clone()));
    }
    Ok(user)
}

// **< new_get / new_post >*************************************************************************

#[derive(Deserialize)]
pub(crate) struct NewUserQuery {
    #[serde(default)]
    provider: Option<String>,
}

// Proveedor activo con el que entrará la cuenta que se da de alta, como `(id, nombre)`.
fn external_provider(id: Option<&str>, cx: &Context) -> Option<(String, String)> {
    let provider = provider::active_provider(id?)?;
    let name = provider.label().lookup(cx).unwrap_or_default();
    Some((provider.id().to_owned(), name))
}

/// GET /admin/user/users/new - Formulario de alta de usuario.
///
/// Con `?provider={id}`, el alta es de una cuenta que entrará con ese proveedor de autenticación:
/// sin contraseña local y marcada para que el proveedor la vincule cuando la persona inicie sesión
/// con el mismo nombre de usuario (ver `provider::allow_linking()`).
pub(crate) async fn new_get(
    request: HttpRequest,
    web::Query(query): web::Query<NewUserQuery>,
) -> Result<Response, ErrorPage> {
    let roles = match available_roles(&[]).await {
        Ok(roles) => roles,
        Err(_) => return Err(ErrorPage::InternalError(request)),
    };
    // El campo "administrador" sólo se ofrece si quien da de alta ya es administrador: no es un
    // permiso del catálogo (igual que conceder/revocar en la edición, ver `set_user_admin`). Una
    // cuenta de proveedor nunca puede serlo.
    let viewer_is_admin = request.extension::<Account>().is_some_and(|a| a.is_admin());
    let page = Page::admin(request);
    let provider = external_provider(query.provider.as_deref(), page.context());
    let allow_admin_field = provider.is_none() && viewer_is_admin;
    let title = Lc::t("title-admin-user-new", &LOCALES_USER);
    Ok(page
        .with_title(title)
        .with_child(
            UserForm::new()
                .with_mode(UserFormMode::New)
                .with_roles(roles)
                .with_allow_admin_field(allow_admin_field)
                .with_provider(provider),
        )
        .render()
        .await
        .into_response())
}

#[derive(Deserialize)]
pub(crate) struct NewUserFormData {
    username: String,
    email: String,
    #[serde(default)]
    password: util::SecretStr,
    #[serde(default)]
    confirm_password: util::SecretStr,
    #[serde(default)]
    provider: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    timezone: String,
    #[serde(default)]
    theme: String,
    #[serde(default)]
    role_ids: Vec<String>,
    #[serde(default)]
    is_admin: bool,
}

/// POST /admin/user/users/new - Crea un usuario.
///
/// Usa `web::RawForm` + `serde_qs` en lugar de `web::Form` (basado en `serde_urlencoded`, que no
/// deserializa claves repetidas como `role_ids=2&role_ids=3` en un `Vec<T>`).
pub(crate) async fn new_post(
    request: HttpRequest,
    raw: web::RawForm,
) -> Result<Response, ErrorPage> {
    let Ok(form) = serde_qs::from_bytes::<NewUserFormData>(&raw.0) else {
        return Err(ErrorPage::BadRequest(request));
    };
    let cx = Context::admin(request.clone());
    let provider = match util::non_blank(&form.provider) {
        Some(id) => match external_provider(Some(id), &cx) {
            Some(provider) => Some(provider),
            None => return Err(ErrorPage::BadRequest(request)),
        },
        None => None,
    };
    let allow_admin_field =
        provider.is_none() && request.extension::<Account>().is_some_and(|a| a.is_admin());
    // Nunca fiarse sólo de que el campo esté presente en el formulario: sólo se concede si quien
    // envía la petición ya es administrador, aunque alguien manipulase la petición a mano.
    let is_admin = form.is_admin && allow_admin_field;
    let role_ids = parse_role_ids(&form.role_ids);

    let password = provider
        .is_none()
        .then(|| (form.password.expose(), form.confirm_password.expose()));
    let result = user_admin::create_user(user_admin::NewUserData {
        username: &form.username,
        email: &form.email,
        password,
        link_provider: provider.as_ref().map(|(id, _)| id.as_str()),
        display_name: util::non_blank(&form.display_name),
        language: util::non_blank(&form.language),
        timezone: util::non_blank(&form.timezone),
        theme: util::non_blank(&form.theme),
        initial_role_ids: &role_ids,
        is_admin,
    })
    .await;

    match result {
        Ok(_) => {
            let target = cx.route(ADMIN_USERS_PATH.as_str());
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let roles = available_roles(&role_ids).await.unwrap_or_default();
            let page = Page::admin(request);
            let form_component = UserForm::new()
                .with_mode(UserFormMode::New)
                .with_provider(provider)
                .with_username(form.username)
                .with_email(form.email)
                .with_display_name(form.display_name)
                .with_language(form.language)
                .with_timezone(form.timezone)
                .with_theme(form.theme)
                .with_roles(roles)
                .with_allow_admin_field(allow_admin_field)
                .with_is_admin(is_admin)
                .with_error(map_auth_error(&err));
            let title = Lc::t("title-admin-user-new", &LOCALES_USER);
            Ok(page
                .with_title(title)
                .with_child(form_component)
                .render()
                .await
                .into_response())
        }
    }
}

// **< edit_get / edit_post >***********************************************************************

// Pantalla de edición con el formulario relleno con los datos guardados del usuario.
async fn render_user_edit(request: HttpRequest, id: i32, error: Option<Lc>) -> Response {
    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return ErrorPage::NotFound(request).into_response(),
    };
    let current = match user_admin::user_role_ids(id).await {
        Ok(ids) => ids,
        Err(_) => return ErrorPage::InternalError(request).into_response(),
    };
    let roles = match available_roles(&current).await {
        Ok(roles) => roles,
        Err(_) => return ErrorPage::InternalError(request).into_response(),
    };
    let policy = provider::account_policy(&user).await;
    let status = UserStatus::from_i16(user.status);
    let is_admin = user.is_admin;
    let form = UserForm::new()
        .with_policy(policy)
        .with_username(user.username)
        .with_email(user.email)
        .with_display_name(user.display_name.unwrap_or_default())
        .with_about(user.about.unwrap_or_default())
        .with_language(user.language.unwrap_or_default())
        .with_timezone(user.timezone.unwrap_or_default())
        .with_theme(user.theme.unwrap_or_default())
        .with_roles(roles)
        .with_error(error);
    render_edit_page(request, id, status, is_admin, form).await
}

// Pantalla de edición del usuario `id` con `form` ya relleno, sea con los datos guardados o con los
// enviados que no se pudieron guardar. Siempre lleva la botonera de `edit_actions()`, porque en
// modo `Edit` el botón "Guardar" está en ella y no dentro del formulario, y lo que añadan otras
// extensiones con `AdminUserFormExtras`.
async fn render_edit_page(
    request: HttpRequest,
    id: i32,
    status: UserStatus,
    is_admin: bool,
    form: UserForm,
) -> Response {
    let can_reset_password = form
        .policy()
        .is_none_or(|policy| policy.password().is_editable());
    let linking = linking_options(id, is_admin).await;
    // El botón de conceder/revocar sólo se muestra si quien lo ve ya es administrador y no está
    // viendo su propio perfil: no es un permiso del catálogo, y nadie puede automodificarse el flag
    // (ver `set_user_admin`). Tampoco para conceder el acceso total a una cuenta que gestiona un
    // proveedor, que `set_user_admin` rechaza.
    let can_toggle_admin = request
        .extension::<Account>()
        .is_some_and(|a| a.is_admin() && a.id() != id)
        && (is_admin || form.policy().is_none());
    // Bloquear y activar exige además `user:block_accounts` (ver `status_post`).
    let can_change_status = has_permission(&request, &UserPermission::BlockAccounts);
    let mut page = Page::admin(request);
    let title = Lc::t("title-admin-user-edit", &LOCALES_USER);
    let actions = edit_actions(
        id,
        EditActions {
            status,
            target_is_admin: is_admin,
            can_toggle_admin,
            can_change_status,
            can_reset_password,
            linking,
        },
        page.context_mut(),
    );

    page.with_title(title)
        .with_child(form.with_mode(UserFormMode::Edit).with_user_id(Some(id)))
        .with_child(actions)
        .with_child(ChildOp::AddMany(AdminUserFormExtras::collect(id)))
        .render()
        .await
        .into_response()
}

// Proveedores con los que un administrador puede permitir (o dejar de permitir) que se vincule una
// cuenta: los activos, si la cuenta no es de administrador y no está vinculada ya a ninguno. Cada
// uno con su nombre y si ya está permitido.
async fn linking_options(user_id: i32, is_admin: bool) -> Vec<(&'static str, Lc, bool)> {
    if is_admin || provider::active_providers().next().is_none() {
        return Vec::new();
    }
    let identities = match provider::identities_of(user_id).await {
        Ok(identities) => identities,
        Err(err) => {
            trace::error!("Cannot load the identities of user {user_id}: {err}");
            return Vec::new();
        }
    };
    if identities.iter().any(|identity| !identity.pending) {
        return Vec::new();
    }
    provider::active_providers()
        .map(|p| {
            let pending = identities
                .iter()
                .any(|identity| identity.provider == p.id());
            (p.id(), p.label(), pending)
        })
        .collect()
}

// Estado de la cuenta que decide qué botones lleva la edición.
struct EditActions {
    status: UserStatus,
    target_is_admin: bool,
    can_toggle_admin: bool,
    can_change_status: bool,
    can_reset_password: bool,
    linking: Vec<(&'static str, Lc, bool)>,
}

// Botonera de la edición: "Guardar", "Cancelar", el enlace a la pantalla de restablecer contraseña
// y los botones de bloqueo/activación (sólo si `can_change_status`) y de concesión/revocación de
// acceso total (sólo si `can_toggle_admin`).
//
// "Guardar" (envía el `<form>` de `UserForm` vía el atributo `form`, ver `USER_ADMIN_FORM_ID`),
// "Cancelar" y "Restablecer contraseña" son botones sueltos; bloqueo/activación y
// concesión/revocación de admin van cada uno en su propio `Form`, con un campo `Hidden` para el
// nuevo valor, para conservar el envío nativo sin JavaScript, mejorado con `hx-post`/`hx-confirm`.
// Todos son hijos directos del mismo `Flex`, que los alinea en fila con espaciado uniforme.
fn edit_actions(user_id: i32, state: EditActions, cx: &mut Context) -> Flex {
    // Una cuenta pendiente de verificar su email también se puede activar directamente, p. ej. si
    // el correo de verificación no llegó; una pendiente de aprobación se aprueba.
    let (next_status, label_key) = match state.status {
        UserStatus::Active => ("blocked", "btn-block"),
        UserStatus::Blocked | UserStatus::Pending => ("active", "btn-activate"),
        UserStatus::AwaitingApproval => ("active", "btn-approve"),
    };
    let (next_is_admin, admin_label_key, admin_confirm_key) = if state.target_is_admin {
        ("false", "btn-revoke-admin", "confirm-revoke-admin")
    } else {
        ("true", "btn-grant-admin", "confirm-grant-admin")
    };

    let password_href = cx.route(user_path(user_id, "password"));
    let status_action = cx.route(user_path(user_id, "status"));
    let admin_action = cx.route(user_path(user_id, "admin"));

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

    let mut container = Flex::new()
        .with_wrap(flex::Behavior::Wrap)
        .with_align(align::Items::Center)
        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
        .with_child(
            Button::submit(Lc::t("btn-save", &LOCALES_USER))
                .with_style(button::Style::Solid(Intent::Primary))
                .with_prop(PropsOp::set("form", USER_ADMIN_FORM_ID)),
        )
        .with_child(cancel_link(cx.route(ADMIN_USERS_PATH.as_str())));
    if state.can_reset_password {
        container = container.with_child(
            Button::anchor(Lc::t("btn-reset-password", &LOCALES_USER), password_href)
                .with_style(button::Style::Solid(Intent::Neutral)),
        );
    }
    if state.can_change_status {
        container = container.with_child(status_form);
    }

    let identity_action = cx.route(user_path(user_id, "identity"));
    for (provider, label, pending) in state.linking {
        let name = label.lookup(cx).unwrap_or_default();
        let (op, key) = if pending {
            ("cancel", "btn-cancel-linking")
        } else {
            ("allow", "btn-allow-linking")
        };
        container = container.with_child(
            Form::new()
                .with_action(identity_action.clone())
                .with_method(form::Method::Post)
                .with_child(form::Hidden::field("provider", provider))
                .with_child(form::Hidden::field("op", op))
                .with_child(
                    Button::submit(Lc::t(key, &LOCALES_USER).with_arg("provider", name))
                        .with_style(button::Style::Solid(Intent::Neutral)),
                ),
        );
    }

    if state.can_toggle_admin {
        let admin_button = Button::submit(Lc::t(admin_label_key, &LOCALES_USER))
            .with_style(button::Style::Solid(Intent::Severe));
        let mut admin_form = Form::new()
            .with_action(admin_action.clone())
            .with_method(form::Method::Post)
            .with_prop(PropsOp::set(hx::POST, admin_action.to_string()))
            .with_child(form::Hidden::field("is_admin", next_is_admin))
            .with_child(admin_button);
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
) -> Result<Response, ErrorPage> {
    require_manageable(&request, id).await?;
    Ok(render_user_edit(request, id, None).await)
}

#[derive(Deserialize)]
pub(crate) struct EditUserFormData {
    username: String,
    email: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    about: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    timezone: String,
    #[serde(default)]
    theme: String,
    #[serde(default)]
    role_ids: Vec<String>,
}

/// POST /admin/user/users/{id}/edit - Actualiza los datos de perfil y los roles de un usuario.
///
/// Usa `RawForm` + `serde_qs` por los roles, igual que `new_post`.
pub(crate) async fn edit_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    raw: web::RawForm,
) -> Result<Response, ErrorPage> {
    let user = require_manageable(&request, id).await?;
    let Ok(form) = serde_qs::from_bytes::<EditUserFormData>(&raw.0) else {
        return Err(ErrorPage::BadRequest(request));
    };
    let role_ids = parse_role_ids(&form.role_ids);
    let policy = provider::account_policy(&user).await;

    // Lo que gestiona un proveedor se conserva como está, llegue lo que llegue del formulario.
    let keep = |editable: bool, submitted: &'_ str, current: &'_ str| -> String {
        if editable { submitted } else { current }.to_owned()
    };
    let username = keep(
        policy.as_ref().is_none_or(|p| p.username().is_editable()),
        &form.username,
        &user.username,
    );
    let email = keep(
        policy.as_ref().is_none_or(|p| p.email().is_editable()),
        &form.email,
        &user.email,
    );
    let field = |field, submitted| managed_or(policy.as_ref(), field, submitted, &user);
    let mut result = user_admin::update_user(
        id,
        user_admin::UserUpdateData {
            username: &username,
            email: &email,
            display_name: field(UserField::DisplayName, &form.display_name),
            about: field(UserField::About, &form.about),
            language: field(UserField::Language, &form.language),
            timezone: field(UserField::Timezone, &form.timezone),
            theme: field(UserField::Theme, &form.theme),
        },
    )
    .await;
    if result.is_ok() {
        let managed = policy.as_ref().map_or(&[][..], |p| p.managed_roles());
        result = user_admin::set_user_roles(id, &role_ids, managed).await;
    }

    match result {
        Ok(()) => {
            let cx = Context::admin(request);
            let target = cx.route(ADMIN_USERS_PATH.as_str());
            Ok(Redirect::see_other(target).into_response())
        }
        Err(err) => {
            let roles = available_roles(&role_ids).await.unwrap_or_default();
            let status = UserStatus::from_i16(user.status);
            let form = UserForm::new()
                .with_policy(policy)
                .with_username(username)
                .with_email(email)
                .with_display_name(form.display_name)
                .with_about(form.about)
                .with_language(form.language)
                .with_timezone(form.timezone)
                .with_theme(form.theme)
                .with_roles(roles)
                .with_error(map_auth_error(&err));
            Ok(render_edit_page(request, id, status, user.is_admin, form).await)
        }
    }
}

// **< view_get >***********************************************************************************

/// GET /admin/user/users/{id}/view - Pantalla de sólo lectura con todos los datos del usuario,
/// incluidos los roles asignados.
pub(crate) async fn view_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
) -> Result<Response, ErrorPage> {
    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return Err(ErrorPage::NotFound(request)),
    };
    let roles = match user_admin::user_roles(id).await {
        Ok(roles) => roles,
        Err(_) => return Err(ErrorPage::InternalError(request)),
    };

    let mut page = Page::admin(request);
    let back_href = page.route(ADMIN_USERS_PATH.as_str());
    let details_block = user_details(&user, page.context_mut()).await;
    let roles_block = user_view_roles(&roles, user.is_admin, page.context_mut()).await;

    let title = Lc::t("title-admin-user-view", &LOCALES_USER);
    Ok(page
        .with_title(title)
        .with_child(details_block)
        .with_child(roles_block)
        .with_child(ChildOp::AddMany(AdminUserFormExtras::collect(id)))
        .with_child(back_link(back_href))
        .render()
        .await
        .into_response())
}

// Bloque de sólo lectura con los roles asignados al usuario, cada uno enlazado a su propia
// pantalla de vista.
async fn user_view_roles(roles: &[role::Model], is_admin: bool, cx: &mut Context) -> Block {
    // Un bloque sin hijos no se renderiza.
    if roles.is_empty() {
        return Block::new();
    }

    let mut table = Table::new().with_prop(PropsOp::add_classes("user-admin-table"));

    for r in roles {
        let system_badge = if r.locked {
            Some(
                Badge::warning(Lc::t("badge-system-role", &LOCALES_USER))
                    .render(cx)
                    .await,
            )
        } else {
            None
        };
        let (id, label) = (r.id, crate::role_label(r.id, &r.label));
        table = table.with_row(
            table::Row::new()
                .with_cell(Html::with(move |cx| {
                    html! { a href=(cx.route(role_path(id, "view"))) { (label.using(cx)) } }
                }))
                .with_cell(r.machine_name.as_str())
                .with_cell(Html::with(move |_| {
                    html! { @if let Some(badge) = &system_badge { (badge) } }
                })),
        );
    }

    let mut block = Block::new()
        .with_title(Lc::t("field-roles", &LOCALES_USER))
        .with_child(table);
    if is_admin {
        block = block.with_child(Html::with(|cx| {
            html! { p { (Lc::t("help-admin-roles", &LOCALES_USER).using(cx)) } }
        }));
    }
    block
}

// Cabecera con los datos básicos del usuario en la pantalla de restablecimiento de contraseña, que
// de otro modo no lo identifica.
fn user_summary(user: &user::Model) -> Table {
    Table::new()
        .with_prop(PropsOp::add_classes("user-admin-table"))
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-username-admin", &LOCALES_USER))
                .with_cell(user.username.as_str()),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-display-name", &LOCALES_USER))
                .with_cell(user.display_name.as_deref().unwrap_or("-")),
        )
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
    web::Form(form): web::Form<StatusFormData>,
) -> Result<Response, ErrorPage> {
    require_permission(&request, &UserPermission::BlockAccounts)?;
    require_manageable(&request, id).await?;
    let Some(account) = request.extension::<Account>().cloned() else {
        return Err(ErrorPage::AccessDenied(request));
    };

    let new_status = match form.status.as_str() {
        "blocked" => UserStatus::Blocked,
        _ => UserStatus::Active,
    };
    // Ver comentario equivalente en `admin_post`: el botón envía la petición vía `hx-post` (para el
    // diálogo `hx-confirm`) sin `hx-target`, así que un `Redirect` normal terminaría anidado dentro
    // del propio `<form>`. `HtmxResponse::redirect()` fuerza una navegación real en el cliente.
    let is_htmx = request.is_htmx();

    match user_admin::set_user_status(id, new_status, account.id()).await {
        Ok(()) => {
            let cx = Context::admin(request);
            let edit_href = cx.route(user_path(id, "edit"));
            if is_htmx {
                Ok(HtmxResponse::empty().redirect(edit_href).into_response())
            } else {
                Ok(Redirect::see_other(edit_href).into_response())
            }
        }
        Err(err) => Ok(render_user_edit(request, id, Some(map_auth_error(&err))).await),
    }
}

// **< admin_post >*********************************************************************************

#[derive(Deserialize)]
pub(crate) struct AdminFormData {
    is_admin: String,
}

/// POST /admin/user/users/{id}/admin - Concede o revoca el acceso total (`is_admin`).
///
/// No pasa por `require_permission`: conceder o revocar este flag no es un permiso del catálogo,
/// se comprueba directamente contra `account.is_admin()` para que sólo un administrador pueda
/// tocarlo (un permiso concedido vía rol nunca basta).
pub(crate) async fn admin_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Form(form): web::Form<AdminFormData>,
) -> Result<Response, ErrorPage> {
    let Some(account) = request.extension::<Account>().cloned() else {
        return Err(ErrorPage::AccessDenied(request));
    };
    if !account.is_admin() {
        return Err(ErrorPage::AccessDenied(request));
    }

    let new_is_admin = form.is_admin == "true";
    // El botón envía la petición vía `hx-post` (para el diálogo `hx-confirm`), sin `hx-target`: si
    // se responde con un `Redirect` normal, HTMX sigue la redirección con `fetch` y sustituye el
    // propio `<form>` (destino por defecto sin `hx-target`) por la página completa que devuelve,
    // anidándola dentro de sí misma. `HtmxResponse::redirect()` evita eso: instruye al cliente
    // (cabecera `HX-Redirect`) para que navegue de verdad a la URL, en vez de intentar un `swap`.
    let is_htmx = request.is_htmx();

    match user_admin::set_user_admin(id, new_is_admin, account.id()).await {
        Ok(()) => {
            let cx = Context::admin(request);
            let edit_href = cx.route(user_path(id, "edit"));
            if is_htmx {
                Ok(HtmxResponse::empty().redirect(edit_href).into_response())
            } else {
                Ok(Redirect::see_other(edit_href).into_response())
            }
        }
        Err(err) => Ok(render_user_edit(request, id, Some(map_auth_error(&err))).await),
    }
}

// **< password_get / password_post >***************************************************************

/// GET /admin/user/users/{id}/password - Formulario de restablecimiento de contraseña por un
/// administrador.
pub(crate) async fn password_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
) -> Result<Response, ErrorPage> {
    let user = require_manageable(&request, id).await?;
    let page = Page::admin(request);
    let title = Lc::t("title-admin-user-password", &LOCALES_USER);
    let page = page.with_title(title).with_child(user_summary(&user));
    let mut page = match external_password(&user).await {
        Some(note) => page.with_child(note),
        None => page.with_child(AdminPasswordForm::new().with_user_id(id)),
    };
    Ok(page.render().await.into_response())
}

// Nota en lugar del formulario de restablecimiento, si la contraseña de la cuenta la gestiona un
// proveedor de autenticación.
async fn external_password(user: &user::Model) -> Option<Html> {
    let policy = provider::account_policy(user).await?;
    if policy.password().is_editable() {
        return None;
    }
    let label = policy.provider_label().clone();
    Some(Html::with(move |cx| {
        let note = Lc::t("help-password-managed", &LOCALES_USER)
            .with_arg("provider", label.lookup(cx).unwrap_or_default());
        html! { p { (note.using(cx)) } }
    }))
}

#[derive(Deserialize)]
pub(crate) struct AdminPasswordFormData {
    password: util::SecretStr,
    confirm_password: util::SecretStr,
}

/// POST /admin/user/users/{id}/password - Aplica la nueva contraseña, invalida las sesiones
/// activas del usuario y le avisa del cambio por correo.
pub(crate) async fn password_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Form(form): web::Form<AdminPasswordFormData>,
) -> Result<Response, ErrorPage> {
    let user = require_manageable(&request, id).await?;

    let result = if external_password(&user).await.is_some() {
        Err(AuthError::PasswordManagedExternally)
    } else {
        match password::passwords_match(form.password.expose(), form.confirm_password.expose()) {
            Ok(()) => user_admin::admin_reset_password(id, form.password.expose()).await,
            Err(err) => Err(err),
        }
    };

    match result {
        Ok(user) => {
            send_notice(&request, Notice::PasswordChangedByAdmin, &user);
            let cx = Context::admin(request);
            let edit_href = cx.route(user_path(id, "edit"));
            Ok(Redirect::see_other(edit_href).into_response())
        }
        Err(err) => {
            let page = Page::admin(request);
            let title = Lc::t("title-admin-user-password", &LOCALES_USER);
            Ok(page
                .with_title(title)
                .with_child(user_summary(&user))
                .with_child(
                    AdminPasswordForm::new()
                        .with_user_id(id)
                        .with_error(map_auth_error(&err)),
                )
                .render()
                .await
                .into_response())
        }
    }
}

// **< identity_post >******************************************************************************

#[derive(Deserialize)]
pub(crate) struct IdentityFormData {
    provider: String,
    op: String,
}

/// POST /admin/user/users/{id}/identity - Permite (`op=allow`) o deja de permitir (`op=cancel`)
/// que un proveedor de autenticación vincule la cuenta cuando la persona con su nombre de usuario
/// inicie sesión con él.
///
/// Es la única forma de que una cuenta existente acabe vinculada a un proveedor: si bastara con
/// que coincidieran los nombres, quien consiguiera en el proveedor el nombre de una cuenta local
/// se quedaría con ella. Nunca para una cuenta de administrador.
pub(crate) async fn identity_post(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
    web::Form(form): web::Form<IdentityFormData>,
) -> Result<Response, ErrorPage> {
    let user = require_manageable(&request, id).await?;
    let Some(provider) = provider::active_provider(&form.provider) else {
        return Err(ErrorPage::BadRequest(request));
    };
    let result = match form.op.as_str() {
        "allow" => provider::allow_linking(id, provider.id(), &user.username).await,
        "cancel" => provider::cancel_linking(id, provider.id()).await,
        _ => return Err(ErrorPage::BadRequest(request)),
    };
    match result {
        Ok(()) => {
            let cx = Context::admin(request);
            Ok(Redirect::see_other(cx.route(user_path(id, "edit"))).into_response())
        }
        Err(err) => Ok(render_user_edit(request, id, Some(map_auth_error(&err))).await),
    }
}
