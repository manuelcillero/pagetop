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
    // `default_menus` (el único campo de `Settings`) no puede recibir su valor por defecto vía
    // `set_default` en `include_config!` (config-rs no soporta defaults de tipo lista, ver arriba),
    // así que si la aplicación no declara `[menu]` en ningún TOML, la sección entera falta en la
    // configuración combinada. `#[serde(default)]` recurre a `Settings::default()` en ese caso.
    #[serde(default)]
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
