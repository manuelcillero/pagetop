// Entrega de los correos con el transporte configurado en `[mail]`.

use pagetop::prelude::*;

use crate::config::{self, MailTransport, SmtpSecurity, SmtpSettings};
use crate::{Email, MailError, Mailbox, outbox};

use lettre::address::Envelope;
use lettre::transport::smtp::PoolConfig;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use lettre::{
    Address, AsyncFileTransport, AsyncSendmailTransport, AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor,
};

use std::path::PathBuf;
use std::sync::OnceLock;

// Transporte y direcciones por defecto, construidos una sola vez a partir de los ajustes.
static MAILER: OnceLock<Mailer> = OnceLock::new();

pub(crate) struct Mailer {
    transport: Transport,
    from: Option<Mailbox>,
    reply_to: Option<Mailbox>,
    redirect_to: Option<Address>,
}

enum Transport {
    Smtp(AsyncSmtpTransport<Tokio1Executor>),
    Sendmail(AsyncSendmailTransport<Tokio1Executor>),
    File(AsyncFileTransport<Tokio1Executor>),
    Log,
    Memory,
}

// Devuelve el transporte, construyéndolo en el primer uso. `Mail::initialize()` lo construye al
// arrancar para detener la aplicación si los ajustes no son válidos; fuera de una aplicación (p.
// ej. en pruebas) se construye en el primer envío.
pub(crate) fn mailer() -> Result<&'static Mailer, MailError> {
    if let Some(mailer) = MAILER.get() {
        return Ok(mailer);
    }
    let mailer = Mailer::from_settings()?;
    // Si otro hilo se adelanta, se queda el suyo y se descarta éste.
    Ok(MAILER.get_or_init(|| mailer))
}

pub(crate) async fn deliver(email: Email) -> Result<(), MailError> {
    let mailer = mailer()?;
    let email = email.prepare(mailer.from.as_ref(), mailer.reply_to.as_ref());
    // Escribirlo en las trazas no necesita remitente: así una aplicación sin configurar muestra sus
    // correos aunque no tenga `mail.from`.
    if let Transport::Log = mailer.transport {
        return log_email(&email, mailer.redirect_to.as_ref());
    }
    let message = email.to_message()?;
    let envelope = match &mailer.redirect_to {
        Some(address) => Envelope::new(message.envelope().from().cloned(), vec![address.clone()])
            .map_err(MailError::Build)?,
        None => message.envelope().clone(),
    };
    let raw = message.formatted();

    match &mailer.transport {
        Transport::Smtp(smtp) => {
            smtp.send_raw(&envelope, &raw)
                .await
                .map_err(MailError::Smtp)?;
        }
        Transport::Sendmail(sendmail) => {
            sendmail
                .send_raw(&envelope, &raw)
                .await
                .map_err(MailError::Sendmail)?;
        }
        Transport::File(file) => {
            file.send_raw(&envelope, &raw)
                .await
                .map_err(MailError::File)?;
        }
        Transport::Memory => {
            outbox::push(email, recipients(&envelope), raw);
        }
        Transport::Log => unreachable!("log transport is handled before building the message"),
    }
    Ok(())
}

// Escribe el correo en las trazas en lugar de enviarlo (`transport = "log"`). Valida las
// direcciones igual que al enviar, salvo que falte el remitente.
fn log_email(email: &Email, redirect_to: Option<&Address>) -> Result<(), MailError> {
    for mailbox in email.from().into_iter().chain(email.reply_to()) {
        mailbox.to_lettre()?;
    }
    let mut recipients = Vec::new();
    for mailbox in email.to().iter().chain(email.cc()).chain(email.bcc()) {
        recipients.push(mailbox.to_lettre()?.email.to_string());
    }
    if recipients.is_empty() {
        return Err(MailError::NoRecipients);
    }
    if let Some(address) = redirect_to {
        recipients = vec![address.to_string()];
    }
    let body = email
        .text()
        .cloned()
        .or_else(|| email.html().map(|html| html.as_str().to_owned()))
        .unwrap_or_default();
    trace::info!(
        "Email not delivered (mail.transport = \"log\")\n\
         From: {}\nTo: {}\nSubject: {}\n\n{}",
        email.from().map(ToString::to_string).unwrap_or_default(),
        recipients.join(", "),
        email.subject(),
        body
    );
    Ok(())
}

// Completa un correo como si se fuera a enviar, para mostrar su vista previa. Si los ajustes no
// son válidos (la aplicación no habría arrancado), se muestra sin las direcciones por defecto.
pub(crate) fn prepare_preview(email: Email) -> Email {
    let mailer = mailer().ok();
    email.prepare(
        mailer.and_then(|m| m.from.as_ref()),
        mailer.and_then(|m| m.reply_to.as_ref()),
    )
}

// Direcciones a las que se entrega el correo.
fn recipients(envelope: &Envelope) -> Vec<String> {
    envelope.to().iter().map(ToString::to_string).collect()
}

impl Mailer {
    fn from_settings() -> Result<Self, MailError> {
        let settings = &*config::SETTINGS;
        let transport = match settings.transport {
            MailTransport::Smtp => Transport::Smtp(smtp_transport(&settings.smtp)?),
            MailTransport::Sendmail => Transport::Sendmail(match &settings.sendmail.command {
                Some(command) => AsyncSendmailTransport::new_with_command(command),
                None => AsyncSendmailTransport::new(),
            }),
            MailTransport::File => {
                let dir = PathBuf::from(&settings.file.dir);
                std::fs::create_dir_all(&dir).map_err(|source| MailError::CreateDir {
                    path: dir.clone(),
                    source,
                })?;
                Transport::File(AsyncFileTransport::new(dir))
            }
            MailTransport::Log => Transport::Log,
            MailTransport::Memory => Transport::Memory,
        };
        let redirect_to = match settings.redirect_to.as_deref().and_then(util::non_blank) {
            Some(address) => Some(address.parse::<Address>().map_err(|source| {
                invalid_setting(
                    "mail.redirect_to",
                    MailError::InvalidAddress {
                        address: address.to_owned(),
                        source,
                    },
                )
            })?),
            None => None,
        };
        if let Some(url) = settings.base_url.as_deref().and_then(util::non_blank)
            && !(url.starts_with("http://") || url.starts_with("https://"))
        {
            return Err(invalid_setting(
                "mail.base_url",
                MailError::InvalidUrl(url.to_owned()),
            ));
        }
        Ok(Mailer {
            transport,
            from: setting_mailbox("mail.from", settings.from.as_deref())?,
            reply_to: setting_mailbox("mail.reply_to", settings.reply_to.as_deref())?,
            redirect_to,
        })
    }

    // Avisa en las trazas de los ajustes que conviene revisar.
    pub(crate) fn log_startup(&self) {
        let settings = &*config::SETTINGS;
        match self.transport {
            Transport::Log => trace::warn!(
                "mail.transport is \"log\": emails are written to the log, not delivered"
            ),
            Transport::Memory => trace::warn!(
                "mail.transport is \"memory\": emails are kept in memory, not delivered \
                 (tests only)"
            ),
            Transport::Smtp(_) | Transport::Sendmail(_) | Transport::File(_) => {
                if self.from.is_none() {
                    trace::warn!("mail.from is not set: every email must set its own sender");
                }
                if settings
                    .base_url
                    .as_deref()
                    .and_then(util::non_blank)
                    .is_none()
                {
                    trace::warn!(
                        "mail.base_url is not set: links in emails point to {}",
                        crate::base_url()
                    );
                }
            }
        }
        if settings.previews {
            trace::warn!(
                "mail.previews is enabled: email previews are public at {} (development only)",
                crate::preview::PREVIEW_PATH
            );
        }
        if let Some(address) = &self.redirect_to {
            trace::warn!("mail.redirect_to is set: all emails are delivered only to {address}");
        }
    }
}

fn smtp_transport(smtp: &SmtpSettings) -> Result<AsyncSmtpTransport<Tokio1Executor>, MailError> {
    let tls_parameters = || {
        TlsParameters::builder(smtp.host.clone())
            .dangerous_accept_invalid_certs(smtp.accept_invalid_certs)
            .build()
            .map_err(|err| invalid_setting("mail.smtp", MailError::Smtp(err)))
    };
    let (tls, default_port) = match smtp.security {
        SmtpSecurity::None => (Tls::None, 25),
        SmtpSecurity::Opportunistic => (Tls::Opportunistic(tls_parameters()?), 25),
        SmtpSecurity::StartTls => (Tls::Required(tls_parameters()?), 587),
        SmtpSecurity::Tls => (Tls::Wrapper(tls_parameters()?), 465),
    };
    let mut builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.host)
        .port(smtp.port.unwrap_or(default_port))
        .tls(tls)
        .timeout(Some(std::time::Duration::from_secs(smtp.timeout_secs)))
        .pool_config(PoolConfig::new().max_size(smtp.pool_max_size));
    if let Some(username) = smtp.username.as_deref().and_then(util::non_blank) {
        builder = builder.credentials(Credentials::new(
            username.to_owned(),
            smtp.password.clone().unwrap_or_default(),
        ));
    }
    if let Some(name) = smtp.hello_name.as_deref().and_then(util::non_blank) {
        builder = builder.hello_name(ClientId::Domain(name.to_owned()));
    }
    Ok(builder.build())
}

// Lee una dirección de los ajustes, comprobando que sea válida.
fn setting_mailbox(key: &'static str, value: Option<&str>) -> Result<Option<Mailbox>, MailError> {
    let Some(value) = value.and_then(util::non_blank) else {
        return Ok(None);
    };
    let mailbox = Mailbox::from(value);
    mailbox
        .to_lettre()
        .map_err(|err| invalid_setting(key, err))?;
    Ok(Some(mailbox))
}

fn invalid_setting(key: &'static str, source: MailError) -> MailError {
    MailError::InvalidSetting {
        key,
        source: Box::new(source),
    }
}
