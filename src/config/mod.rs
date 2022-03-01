/// Nombre del directorio donde se encuentra la configuración.
pub const CONFIG_DIR: &'static str = "config";

mod settings;
pub use settings::{CONFIG, SETTINGS};
