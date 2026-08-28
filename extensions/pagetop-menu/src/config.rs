//! Configuración de `pagetop-menu`.
//!
//! Todos los valores pueden sobreescribirse en los ficheros TOML de la aplicación:
//!
//! ```toml
//! [menu]
//! default_menus = ["main", "footer", "user"]
//! ```

use pagetop::prelude::*;
use serde::Deserialize;

use std::sync::LazyLock;

// **< CONFIG_MENU >********************************************************************************

include_config!(CONFIG_MENU: MenuTopConfig => [
    // Menús que se crean automáticamente si no los declara ninguna extensión.
    // (Nota: los valores de lista no son soportados por config-rs vía set_default; se leen del
    // TOML.)
]);

// **< MenuTopConfig >******************************************************************************

/// Estructura raíz para la sección `[menu]` del fichero de configuración.
#[derive(Clone, Debug, Deserialize)]
pub struct MenuTopConfig {
    pub menu: Settings,
}

// **< SETTINGS >***********************************************************************************

/// Acceso directo a los ajustes de `pagetop-menu` (alias de `CONFIG_MENU.menu`).
pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| CONFIG_MENU.menu.clone());

// **< Settings >***********************************************************************************

/// Ajustes de la extensión `pagetop-menu`, accesibles en la sección `[menu]` del TOML.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    /// Menús que se aseguran en BD al arrancar si ninguna extensión los declara.
    #[serde(default = "default_menus")]
    pub default_menus: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_menus: default_menus(),
        }
    }
}

fn default_menus() -> Vec<String> {
    vec!["main".into(), "footer".into(), "user".into()]
}
