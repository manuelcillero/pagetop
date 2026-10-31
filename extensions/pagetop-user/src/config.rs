//! Ajustes de configuración de `pagetop-user`.
//!
//! Ejemplo:
//!
//! ```toml
//! [user]
//! allow_registration = false
//! identity_policy    = "unicode"
//!
//! [user.session]
//! secure_cookie = true
//!
//! [user.password]
//! min_length = 10
//! ```
//!
//! Uso:
//!
//! ```rust,no_run
//! use pagetop_user::config;
//!
//! assert_eq!(config::SETTINGS.password.min_length, 10);
//! ```
//!
//! Todas las opciones, con su valor por defecto, se documentan en [`UserSettings`] y sus
//! subsecciones ([`SessionSettings`], [`PasswordSettings`], [`SeedSettings`] y [`AdminSettings`]).
//! Las reglas de los nombres de usuario y del email se explican en [`IdentityPolicy`].
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

/// Ajustes de la sección `[user]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<UserSettings> = LazySettings::new();

/// Sección **`[user]`** de la configuración.
#[settings(section = "user")]
pub struct UserSettings {
    /// Permite que los visitantes creen su propia cuenta.
    #[default = true]
    pub allow_registration: bool,
    /// Las cuentas nuevas quedan pendientes (*"Pending"*) en lugar de activas hasta que su titular
    /// confirma su dirección desde el enlace del correo de verificación que se le envía al
    /// registrarse.
    #[default = false]
    pub require_email_verification: bool,
    /// Qué nombres de usuario se aceptan. Ver [`IdentityPolicy`].
    pub identity_policy: IdentityPolicy,

    /// Intentos fallidos de acceso seguidos que bloquean temporalmente la cuenta.
    ///
    /// Un fallo se suma a los anteriores si llega antes de
    /// [`failed_login_window_secs`](Self::failed_login_window_secs) desde el último; si no, el
    /// recuento empieza de nuevo. Un acceso correcto o una recuperación de contraseña lo ponen a
    /// cero, y al bloquear también vuelve a cero, así que pasado el bloqueo se dispone otra vez de
    /// todos los intentos. Con `0` no se bloquea nunca.
    #[default = 5]
    pub max_failed_logins: i32,
    /// Segundos desde el último intento fallido en los que el siguiente se suma al recuento.
    #[default = 900]
    pub failed_login_window_secs: i64,
    /// Duración del bloqueo de la cuenta, en segundos. Durante el bloqueo se rechaza el acceso
    /// aunque la contraseña sea correcta.
    #[default = 600]
    pub locked_for_secs: i64,

    /// Solicitudes de recuperación de contraseña que puede recibir una misma cuenta en
    /// [`password_reset_window_secs`](Self::password_reset_window_secs).
    ///
    /// Cada solicitud envía un correo con un enlace nuevo que anula los anteriores. Alcanzado el
    /// límite, las siguientes no envían nada hasta que la más antigua de la ventana queda fuera de
    /// ella, y la respuesta es la misma para no revelar si la cuenta existe. Así nadie puede
    /// inundar el buzón de otra persona ni dañar la reputación del servidor de correo. Con `0` no
    /// hay límite.
    #[default = 3]
    pub max_password_resets: u32,
    /// Segundos en los que se cuentan las solicitudes de recuperación de contraseña de una cuenta.
    #[default = 3_600]
    pub password_reset_window_secs: i64,

    /// Sesiones, en la subsección `[user.session]`.
    pub session: SessionSettings,

    /// Contraseñas, en la subsección `[user.password]`.
    pub password: PasswordSettings,

    /// Primer administrador, en la subsección `[user.seed]`.
    pub seed: SeedSettings,

    /// Listados de administración, en la subsección `[user.admin]`.
    pub admin: AdminSettings,

    /// Activa las medidas para dificultar que el navegador recuerde o autorrellene las
    /// credenciales en la pantalla de login (gestor de contraseñas, autocompletado agresivo...).
    #[default = false]
    pub login_strict: bool,
}

/// Política de identidad: qué nombres de usuario se aceptan al crear una cuenta o al cambiar su
/// nombre. Usado en [`UserSettings::identity_policy`].
///
/// Cada nivel admite todo lo del anterior y algo más:
///
/// | Valor | Caracteres | Longitud | Como en |
/// |---|---|---|---|
/// | `"strict"` (defecto) | Letras ASCII sin tildes, dígitos, `.`, `_`, `-` | 3 a 32 | Kanidm |
/// | `"unicode"` | Letras y dígitos de cualquier alfabeto, `.`, `_`, `-` | 3 a 32 | RFC 8265 |
/// | `"permissive"` | Los de `"unicode"`, espacios simples, `'` y `+` | 3 a 60 | Drupal |
///
/// # Reglas comunes
///
/// Con cualquier política:
///
/// - Se ignoran los espacios al principio y al final, y el texto se normaliza (Unicode NFKC): las
///   formas equivalentes de un mismo carácter, como `é` compuesta o con la tilde aparte, o las
///   letras de ancho completo (`Ａｎａ`), cuentan como la misma, y el nombre se guarda ya
///   normalizado (`Ana`).
/// - El nombre empieza por una letra o un dígito.
/// - **Nunca se admite `@`.** El email es un dato aparte y obligatorio, y también sirve para
///   iniciar sesión: sin `@`, un nombre de usuario nunca se confunde con el email de otra cuenta.
/// - El nombre se guarda con sus mayúsculas, pero **no se distinguen**: `Ana` y `ana` son el mismo
///   usuario al iniciar sesión, y no pueden existir los dos.
/// - **No se admiten nombres que se confunden a simple vista con otro existente**, según la tabla
///   de caracteres confusables de Unicode (UTS #39): si existe `maria`, no se puede crear
///   `rnaria`; si existe `admin`, tampoco `аdmin` con una `а` cirílica.
///
/// Con `"unicode"` y `"permissive"`, además, un nombre no puede mezclar alfabetos (`pаypal`, con
/// una `а` cirílica entre letras latinas), salvo las combinaciones habituales de una misma
/// escritura (como kanji y kana en japonés), ni usar caracteres que Unicode desaconseja en
/// identificadores (invisibles, obsoletos o de control). En `"permissive"` tampoco se admiten
/// dos espacios seguidos.
///
/// El email se valida igual con cualquier política: el formato que exige `pagetop-mail` para poder
/// enviarle correos, hasta 254 caracteres, y sin distinguir mayúsculas.
///
/// # Elegir una política
///
/// El riesgo no depende tanto de dónde se publica la aplicación como de **quién elige los
/// nombres**. Con el registro abierto (`allow_registration`), cualquiera escoge un nombre que verán
/// otros usuarios y que aparece en el saludo de sus correos en cuanto verifica su dirección. Si las
/// cuentas sólo las crean administradores, ese riesgo casi desaparece.
///
/// | Situación | Política |
/// |---|---|
/// | Aplicación en Internet con registro abierto | `"strict"` |
/// | Comunidad en Internet cuyos usuarios escriben su nombre en su alfabeto | `"unicode"` |
/// | Aplicación interna con cuentas creadas por administradores | `"strict"` o `"unicode"` |
/// | Importar cuentas de un sistema con espacios en los nombres (Drupal...) | `"permissive"` |
///
/// `"strict"` es la más segura y basta en la mayoría de las aplicaciones: los nombres con tildes o
/// en otros alfabetos tienen su sitio en el nombre visible del usuario (`display_name`).
/// `"unicode"` permite que el nombre de usuario se escriba en el idioma de cada persona, con la
/// misma protección contra nombres que se confunden. `"permissive"` no es aconsejable con el
/// registro abierto: los espacios permiten escribir frases («Soporte técnico», «Llama al 900...»)
/// que luego aparecen donde se muestra el nombre, también en el saludo de los correos.
///
/// # Cambiar de política
///
/// La política sólo decide qué nombres **nuevos** se aceptan: buscar y comparar usuarios funciona
/// igual con cualquiera, así que nadie pierde el acceso al cambiarla.
///
/// - **A una más abierta** (de `"strict"` a `"unicode"`, por ejemplo), las cuentas existentes no
///   cambian. Los nombres nuevos pueden usar más caracteres, y siguen sin poder confundirse con
///   ninguno de los existentes.
/// - **A una más estricta**, los nombres que ya no la cumplen (o que superan la nueva longitud
///   máxima) se conservan: sus titulares pueden iniciar sesión y guardar su perfil, incluso
///   cambiando sólo las mayúsculas de su nombre. Pero si alguien lo cambia, el propio usuario o un
///   administrador, el nuevo nombre debe cumplir la política. No hay renombrado automático: para
///   ajustar los nombres existentes, un administrador los cambia a mano.
/// - **El administrador inicial** ([`SeedSettings::admin_username`]) se comprueba en cada
///   arranque, aunque ya exista. Si deja de cumplir la política, la aplicación no arranca hasta
///   que se configure un nombre válido. Cambiar ese ajuste no renombra al administrador existente:
///   sólo se usa para crearlo cuando la base de datos está vacía. Con el valor por defecto,
///   `"admin"`, no ocurre con ninguna política.
///
/// # Referencias
///
/// - [UAX #15](https://www.unicode.org/reports/tr15/): formas de normalización (NFKC).
/// - [UAX #31](https://www.unicode.org/reports/tr31/): qué caracteres forman un identificador.
/// - [UTS #39](https://www.unicode.org/reports/tr39/): caracteres confusables, mezcla de alfabetos
///   y caracteres desaconsejados en identificadores.
/// - [RFC 8265](https://www.rfc-editor.org/rfc/rfc8265): nombres de usuario internacionales
///   (PRECIS), el modelo de `"unicode"`.
#[derive(AutoDefault, Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdentityPolicy {
    /// Letras ASCII sin tildes, dígitos, `.`, `_` y `-`, de 3 a 32 caracteres.
    #[default]
    Strict,
    /// Letras y dígitos de cualquier alfabeto, `.`, `_` y `-`, de 3 a 32 caracteres, sin mezclar
    /// alfabetos.
    Unicode,
    /// Lo mismo que [`Unicode`](Self::Unicode), más espacios simples, `'` y `+`, de 3 a 60
    /// caracteres.
    Permissive,
}

/// Subsección **`[user.session]`** de la configuración.
#[settings]
pub struct SessionSettings {
    /// Nombre de la cookie de sesión.
    #[default = "pagetop"]
    pub cookie_name: String,
    /// Marca la cookie de sesión como `Secure` (sólo HTTPS). Debe activarse en producción.
    #[default = false]
    pub secure_cookie: bool,
    /// Segundos sin uso tras los que caduca una sesión iniciada sin "recordarme". El tiempo se
    /// cuenta desde la última petición, con la precisión que fija
    /// [`write_interval_secs`](Self::write_interval_secs).
    #[default = 7_200]
    pub idle_secs: i64,
    /// Segundos sin uso tras los que caduca una sesión iniciada con "recordarme". El tiempo se
    /// cuenta desde la última petición, con la precisión que fija
    /// [`write_interval_secs`](Self::write_interval_secs).
    #[default = 1_209_600]
    pub remember_idle_secs: i64,
    /// Segundos mínimos entre dos actualizaciones de la misma sesión en la base de datos.
    ///
    /// Cada petición de una sesión válida la prorroga, pero sólo se escribe en la base de datos
    /// (y se renueva la cookie de "recordarme") si ha pasado al menos este tiempo desde la última
    /// actualización. A cambio, una sesión puede caducar hasta este tiempo antes de lo configurado
    /// en [`idle_secs`](Self::idle_secs) y [`remember_idle_secs`](Self::remember_idle_secs). Con
    /// `0` se escribe en cada petición.
    ///
    /// El valor por defecto sirve para la mayoría de aplicaciones. Conviene cambiarlo:
    ///
    /// - **Al alza** (p. ej. 180, como Drupal) si hay muchos usuarios simultáneos o la base de
    ///   datos es remota o lenta, para reducir las escrituras.
    /// - **A la baja** (o `0`) si las sesiones caducan tras pocos minutos sin uso y ese margen de
    ///   adelanto no es aceptable.
    ///
    /// Debe ser siempre muy inferior a `idle_secs`.
    #[default = 60]
    pub write_interval_secs: i64,
}

/// Subsección **`[user.password]`** de la configuración.
///
/// Las contraseñas se cifran con Argon2id. Si cambia cualquiera de sus tres costes, cada contraseña
/// se vuelve a cifrar con los nuevos en el siguiente acceso de su usuario.
#[settings]
pub struct PasswordSettings {
    /// Coste de memoria de Argon2id, en KiB.
    #[default = 19_456]
    pub argon2_m_cost: u32,
    /// Coste de tiempo de Argon2id (número de iteraciones).
    #[default = 2]
    pub argon2_t_cost: u32,
    /// Grado de paralelismo de Argon2id (número de hilos).
    #[default = 1]
    pub argon2_p_cost: u32,
    /// Longitud mínima de las contraseñas, en caracteres.
    #[default = 8]
    pub min_length: usize,
}

/// Subsección **`[user.seed]`** de la configuración.
///
/// El primer administrador, que se crea si no existe ningún usuario.
#[settings]
pub struct SeedSettings {
    /// Nombre de usuario del primer administrador. Debe cumplir la política de identidad
    /// configurada ([`IdentityPolicy`]); si no, la aplicación no arranca.
    #[default = "admin"]
    pub admin_username: String,
    /// Correo electrónico del primer administrador. Si no es una dirección válida, la aplicación no
    /// arranca.
    #[default = "admin@example.com"]
    pub admin_email: String,
    /// Contraseña del primer administrador. Sin valor, se genera una aleatoria y se muestra una
    /// sola vez por la salida estándar al crearlo.
    pub admin_password: Option<String>,
}

/// Subsección **`[user.admin]`** de la configuración.
#[settings]
pub struct AdminSettings {
    /// Número de filas por página en los listados de administración (usuarios, roles...).
    #[default = 20]
    pub list_page_size: u64,
}

// **< Preferencias del usuario >*******************************************************************

// Si se aplica el idioma propio del usuario: no con `lang_negotiation = "ConfigOnly"`, que usa
// siempre el de la configuración. Si no se aplica, no se ofrece elegirlo ni se borra el guardado.
pub(crate) fn user_language_applies() -> bool {
    global::SETTINGS.app.lang_negotiation != global::LangNegotiation::ConfigOnly
}

// Si se aplica la zona horaria propia del usuario (`app.timezone_per_user`). Si no se aplica, no se
// ofrece elegirla ni se borra la guardada.
pub(crate) fn user_timezone_applies() -> bool {
    global::SETTINGS.app.timezone_per_user
}

#[cfg(test)]
mod tests {
    use super::{UserSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../../../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<UserSettings>(REFERENCE).unwrap();
    }
}
