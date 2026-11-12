//! Handlers HTTP para el perfil del propio usuario autenticado (consulta, edición de sus datos,
//! cambio de contraseña y de email) y para el perfil público de cualquier usuario.

use serde::Deserialize;

use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

use crate::account::{Account, UserStatus};
use crate::auth;
use crate::component::admin::{UserForm, UserFormMode};
use crate::component::{ChangeEmailForm, ChangePasswordForm, multiline_text, user_details};
use crate::entity::{role, user};
use crate::error::AuthError;
use crate::handlers::admin::map_auth_error;
use crate::handlers::{confirm_page, message_page, no_referrer};
use crate::mail::{Notice, send_email_change_mail, send_notice};
use crate::password;
use crate::permission::UserPermission;
use crate::provider::{self, AccountPolicy, FieldPolicy, UserField};
use crate::service::user_admin;
use crate::session;
use crate::throttle;
use crate::{
    EMAIL_PATH, LOCALES_USER, LOGIN_PATH, PROFILE_EDIT_PATH, PROFILE_PASSWORD_PATH, PROFILE_PATH,
};

// **< current_user_id / login_redirect >***********************************************************

// Identificador del usuario autenticado; el middleware de sesión sólo inserta `Account` si hay una
// sesión activa.
fn current_user_id(request: &HttpRequest) -> Option<i32> {
    request.extension::<Account>().map(|a| a.id())
}

// Redirección al formulario de inicio de sesión con `next` como `waypoint`: la ruta a la que
// volver tras iniciar sesión.
fn login_redirect(request: HttpRequest, next: &'static str) -> Response {
    let cx = Context::new(request);
    let waypoint = Waypoint::new(cx.route(next).to_string());
    Redirect::see_other(waypoint.append_to(cx.route(LOGIN_PATH))).into_response()
}

// Respuesta a un usuario anónimo sin acceso a una ruta (acción `AnonymousDenied`): el formulario de
// inicio de sesión, que vuelve después a la página de la que venía (ver `return_path()`). Si
// enviaba algo, se le avisa de que no se ha guardado: el envío se pierde, y no se repite al volver.
//
// Una petición HTMX seguiría la redirección sin avisar y pintaría el formulario dentro del
// fragmento que esperaba, así que se le pide con `HX-Redirect` que cargue la página completa.
pub(crate) fn offer_login(request: &HttpRequest) -> Option<Response> {
    let cx = Context::new(request.clone());
    let login = Waypoint::new(return_path(request)).append_to(cx.route(LOGIN_PATH));
    let response = if request.is_htmx() {
        HtmxResponse::empty().redirect(login).into_response()
    } else {
        Redirect::see_other(login).into_response()
    };
    Some(if request.method().is_safe() {
        response
    } else {
        response.with_message(
            &cx,
            MessageLevel::Warning,
            Lc::t("msg-login-required", &LOCALES_USER),
        )
    })
}

// Ruta a la que volver tras iniciar sesión: la pedida, si se puede abrir de nuevo. La de un
// fragmento HTMX o la de un envío puede no admitir `GET`, así que entonces es la página que tenía
// abierta el navegador (`HX-Current-URL` o `Referer`). Un `Referer` de otro sitio, posible con
// `server.trusted_origins`, no dice nada de esta aplicación: se vuelve sin destino.
fn return_path(request: &HttpRequest) -> Option<String> {
    use web::http::{Uri, header};

    let path_of = |uri: Uri| uri.path_and_query().map(|pq| pq.as_str().to_owned());
    if request.is_htmx() {
        return path_of(request.hx_current_url()?.parse().ok()?);
    }
    if request.method().is_safe() {
        return Some(request.uri().to_owned());
    }
    let headers = request.headers();
    let referer: Uri = headers.get(header::REFERER)?.to_str().ok()?.parse().ok()?;
    let host = headers.get(header::HOST)?.to_str().ok()?;
    (referer.authority()?.as_str() == host).then(|| path_of(referer))?
}

// **< profile_get >********************************************************************************

/// GET /user - Perfil del usuario autenticado. Redirige al formulario de inicio de sesión si no hay
/// sesión activa, conservando la URL de retorno.
pub(crate) async fn profile_get(request: HttpRequest) -> Response {
    let Some(id) = current_user_id(&request) else {
        return login_redirect(request, PROFILE_PATH);
    };

    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return ErrorPage::NotFound(request).into_response(),
    };
    let roles = match user_admin::user_roles(id).await {
        Ok(roles) => roles,
        Err(_) => return ErrorPage::InternalError(request).into_response(),
    };
    // Con la cuenta gestionada por un proveedor, la contraseña y el email que no son editables no
    // se ofrecen: se indica quién los gestiona y, si lo hay, el enlace de ayuda.
    let policy = provider::account_policy(&user).await;
    let editable =
        |f: fn(&AccountPolicy) -> FieldPolicy| policy.as_ref().is_none_or(|p| f(p).is_editable());
    let can_edit = has_permission(&request, &UserPermission::EditOwnProfile);
    let can_change_email = can_edit && editable(AccountPolicy::email);
    let can_change_password = has_permission(&request, &UserPermission::ChangeOwnPassword)
        && editable(AccountPolicy::password);

    let mut page = Page::new(request);
    let details_block = user_details(&user, page.context_mut()).await;
    let roles_block = profile_roles(&roles, page.context_mut()).await;
    let actions = profile_actions(
        can_edit,
        can_change_email,
        can_change_password,
        page.context(),
    );

    page.with_title(Lc::t("title-profile", &LOCALES_USER))
        .with_child(details_block)
        .with_child(managed_note(policy.as_ref()))
        .with_child(actions)
        .with_child(roles_block)
        .render()
        .await
        .into_response()
}

// Nota «Tu cuenta se gestiona en ...», con el enlace de ayuda del proveedor si lo hay. Sin
// proveedor, no pinta nada.
fn managed_note(policy: Option<&AccountPolicy>) -> Html {
    let note = policy.map(|p| (p.provider_label().clone(), p.help().cloned()));
    Html::with(move |cx| {
        let Some((label, help)) = &note else {
            return html! {};
        };
        let text = Lc::t("help-account-managed", &LOCALES_USER)
            .with_arg("provider", label.lookup(cx).unwrap_or_default());
        html! {
            p class="user-managed-note" {
                (text.using(cx))
                @if let Some(help) = help {
                    " "
                    a href=(help.url()) {
                        @match help.text().lookup(cx) {
                            Some(text) => (text),
                            None => (help.url()),
                        }
                    }
                }
            }
        }
    })
}

// Página informativa en lugar de un formulario que la política de la cuenta no permite.
async fn managed_page(request: HttpRequest, title: Lc, policy: &AccountPolicy) -> Response {
    let note = managed_note(Some(policy));
    Page::new(request)
        .with_title(title.clone())
        .with_child(Block::new().with_title(title).with_child(note))
        .render()
        .await
        .into_response()
}

// Política de la cuenta si gestiona el campo de forma que el usuario no puede cambiarlo.
async fn locked_by_policy(
    user_id: i32,
    field: fn(&AccountPolicy) -> FieldPolicy,
) -> Option<AccountPolicy> {
    let user = user_admin::find_user(user_id).await.ok()?;
    provider::account_policy(&user)
        .await
        .filter(|policy| !field(policy).is_editable())
}

// Botones para editar el perfil, cambiar el email y cambiar la contraseña, cada uno sólo si se
// tiene el permiso (el email, con el de editar el perfil) y la cuenta lo permite. Sin ninguno, el
// contenedor queda vacío y no se renderiza.
fn profile_actions(
    can_edit: bool,
    can_change_email: bool,
    can_change_password: bool,
    cx: &Context,
) -> Flex {
    let mut actions = Flex::new()
        .with_wrap(flex::Behavior::Wrap)
        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)));
    if can_edit {
        actions = actions.with_child(
            Button::anchor(
                Lc::t("btn-edit-profile", &LOCALES_USER),
                cx.route(PROFILE_EDIT_PATH),
            )
            .with_style(button::Style::Solid(Intent::Primary)),
        );
    }
    if can_change_email {
        actions = actions.with_child(
            Button::anchor(
                Lc::t("btn-change-email", &LOCALES_USER),
                cx.route(EMAIL_PATH),
            )
            .with_style(button::Style::Solid(Intent::Neutral)),
        );
    }
    if can_change_password {
        actions = actions.with_child(
            Button::anchor(
                Lc::t("btn-set-password", &LOCALES_USER),
                cx.route(PROFILE_PASSWORD_PATH),
            )
            .with_style(button::Style::Solid(Intent::Neutral)),
        );
    }
    actions
}

// Bloque de sólo lectura con los roles del usuario autenticado. A diferencia de la vista de
// administración, no enlaza cada rol a su pantalla de detalle: un usuario sin permisos de
// administración no puede acceder a ella.
async fn profile_roles(roles: &[role::Model], cx: &mut Context) -> Block {
    // Un bloque sin hijos no se renderiza.
    if roles.is_empty() {
        return Block::new();
    }

    let mut items: Vec<(Lc, Option<Markup>)> = Vec::with_capacity(roles.len());
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
        items.push((crate::role_label(r.id, &r.label), system_badge));
    }

    Block::new()
        .with_title(Lc::t("field-roles", &LOCALES_USER))
        .with_child(Html::with(move |cx| {
            html! {
                ul class="user-profile-roles" {
                    @for (label, system_badge) in &items {
                        li {
                            (label.using(cx))
                            @if let Some(badge) = system_badge {
                                " "
                                (badge)
                            }
                        }
                    }
                }
            }
        }))
}

// **< public_profile_get >*************************************************************************

/// GET /user/{id} - Perfil público de un usuario: nombre de usuario, nombre visible, "Sobre mí" y
/// fecha de alta. Requiere `user:view_profiles`, salvo para ver el propio. El perfil de una cuenta
/// que no está activa sólo lo ve quien administra usuarios.
pub(crate) async fn public_profile_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
) -> Result<Response, ErrorPage> {
    // El permiso se comprueba antes de buscar al usuario para no revelar qué ids existen. Sin él,
    // la misma respuesta que una ruta sin acceso: el inicio de sesión a un anónimo y 403 al resto.
    if current_user_id(&request) != Some(id)
        && !has_permission(&request, &UserPermission::ViewProfiles)
    {
        return Ok(deny_access(request));
    }
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(request));
    };
    if UserStatus::from_i16(user.status) != UserStatus::Active
        && !has_permission(&request, &UserPermission::AdminUsers)
    {
        return Err(ErrorPage::NotFound(request));
    }

    let title = match util::non_blank(user.display_name.as_deref().unwrap_or_default()) {
        Some(name) => name.to_owned(),
        None => user.username.clone(),
    };
    let page = Page::new(request);
    let details = public_profile_details(&user, page.context());

    Ok(page
        .with_title(Lc::n(title))
        .with_child(details)
        .render()
        .await
        .into_response())
}

// Bloque con los únicos datos que se muestran a otros usuarios; nunca el email, los roles, el
// estado ni la zona horaria.
fn public_profile_details(user: &user::Model, cx: &Context) -> Block {
    let member_since = cx.format_date(
        user.created_at.with_timezone(&cx.timezone()).date_naive(),
        DateFormat::Long,
    );
    let table = Table::new()
        .with_prop(PropsOp::add_classes("user-details-table"))
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
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-about", &LOCALES_USER))
                .with_cell(multiline_text(
                    user.about.clone().unwrap_or_else(|| "-".into()),
                )),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-member-since", &LOCALES_USER))
                .with_cell(member_since),
        );
    Block::new().with_child(table)
}

// **< profile_edit_get / profile_edit_post >*******************************************************

/// GET /user/edit - Formulario de edición del perfil propio.
pub(crate) async fn profile_edit_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_EDIT_PATH));
    };
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(request));
    };

    let policy = provider::account_policy(&user).await;
    let form = UserForm::new()
        .with_policy(policy)
        .with_username(user.username)
        .with_email(user.email)
        .with_display_name(user.display_name.unwrap_or_default())
        .with_about(user.about.unwrap_or_default())
        .with_language(user.language.unwrap_or_default())
        .with_timezone(user.timezone.unwrap_or_default())
        .with_theme(user.theme.unwrap_or_default());
    Ok(render_profile_edit(request, form).await)
}

#[derive(Deserialize)]
pub(crate) struct ProfileFormData {
    #[serde(default)]
    username: String,
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
}

/// POST /user/edit - Guarda los datos del perfil propio. El nombre de usuario sólo cambia con
/// `user:change_own_username`; el email, desde [`email_get()`]; roles, estado y acceso
/// total no se tocan nunca desde aquí.
pub(crate) async fn profile_edit_post(
    request: HttpRequest,
    web::Form(form): web::Form<ProfileFormData>,
) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_EDIT_PATH));
    };
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(request));
    };

    // Sin permiso se conserva el nombre actual, aunque la petición se haya manipulado a mano; y lo
    // que gestiona un proveedor, también.
    let policy = provider::account_policy(&user).await;
    let username = if has_permission(&request, &UserPermission::ChangeOwnUsername)
        && policy.as_ref().is_none_or(|p| p.username().is_editable())
    {
        form.username
    } else {
        user.username.clone()
    };

    let field = |field, submitted| managed_or(policy.as_ref(), field, submitted, &user);
    let result = user_admin::update_user(
        id,
        user_admin::UserUpdateData {
            username: &username,
            email: &user.email,
            display_name: field(UserField::DisplayName, &form.display_name),
            about: field(UserField::About, &form.about),
            language: field(UserField::Language, &form.language),
            timezone: field(UserField::Timezone, &form.timezone),
            theme: field(UserField::Theme, &form.theme),
        },
    )
    .await;

    match result {
        Ok(()) => {
            let cx = Context::new(request);
            Ok(Redirect::see_other(cx.route(PROFILE_PATH)).into_response())
        }
        Err(err) => {
            let form = UserForm::new()
                .with_policy(policy)
                .with_username(username)
                .with_email(user.email)
                .with_display_name(form.display_name)
                .with_about(form.about)
                .with_language(form.language)
                .with_timezone(form.timezone)
                .with_theme(form.theme)
                .with_error(map_auth_error(&err));
            Ok(render_profile_edit(request, form).await)
        }
    }
}

// Página de edición del perfil propio con el formulario ya relleno.
async fn render_profile_edit(request: HttpRequest, form: UserForm) -> Response {
    let can_change_username = has_permission(&request, &UserPermission::ChangeOwnUsername);
    let form = form
        .with_mode(UserFormMode::Profile)
        .with_allow_username_field(can_change_username);
    render_profile_page(request, Lc::t("title-profile-edit", &LOCALES_USER), form).await
}

// **< password_get / password_post >***************************************************************

/// GET /user/password - Formulario de cambio de la contraseña propia.
pub(crate) async fn password_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_PASSWORD_PATH));
    };
    let title = Lc::t("title-profile-password", &LOCALES_USER);
    if let Some(policy) = locked_by_policy(id, AccountPolicy::password).await {
        return Ok(managed_page(request, title, &policy).await);
    }
    Ok(render_profile_page(request, title, ChangePasswordForm::new()).await)
}

#[derive(Deserialize)]
pub(crate) struct ChangePasswordFormData {
    current_password: util::SecretStr,
    password: util::SecretStr,
    confirm_password: util::SecretStr,
}

/// POST /user/password - Cambia la contraseña propia tras comprobar la actual, cierra el resto de
/// sesiones abiertas del usuario, conservando la actual, y avisa del cambio a su dirección.
pub(crate) async fn password_post(
    request: HttpRequest,
    web::Form(form): web::Form<ChangePasswordFormData>,
) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_PASSWORD_PATH));
    };
    if let Some(policy) = locked_by_policy(id, AccountPolicy::password).await {
        let title = Lc::t("title-profile-password", &LOCALES_USER);
        return Ok(managed_page(request, title, &policy).await);
    }

    let sid = session::extract_sid(Some(request.headers()));
    let result =
        match password::passwords_match(form.password.expose(), form.confirm_password.expose()) {
            Ok(()) => {
                auth::change_own_password(id, &form.current_password, &form.password, sid).await
            }
            Err(err) => Err(err),
        };

    match result {
        Ok(user) => {
            send_notice(&request, Notice::PasswordChanged, &user);
            let cx = Context::new(request);
            Ok(Redirect::see_other(cx.route(PROFILE_PATH)).into_response())
        }
        Err(err) => {
            let error = match err {
                AuthError::InvalidCredentials => Lc::t("error-current-password", &LOCALES_USER),
                err => map_auth_error(&err),
            };
            let title = Lc::t("title-profile-password", &LOCALES_USER);
            let form = ChangePasswordForm::new().with_error(error);
            Ok(render_profile_page(request, title, form).await)
        }
    }
}

// **< email_get / email_post >*********************************************************************

/// GET /user/email - Formulario de cambio del email propio.
pub(crate) async fn email_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, EMAIL_PATH));
    };
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(request));
    };
    let title = Lc::t("title-profile-email", &LOCALES_USER);
    if let Some(policy) = provider::account_policy(&user)
        .await
        .filter(|policy| !policy.email().is_editable())
    {
        return Ok(managed_page(request, title, &policy).await);
    }
    let form = ChangeEmailForm::new().with_pending_email(user.pending_email);
    Ok(render_profile_page(request, title, form).await)
}

#[derive(Deserialize)]
pub(crate) struct ChangeEmailFormData {
    email: String,
    current_password: util::SecretStr,
}

/// POST /user/email - Pide cambiar el email propio tras comprobar la contraseña actual: envía a la
/// nueva dirección el enlace para confirmarla y avisa a la actual (ver
/// `auth::request_email_change()`).
pub(crate) async fn email_post(
    request: HttpRequest,
    web::Form(form): web::Form<ChangeEmailFormData>,
) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, EMAIL_PATH));
    };
    if let Some(policy) = locked_by_policy(id, AccountPolicy::email).await {
        let title = Lc::t("title-profile-email", &LOCALES_USER);
        return Ok(managed_page(request, title, &policy).await);
    }

    let result = if throttle::allow(&request) {
        auth::request_email_change(id, &form.current_password, &form.email).await
    } else {
        Err(AuthError::TooManyRequests)
    };
    match result {
        Ok((user, new_email, token)) => {
            send_email_change_mail(&request, &user, &new_email, &token);
            send_notice(&request, Notice::EmailChangeRequested, &user);
            let title = Lc::t("title-profile-email", &LOCALES_USER);
            Ok(message_page(
                request,
                title,
                Lc::t("msg-email-change-sent", &LOCALES_USER),
            )
            .await)
        }
        Err(err) => {
            let error = match err {
                AuthError::InvalidCredentials => Lc::t("error-current-password", &LOCALES_USER),
                err => map_auth_error(&err),
            };
            let pending = user_admin::find_user(id)
                .await
                .ok()
                .and_then(|user| user.pending_email);
            let title = Lc::t("title-profile-email", &LOCALES_USER);
            let form = ChangeEmailForm::new()
                .with_email(form.email)
                .with_pending_email(pending)
                .with_error(error);
            Ok(render_profile_page(request, title, form).await)
        }
    }
}

// **< email_confirm_get / email_confirm_post >*****************************************************

/// GET /user/email/{uid}/{token} - Pide confirmar la nueva dirección con un botón.
///
/// Como en la verificación del email al registrarse, abrir el enlace no confirma nada: los
/// servidores de correo abren los enlaces para analizarlos. Sólo el envío del formulario
/// ([`email_confirm_post()`]) consume el token. No exige sesión: el enlace basta para demostrar que
/// se reciben los correos de la nueva dirección.
pub(crate) async fn email_confirm_get(
    request: HttpRequest,
    web::Path((uid, token)): web::Path<(i32, String)>,
) -> Result<Response, ErrorPage> {
    if !auth::is_valid_email_change_token(uid, &token)
        .await
        .unwrap_or(false)
    {
        return Err(ErrorPage::NotFound(request));
    }
    Ok(no_referrer(
        confirm_page(
            request,
            Lc::t("title-confirm-email-change", &LOCALES_USER),
            Lc::t("msg-confirm-email-change", &LOCALES_USER),
            Lc::t("btn-confirm-email-change", &LOCALES_USER),
        )
        .await,
    ))
}

/// POST /user/email/{uid}/{token} - Aplica la nueva dirección y la da por verificada.
pub(crate) async fn email_confirm_post(
    request: HttpRequest,
    web::Path((uid, token)): web::Path<(i32, String)>,
) -> Response {
    match auth::confirm_email_change(uid, &token).await {
        Ok(()) => {
            let cx = Context::new(request);
            Redirect::see_other(cx.route(PROFILE_PATH)).with_message(
                &cx,
                MessageLevel::Info,
                Lc::t("msg-email-changed", &LOCALES_USER),
            )
        }
        Err(AuthError::EmailTaken) => {
            let title = Lc::t("title-confirm-email-change", &LOCALES_USER);
            no_referrer(
                message_page(request, title, Lc::t("error-email-taken", &LOCALES_USER)).await,
            )
        }
        Err(_) => ErrorPage::NotFound(request).into_response(),
    }
}

// **< render_profile_page >************************************************************************

// Página del perfil propio con un formulario.
async fn render_profile_page(
    request: HttpRequest,
    title: Lc,
    form: impl Component + 'static,
) -> Response {
    Page::new(request)
        .with_title(title.clone())
        .with_child(Block::new().with_title(title).with_child(form))
        .render()
        .await
        .into_response()
}

// Valor de un dato del perfil para guardar: el enviado o, si lo gestiona el proveedor de la cuenta,
// el guardado.
pub(crate) fn managed_or<'a>(
    policy: Option<&AccountPolicy>,
    field: UserField,
    submitted: &'a str,
    user: &'a user::Model,
) -> Option<&'a str> {
    if !policy.is_some_and(|p| p.manages(field)) {
        return util::non_blank(submitted);
    }
    match field {
        UserField::DisplayName => user.display_name.as_deref(),
        UserField::About => user.about.as_deref(),
        UserField::Language => user.language.as_deref(),
        UserField::Timezone => user.timezone.as_deref(),
        UserField::Theme => user.theme.as_deref(),
        UserField::Email => Some(user.email.as_str()),
    }
}
