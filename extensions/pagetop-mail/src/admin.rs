//! Página del panel de administración para comprobar la configuración del correo enviando un correo
//! de prueba.

use pagetop::prelude::*;

use serde::Deserialize;

use crate::config::{self, MailTransport, SmtpSecurity};
use crate::{Email, LOCALES_MAIL, Mailbox};

// Ruta de la página, relativa a la raíz del panel.
const TEST_MAIL: &str = "config/mail";

// **< Permiso >************************************************************************************

/// Permisos propios de `pagetop-mail`.
#[derive(Clone, Copy, Debug)]
pub enum MailPermission {
    /// Ver la configuración del correo (sin credenciales) y enviar correos de prueba desde el panel
    /// de administración.
    SendTest,
}

impl Permission for MailPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::SendTest => "mail:send_test".into(),
        }
    }

    fn label(&self) -> Lc {
        match self {
            Self::SendTest => Lc::t("perm_send_test", &LOCALES_MAIL),
        }
    }

    fn group(&self) -> &'static str {
        "mail"
    }

    fn group_label(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_MAIL)
    }
}

pub(crate) fn declare_permissions(registry: &mut PermissionRegistry) {
    registry.register(&MailPermission::SendTest);
}

// **< Página del panel >***************************************************************************

pub(crate) fn declare_pages(bag: &mut AdminPageBag) {
    bag.add(
        AdminPage::new(
            TEST_MAIL,
            "config",
            Lc::t("test_title", &LOCALES_MAIL),
            &MailPermission::SendTest,
        )
        .with_description(Lc::t("test_description", &LOCALES_MAIL)),
    );
}

pub(crate) fn configure_router(routes: Routes) -> Routes {
    routes.admin_route(TEST_MAIL, web::get(test_mail_get).post(test_mail_post))
}

// Muestra la configuración del correo y el formulario para enviar un correo de prueba.
async fn test_mail_get(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let mut page = Page::admin(request).with_title(Lc::t("test_title", &LOCALES_MAIL));
    if let Some(warning) = not_delivered_warning() {
        page.push_message(MessageLevel::Warning, warning);
    }
    page.with_child(settings_summary())
        .with_child(test_form())
        .render()
        .await
}

#[derive(Deserialize)]
struct TestMailForm {
    to: String,
}

// Envía el correo de prueba y vuelve a la página con el resultado. Espera a la entrega, sin pasar
// por la cola, para poder informar del error del transporte.
async fn test_mail_post(
    request: HttpRequest,
    web::Form(form): web::Form<TestMailForm>,
) -> Result<Response, ErrorPage> {
    let cx = Context::admin(request);
    let to = form.to.trim().to_owned();
    let back = Redirect::see_other(cx.route(admin_path(TEST_MAIL)));

    if Mailbox::new(to.as_str()).validate().is_err() {
        let text = Lc::t("test_invalid_address", &LOCALES_MAIL).with_arg("to", to);
        return Ok(back.with_message(&cx, MessageLevel::Error, text));
    }

    let app = global::SETTINGS.app.name.clone();
    let subject = Lc::t("test_subject", &LOCALES_MAIL).with_arg("app", app.clone());
    let body = Lc::t("test_body", &LOCALES_MAIL).with_arg("app", app);
    let result = Email::new()
        .with_to(to.as_str())
        .with_subject(subject.lookup(&cx).unwrap_or_default())
        .with_html(html! { p { (body.using(&cx)) } })
        .send()
        .await;

    Ok(match result {
        Ok(()) => {
            trace::info!("Test email sent to {to}");
            let text = Lc::t("test_sent", &LOCALES_MAIL).with_arg("to", to);
            back.with_message(&cx, MessageLevel::Info, text)
        }
        Err(err) => {
            trace::warn!("Test email to {to} failed: {err}");
            let text = Lc::t("test_failed", &LOCALES_MAIL).with_arg("error", err.to_string());
            back.with_message(&cx, MessageLevel::Error, text)
        }
    })
}

// Aviso si el transporte configurado no entrega los correos.
fn not_delivered_warning() -> Option<Lc> {
    let key = match config::SETTINGS.transport {
        MailTransport::Log => "test_log_warning",
        MailTransport::Memory => "test_memory_warning",
        _ => return None,
    };
    Some(Lc::t(key, &LOCALES_MAIL))
}

// Ajustes del correo que ayudan a interpretar el resultado de la prueba, nunca las credenciales.
fn settings_summary() -> Block {
    let settings = &config::SETTINGS;
    let none = || Lc::t("test_not_set", &LOCALES_MAIL);
    let text = |value: &Option<String>| value.clone().map_or_else(none, Lc::n);

    let transport = match settings.transport {
        MailTransport::Smtp => "transport_smtp",
        MailTransport::Sendmail => "transport_sendmail",
        MailTransport::File => "transport_file",
        MailTransport::Log => "transport_log",
        MailTransport::Memory => "transport_memory",
    };
    let mut rows = vec![
        ("test_transport", Lc::t(transport, &LOCALES_MAIL)),
        ("test_from", text(&settings.from)),
        ("test_reply_to", text(&settings.reply_to)),
    ];
    if settings.redirect_to.is_some() {
        rows.push(("test_redirect_to", text(&settings.redirect_to)));
    }
    match settings.transport {
        MailTransport::Smtp => {
            let smtp = &settings.smtp;
            let (security, default_port) = match smtp.security {
                SmtpSecurity::None => ("security_none", 25),
                SmtpSecurity::Opportunistic => ("security_opportunistic", 25),
                SmtpSecurity::StartTls => ("security_starttls", 587),
                SmtpSecurity::Tls => ("security_tls", 465),
            };
            let port = smtp.port.unwrap_or(default_port).to_string();
            rows.push(("test_smtp_server", Lc::n(util::join!(smtp.host, ":", port))));
            rows.push(("test_smtp_security", Lc::t(security, &LOCALES_MAIL)));
        }
        MailTransport::Sendmail => {
            rows.push(("test_sendmail_command", text(&settings.sendmail.command)));
        }
        MailTransport::File => {
            rows.push(("test_file_dir", Lc::n(settings.file.dir.clone())));
        }
        MailTransport::Log | MailTransport::Memory => {}
    }

    let mut table = Table::new();
    for (label, value) in rows {
        table = table.with_row(
            table::Row::new()
                .with_cell(Lc::t(label, &LOCALES_MAIL))
                .with_cell(value),
        );
    }
    Block::new()
        .with_title(Lc::t("test_settings", &LOCALES_MAIL))
        .with_child(table)
}

fn test_form() -> Form {
    Form::new()
        .with_id("mail-test-form")
        .with_action(admin_path(TEST_MAIL))
        .with_method(form::Method::Post)
        .with_child(
            form::input::Field::email()
                .with_name("to")
                .with_label(Lc::t("test_to", &LOCALES_MAIL))
                .with_autocomplete(Some(form::Autocomplete::email()))
                .with_required(true)
                .with_maxlength(254),
        )
        .with_child(
            Button::submit(Lc::t("test_send", &LOCALES_MAIL))
                .with_style(button::Style::Solid(Intent::Primary)),
        )
}
