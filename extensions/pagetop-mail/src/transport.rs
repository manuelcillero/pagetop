// Entrega de los correos con el transporte configurado en `[mail]`.

use pagetop::prelude::*;

use crate::config::{self, MailTransport, SmtpSecurity, SmtpSettings};
use crate::{Email, MailError, Mailbox, outbox};

use lettre::Transport as _;
use lettre::address::Envelope;
use lettre::transport::smtp::PoolConfig;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use lettre::{
    Address, AsyncFileTransport, AsyncSendmailTransport, AsyncTransport, SmtpTransport,
    Tokio1Executor,
};

use std::path::PathBuf;
use std::sync::OnceLock;

use tokio::sync::{Semaphore, SemaphorePermit};

// Transporte y direcciones por defecto, construidos una sola vez a partir de los ajustes.
static MAILER: OnceLock<Mailer> = OnceLock::new();

pub(crate) struct Mailer {
    transport: Transport,
    // Turnos de envío: como mucho tantas entregas a la vez.
    slots: Semaphore,
    // Plazas para las entregas en curso o que esperan turno.
    tickets: Semaphore,
    from: Option<Mailbox>,
    reply_to: Option<Mailbox>,
    redirect_to: Option<Address>,
}

// Plaza para una entrega, obtenida con `Mailer::ticket()`.
pub(crate) type Ticket = SemaphorePermit<'static>;

enum Transport {
    // El transporte SMTP síncrono, no el asíncrono: ver `deliver_raw()`.
    Smtp(SmtpTransport),
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
    if mailer.is_log() {
        return log(mailer, email);
    }
    let ticket = mailer.ticket()?;
    let Composed {
        email,
        envelope,
        raw,
    } = compose(mailer, email)?;
    deliver_raw(mailer, ticket, &envelope, raw, Some(email)).await
}

// Correo ya completo y compuesto, listo para entregarlo o guardarlo en la cola de envío.
pub(crate) struct Composed {
    // El correo completo, con las direcciones por defecto y el documento HTML con el diseño común.
    pub(crate) email: Email,
    // El sobre con los destinatarios originales: `mail.redirect_to` se aplica al entregarlo.
    pub(crate) envelope: Envelope,
    // El mensaje en formato RFC 5322.
    pub(crate) raw: Vec<u8>,
}

pub(crate) fn compose(mailer: &Mailer, email: Email) -> Result<Composed, MailError> {
    let email = email.prepare(mailer.from.as_ref(), mailer.reply_to.as_ref());
    let message = email.to_message()?;
    Ok(Composed {
        envelope: message.envelope().clone(),
        raw: message.formatted(),
        email,
    })
}

// Entrega un mensaje ya compuesto, con la plaza obtenida antes con `Mailer::ticket()`. Espera su
// turno de envío sin ocupar un hilo. El transporte "memory" guarda también el correo completo, si
// se conoce, para que las pruebas lo consulten.
//
// SMTP usa el transporte síncrono de `lettre` en un hilo de `spawn_blocking`, porque el asíncrono
// (0.11) sólo aplica `mail.smtp.timeout_secs` a la conexión TCP: un servidor que deja de responder
// colgaría el envío para siempre. El síncrono lo aplica a cada lectura y escritura, y como un hilo
// no se cancela, cada envío termina o falla (salvo en el saludo TLS, ver `within_time_limit()`), y
// `lettre` descarta la conexión que falla: nunca vuelve a su *pool* una conexión a medio enviar,
// cuyas respuestas quedarían desfasadas.
pub(crate) async fn deliver_raw(
    mailer: &'static Mailer,
    _ticket: Ticket,
    envelope: &Envelope,
    raw: Vec<u8>,
    email: Option<Email>,
) -> Result<(), MailError> {
    // Sólo "memory" usa el correo completo. Los demás lo liberan antes de esperar turno: sus
    // adjuntos también van en el mensaje compuesto, y cada correo en espera los tendría dos veces.
    let email = email.filter(|_| matches!(mailer.transport, Transport::Memory));
    let redirected;
    let envelope = match &mailer.redirect_to {
        Some(address) => {
            redirected = Envelope::new(envelope.from().cloned(), vec![address.clone()])
                .map_err(MailError::Build)?;
            &redirected
        }
        None => envelope,
    };

    match &mailer.transport {
        Transport::Smtp(smtp) => {
            let (smtp, envelope) = (smtp.clone(), envelope.clone());
            // Cada envío en curso ocupa un hilo del *pool* de bloqueo de Tokio, que comparten otras
            // tareas (p. ej. el cálculo de contraseñas): sin turnos, una avalancha de correos con
            // el servidor lento lo agotaría. El turno pasa al hilo para liberarse cuando termina el
            // envío, no cuando se deja de esperarlo.
            let delivery = async move {
                let slot = mailer.slot().await;
                tokio::task::spawn_blocking(move || {
                    let _slot = slot;
                    smtp.send_raw(&envelope, &raw)
                })
                .await
            };
            match within_time_limit(delivery).await? {
                Ok(result) => result.map_err(MailError::Smtp)?,
                Err(err) if err.is_panic() => std::panic::resume_unwind(err.into_panic()),
                Err(_) => return Err(MailError::Interrupted),
            };
        }
        Transport::Sendmail(sendmail) => {
            within_time_limit(async {
                let _slot = mailer.slot().await;
                sendmail.send_raw(envelope, &raw).await
            })
            .await?
            .map_err(MailError::Sendmail)?;
        }
        Transport::File(file) => {
            within_time_limit(async {
                let _slot = mailer.slot().await;
                file.send_raw(envelope, &raw).await
            })
            .await?
            .map_err(MailError::File)?;
        }
        Transport::Memory => match email {
            Some(email) => outbox::push(email, recipients(envelope), raw),
            None => trace::warn!(
                "Email for {} not kept in memory: it was queued by another process or before a \
                 restart",
                recipients(envelope).join(", ")
            ),
        },
        // Sólo llega aquí un correo que se guardó en la cola con otro transporte.
        Transport::Log => trace::info!(
            "Email not delivered (mail.transport = \"log\")\nTo: {}\n\n{}",
            recipients(envelope).join(", "),
            String::from_utf8_lossy(&raw)
        ),
    }
    Ok(())
}

// Limita el tiempo de una entrega completa, como red de seguridad: la espera del turno de envío,
// *sendmail* y los archivos no tienen límite propio, y en SMTP `mail.smtp.timeout_secs` no cubre el
// saludo TLS con `security = "tls"`, que `lettre` hace antes de fijar los límites del *socket*.
//
// Cancelar *sendmail* es seguro (`lettre` mata el proceso). En SMTP sólo se deja de esperar: el
// hilo sigue hasta terminar o fallar por su cuenta, y si termina, el correo se habrá entregado
// aunque aquí se haya dado por fallido (la cola lo reintentará: entrega «al menos una vez»). En el
// saludo TLS, sin límite, el hilo sigue hasta que el sistema da la conexión por perdida, o para
// siempre si el servidor la mantiene viva sin responder, y retiene su turno mientras tanto.
async fn within_time_limit<T>(delivery: impl Future<Output = T>) -> Result<T, MailError> {
    let secs = config::SETTINGS.delivery_timeout_secs;
    tokio::time::timeout(std::time::Duration::from_secs(secs), delivery)
        .await
        .map_err(|_| MailError::Timeout(secs))
}

// Completa el correo y lo escribe en las trazas. No necesita remitente: así una aplicación sin
// configurar muestra sus correos aunque no tenga `mail.from`.
pub(crate) fn log(mailer: &Mailer, email: Email) -> Result<(), MailError> {
    let email = email.prepare(mailer.from.as_ref(), mailer.reply_to.as_ref());
    log_email(&email, mailer.redirect_to.as_ref())
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

// Completa un correo como si se fuera a enviar, para mostrar su vista previa. Si los ajustes no son
// válidos (la aplicación no habría arrancado), se muestra sin las direcciones por defecto.
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
    // Indica si los correos sólo se escriben en las trazas (`transport = "log"`).
    pub(crate) fn is_log(&self) -> bool {
        matches!(self.transport, Transport::Log)
    }

    // Reserva una plaza para entregar un correo. Si ya esperan turno `mail.max_waiting` entregas
    // por cada envío simultáneo, falla enseguida con `MailError::Overloaded`: en una avalancha de
    // correos que el servidor no despacha a tiempo, acumularlos en memoria sólo retrasaría el
    // colapso, y cada uno acabaría fallando igualmente por tiempo agotado.
    pub(crate) fn ticket(&'static self) -> Result<Ticket, MailError> {
        self.tickets
            .try_acquire()
            .map_err(|_| MailError::Overloaded)
    }

    // Espera un turno de envío.
    async fn slot(&self) -> SemaphorePermit<'_> {
        self.slots
            .acquire()
            .await
            .expect("mail delivery slots are never closed")
    }

    // Indica si los correos se guardan en memoria para las pruebas (`transport = "memory"`).
    #[cfg(feature = "queue")]
    pub(crate) fn is_memory(&self) -> bool {
        matches!(self.transport, Transport::Memory)
    }

    fn from_settings() -> Result<Self, MailError> {
        let settings = &*config::SETTINGS;
        check_range(
            "mail.delivery_timeout_secs",
            settings.delivery_timeout_secs,
            1,
            86_400,
        )?;
        #[cfg(feature = "queue")]
        crate::queue::check_settings()?;
        let slots = match settings.transport {
            MailTransport::Smtp => slots("mail.smtp.pool_max_size", settings.smtp.pool_max_size)?,
            MailTransport::Sendmail => slots(
                "mail.sendmail.max_concurrent",
                settings.sendmail.max_concurrent,
            )?,
            MailTransport::File => slots("mail.file.max_concurrent", settings.file.max_concurrent)?,
            // Sin turnos: se entregan al momento.
            MailTransport::Log | MailTransport::Memory => 1,
        };
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
            slots: Semaphore::new(slots),
            tickets: Semaphore::new(tickets(slots, settings.max_waiting)),
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
                if matches!(self.transport, Transport::Smtp(_))
                    && settings.delivery_timeout_secs
                        <= settings.smtp.timeout_secs.saturating_mul(2)
                {
                    trace::warn!(
                        "mail.delivery_timeout_secs should be well above twice \
                         mail.smtp.timeout_secs: slow but valid deliveries may fail"
                    );
                }
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

// Envíos simultáneos según el ajuste `key`, que debe ser al menos uno.
fn slots(key: &'static str, setting: u32) -> Result<usize, MailError> {
    check_range(key, setting.into(), 1, u32::MAX.into())?;
    Ok(usize::try_from(setting)
        .unwrap_or(usize::MAX)
        .min(Semaphore::MAX_PERMITS))
}

// Plazas para las entregas: las que caben en curso y las que pueden esperar turno.
fn tickets(slots: usize, max_waiting: u32) -> usize {
    let waiting = usize::try_from(max_waiting).unwrap_or(usize::MAX);
    slots
        .saturating_mul(waiting.saturating_add(1))
        .min(Semaphore::MAX_PERMITS)
}

fn smtp_transport(smtp: &SmtpSettings) -> Result<SmtpTransport, MailError> {
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
    let mut builder = SmtpTransport::builder_dangerous(&smtp.host)
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

// Comprueba que un ajuste numérico esté entre `min` y `max`, ambos incluidos.
pub(crate) fn check_range(
    key: &'static str,
    value: u64,
    min: u64,
    max: u64,
) -> Result<(), MailError> {
    if (min..=max).contains(&value) {
        return Ok(());
    }
    Err(invalid_setting(
        key,
        MailError::OutOfRange { value, min, max },
    ))
}

fn invalid_setting(key: &'static str, source: MailError) -> MailError {
    MailError::InvalidSetting {
        key,
        source: Box::new(source),
    }
}

#[cfg(test)]
mod tests {
    use super::check_range;

    #[test]
    fn out_of_range_settings_name_the_key_and_the_range() {
        let err = check_range("mail.delivery_timeout_secs", 0, 1, 86_400).unwrap_err();
        assert_eq!(
            err.describe(),
            "invalid mail setting \"mail.delivery_timeout_secs\": 0 is not between 1 and 86400"
        );
        assert!(check_range("mail.delivery_timeout_secs", 1, 1, 86_400).is_ok());
        assert!(check_range("mail.delivery_timeout_secs", 86_400, 1, 86_400).is_ok());
        assert!(check_range("mail.delivery_timeout_secs", 86_401, 1, 86_400).is_err());
    }
}
