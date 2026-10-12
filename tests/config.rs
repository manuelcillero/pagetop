use pagetop::prelude::*;

use std::sync::atomic::{AtomicBool, Ordering};

// The `testing` *feature* (active with `cargo ts` / `cargo tw`) fixes the "test" run mode at
// compile time, so that `global::SETTINGS` and any other settings automatically load the "test"
// mode configuration from `config/test.toml` (versioned, unlike the `local.*.toml` files).

#[settings(section = "test")]
pub struct TestSettings {
    #[default = "Test String"]
    pub string_value: String,
    #[default(-321)]
    pub int_value: i32,
    #[default = 2.3456]
    pub float_value: f32,
    pub nested: NestedSettings,
}

#[settings]
pub struct NestedSettings {
    #[default = true]
    pub enabled: bool,
}

static SETTINGS: LazySettings<TestSettings> = LazySettings::new();

// Settings only reachable through `Extension::settings()`: `STARTUP_LOADED` can only be set by the
// application loading them at startup.
static STARTUP_LOADED: AtomicBool = AtomicBool::new(false);

struct StartupSettings;

impl LoadSettings for StartupSettings {
    fn load(&self) -> Result<(), BoxError> {
        STARTUP_LOADED.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn section(&self) -> &'static str {
        "test"
    }
}

// A section that no configuration file defines.
#[settings(section = "absent")]
pub struct AbsentSettings {
    #[default = 7]
    pub value: u8,
}

// Only one of the keys of `[test]` in `config/test.toml`: the other one is unknown.
#[settings(section = "test")]
pub struct PartialTestSettings {
    pub float_value: f32,
}

// `EXTENSIONS` is a global `OnceLock`: all tests in this file share the same root extension.
struct SettingsExtension;

#[async_trait]
impl Extension for SettingsExtension {
    fn settings(&self) -> Option<&'static dyn LoadSettings> {
        Some(&StartupSettings)
    }
}

#[pagetop::test]
async fn check_global_config() {
    assert_eq!(global::SETTINGS.app.run_mode, "test");
    assert_eq!(global::SETTINGS.app.name, "Testing");
    assert_eq!(global::SETTINGS.server.bind_port, 9000);
    // Not defined in `config/test.toml`: default value.
    assert_eq!(global::SETTINGS.server.bind_address, "localhost");
}

#[pagetop::test]
async fn files_override_defaults_key_by_key() {
    assert_eq!(SETTINGS.string_value, "Modified value");
    assert_eq!(SETTINGS.int_value, -321);
    assert_eq!(SETTINGS.float_value, 8.7654);
    assert!(SETTINGS.nested.enabled);
}

#[pagetop::test]
async fn absent_section_uses_all_defaults() {
    static ABSENT: LazySettings<AbsentSettings> = LazySettings::new();
    assert_eq!(ABSENT.value, 7);
}

#[pagetop::test]
async fn defaults_come_from_the_fields() {
    let defaults = TestSettings::default();
    assert_eq!(defaults.string_value, "Test String");
    assert_eq!(defaults.int_value, -321);
    assert!(defaults.nested.enabled);
}

#[pagetop::test]
async fn unknown_keys_are_rejected() {
    // A typo in a key must not be silently ignored.
    let result = serde_json::from_str::<TestSettings>(r#"{ "string_valu": "typo" }"#);
    let error = result.expect_err("unknown key accepted").to_string();
    assert!(error.contains("string_valu"));

    let result = serde_json::from_str::<TestSettings>(r#"{ "nested": { "enable": false } }"#);
    assert!(result.is_err());
}

#[pagetop::test]
async fn application_loads_settings_at_startup() {
    let _ = Application::prepare(&SettingsExtension).await.test();
    assert!(STARTUP_LOADED.load(Ordering::SeqCst));
}

#[pagetop::test]
async fn load_from_str_rejects_unknown_keys() {
    let error = config::load_from_str::<TestSettings>("[test]\nstring_valu = \"typo\"\n")
        .expect_err("unknown key accepted")
        .to_string();
    assert!(error.contains("string_valu"));
}

#[pagetop::test]
async fn load_reports_invalid_settings_instead_of_panicking() {
    let settings = LazySettings::<PartialTestSettings>::new();
    let error = settings
        .load()
        .expect_err("unknown key accepted")
        .to_string();
    assert!(error.contains("string_value"), "{error}");
}

#[pagetop::test]
async fn load_keeps_the_settings_it_reads() {
    let settings = LazySettings::<TestSettings>::new();
    settings.load().expect("valid settings rejected");
    assert_eq!(settings.string_value, "Modified value");
}
