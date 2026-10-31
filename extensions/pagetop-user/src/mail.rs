//! Correos de `pagetop-user`: los que llevan un enlace de un solo uso (recuperación de contraseña,
//! verificación del email y confirmación de una nueva dirección) y los avisos de seguridad (cambio
//! de contraseña y petición de cambio de dirección).

use pagetop::prelude::*;
use pagetop_mail::prelude::*;

use crate::config;
use crate::entity::user;
use crate::token::TokenKind;
use crate::{EMAIL_PATH, LOCALES_USER, PASSWORD_RESET_PATH, VERIFY_PATH};

// Estilo del enlace principal, con aspecto de botón.
const BUTTON_STYLE: &str = concat!(
    "display: inline-block; padding: 10px 20px; border-radius: 4px; ",
    "background-color: #2563eb; color: #ffffff; text-decoration: none; font-weight: 600;"
);

// **< Notice >*************************************************************************************

/// Avisos de seguridad que se envían a la dirección actual de la cuenta.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Notice {
    /// El propio usuario ha cambiado su contraseña.
    PasswordChanged,
    /// Un administrador ha cambiado la contraseña del usuario.
    PasswordChangedByAdmin,
    /// Se ha pedido cambiar la dirección de la cuenta por otra, pendiente de confirmar.
    EmailChangeRequested,
}

// **< UserMail >***********************************************************************************

// Correo con un enlace principal: el que consume un token o, en los avisos, el de la recuperación
// de contraseña.
struct UserMail {
    // Claves del asunto, la introducción, el texto del enlace y la nota final.
    keys: [&'static str; 4],
    // Nombre de usuario para el saludo, sólo si la dirección está verificada (ver `recipient()`).
    username: Option<String>,
    link: String,
    // Horas que vale el enlace, si caduca.
    expires_hours: Option<u32>,
}

impl UserMail {
    // Correo con el enlace que consume un token del tipo indicado.
    fn token(kind: TokenKind, username: Option<String>, link: String) -> Self {
        let keys = match kind {
            TokenKind::PasswordReset => [
                "mail-reset-subject",
                "mail-reset-intro",
                "mail-reset-action",
                "mail-reset-ignore",
            ],
            TokenKind::EmailVerification => [
                "mail-verify-subject",
                "mail-verify-intro",
                "mail-verify-action",
                "mail-verify-ignore",
            ],
            TokenKind::EmailChange => [
                "mail-email-change-subject",
                "mail-email-change-intro",
                "mail-email-change-action",
                "mail-email-change-ignore",
            ],
        };
        UserMail {
            keys,
            username,
            link,
            expires_hours: Some(u32::try_from(kind.ttl_secs() / 3600).unwrap_or(u32::MAX)),
        }
    }

    // Aviso de seguridad, con el enlace a la recuperación de contraseña.
    fn notice(notice: Notice, username: Option<String>, link: String) -> Self {
        let [subject, intro] = match notice {
            Notice::PasswordChanged => [
                "mail-password-changed-subject",
                "mail-password-changed-intro",
            ],
            Notice::PasswordChangedByAdmin => [
                "mail-password-changed-subject",
                "mail-password-changed-admin-intro",
            ],
            Notice::EmailChangeRequested => [
                "mail-email-change-requested-subject",
                "mail-email-change-requested-intro",
            ],
        };
        UserMail {
            keys: [subject, intro, "mail-notice-action", "mail-notice-ignore"],
            username,
            link,
            expires_hours: None,
        }
    }
}

impl MailTemplate for UserMail {
    fn subject(&self, cx: &Context) -> String {
        Lc::t(self.keys[0], &LOCALES_USER)
            .with_arg("app", &global::SETTINGS.app.name)
            .lookup(cx)
            .unwrap_or_default()
    }

    fn html(&self, cx: &Context) -> Option<Markup> {
        let [_, intro, action, note] = self.keys;
        let greeting = match &self.username {
            // El nombre de usuario lo elige quien creó la cuenta: va con `with_arg()`, que lo
            // escapa, nunca con `with_markup()`.
            Some(username) => {
                Lc::t("mail-greeting", &LOCALES_USER).with_arg("username", username.clone())
            }
            None => Lc::t("mail-greeting-plain", &LOCALES_USER),
        };
        let expires = self
            .expires_hours
            .map(|hours| Lc::t("mail-link-expires", &LOCALES_USER).with_number("hours", hours));
        Some(html! {
            p { (greeting.using(cx)) }
            p { (Lc::t(intro, &LOCALES_USER).using(cx)) }
            p style="margin: 24px 0;" {
                a href=(self.link) style=(BUTTON_STYLE) { (Lc::t(action, &LOCALES_USER).using(cx)) }
            }
            @if let Some(expires) = expires {
                p { (expires.using(cx)) }
            }
            p style="color: #6b7280;" { (Lc::t(note, &LOCALES_USER).using(cx)) }
        })
    }
}

// **< Envío >**************************************************************************************

// Todos se envían en segundo plano: el tiempo de respuesta no debe revelar si existe la cuenta, y
// un fallo del servidor de correo no debe impedir completar la petición (se registra en las
// trazas). Salen en el idioma propio del usuario si lo tiene (y se aplica), o en el de la petición.

// Envía el correo con el enlace que consume `token` a la dirección actual del usuario.
pub(crate) fn send_token_mail(
    request: &HttpRequest,
    kind: TokenKind,
    user: &user::Model,
    token: &str,
) {
    let cx = user_context(request, user);
    let (username, to) = recipient(user);
    token_email(&cx, kind, user.id, username, to, token).send_later();
}

// Envía a `new_email` el enlace para confirmarla como nueva dirección del usuario.
//
// La dirección la ha escrito quien está en la cuenta y puede ser de otra persona: como con una
// dirección sin verificar (ver `recipient()`), el correo no incluye nada que haya elegido, ni
// siquiera el nombre de usuario.
pub(crate) fn send_email_change_mail(
    request: &HttpRequest,
    user: &user::Model,
    new_email: &str,
    token: &str,
) {
    let cx = user_context(request, user);
    let to = Mailbox::new(new_email);
    token_email(&cx, TokenKind::EmailChange, user.id, None, to, token).send_later();
}

// Envía un aviso de seguridad a la dirección actual del usuario.
pub(crate) fn send_notice(request: &HttpRequest, notice: Notice, user: &user::Model) {
    let cx = user_context(request, user);
    let (username, to) = recipient(user);
    notice_email(&cx, notice, username, to).send_later();
}

// Contexto de la petición con el idioma propio del usuario, si lo tiene y se aplica.
fn user_context(request: &HttpRequest, user: &user::Model) -> Context {
    let mut cx = Context::new(request.clone());
    if config::user_language_applies()
        && let Some(locale @ Locale::Resolved(_)) = user.language.as_deref().map(Locale::resolve)
    {
        cx.alter_langid(&locale);
    }
    cx
}

// Destinatario del correo y nombre de usuario para el saludo.
//
// Mientras la dirección no esté verificada, el correo puede llegar a otra persona: cualquiera puede
// crear una cuenta con una dirección ajena, y con `require_email_verification = false` la cuenta
// queda activa y puede pedir la recuperación de contraseña. El correo sale del dominio de la
// aplicación y con su diseño, así que pasa los filtros y resulta creíble: no incluye nada que elija
// quien creó la cuenta, ni el nombre de usuario ni el nombre visible (tampoco en la cabecera `To`),
// porque bastaría para colar un texto o un enlace engañoso (*phishing*). Se puede iniciar sesión
// con el email, así que el nombre de usuario no hace falta para recuperar el acceso.
fn recipient(user: &user::Model) -> (Option<String>, Mailbox) {
    if user.email_verified_at.is_none() {
        return (None, Mailbox::new(user.email.as_str()));
    }
    let name =
        util::non_blank(user.display_name.as_deref().unwrap_or_default()).unwrap_or(&user.username);
    (
        Some(user.username.clone()),
        Mailbox::from((name, user.email.as_str())),
    )
}

// Compone el correo con el enlace que consume `token` del usuario `user_id`.
fn token_email(
    cx: &Context,
    kind: TokenKind,
    user_id: i32,
    username: Option<String>,
    to: Mailbox,
    token: &str,
) -> Email {
    let base = match kind {
        TokenKind::PasswordReset => PASSWORD_RESET_PATH,
        TokenKind::EmailVerification => VERIFY_PATH,
        TokenKind::EmailChange => EMAIL_PATH,
    };
    let path = util::join!(base, "/", user_id.to_string(), "/", token);
    let template = UserMail::token(kind, username, absolute_url(cx.route(path)));
    Email::from_template(&template, cx).with_to(to)
}

// Compone un aviso de seguridad, con el enlace a la recuperación de contraseña.
fn notice_email(cx: &Context, notice: Notice, username: Option<String>, to: Mailbox) -> Email {
    let link = absolute_url(cx.route(PASSWORD_RESET_PATH));
    Email::from_template(&UserMail::notice(notice, username, link), cx).with_to(to)
}

// **< Vistas previas >*****************************************************************************

// Declara las vistas previas de los correos de `pagetop-user`, con datos de ejemplo.
pub(crate) fn declare_previews(previews: &mut MailPreviews) {
    previews.add(
        "user.email_verification",
        Lc::t("mail-preview-verify", &LOCALES_USER),
        |cx| sample_token(cx, TokenKind::EmailVerification),
    );
    previews.add(
        "user.password_reset",
        Lc::t("mail-preview-reset", &LOCALES_USER),
        |cx| sample_token(cx, TokenKind::PasswordReset),
    );
    previews.add(
        "user.email_change",
        Lc::t("mail-preview-email-change", &LOCALES_USER),
        |cx| sample_token(cx, TokenKind::EmailChange),
    );
    previews.add(
        "user.email_change_requested",
        Lc::t("mail-preview-email-change-requested", &LOCALES_USER),
        |cx| sample_notice(cx, Notice::EmailChangeRequested),
    );
    previews.add(
        "user.password_changed",
        Lc::t("mail-preview-password-changed", &LOCALES_USER),
        |cx| sample_notice(cx, Notice::PasswordChanged),
    );
    previews.add(
        "user.password_changed_by_admin",
        Lc::t("mail-preview-password-changed-admin", &LOCALES_USER),
        |cx| sample_notice(cx, Notice::PasswordChangedByAdmin),
    );
}

// La verificación y la confirmación de una nueva dirección se envían a una dirección sin verificar
// (sin nombre); el resto, a la de una cuenta verificada (ver `recipient()`).
fn sample_token(cx: &Context, kind: TokenKind) -> Email {
    let (username, to) = match kind {
        TokenKind::EmailVerification | TokenKind::EmailChange => {
            (None, Mailbox::new("ana@example.com"))
        }
        TokenKind::PasswordReset => sample_verified(),
    };
    token_email(cx, kind, 42, username, to, "EXAMPLE-TOKEN")
}

fn sample_notice(cx: &Context, notice: Notice) -> Email {
    let (username, to) = sample_verified();
    notice_email(cx, notice, username, to)
}

fn sample_verified() -> (Option<String>, Mailbox) {
    (
        Some("ana".to_owned()),
        Mailbox::from(("Ana Pérez", "ana@example.com")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const USERNAME: &str = r#"<a href="https://evil.example">ana</a>"#;

    fn html(username: Option<&str>) -> String {
        let mail = UserMail::token(
            TokenKind::PasswordReset,
            username.map(str::to_owned),
            "https://example.com/user/token".to_owned(),
        );
        mail.html(&Context::default()).unwrap().into_string()
    }

    // The username is chosen by whoever creates the account.
    #[pagetop::test]
    async fn username_cannot_inject_markup() {
        let html = html(Some(USERNAME));
        assert!(
            !html.contains("evil.example\">"),
            "username must be escaped: {html}"
        );
        assert!(html.contains("&lt;a href="));
    }

    // An unverified address may belong to someone else: nothing chosen by whoever created the
    // account may appear in the email.
    #[pagetop::test]
    async fn unverified_address_gets_a_plain_greeting() {
        let html = html(None);
        assert!(
            !html.contains("evil.example"),
            "username must not appear: {html}"
        );
    }

    // Notices link to the password reset page and have no expiry line.
    #[pagetop::test]
    async fn notices_link_to_password_reset_without_expiry() {
        let mail = UserMail::notice(
            Notice::PasswordChanged,
            None,
            "https://example.com/user/password/reset".to_owned(),
        );
        assert!(mail.expires_hours.is_none());
        let html = mail.html(&Context::default()).unwrap().into_string();
        assert!(html.contains("href=\"https://example.com/user/password/reset\""));
    }
}
