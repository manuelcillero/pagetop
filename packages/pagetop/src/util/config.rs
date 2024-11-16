//! Retrieve settings values from configuration files.

use crate::join;
use crate::util::data::ConfigData;
use crate::util::file::File;

use std::sync::LazyLock;

use std::env;
use std::path::Path;

/// Original configuration values in `key = value` pairs gathered from configuration files.
pub static CONFIG_DATA: LazyLock<ConfigData> = LazyLock::new(|| {
    // Identify the configuration directory.
    let config_dir = env::var("CARGO_MANIFEST_DIR")
        .map(|manifest_dir| {
            let manifest_config = Path::new(&manifest_dir).join("config");
            if manifest_config.exists() {
                manifest_config.to_string_lossy().to_string()
            } else {
                "config".to_string()
            }
        })
        .unwrap_or_else(|_| "config".to_string());

    // Execution mode based on the environment variable PAGETOP_RUN_MODE, defaults to 'default'.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| "default".into());

    // Initialize settings.
    let mut settings = ConfigData::default();

    // Merge (optional) configuration files and set the execution mode.
    settings
        // First, add the common configuration for all environments. Defaults to 'common.toml'.
        .merge(File::with_name(&join!(config_dir, "/common.toml")).required(false))
        .expect("Failed to merge common configuration (common.toml)")
        // Add the environment-specific configuration. Defaults to 'default.toml'.
        .merge(File::with_name(&join!(config_dir, "/", run_mode, ".toml")).required(false))
        .expect(&format!("Failed to merge {run_mode}.toml configuration"))
        // Add reserved local configuration for the environment. Defaults to 'local.default.toml'.
        .merge(File::with_name(&join!(config_dir, "/local.", run_mode, ".toml")).required(false))
        .expect("Failed to merge reserved local environment configuration")
        // Add the general reserved local configuration. Defaults to 'local.toml'.
        .merge(File::with_name(&join!(config_dir, "/local.toml")).required(false))
        .expect("Failed to merge general reserved local configuration")
        // Save the execution mode.
        .set("app.run_mode", run_mode)
        .expect("Failed to set application run mode");

    settings
});
