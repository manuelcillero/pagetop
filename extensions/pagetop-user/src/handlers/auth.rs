//! Handlers HTTP para autenticación y gestión de cuenta.

use serde::Deserialize;

use pagetop::auth::CurrentUser;
use pagetop::prelude::*;

use crate::auth;
use crate::component::{LoginForm, PasswordResetConfirmForm, PasswordResetForm, RegisterForm};
use crate::config::SETTINGS;
use crate::error::AuthError;
use crate::handlers::admin::map_auth_error;
use crate::password;
use crate::session;
use crate::token::{TokenKind, consume_token, create_token};
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
        auth::logout(&sid).await.ok();
    }
    let expiry = session::expiry_cookie();
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
        Ok(_user_id) => Redirect::see_other(cx.route(LOGIN_PATH)),
        Err(err) => {
            let error_lc = match &err {
                AuthError::PasswordTooShort(n) => {
                    Lc::t("error-password-too-short", &LOCALES_USER).with_arg("n", n.to_string())
                }
                AuthError::PasswordMismatch => Lc::t("error-password-mismatch", &LOCALES_USER),
                AuthError::UsernameTaken => Lc::t("error-username-taken", &LOCALES_USER),
                AuthError::EmailTaken => Lc::t("error-email-taken", &LOCALES_USER),
                _ => Lc::t("error-internal", &LOCALES_USER),
            };
            Page::new(request)
                .with_title(Lc::t("title-register", &LOCALES_USER))
                .with_child(RegisterForm::new().with_error(error_lc))
                .render()
                .await
                .into_response()
        }
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
    // Respondemos igual exista o no el email para no revelar qué emails están registrados.
    {
        use crate::entity::user;
        use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};
        if let Ok(Some(user_model)) = user::Entity::find()
            .filter(user::Column::Email.eq(&form.email))
            .one(dbconn())
            .await
        {
            create_token(user_model.id, TokenKind::PasswordReset)
                .await
                .ok();
            // TODO: enviar email con el token al usuario.
        }
    }
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
    let valid = {
        use crate::entity::user_token;
        use crate::token::hash_token;
        use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};
        let hash = hash_token(&token);
        let now = Utc::now().naive_utc();
        user_token::Entity::find()
            .filter(user_token::Column::TokenHash.eq(&hash))
            .filter(user_token::Column::UserId.eq(uid))
            .filter(user_token::Column::ConsumedAt.is_null())
            .one(dbconn())
            .await
            .ok()
            .flatten()
            .is_some_and(|r| r.expires_at > now)
    };
    if !valid {
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
pub async fn password_reset_confirm_post(
    request: HttpRequest,
    web::Path((_uid, token)): web::Path<(i32, String)>,
    web::Form(form): web::Form<PasswordResetConfirmFormData>,
) -> Response {
    let cx = Context::new(request.clone());
    if let Err(err) = password::passwords_match(&form.password, &form.confirm_password) {
        return Page::new(request)
            .with_title(Lc::t("title-new-password", &LOCALES_USER))
            .with_child(PasswordResetConfirmForm::new().with_error(map_auth_error(&err)))
            .render()
            .await
            .into_response();
    }
    let result = async {
        use crate::entity::user;
        use pagetop_seaorm::db::{ActiveModelTrait, Set, dbconn};
        let user_id = consume_token(&token, TokenKind::PasswordReset).await?;
        password::validate_strength(&form.password)?;
        let hash = password::hash_password(&form.password)?;
        let now = Utc::now().naive_utc();
        user::ActiveModel {
            id: Set(user_id),
            password_hash: Set(hash),
            updated_at: Set(now),
            ..Default::default()
        }
        .update(dbconn())
        .await?;
        session::destroy_user_sessions(user_id)
            .await
            .map_err(AuthError::Database)?;
        Ok::<(), AuthError>(())
    }
    .await;
    match result {
        Ok(()) => Redirect::see_other(cx.route(LOGIN_PATH)),
        Err(_) => Page::new(request)
            .with_title(Lc::t("title-new-password", &LOCALES_USER))
            .with_child(
                PasswordResetConfirmForm::new()
                    .with_error(Lc::t("error-token-invalid", &LOCALES_USER)),
            )
            .render()
            .await
            .into_response(),
    }
}

// **< verify_email_get >***************************************************************************

/// GET /user/verify/{uid}/{token} - Confirma la dirección de email.
pub async fn verify_email_get(
    request: HttpRequest,
    web::Path((_uid, token)): web::Path<(i32, String)>,
) -> Response {
    let cx = Context::new(request.clone());
    let result = async {
        use crate::account::UserStatus;
        use crate::entity::user;
        use pagetop_seaorm::db::{ActiveModelTrait, Set, dbconn};
        let user_id = consume_token(&token, TokenKind::EmailVerification).await?;
        let now = Utc::now().naive_utc();
        user::ActiveModel {
            id: Set(user_id),
            email_verified_at: Set(Some(now)),
            status: Set(UserStatus::Active.as_i16()),
            updated_at: Set(now),
            ..Default::default()
        }
        .update(dbconn())
        .await?;
        Ok::<(), AuthError>(())
    }
    .await;
    match result {
        Ok(()) => Redirect::see_other(cx.route(LOGIN_PATH)),
        Err(_) => ErrorPage::NotFound(Some(request)).into_response(),
    }
}

// **< helpers privados >***************************************************************************

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
