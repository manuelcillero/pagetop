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
//! subsecciones ([`SmtpSettings`], [`SendmailSettings`], [`FileSettings`] y [`QueueSettings`]). Un
//! valor fuera del rango que indica cada opción impide arrancar la aplicación.
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
    /// valor, cada correo debe indicar su remitente para enviarse, salvo con
    /// [`MailTransport::Log`], que sólo lo escribe en las trazas. Los correos de otras extensiones,
    /// como los de `pagetop-user`, no lo indican: conviene configurarlo siempre.
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
    /// Tiempo máximo de cada entrega completa, en segundos (entre 1 y 86400), incluida la espera de
    /// su turno de envío. Pasado ese tiempo, la entrega falla con [`MailError::Timeout`]. Es una
    /// red de seguridad: *sendmail* y los archivos no tienen límite propio, y en SMTP
    /// [`timeout_secs`] limita cada orden al servidor, no la entrega entera.
    ///
    /// **Se recomienda el valor predeterminado, 300 (5 minutos).**
    ///
    /// - **Reducirlo** (p. ej. a 60 o 120) sirve con un servidor de correo cercano y rápido y sin
    ///   adjuntos grandes: los fallos se detectan antes y, en una avalancha, los correos que
    ///   esperan turno fallan antes en lugar de acumularse. Pero debe superar con holgura el doble
    ///   de [`timeout_secs`] (se avisa al arrancar si no), o fallarán envíos lentos pero correctos;
    ///   y cuanto más corto, más correos que el servidor sí acepta se dan por fallidos y, con la
    ///   cola de envío, se entregan dos veces.
    /// - **Aumentarlo** sólo con servidores muy lentos o adjuntos grandes por conexiones lentas:
    ///   cada correo retiene su turno durante más tiempo y, con la cola de envío, un correo cuya
    ///   entrega se interrumpe (p. ej. porque la aplicación cae) tarda más en reintentarse, porque
    ///   su reserva dura el triple de este tiempo.
    ///
    /// [`MailError::Timeout`]: crate::MailError::Timeout
    /// [`timeout_secs`]: SmtpSettings::timeout_secs
    #[default = 300]
    pub delivery_timeout_secs: u64,
    /// Número máximo de correos que pueden esperar su turno de envío **por cada envío simultáneo**
    /// ([`SmtpSettings::pool_max_size`], [`SendmailSettings::max_concurrent`] o
    /// [`FileSettings::max_concurrent`]). Con todos los turnos de espera ocupados, la entrega falla
    /// enseguida, sin empezar, con [`MailError::Overloaded`](crate::MailError::Overloaded); con la
    /// *feature* `queue`, los correos de [`send_later()`](crate::Email::send_later) quedan entonces
    /// en la cola de envío. Con 0, ningún correo espera: sólo se entregan los que encuentran un
    /// envío libre.
    ///
    /// **No se recomiendan valores mayores que el predeterminado.** Cada correo en espera retiene
    /// en memoria el mensaje completo, adjuntos incluidos, y la espera cuenta dentro del tiempo
    /// máximo de cada entrega ([`delivery_timeout_secs`](Self::delivery_timeout_secs)): con más
    /// correos esperando, los últimos agotan ese tiempo antes de empezar y fallan igualmente,
    /// después de haber ocupado la memoria. Con poca memoria, o con correos con adjuntos grandes,
    /// conviene reducirlo.
    #[default = 100]
    pub max_waiting: u32,

    /// Servidor SMTP, en la subsección `[mail.smtp]`.
    pub smtp: SmtpSettings,

    /// Programa *sendmail*, en la subsección `[mail.sendmail]`.
    pub sendmail: SendmailSettings,

    /// Archivos de correo, en la subsección `[mail.file]`.
    pub file: FileSettings,

    /// Cola de envío, en la subsección `[mail.queue]`. Sólo se usa con la *feature* `queue`.
    pub queue: QueueSettings,
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
    /// Contraseña del usuario. Conviene guardarla en `config/local.toml`, que no se versiona, o
    /// pasarla con la variable de entorno `PAGETOP_MAIL__SMTP__PASSWORD`.
    pub password: Option<String>,
    /// Nombre con el que se presenta la aplicación al servidor (`EHLO`). Sin valor, el nombre del
    /// equipo.
    pub hello_name: Option<String>,
    /// Segundos de espera máxima de cada intento de conexión y de cada orden al servidor. Cada
    /// entrega completa tiene además un tiempo máximo
    /// ([`mail.delivery_timeout_secs`](MailSettings::delivery_timeout_secs)), pasado el cual falla
    /// con [`MailError::Timeout`](crate::MailError::Timeout).
    ///
    /// Con [`SmtpSecurity::Tls`] no cubre el saludo TLS inicial: si el servidor acepta la conexión
    /// y no responde, el envío falla al agotar ese tiempo máximo, pero sigue ocupando uno de los
    /// envíos simultáneos ([`pool_max_size`](Self::pool_max_size)) hasta que el sistema da la
    /// conexión por perdida, o hasta reiniciar si el servidor la mantiene abierta. Si el servidor
    /// lo admite, [`SmtpSecurity::StartTls`] no tiene este problema.
    #[default = 60]
    pub timeout_secs: u64,
    /// Número máximo de envíos simultáneos al servidor (al menos uno), y de conexiones que se
    /// conservan abiertas para reutilizarlas. Los correos que llegan con todos los envíos en curso
    /// esperan su turno sin ocupar un hilo, pero la espera cuenta dentro del tiempo máximo de cada
    /// entrega ([`mail.delivery_timeout_secs`](MailSettings::delivery_timeout_secs)).
    ///
    /// Por cada envío simultáneo esperan turno como mucho
    /// [`mail.max_waiting`](MailSettings::max_waiting) correos; con más, la entrega falla enseguida
    /// con [`MailError::Overloaded`](crate::MailError::Overloaded), para no acumularlos en memoria.
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
    /// Exige `STARTTLS`: empieza sin cifrar y cifra antes de autenticarse. Es el valor por defecto
    /// (puerto 587).
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
    /// Número máximo de correos que se entregan a la vez (al menos uno). Cada entrega abre un
    /// proceso *sendmail*, y los correos que llegan con todas en curso esperan su turno, como mucho
    /// [`mail.max_waiting`](MailSettings::max_waiting) por cada entrega simultánea; con más, la
    /// entrega falla enseguida con [`MailError::Overloaded`](crate::MailError::Overloaded).
    ///
    /// **No se recomiendan valores mayores que el predeterminado.** Con un servidor de correo
    /// local, *sendmail* sólo deja el correo en su cola, que lo acepta en milisegundos, así que más
    /// procesos a la vez apenas aceleran el envío. En cambio, en una avalancha de correos se abren
    /// otros tantos procesos, que compiten por la memoria y la CPU con la aplicación, y crece en la
    /// misma proporción el número de correos que pueden esperar turno en memoria.
    ///
    /// Si el programa no tiene cola propia y envía cada correo a un servidor remoto (p. ej. `msmtp`
    /// o `ssmtp`), cada proceso mantiene abierta una conexión con ese servidor: no conviene superar
    /// su límite de conexiones simultáneas, o rechazará los correos que lo excedan.
    #[default = 10]
    pub max_concurrent: u32,
}

/// Subsección **`[mail.file]`** de la configuración.
#[settings]
pub struct FileSettings {
    /// Directorio donde se guardan los correos. Se crea al arrancar si no existe.
    #[default = "mail"]
    pub dir: String,
    /// Número máximo de correos que se guardan a la vez (al menos uno). Los correos que llegan con
    /// todos en curso esperan su turno, como mucho [`mail.max_waiting`](MailSettings::max_waiting)
    /// por cada uno; con más, la entrega falla enseguida con
    /// [`MailError::Overloaded`](crate::MailError::Overloaded).
    ///
    /// **No se recomiendan valores mayores que el predeterminado.** Cada archivo se escribe en un
    /// hilo del *pool* de bloqueo de Tokio, que comparten otras tareas de la aplicación (p. ej. el
    /// cálculo de contraseñas al iniciar sesión), y más escrituras a la vez sobre el mismo disco
    /// apenas aceleran el guardado. En una avalancha de correos, ocuparían más hilos de ese *pool*
    /// y crecería en la misma proporción el número de correos que pueden esperar turno en memoria.
    #[default = 10]
    pub max_concurrent: u32,
}

/// Subsección **`[mail.queue]`** de la configuración.
///
/// Sólo se usa con la *feature* `queue` (ver [`Email::queue()`](crate::Email)). El ritmo de envío
/// no se configura aquí: es la programación del trabajo `mail.queue` de `pagetop-cron`, que por
/// defecto se ejecuta un minuto después de terminar la ejecución anterior y se cambia en
/// `[cron.jobs."mail.queue"]`.
#[settings]
pub struct QueueSettings {
    /// Número máximo de correos que envía cada ejecución del trabajo `mail.queue`, uno tras otro
    /// (al menos uno). Limita la carga del equipo y del servidor de correo en los envíos masivos.
    #[default = 50]
    pub batch_size: u32,
    /// Número máximo de intentos de cada correo, contando el primero. Agotados, o ante un rechazo
    /// definitivo del servidor (SMTP 5xx), el correo queda como fallido. Con 0 se intenta una vez.
    #[default = 5]
    pub max_attempts: u32,
    /// Segundos de espera tras el primer fallo temporal. La espera se duplica en cada intento
    /// fallido (60, 120, 240... con el valor por defecto), hasta
    /// [`max_retry_secs`](Self::max_retry_secs).
    #[default = 60]
    pub retry_base_secs: u64,
    /// Espera máxima entre dos intentos, en segundos (entre 1 y 31536000, un año).
    ///
    /// **Se recomienda el valor predeterminado, 86400 (un día).** Con los demás valores por defecto
    /// no se llega a alcanzar (la mayor espera es de 8 minutos); sólo limita las esperas cuando se
    /// aumentan [`max_attempts`](Self::max_attempts) o [`retry_base_secs`](Self::retry_base_secs).
    ///
    /// - **Reducirlo** hace que los correos se reintenten antes tras una caída larga del servidor
    ///   de correo, a costa de más intentos (y más trabajo para la aplicación y el servidor)
    ///   mientras dura; con muchos intentos, el correo puede darse por fallido antes de que el
    ///   servidor se recupere.
    /// - **Aumentarlo** reparte los intentos a lo largo de más tiempo, pero un correo puede llegar
    ///   con días de retraso, cuando quizá ya no sirve (p. ej. un enlace de recuperación de
    ///   contraseña ya caducado).
    #[default = 86_400]
    pub max_retry_secs: u64,
    /// Días que se conservan los correos fallidos antes de borrarlos. Con 0 no se borran. Los
    /// enviados se borran en cuanto se entregan, porque pueden contener enlaces de un solo uso.
    #[default = 30]
    pub keep_failed_days: u32,
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
