/// Nombre del directorio donde se encuentra la configuración.
pub const CONFIG_DIR: &'static str = "config";

mod settings;
pub use crate::config::settings::{CONFIG, SETTINGS};
