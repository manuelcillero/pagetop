//! Correos electrónicos ([`Email`]).

use pagetop::prelude::*;

use crate::{Attachment, MailError, MailTemplate, Mailbox, layout, text, transport};

use lettre::Message;
use lettre::message::{MultiPart, MultiPartBuilder, SinglePart};

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Correo electrónico listo para enviar.
///
/// Se compone con los métodos `with_*()` y se envía con [`send()`](Self::send), que espera a que
/// el servidor lo acepte, o con [`send_later()`](Self::send_later), que lo envía en segundo plano.
/// También puede crearse a partir de una plantilla con [`from_template()`](Self::from_template).
///
/// Un correo puede tener texto plano, HTML o ambos (los clientes de correo muestran el que
/// prefieran), además de archivos adjuntos y de imágenes incrustadas en el HTML (ver
/// [`Attachment`]). El HTML de [`with_html()`](Self::with_html) es sólo el contenido: al enviarlo
/// se envuelve en el diseño común de los correos (ver
/// [`RenderMailLayout`](crate::RenderMailLayout)).
///
/// Si sólo tiene HTML, al enviarlo se genera el texto plano a partir de él, porque los filtros de
/// correo no deseado penalizan los correos sin texto. Escríbelo con
/// [`with_text()`](Self::with_text) sólo si el generado no queda bien.
///
/// Si no se indica el remitente o la dirección de respuesta se usan los de la configuración
/// (`mail.from` y `mail.reply_to`). Las direcciones se validan al enviar.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_mail::prelude::*;
///
/// async fn send_report(pdf: Vec<u8>) -> Result<(), MailError> {
///     Email::new()
///         .with_to("Ana Pérez <ana@example.com>")
///         .with_cc("contabilidad@example.com")
///         .with_subject("Informe mensual")
///         .with_text("Adjunto el informe del mes.")
///         .with_html(html! {
///             p { "Adjunto el " strong { "informe" } " del mes." }
///         })
///         .with_attachment(Attachment::new("informe.pdf", pdf))
///         .send()
///         .await
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Email {
    /// Remitente, si se ha indicado.
    from: Option<Mailbox>,
    /// Dirección de respuesta, si se ha indicado.
    reply_to: Option<Mailbox>,
    /// Destinatarios principales.
    to: Vec<Mailbox>,
    /// Destinatarios en copia.
    cc: Vec<Mailbox>,
    /// Destinatarios en copia oculta.
    bcc: Vec<Mailbox>,
    /// Asunto.
    subject: String,
    /// Texto plano, si lo tiene.
    text: Option<String>,
    /// HTML, si lo tiene: el contenido mientras se compone y el documento completo una vez
    /// enviado.
    html: Option<Markup>,
    #[getters(skip)]
    html_is_document: bool,
    /// Archivos adjuntos e imágenes incrustadas.
    attachments: Vec<Attachment>,
    #[getters(skip)]
    langid: Option<&'static LanguageIdentifier>,
}

impl Email {
    /// Crea un correo vacío.
    pub fn new() -> Self {
        Email::default()
    }

    /// Crea un correo con el asunto, el HTML y el texto de una plantilla, generados en el idioma
    /// del contexto, que también es el del diseño común del HTML.
    ///
    /// El contexto puede ser el de la petición en curso, si el destinatario es quien la hace (p.
    /// ej. al registrarse), o uno creado para el destinatario con
    /// `Context::default().with_langid(...)`.
    pub fn from_template(template: &impl MailTemplate, cx: &Context) -> Self {
        let mut email = Email::new()
            .with_subject(template.subject(cx))
            .with_langid(cx);
        if let Some(html) = template.html(cx) {
            email.alter_html(html);
        }
        if let Some(text) = template.text(cx) {
            email.alter_text(text);
        }
        email
    }

    /// Idioma del correo: el indicado con [`with_langid()`](Self::with_langid) o, si no, el idioma
    /// por defecto de la aplicación.
    pub fn langid(&self) -> &'static LanguageIdentifier {
        self.langid.unwrap_or_else(Locale::default_langid)
    }

    /// Envía el correo y espera a que se entregue (al servidor SMTP, a *sendmail*, al archivo...).
    ///
    /// Falla si alguna dirección o el tipo MIME de algún adjunto no son válidos, si falta el
    /// remitente (salvo con `transport = "log"`, que sólo escribe el correo en las trazas) o los
    /// destinatarios, o si la entrega falla.
    pub async fn send(self) -> Result<(), MailError> {
        transport::deliver(self).await
    }

    /// Envía el correo en segundo plano, sin esperar al resultado. Si falla, se registra el error
    /// en las trazas.
    ///
    /// La respuesta de la petición no espera al servidor de correo, y su tiempo no revela si el
    /// correo se ha enviado (p. ej., si existe la cuenta para la que se pide recuperar la
    /// contraseña). A cambio, los correos pendientes se pierden si la aplicación se detiene antes
    /// de enviarlos, y no se reintentan si fallan.
    pub fn send_later(self) {
        tokio::spawn(async move {
            let subject = self.subject.clone();
            if let Err(err) = self.send().await {
                let mut message = err.to_string();
                let mut source = std::error::Error::source(&err);
                while let Some(cause) = source {
                    message.push_str(": ");
                    message.push_str(&cause.to_string());
                    source = cause.source();
                }
                trace::error!("Cannot send email \"{subject}\": {message}");
            }
        });
    }

    // Completa el correo antes de enviarlo: remitente y dirección de respuesta por defecto, el
    // texto plano a partir del HTML si no tiene, y el documento HTML con el diseño común. El texto
    // se genera antes del diseño para que no incluya su pie.
    pub(crate) fn prepare(mut self, from: Option<&Mailbox>, reply_to: Option<&Mailbox>) -> Self {
        if self.from.is_none() {
            self.from = from.cloned();
        }
        if self.reply_to.is_none() {
            self.reply_to = reply_to.cloned();
        }
        if self.text.is_none()
            && let Some(html) = &self.html
        {
            self.text = Some(text::html_to_text(html.as_str()));
        }
        if !self.html_is_document
            && let Some(content) = self.html.take()
        {
            let cx = Context::default().with_langid(&Locale::Resolved(self.langid()));
            self.html = Some(layout::render(&cx, &self, &content));
            self.html_is_document = true;
        }
        self
    }

    // Compone el mensaje MIME:
    //
    // - texto y HTML -> multipart/alternative (el texto primero, el preferido al final).
    // - HTML con imágenes incrustadas -> multipart/related con el HTML y las imágenes.
    // - con adjuntos -> multipart/mixed con el cuerpo y los adjuntos.
    pub(crate) fn to_message(&self) -> Result<Message, MailError> {
        let from = self.from.as_ref().ok_or(MailError::MissingFrom)?;
        if self.to.is_empty() && self.cc.is_empty() && self.bcc.is_empty() {
            return Err(MailError::NoRecipients);
        }

        let from = from.to_lettre()?;
        let mut builder = Message::builder()
            .message_id(Some(message_id(from.email.domain())))
            .from(from)
            .subject(self.subject.as_str());
        if let Some(reply_to) = &self.reply_to {
            builder = builder.reply_to(reply_to.to_lettre()?);
        }
        for mailbox in &self.to {
            builder = builder.to(mailbox.to_lettre()?);
        }
        for mailbox in &self.cc {
            builder = builder.cc(mailbox.to_lettre()?);
        }
        for mailbox in &self.bcc {
            builder = builder.bcc(mailbox.to_lettre()?);
        }

        let has_html = self.html.is_some();
        let html = match &self.html {
            Some(html) => {
                let part = SinglePart::html(html.as_str().to_owned());
                let mut inline = self.attachments.iter().filter(|a| a.is_inline()).peekable();
                if inline.peek().is_some() {
                    let images = inline
                        .map(|attachment| attachment.to_part(true).map(Part::Single))
                        .collect::<Result<Vec<_>, _>>()?;
                    Some(multipart(MultiPart::related(), Part::Single(part), images))
                } else {
                    Some(Part::Single(part))
                }
            }
            None => None,
        };
        let text = self
            .text
            .as_ref()
            .map(|t| Part::Single(SinglePart::plain(t.clone())));
        let mut body = match (text, html) {
            (Some(text), Some(html)) => multipart(MultiPart::alternative(), text, vec![html]),
            (Some(part), None) | (None, Some(part)) => part,
            (None, None) => Part::Single(SinglePart::plain(String::new())),
        };

        let attached = self
            .attachments
            .iter()
            .filter(|a| !(has_html && a.is_inline()))
            .map(|attachment| attachment.to_part(false).map(Part::Single))
            .collect::<Result<Vec<_>, _>>()?;
        if !attached.is_empty() {
            body = multipart(MultiPart::mixed(), body, attached);
        }

        match body {
            Part::Single(part) => builder.singlepart(part),
            Part::Multi(multi) => builder.multipart(multi),
        }
        .map_err(MailError::Build)
    }
}

#[builder_impl]
impl Email {
    // **< Email BUILDER >**************************************************************************

    /// Establece el remitente, en lugar del de la configuración (`mail.from`).
    pub fn with_from(mut self, from: impl Into<Mailbox>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Establece la dirección de respuesta, en lugar de la de la configuración (`mail.reply_to`).
    pub fn with_reply_to(mut self, reply_to: impl Into<Mailbox>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    /// Añade un destinatario principal (`To`).
    pub fn with_to(mut self, to: impl Into<Mailbox>) -> Self {
        self.to.push(to.into());
        self
    }

    /// Añade un destinatario en copia (`Cc`).
    pub fn with_cc(mut self, cc: impl Into<Mailbox>) -> Self {
        self.cc.push(cc.into());
        self
    }

    /// Añade un destinatario en copia oculta (`Bcc`), que no ven los demás destinatarios.
    pub fn with_bcc(mut self, bcc: impl Into<Mailbox>) -> Self {
        self.bcc.push(bcc.into());
        self
    }

    /// Establece el asunto. Los caracteres de control (saltos de línea, tabuladores...) se cambian
    /// por espacios: el asunto puede incluir datos de usuario, y una cabecera es una sola línea.
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject
            .into()
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect();
        self
    }

    /// Establece el texto plano.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Establece el contenido HTML, que al enviarse se envuelve en el diseño común de los correos.
    pub fn with_html(mut self, content: Markup) -> Self {
        self.html = Some(content);
        self.html_is_document = false;
        self
    }

    /// Establece el documento HTML completo, que se envía tal cual, sin el diseño común. Útil si el
    /// HTML se genera con otra herramienta.
    pub fn with_html_document(mut self, document: Markup) -> Self {
        self.html = Some(document);
        self.html_is_document = true;
        self
    }

    /// Añade un archivo adjunto o una imagen incrustada.
    pub fn with_attachment(mut self, attachment: Attachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Establece el idioma del correo, que usa el diseño común del HTML.
    pub fn with_langid(mut self, language: &impl LangId) -> Self {
        self.langid = Some(language.langid());
        self
    }
}

// Genera la cabecera `Message-ID`, que los filtros de correo no deseado penalizan si falta. Sólo
// tiene que ser única: la hora, un contador y el proceso lo garantizan sin depender del azar. El
// dominio es el del remitente y no el nombre del equipo, que en un contenedor suele ser aleatorio y
// también penalizan algunos filtros.
fn message_id(domain: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("<{nanos:x}.{count:x}.{:x}@{domain}>", std::process::id())
}

// Parte de un mensaje MIME: simple o compuesta por otras partes.
enum Part {
    Single(SinglePart),
    Multi(MultiPart),
}

// Compone una parte del tipo dado con `first` y el resto de partes, en ese orden.
fn multipart(kind: MultiPartBuilder, first: Part, rest: Vec<Part>) -> Part {
    let mut multi = match first {
        Part::Single(part) => kind.singlepart(part),
        Part::Multi(multi) => kind.multipart(multi),
    };
    for part in rest {
        multi = match part {
            Part::Single(part) => multi.singlepart(part),
            Part::Multi(part) => multi.multipart(part),
        };
    }
    Part::Multi(multi)
}
