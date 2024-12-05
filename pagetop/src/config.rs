//! Load configuration settings.
//!
//! These settings are loaded from [TOML](https://toml.io) files as `key = value` pairs and mapped
//! into type-safe structures with predefined values.
//!
//! Following the [Twelve-Factor App](https://12factor.net/config) methodology, `PageTop` separates
//! code from configuration. This approach allows configurations to vary across deployments, such as
//! development, staging, or production, without changing the codebase.
//!
//!
//! # Loading configuration settings
//!
//! If your application requires configuration files, create a `config` directory in the root of
//! your project, at the same level as the *Cargo.toml* file or the application's binary.
//!
//! `PageTop` automatically loads configuration settings by reading the following TOML files in
//! order (all files are optional):
//!
//! 1. **config/common.toml**, for settings shared across all environments. This approach simplifies
//!    maintenance by centralizing common configuration values.
//!
//! 2. **config/{rm}.toml**, where `{rm}` corresponds to the environment variable
//!    `PAGETOP_RUN_MODE`:
//!
//!     * If `PAGETOP_RUN_MODE` is not set, it defaults to `default`, and `PageTop` attempts to load
//!       *config/default.toml* if available.
//!
//!     * Useful for environment-specific configurations, ensuring that each environment
//!       (e.g., development, staging, production) has its own settings without affecting others,
//!       such as API keys, URLs, or performance-related adjustments.
//!
//! 3. **config/local.{rm}.toml**, useful for local machine-specific configurations:
//!
//!     * This file allows you to add or override settings specific to the environment. For example,
//!       `local.devel.toml` for development or `local.production.toml` for production tweaks.
//!
//!     * It enables developers to tailor settings for their machines within a given environment and
//!       is typically not shared or committed to version control systems.
//!
//! 4. **config/local.toml**, for general local settings across all environments, ideal for quick
//!    adjustments or temporary values not tied to any specific environment.
//!
//! The configuration settings are merged in the order listed above, with later files overriding
//! earlier ones if there are conflicts.
//!
//!
//! # Adding configuration settings
//!
//! To give your **module** its own configuration settings, add [*serde*](https://docs.rs/serde) as
//! a dependency in your *Cargo.toml* file with the `derive` feature enabled:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! Then, use the [`include_config!`](crate::include_config) macro to initialize your settings with
//! type-safe structures and predefined values:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//! use serde::Deserialize;
//!
//! include_config!(SETTINGS: Settings => [
//!     // [myapp]
//!     "myapp.name" => "Value Name",
//!     "myapp.width" => 900,
//!     "myapp.height" => 320,
//! ]);
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Settings {
//!    pub myapp: MyApp,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MyApp {
//!     pub name: String,
//!     pub description: Option<String>,
//!     pub width: u16,
//!     pub height: u16,
//! }
//! ```
//!
//! This is how global configuration settings are declared (see [`SETTINGS`](crate::global::SETTINGS)).
//!
//! You can add a new `[myapp]` section in the configuration files using the
//! [TOML syntax](https://toml.io/en/v1.0.0#table), just like the `[log]` or `[server]` sections in
//! the global settings (see [`Settings`](crate::global::Settings)).
//!
//! It is recommended to initialize all settings with predefined values or use `Option<T>` for
//! optional settings handled within the code.
//!
//! If configuration settings fail to initialize correctly, the application will panic and stop
//! execution.
//!
//! Configuration settings are always read-only.
//!
//!
//! # Using your new configuration settings
//!
//! Access the settings directly in your code:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//! use crate::config;
//!
//! fn global_settings() {
//!     println!("App name: {}", &global::SETTINGS.app.name);
//!     println!("App description: {}", &global::SETTINGS.app.description);
//!     println!("Value of PAGETOP_RUN_MODE: {}", &global::SETTINGS.app.run_mode);
//! }
//!
//! fn package_settings() {
//!     println!("{} - {:?}", &config::SETTINGS.myapp.name, &config::SETTINGS.myapp.description);
//!     println!("{}", &config::SETTINGS.myapp.width);
//! }
//! ```

mod data;
mod de;
mod error;
mod file;
mod path;
mod source;
mod value;

use crate::concat_string;
use crate::config::data::ConfigData;
use crate::config::file::File;

use std::sync::LazyLock;

use std::env;
use std::path::Path;

/// Original values read from configuration files in `key = value` pairs.
pub static CONFIG_VALUES: LazyLock<ConfigData> = LazyLock::new(|| {
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
    let rm = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| "default".into());

    // Initialize config values.
    let mut values = ConfigData::default();

    // Merge (optional) configuration files and set the execution mode.
    values
        // First, add the common configuration for all environments. Defaults to 'common.toml'.
        .merge(File::with_name(&concat_string!(config_dir, "/common.toml")).required(false))
        .expect("Failed to merge common configuration (common.toml)")
        // Add the environment-specific configuration. Defaults to 'default.toml'.
        .merge(File::with_name(&concat_string!(config_dir, "/", rm, ".toml")).required(false))
        .expect(&format!("Failed to merge {rm}.toml configuration"))
        // Add reserved local configuration for the environment. Defaults to 'local.default.toml'.
        .merge(File::with_name(&concat_string!(config_dir, "/local.", rm, ".toml")).required(false))
        .expect("Failed to merge reserved local environment configuration")
        // Add common reserved local configuration. Defaults to 'local.toml'.
        .merge(File::with_name(&concat_string!(config_dir, "/local.toml")).required(false))
        .expect("Failed to merge general reserved local configuration")
        // Save the execution mode.
        .set("app.run_mode", rm)
        .expect("Failed to set application run mode");

    values
});

#[macro_export]
macro_rules! include_config {
    ( $SETTINGS:ident: $Settings:ty => [ $($key:literal => $value:literal),* $(,)? ] ) => {
        #[doc = concat!(
            "Assigned or predefined values for configuration settings associated to the ",
            "[`", stringify!($Settings), "`] type."
        )]
        pub static $SETTINGS: std::sync::LazyLock<$Settings> = std::sync::LazyLock::new(|| {
            let mut settings = $crate::config::CONFIG_VALUES.clone();
            $(
                settings.set_default($key, $value).unwrap();
            )*
            match settings.try_into() {
                Ok(s) => s,
                Err(e) => panic!("Error parsing settings: {}", e),
            }
        });
    };
}
