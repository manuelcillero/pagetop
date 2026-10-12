//! Configuración de `pagetop-user`.
//!
//! Todos los valores pueden sobreescribirse en los ficheros TOML de la aplicación:
//!
//! ```toml
//! [user.password]
//! min_length = 10
//! ```

use pagetop::prelude::*;
use serde::Deserialize;

use std::sync::LazyLock;

// **< CONFIG_USER >********************************************************************************

include_config!(CONFIG_USER: UserTopConfig => [
    // Política de registro y verificación.
    "user.allow_registration"          => true,
    "user.require_email_verification"  => false,

    // Sesiones.
    "user.session.cookie_name"         => "pagetop",
    "user.session.secure_cookie"       => false,
    "user.session.idle_secs"           => 7_200_i64,      // 2 horas
    "user.session.remember_idle_secs"  => 1_209_600_i64,  // 14 días
    "user.session.write_interval_secs" => 60_i64,         // 1 minuto

    // Anti-fuerza-bruta.
    "user.max_failed_logins"           => 5_i32,
    "user.failed_login_window_secs"    => 900_i64,   // 15 min
    "user.locked_for_secs"             => 600_i64,   // 10 min

    // Contraseñas (Argon2id).
    "user.password.argon2_m_cost"      => 19456_u32,
    "user.password.argon2_t_cost"      => 2_u32,
    "user.password.argon2_p_cost"      => 1_u32,
    "user.password.min_length"         => 8_u64,

    // Semilla del primer administrador.
    "user.seed.admin_username"         => "admin",
    "user.seed.admin_email"            => "admin@example.com",

    // Listados de administración (usuarios, roles...).
    "user.admin.list_page_size"        => 20_u64,

    // Modo de login.
    "user.login_strict"                => false,
]);

// **< UserTopConfig >******************************************************************************

/// Estructura raíz para la sección `[user]` del fichero de configuración.
#[derive(Clone, Debug, Deserialize)]
pub struct UserTopConfig {
    pub user: Settings,
}

// **< SETTINGS >***********************************************************************************

/// Acceso directo a los ajustes de `pagetop-user` (alias de `CONFIG_USER.user`).
pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| CONFIG_USER.user.clone());

// **< Settings >***********************************************************************************

/// Ajustes de la extensión `pagetop-user`, accesibles en la sección `[user]` del TOML.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    // Política de registro y verificación.
    pub allow_registration: bool,
    pub require_email_verification: bool,

    // Anti-fuerza-bruta.
    pub max_failed_logins: i32,
    pub failed_login_window_secs: i64,
    pub locked_for_secs: i64,

    // Sesiones.
    pub session: SessionConfig,

    // Contraseñas.
    pub password: PasswordConfig,

    // Semilla del primer administrador.
    pub seed: SeedConfig,

    // Listados de administración.
    pub admin: AdminConfig,

    /// Activa las medidas para dificultar que el navegador recuerde o autorrellene las
    /// credenciales en la pantalla de login (gestor de contraseñas, autocompletado agresivo...).
    pub login_strict: bool,
}

/// Ajustes de la sección `[user.session]`.
#[derive(Clone, Debug, Deserialize)]
pub struct SessionConfig {
    /// Nombre de la cookie de sesión.
    pub cookie_name: String,
    /// Marca la cookie de sesión como `Secure` (sólo HTTPS). Debe activarse en producción.
    pub secure_cookie: bool,
    /// Segundos sin uso tras los que caduca una sesión iniciada sin "recordarme". El tiempo se
    /// cuenta desde la última petición, con la precisión que fija
    /// [`write_interval_secs`](Self::write_interval_secs).
    pub idle_secs: i64,
    /// Segundos sin uso tras los que caduca una sesión iniciada con "recordarme". El tiempo se
    /// cuenta desde la última petición, con la precisión que fija
    /// [`write_interval_secs`](Self::write_interval_secs).
    pub remember_idle_secs: i64,
    /// Segundos mínimos entre dos actualizaciones de la misma sesión en la base de datos.
    ///
    /// Cada petición de una sesión válida la prorroga, pero sólo se escribe en la base de datos
    /// (y se renueva la cookie de "recordarme") si ha pasado al menos este tiempo desde la última
    /// actualización. A cambio, una sesión puede caducar hasta este tiempo antes de lo configurado
    /// en [`idle_secs`](Self::idle_secs) y [`remember_idle_secs`](Self::remember_idle_secs). Con
    /// `0` se escribe en cada petición.
    ///
    /// El valor por defecto (60) sirve para la mayoría de aplicaciones. Conviene cambiarlo:
    ///
    /// - **Al alza** (p. ej. 180, como Drupal) si hay muchos usuarios simultáneos o la base de
    ///   datos es remota o lenta, para reducir las escrituras.
    /// - **A la baja** (o `0`) si las sesiones caducan tras pocos minutos sin uso y ese margen de
    ///   adelanto no es aceptable.
    ///
    /// Debe ser siempre muy inferior a `idle_secs`.
    pub write_interval_secs: i64,
}

/// Ajustes de la sección `[user.password]`.
#[derive(Clone, Debug, Deserialize)]
pub struct PasswordConfig {
    pub argon2_m_cost: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,
    pub min_length: usize,
}

/// Ajustes de la sección `[user.seed]`.
#[derive(Clone, Debug, Deserialize)]
pub struct SeedConfig {
    pub admin_username: String,
    pub admin_email: String,
    pub admin_password: Option<String>,
}

/// Ajustes de la sección `[user.admin]`.
#[derive(Clone, Debug, Deserialize)]
pub struct AdminConfig {
    /// Número de filas por página en los listados de administración (usuarios, roles...).
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
