//! Correos con enlaces de un solo uso: recuperación de contraseña y verificación de email.

use pagetop::prelude::*;
use pagetop_mail::prelude::*;

use crate::config;
use crate::token::TokenKind;
use crate::{LOCALES_USER, PASSWORD_RESET_PATH, VERIFY_PATH};

// Estilo del enlace principal, con aspecto de botón.
const BUTTON_STYLE: &str = concat!(
    "display: inline-block; padding: 10px 20px; border-radius: 4px; ",
    "background-color: #2563eb; color: #ffffff; text-decoration: none; font-weight: 600;"
);

// Correo con el enlace que consume un token del usuario.
struct TokenMail {
    kind: TokenKind,
    // Nombre de usuario para el saludo, sólo si la dirección está verificada (ver `recipient()`).
    username: Option<String>,
    link: String,
}

impl TokenMail {
    // Claves de los textos propios de cada tipo de token: asunto, introducción, texto del enlace y
    // aviso para quien no lo haya solicitado.
    fn keys(&self) -> [&'static str; 4] {
        match self.kind {
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
        }
    }

    fn expires(&self) -> Lc {
        let hours = u32::try_from(self.kind.ttl_secs() / 3600).unwrap_or(u32::MAX);
        Lc::t("mail-link-expires", &LOCALES_USER).with_number("hours", hours)
    }
}

impl MailTemplate for TokenMail {
    fn subject(&self, cx: &Context) -> String {
        Lc::t(self.keys()[0], &LOCALES_USER)
            .with_arg("app", &global::SETTINGS.app.name)
            .lookup(cx)
            .unwrap_or_default()
    }

    fn html(&self, cx: &Context) -> Option<Markup> {
        let [_, intro, action, ignore] = self.keys();
        let greeting = match &self.username {
            // El nombre de usuario lo elige quien creó la cuenta: va con `with_arg()`, que lo
            // escapa, nunca con `with_markup()`.
            Some(username) => {
                Lc::t("mail-greeting", &LOCALES_USER).with_arg("username", username.clone())
            }
            None => Lc::t("mail-greeting-plain", &LOCALES_USER),
        };
        Some(html! {
            p { (greeting.using(cx)) }
            p { (Lc::t(intro, &LOCALES_USER).using(cx)) }
            p style="margin: 24px 0;" {
                a href=(self.link) style=(BUTTON_STYLE) { (Lc::t(action, &LOCALES_USER).using(cx)) }
            }
            p { (self.expires().using(cx)) }
            p style="color: #6b7280;" { (Lc::t(ignore, &LOCALES_USER).using(cx)) }
        })
    }
}

// Envía en segundo plano el correo con el enlace que consume `token`.
//
// El correo sale en el idioma propio del usuario si lo tiene (y se aplica), o en el de la petición
// en curso. Se envía en segundo plano para que el tiempo de respuesta no revele si existe la
// cuenta, y porque un fallo del servidor de correo no debe impedir completar la petición: se
// registra en las trazas.
pub(crate) fn send_token_mail(
    request: &HttpRequest,
    kind: TokenKind,
    user: &crate::entity::user::Model,
    token: &str,
) {
    let mut cx = Context::new(request.clone());
    if config::user_language_applies()
        && let Some(locale @ Locale::Resolved(_)) = user.language.as_deref().map(Locale::resolve)
    {
        cx.alter_langid(&locale);
    }
    let (username, to) = recipient(user);
    token_email(&cx, kind, user.id, username, to, token).send_later();
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
fn recipient(user: &crate::entity::user::Model) -> (Option<String>, Mailbox) {
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
    };
    let path = util::join!(base, "/", user_id.to_string(), "/", token);
    let template = TokenMail {
        kind,
        username,
        link: absolute_url(cx.route(path)),
    };
    Email::from_template(&template, cx).with_to(to)
}

// **< Vistas previas >*****************************************************************************

// Declara las vistas previas de los correos de `pagetop-user`, con datos de ejemplo.
pub(crate) fn declare_previews(previews: &mut MailPreviews) {
    previews.add(
        "user.email_verification",
        Lc::t("mail-preview-verify", &LOCALES_USER),
        |cx| sample(cx, TokenKind::EmailVerification),
    );
    previews.add(
        "user.password_reset",
        Lc::t("mail-preview-reset", &LOCALES_USER),
        |cx| sample(cx, TokenKind::PasswordReset),
    );
}

// La verificación se envía a una dirección sin verificar (sin nombre); la recuperación, a la de una
// cuenta verificada (ver `recipient()`).
fn sample(cx: &Context, kind: TokenKind) -> Email {
    let (username, to) = match kind {
        TokenKind::EmailVerification => (None, Mailbox::new("ana@example.com")),
        TokenKind::PasswordReset => (
            Some("ana".to_owned()),
            Mailbox::from(("Ana Pérez", "ana@example.com")),
        ),
    };
    token_email(cx, kind, 42, username, to, "EXAMPLE-TOKEN")
}

#[cfg(test)]
mod tests {
    use super::*;

    const USERNAME: &str = r#"<a href="https://evil.example">ana</a>"#;

    fn html(username: Option<&str>) -> String {
        let mail = TokenMail {
            kind: TokenKind::PasswordReset,
            username: username.map(str::to_owned),
            link: "https://example.com/user/token".to_owned(),
        };
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
}
