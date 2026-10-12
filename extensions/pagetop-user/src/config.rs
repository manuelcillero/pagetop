//! Ajustes de configuración de `pagetop-user`.
//!
//! Ejemplo:
//!
//! ```toml
//! [user]
//! allow_registration = false
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
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

/// Ajustes de la sección `[user]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<UserSettings> = LazySettings::new();

/// Sección **`[user]`** de la configuración.
#[settings(section = "user")]
pub struct UserSettings {
    /// Permite que los visitantes creen su propia cuenta.
    #[default = true]
    pub allow_registration: bool,
    /// Las cuentas nuevas quedan pendientes (*"Pending"*) en lugar de activas hasta verificar el
    /// correo.
    #[default = false]
    pub require_email_verification: bool,

    /// Intentos fallidos de acceso antes de bloquear la cuenta.
    #[default = 5]
    pub max_failed_logins: i32,
    /// Ventana de recuento de intentos fallidos, en segundos. Reservada: todavía no tiene efecto.
    #[default = 900]
    pub failed_login_window_secs: i64,
    /// Duración del bloqueo de la cuenta, en segundos.
    #[default = 600]
    pub locked_for_secs: i64,

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
/// Las contraseñas se cifran con Argon2id.
#[settings]
pub struct PasswordSettings {
    /// Coste de memoria de Argon2id, en KiB. Si cambia, las contraseñas se vuelven a cifrar en el
    /// siguiente acceso de cada usuario.
    #[default = 19_456]
    pub argon2_m_cost: u32,
    /// Coste de tiempo de Argon2id (número de iteraciones).
    #[default = 2]
    pub argon2_t_cost: u32,
    /// Grado de paralelismo de Argon2id (número de hilos).
    #[default = 1]
    pub argon2_p_cost: u32,
    /// Longitud mínima de las contraseñas.
    #[default = 8]
    pub min_length: usize,
}

/// Subsección **`[user.seed]`** de la configuración.
///
/// El primer administrador, que se crea si no existe ningún usuario.
#[settings]
pub struct SeedSettings {
    /// Nombre de usuario del primer administrador.
    #[default = "admin"]
    pub admin_username: String,
    /// Correo electrónico del primer administrador.
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
