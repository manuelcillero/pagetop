//! Ajustes de configuración de `pagetop-mail`.
//!
//! Ejemplo:
//!
//! ```toml
//! [mail]
//! transport = "smtp"
//! from      = "Mi Aplicación <no-reply@example.com>"
//! base_url  = "https://example.com"
//!
//! [mail.smtp]
//! host     = "smtp.example.com"
//! security = "starttls"
//! username = "no-reply@example.com"
//! password = "secreto"
//! ```
//!
//! Uso:
//!
//! ```rust,no_run
//! use pagetop_mail::config;
//!
//! assert_eq!(config::SETTINGS.smtp.host, "smtp.example.com");
//! ```
//!
//! Todas las opciones, con su valor por defecto, se documentan en [`MailSettings`] y sus
//! subsecciones ([`SmtpSettings`], [`SendmailSettings`] y [`FileSettings`]).
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

/// Ajustes de la sección `[mail]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<MailSettings> = LazySettings::new();

/// Sección **`[mail]`** de la configuración.
#[settings(section = "mail")]
pub struct MailSettings {
    /// Cómo se entregan los correos. Por defecto se escriben en las trazas sin enviarlos
    /// ([`MailTransport::Log`]), para que una aplicación sin configurar no envíe correo real.
    pub transport: MailTransport,
    /// Remitente por defecto, para los correos que no indican uno propio. Admite la dirección sola
    /// (`"no-reply@example.com"`) o con nombre (`"Mi Aplicación <no-reply@example.com>"`). Sin
    /// valor, cada correo debe indicar su remitente.
    pub from: Option<String>,
    /// Dirección de respuesta por defecto, para los correos que no indican una propia.
    pub reply_to: Option<String>,
    /// Si se indica, todos los correos se entregan **sólo** a esta dirección en lugar de a sus
    /// destinatarios, que se conservan en las cabeceras (`To`, `Cc`). Pensado para entornos de
    /// pruebas o preproducción con datos reales.
    pub redirect_to: Option<String>,
    /// URL pública de la aplicación (p. ej. `"https://example.com"`), para construir los enlaces
    /// absolutos de los correos con [`absolute_url()`](crate::absolute_url). Debe empezar por
    /// `http://` o `https://`, o la aplicación no arranca. Sin valor, se usa la dirección y el
    /// puerto de escucha del servidor (`http://localhost:8080`), que sólo sirve en desarrollo.
    pub base_url: Option<String>,
    /// Ofrece en `/mail/preview` las vistas previas de los correos que declaran las extensiones
    /// (ver [`preview`](crate::preview)). Sólo para desarrollo: muestra las plantillas a cualquiera
    /// que conozca la ruta.
    #[default = false]
    pub previews: bool,

    /// Servidor SMTP, en la subsección `[mail.smtp]`.
    pub smtp: SmtpSettings,

    /// Programa *sendmail*, en la subsección `[mail.sendmail]`.
    pub sendmail: SendmailSettings,

    /// Archivos de correo, en la subsección `[mail.file]`.
    pub file: FileSettings,
}

/// Forma de entregar los correos. Usado en [`MailSettings::transport`].
#[derive(AutoDefault, Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MailTransport {
    /// Envía los correos a un servidor SMTP (ver [`SmtpSettings`]).
    Smtp,
    /// Entrega los correos al programa *sendmail* local (ver [`SendmailSettings`]).
    Sendmail,
    /// Guarda cada correo como un archivo `.eml` en un directorio (ver [`FileSettings`]). Útil en
    /// desarrollo para abrirlos con un cliente de correo.
    File,
    /// No envía nada: escribe en las trazas el remitente, los destinatarios, el asunto y el texto
    /// de cada correo. Es el valor por defecto. No debe usarse en producción, porque los textos
    /// pueden incluir enlaces con *tokens* de un solo uso.
    #[default]
    Log,
    /// No envía nada: guarda los correos en memoria para que las pruebas los consulten con
    /// [`outbox`](crate::outbox).
    Memory,
}

/// Subsección **`[mail.smtp]`** de la configuración.
#[settings]
pub struct SmtpSettings {
    /// Servidor SMTP.
    #[default = "localhost"]
    pub host: String,
    /// Puerto del servidor. Sin valor, el habitual según la seguridad: 465 con
    /// [`SmtpSecurity::Tls`], 587 con [`SmtpSecurity::StartTls`] y 25 en los demás casos.
    pub port: Option<u16>,
    /// Cifrado de la conexión.
    #[default(SmtpSecurity::StartTls)]
    pub security: SmtpSecurity,
    /// Usuario para autenticarse en el servidor. Sin valor, no se autentica.
    pub username: Option<String>,
    /// Contraseña del usuario. Conviene guardarla en `config/local.toml`, que no se versiona.
    pub password: Option<String>,
    /// Nombre con el que se presenta la aplicación al servidor (`EHLO`). Sin valor, el nombre del
    /// equipo.
    pub hello_name: Option<String>,
    /// Segundos de espera máxima de cada orden al servidor.
    #[default = 60]
    pub timeout_secs: u64,
    /// Número máximo de conexiones abiertas con el servidor, que se reutilizan entre envíos.
    #[default = 10]
    pub pool_max_size: u32,
    /// Acepta certificados no válidos (caducados, autofirmados o de otro servidor). **Peligroso**:
    /// sólo para servidores de pruebas, nunca en producción.
    #[default = false]
    pub accept_invalid_certs: bool,
}

/// Cifrado de la conexión SMTP. Usado en [`SmtpSettings::security`].
#[derive(AutoDefault, Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SmtpSecurity {
    /// Sin cifrado. Sólo para servidores locales o de pruebas (p. ej. Mailpit o MailHog).
    None,
    /// Cifra con `STARTTLS` si el servidor lo ofrece y, si no, envía sin cifrar.
    Opportunistic,
    /// Exige `STARTTLS`: empieza sin cifrar y cifra antes de autenticarse. Es el valor por
    /// defecto (puerto 587).
    #[default]
    StartTls,
    /// Conexión cifrada desde el principio, también llamada SMTPS (puerto 465).
    Tls,
}

/// Subsección **`[mail.sendmail]`** de la configuración.
#[settings]
pub struct SendmailSettings {
    /// Programa que recibe los correos. Sin valor, `sendmail` del `PATH`.
    pub command: Option<String>,
}

/// Subsección **`[mail.file]`** de la configuración.
#[settings]
pub struct FileSettings {
    /// Directorio donde se guardan los correos. Se crea al arrancar si no existe.
    #[default = "mail"]
    pub dir: String,
}

#[cfg(test)]
mod tests {
    use super::{MailSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../../../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<MailSettings>(REFERENCE).unwrap();
    }
}
