use pagetop::prelude::*;

use serde_json;

#[pagetop::test]
async fn test_deserialize_absolute_units() {
    let value: unit::Value = serde_json::from_str("\"50px\"").unwrap();
    assert!(matches!(value, unit::Value::Px(50)));

    let value: unit::Value = serde_json::from_str("\"10cm\"").unwrap();
    assert!(matches!(value, unit::Value::Cm(10)));
}

#[pagetop::test]
async fn test_deserialize_relative_units() {
    let value: unit::Value = serde_json::from_str("\"1.5em\"").unwrap();
    assert!(matches!(value, unit::Value::RelEm(1.5)));

    let value: unit::Value = serde_json::from_str("\"100%\"").unwrap();
    assert!(matches!(value, unit::Value::RelPct(100.0)));
}

#[pagetop::test]
async fn test_invalid_format() {
    let result: Result<unit::Value, _> = serde_json::from_str("\"invalid\"");
    assert!(result.is_err());
}
