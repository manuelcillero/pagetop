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
    "user.allow_registration"        => true,
    "user.require_email_verification" => false,

    // Sesiones.
    "user.session_cookie_name"  => "pgt_session",
    "user.session_ttl_secs"     => 1_209_600_i64,  // 14 días
    "user.session_idle_ttl_secs" => 7_200_i64,     // 2 horas
    "user.secure_cookie"        => false,

    // Anti-fuerza-bruta.
    "user.max_failed_logins"         => 5_i32,
    "user.failed_login_window_secs"  => 900_i64,   // 15 min
    "user.locked_for_secs"           => 600_i64,   // 10 min

    // Contraseñas (Argon2id).
    "user.password.argon2_m_cost"  => 19456_u32,
    "user.password.argon2_t_cost"  => 2_u32,
    "user.password.argon2_p_cost"  => 1_u32,
    "user.password.min_length"     => 8_u64,

    // Semilla del primer administrador.
    "user.seed.admin_username"  => "admin",
    "user.seed.admin_email"     => "admin@example.com",

    // Listados de administración (usuarios, roles...).
    "user.admin.list_page_size" => 20_u64,

    // Modo de login.
    "user.login_strict" => false,
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
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Settings {
    // Política de registro y verificación.
    pub allow_registration: bool,
    pub require_email_verification: bool,

    // Sesiones.
    pub session_cookie_name: String,
    pub session_ttl_secs: i64,
    pub session_idle_ttl_secs: i64,
    pub secure_cookie: bool,

    // Anti-fuerza-bruta.
    pub max_failed_logins: i32,
    pub failed_login_window_secs: i64,
    pub locked_for_secs: i64,

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

/// Ajustes de la sección `[user.password]`.
#[derive(Clone, Debug, Deserialize)]
pub struct PasswordConfig {
    pub argon2_m_cost: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,
    pub min_length: usize,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        PasswordConfig {
            argon2_m_cost: 19456,
            argon2_t_cost: 2,
            argon2_p_cost: 1,
            min_length: 8,
        }
    }
}

/// Ajustes de la sección `[user.seed]`.
#[derive(Clone, Debug, Default, Deserialize)]
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

impl Default for AdminConfig {
    fn default() -> Self {
        AdminConfig { list_page_size: 20 }
    }
}
