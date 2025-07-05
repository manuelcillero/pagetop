use pagetop::prelude::*;

use serde::Deserialize;

use std::env;

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

#[pagetop::test]
async fn check_global_config() {
    env::set_var("PAGETOP_RUN_MODE", "test");

    assert_eq!(global::SETTINGS.app.run_mode, "test");
    assert_eq!(global::SETTINGS.app.name, "Testing");
    assert_eq!(global::SETTINGS.server.bind_port, 9000);
}

#[pagetop::test]
async fn check_local_config() {
    env::set_var("PAGETOP_RUN_MODE", "test");

    assert_eq!(SETTINGS.test.string_value, "Modified value");
    assert_eq!(SETTINGS.test.int_value, -321);
    assert_eq!(SETTINGS.test.float_value, 8.7654);
}
