use pagetop::prelude::*;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    "test.string_value" => "Test String",
    "test.int_value"    => -321,
    "test.float_value"  => 2.3456,
]);

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub test: Test,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub string_value: String,
    pub int_value: i32,
    pub float_value: f32,
}

// La *feature* `testing` (activo con `cargo ts` / `cargo tw`) fija el modo "test" en tiempo de
// compilación dentro de `config::CONFIG_VALUES`, de forma que `global::SETTINGS` y cualquier
// `include_config!` local cargan automáticamente la configuración del modo "test".

#[pagetop::test]
async fn check_global_config() {
    assert_eq!(global::SETTINGS.app.run_mode, "test");
    assert_eq!(global::SETTINGS.app.name, "Testing");
    assert_eq!(global::SETTINGS.server.bind_port, 9000);
}

#[pagetop::test]
async fn check_local_config() {
    assert_eq!(SETTINGS.test.string_value, "Modified value");
    assert_eq!(SETTINGS.test.int_value, -321);
    assert_eq!(SETTINGS.test.float_value, 8.7654);
}
