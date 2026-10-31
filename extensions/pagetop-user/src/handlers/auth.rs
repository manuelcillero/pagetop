//! Handlers HTTP para autenticación y gestión de cuenta.

use serde::Deserialize;

use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

use crate::auth;
use crate::component::{LoginForm, PasswordResetConfirmForm, PasswordResetForm, RegisterForm};
use crate::config::SETTINGS;
use crate::error::AuthError;
use crate::handlers::admin::map_auth_error;
use crate::mail::send_token_mail;
use crate::password;
use crate::session;
use crate::token::{TokenKind, consume_token, count_tokens_since, create_token, is_valid_token};
use crate::{LOCALES_USER, LOGIN_PATH, PROFILE_PATH};

// **< Tipos de formularios >***********************************************************************

#[derive(Deserialize)]
pub struct LoginFormData {
    // Acepta también `ident`, el nombre de campo usado en el modo de login estricto
    // (ver `[user.login]`).
    #[serde(alias = "ident")]
    username: String,
    // Acepta también `token`, el nombre de campo usado en el modo de login estricto.
    #[serde(alias = "token")]
    password: String,
    #[serde(default)]
    remember: bool,
}

#[derive(Deserialize)]
pub struct RegisterFormData {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[derive(Deserialize)]
pub struct PasswordResetFormData {
    email: String,
}

#[derive(Deserialize)]
pub struct PasswordResetConfirmFormData {
    password: String,
    confirm_password: String,
}

// **< login_get >**********************************************************************************

/// GET /user/login - Formulario de inicio de sesión.
pub async fn login_get(
    request: HttpRequest,
    web::Query(waypoint): web::Query<Waypoint>,
) -> Response {
    // Un usuario ya autenticado no debe volver a ver el formulario de login: se le redirige al
    // waypoint transportado si lo hay (p. ej. llegó aquí desde un enlace obsoleto o una pestaña
    // duplicada), o a su perfil en caso contrario.
    if request
        .extension::<CurrentUser>()
        .is_some_and(CurrentUser::is_authenticated)
    {
        let cx = Context::new(request.clone());
        return Redirect::see_other(waypoint.or(cx.route(PROFILE_PATH)));
    }
    Page::new(request)
        .with_title(Lc::t("title-login", &LOCALES_USER))
        .with_child(LoginForm::new().with_waypoint(waypoint))
        .render()
        .await
        .into_response()
}

// **< login_post >*********************************************************************************

/// POST /user/login - Procesa las credenciales y abre la sesión.
pub async fn login_post(
    request: HttpRequest,
    web::Query(waypoint): web::Query<Waypoint>,
    web::Form(form): web::Form<LoginFormData>,
) -> Response {
    let cx = Context::new(request.clone());
    let next = waypoint.or(cx.route("/"));
    let result = auth::login(&form.username, &form.password, form.remember).await;
    match result {
        Ok(sid) => {
            let cookie = session::build_cookie(&sid, form.remember);
            redirect_with_cookie(next, &cookie)
        }
        Err(err) => {
            let error_key = match &err {
                AuthError::AccountBlocked => "error-account-blocked",
                AuthError::AccountPending => "error-account-pending",
                AuthError::AccountLocked => "error-account-locked",
                _ => "error-invalid-credentials",
            };
            Page::new(request)
                .with_title(Lc::t("title-login", &LOCALES_USER))
                .with_child(
                    LoginForm::new()
                        .with_username(form.username)
                        .with_error(Lc::t(error_key, &LOCALES_USER))
                        .with_waypoint(waypoint),
                )
                .render()
                .await
                .into_response()
        }
    }
}

// **< logout_post >********************************************************************************

/// POST /user/logout - Cierra la sesión y redirige al login.
pub async fn logout_post(request: HttpRequest) -> Response {
    let cx = Context::new(request.clone());
    if let Some(sid) = session::extract_sid(Some(request.headers())) {
        auth::logout(sid).await.ok();
    }
    let expiry = session::expiry_cookie();
    if request.is_htmx() {
        // La petición sale de un elemento con `hx-post` sin destino: `HX-Redirect` fuerza una
        // navegación real en vez de insertar la página de login dentro del elemento.
        let mut response = HtmxResponse::empty()
            .redirect(cx.route(LOGIN_PATH))
            .into_response();
        match expiry.parse() {
            Ok(value) => {
                response
                    .headers_mut()
                    .insert(web::http::header::SET_COOKIE, value);
            }
            Err(_) => trace::warn!("logout: invalid session expiry cookie, header discarded"),
        }
        return response;
    }
    redirect_with_cookie(cx.route(LOGIN_PATH), &expiry)
}

// **< register_get >*******************************************************************************

/// GET /user/register - Formulario de registro de cuenta.
pub async fn register_get(request: HttpRequest) -> Result<Markup, ErrorPage> {
    if !SETTINGS.allow_registration {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    Page::new(request)
        .with_title(Lc::t("title-register", &LOCALES_USER))
        .with_child(RegisterForm::new())
        .render()
        .await
}

// **< register_post >******************************************************************************

/// POST /user/register - Registra un nuevo usuario.
pub async fn register_post(
    request: HttpRequest,
    web::Form(form): web::Form<RegisterFormData>,
) -> Response {
    if !SETTINGS.allow_registration {
        return ErrorPage::NotFound(Some(request)).into_response();
    }
    let cx = Context::new(request.clone());
    let result = auth::register(
        &form.username,
        &form.email,
        &form.password,
        &form.confirm_password,
    )
    .await;
    match result {
        Ok(user_id) if SETTINGS.require_email_verification => {
            send_verification_mail(&request, user_id).await;
            Page::new(request)
                .with_title(Lc::t("title-register", &LOCALES_USER))
                .with_child(Html::with(|cx| {
                    html! { p { (Lc::t("msg-verification-sent", &LOCALES_USER).using(cx)) } }
                }))
                .render()
                .await
                .into_response()
        }
        Ok(_) => Redirect::see_other(cx.route(LOGIN_PATH)),
        Err(err) => Page::new(request)
            .with_title(Lc::t("title-register", &LOCALES_USER))
            .with_child(
                RegisterForm::new()
                    .with_username(form.username)
                    .with_email(form.email)
                    .with_error(map_auth_error(&err)),
            )
            .render()
            .await
            .into_response(),
    }
}

// **< password_reset_get >*************************************************************************

/// GET /user/password/reset - Formulario de solicitud de restablecimiento.
pub async fn password_reset_get(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_title(Lc::t("title-password-reset", &LOCALES_USER))
        .with_child(PasswordResetForm::new())
        .render()
        .await
}

// **< password_reset_post >************************************************************************

/// POST /user/password/reset - Inicia el flujo de restablecimiento de contraseña.
pub async fn password_reset_post(
    request: HttpRequest,
    web::Form(form): web::Form<PasswordResetFormData>,
) -> Result<Markup, ErrorPage> {
    // Respondemos igual exista o no el email para no revelar qué emails están registrados, también
    // en el tiempo de respuesta: la búsqueda, el token y el correo se resuelven en segundo plano.
    // Sólo a cuentas activas: una pendiente o bloqueada no podría iniciar sesión con la nueva
    // contraseña.
    let mail_request = request.clone();
    tokio::spawn(async move {
        use crate::account::UserStatus;
        use crate::entity::user;
        use crate::identity::email_key;
        use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};
        let found = user::Entity::find()
            .filter(user::Column::EmailKey.eq(email_key(&form.email)))
            .filter(user::Column::Status.eq(UserStatus::Active.as_i16()))
            .one(dbconn())
            .await;
        let user_model = match found {
            Ok(Some(user_model)) => user_model,
            Ok(None) => return,
            Err(err) => {
                trace::error!("Cannot look up account for password reset: {err}");
                return;
            }
        };
        let max = SETTINGS.max_password_resets;
        if max > 0 {
            let since = Utc::now() - Duration::seconds(SETTINGS.password_reset_window_secs);
            match count_tokens_since(user_model.id, TokenKind::PasswordReset, since).await {
                Ok(count) if count >= u64::from(max) => {
                    trace::warn!("Password reset limit reached for user {}", user_model.id);
                    return;
                }
                Ok(_) => {}
                Err(err) => {
                    trace::error!("Cannot count password reset requests: {err}");
                    return;
                }
            }
        }
        match create_token(user_model.id, TokenKind::PasswordReset).await {
            Ok(token) => {
                send_token_mail(&mail_request, TokenKind::PasswordReset, &user_model, &token);
            }
            Err(err) => trace::error!("Cannot create password reset token: {err}"),
        }
    });
    Page::new(request)
        .with_title(Lc::t("title-password-reset", &LOCALES_USER))
        .with_child(Html::with(|cx| {
            html! { p { (Lc::t("msg-password-reset-sent", &LOCALES_USER).using(cx)) } }
        }))
        .render()
        .await
}

// **< password_reset_confirm_get >*****************************************************************

/// GET /user/password/reset/{uid}/{token} - Formulario para introducir la nueva contraseña.
pub async fn password_reset_confirm_get(
    request: HttpRequest,
    web::Path((uid, token)): web::Path<(i32, String)>,
) -> Result<Markup, ErrorPage> {
    if !is_valid_token(uid, &token, TokenKind::PasswordReset)
        .await
        .unwrap_or(false)
    {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    Page::new(request)
        .with_title(Lc::t("title-new-password", &LOCALES_USER))
        .with_child(PasswordResetConfirmForm::new())
        .render()
        .await
}

// **< password_reset_confirm_post >****************************************************************

/// POST /user/password/reset/{uid}/{token} - Aplica la nueva contraseña.
///
/// La contraseña se valida antes de consumir el token: si no es válida, el mismo enlace sirve para
/// intentarlo de nuevo. Al aplicarla se cierran todas las sesiones del usuario y se levanta el
/// bloqueo temporal por intentos fallidos, porque quien abre el enlace ha demostrado ser el titular
/// de la dirección.
pub async fn password_reset_confirm_post(
    request: HttpRequest,
    web::Path((uid, token)): web::Path<(i32, String)>,
    web::Form(form): web::Form<PasswordResetConfirmFormData>,
) -> Response {
    let cx = Context::new(request.clone());
    let result = async {
        use crate::entity::user;
        use pagetop_seaorm::db::{ActiveModelTrait, Set, dbconn};
        password::passwords_match(&form.password, &form.confirm_password)?;
        password::validate_strength(&form.password)?;
        let hash = password::hash_password_async(&form.password).await?;
        consume_token(uid, &token, TokenKind::PasswordReset).await?;
        user::ActiveModel {
            id: Set(uid),
            password_hash: Set(hash),
            failed_login_count: Set(0),
            failed_login_at: Set(None),
            locked_until: Set(None),
            updated_at: Set(Utc::now()),
            ..Default::default()
        }
        .update(dbconn())
        .await?;
        session::destroy_user_sessions(uid)
            .await
            .map_err(AuthError::Database)?;
        Ok::<(), AuthError>(())
    }
    .await;
    match result {
        Ok(()) => Redirect::see_other(cx.route(LOGIN_PATH)),
        Err(err) => {
            let error = match err {
                AuthError::InvalidToken => Lc::t("error-token-invalid", &LOCALES_USER),
                err => map_auth_error(&err),
            };
            Page::new(request)
                .with_title(Lc::t("title-new-password", &LOCALES_USER))
                .with_child(PasswordResetConfirmForm::new().with_error(error))
                .render()
                .await
                .into_response()
        }
    }
}

// **< verify_email_get / verify_email_post >*******************************************************

/// GET /user/verify/{uid}/{token} - Pide confirmar la dirección de email con un botón.
///
/// Abrir el enlace no verifica nada: los servidores de correo y los antivirus abren por su cuenta
/// los enlaces de los correos para analizarlos, y así quedaría verificada una cuenta creada con la
/// dirección de otra persona sin que ésta haya hecho nada. Sólo el envío del formulario
/// ([`verify_email_post()`]) consume el token.
pub async fn verify_email_get(
    request: HttpRequest,
    web::Path((uid, token)): web::Path<(i32, String)>,
) -> Result<Markup, ErrorPage> {
    if !is_valid_token(uid, &token, TokenKind::EmailVerification)
        .await
        .unwrap_or(false)
    {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    // Sin `with_action()`: se envía a la propia URL, que incluye el `{uid}/{token}`.
    let form = Form::new()
        .with_id("user-verify-email-form")
        .with_method(form::Method::Post)
        .with_child(Html::with(|cx| {
            html! { p { (Lc::t("msg-verify-email", &LOCALES_USER).using(cx)) } }
        }))
        .with_child(
            Button::submit(Lc::t("btn-verify-email", &LOCALES_USER))
                .with_style(button::Style::Solid(Intent::Primary)),
        );
    Page::new(request)
        .with_title(Lc::t("title-verify-email", &LOCALES_USER))
        .with_child(form)
        .render()
        .await
}

/// POST /user/verify/{uid}/{token} - Confirma la dirección de email.
///
/// Activa la cuenta si estaba pendiente de esta verificación; una cuenta bloqueada sigue bloqueada.
pub async fn verify_email_post(
    request: HttpRequest,
    web::Path((uid, token)): web::Path<(i32, String)>,
) -> Response {
    let cx = Context::new(request.clone());
    let result = async {
        use crate::account::UserStatus;
        use crate::entity::user;
        use pagetop_seaorm::db::query::Expr;
        use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};
        consume_token(uid, &token, TokenKind::EmailVerification).await?;
        let now = Utc::now();
        user::Entity::update_many()
            .col_expr(user::Column::EmailVerifiedAt, Expr::value(now))
            .col_expr(user::Column::UpdatedAt, Expr::value(now))
            .filter(user::Column::Id.eq(uid))
            .exec(dbconn())
            .await?;
        user::Entity::update_many()
            .col_expr(
                user::Column::Status,
                Expr::value(UserStatus::Active.as_i16()),
            )
            .filter(user::Column::Id.eq(uid))
            .filter(user::Column::Status.eq(UserStatus::Pending.as_i16()))
            .exec(dbconn())
            .await?;
        Ok::<(), AuthError>(())
    }
    .await;
    match result {
        Ok(()) => Redirect::see_other(cx.route(LOGIN_PATH)),
        Err(_) => ErrorPage::NotFound(Some(request)).into_response(),
    }
}

// **< HELPERS >************************************************************************************

// Envía el correo para verificar la dirección de la cuenta recién creada. Un fallo no impide el
// registro: la cuenta queda pendiente y un administrador puede activarla.
//
// La dirección todavía no está verificada: cualquiera puede registrarse con la de otra persona y
// hacer que le llegue este correo, que sale del dominio de la aplicación y con su diseño, así que
// pasa los filtros y resulta creíble. Por eso no incluye nada que elija quien se registra, ni
// siquiera el nombre de usuario: bastaría para colar un texto o un enlace engañoso (*phishing*).
async fn send_verification_mail(request: &HttpRequest, user_id: i32) {
    use crate::entity::user;
    use pagetop_seaorm::db::{EntityTrait, dbconn};
    let user = match user::Entity::find_by_id(user_id).one(dbconn()).await {
        Ok(Some(user)) => user,
        Ok(None) => return,
        Err(err) => {
            trace::error!("Cannot load new user {user_id} to send verification email: {err}");
            return;
        }
    };
    match create_token(user_id, TokenKind::EmailVerification).await {
        Ok(token) => send_token_mail(request, TokenKind::EmailVerification, &user, &token),
        Err(err) => trace::error!("Cannot create email verification token: {err}"),
    }
}

fn redirect_with_cookie(to: impl Into<RoutePath>, cookie: &str) -> Response {
    (
        web::http::StatusCode::SEE_OTHER,
        [
            (web::http::header::LOCATION, to.into().to_string()),
            (web::http::header::SET_COOKIE, cookie.to_owned()),
        ],
    )
        .into_response()
}
