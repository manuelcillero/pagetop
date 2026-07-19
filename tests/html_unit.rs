use pagetop::prelude::*;

use std::str::FromStr;

#[pagetop::test]
async fn unit_value_empty_and_auto_and_zero_without_unit() {
    assert_eq!(UnitValue::from_str("").unwrap(), UnitValue::None);
    assert_eq!(UnitValue::from_str("auto").unwrap(), UnitValue::Auto);
    assert_eq!(UnitValue::from_str("AUTO").unwrap(), UnitValue::Auto);

    // Zero without a unit.
    assert_eq!(UnitValue::from_str("0").unwrap(), UnitValue::Zero);
    assert_eq!(UnitValue::from_str("+0").unwrap(), UnitValue::Zero);
    assert_eq!(UnitValue::from_str("-0").unwrap(), UnitValue::Zero);
}

#[pagetop::test]
async fn unit_value_absolute_integers_with_signs_and_spaces_and_case() {
    // Positive, negative, and with spaces.
    assert_eq!(UnitValue::from_str("12px").unwrap(), UnitValue::Px(12));
    assert_eq!(UnitValue::from_str("-5pt").unwrap(), UnitValue::Pt(-5));
    assert_eq!(UnitValue::from_str("  7 cm ").unwrap(), UnitValue::Cm(7));
    assert_eq!(UnitValue::from_str("+9  in").unwrap(), UnitValue::In(9));
    assert_eq!(UnitValue::from_str(" 13   mm ").unwrap(), UnitValue::Mm(13));
    assert_eq!(UnitValue::from_str("4   pc").unwrap(), UnitValue::Pc(4));

    // Case insensitivity.
    assert_eq!(UnitValue::from_str("10PX").unwrap(), UnitValue::Px(10));
    assert_eq!(UnitValue::from_str("15Pt").unwrap(), UnitValue::Pt(15));
}

#[pagetop::test]
async fn unit_value_relative_floats_with_signs_and_spaces_and_case() {
    assert_eq!(
        UnitValue::from_str("1.25rem").unwrap(),
        UnitValue::RelRem(1.25)
    );
    assert_eq!(
        UnitValue::from_str("-0.5em").unwrap(),
        UnitValue::RelEm(-0.5)
    );
    assert_eq!(
        UnitValue::from_str(" 33% ").unwrap(),
        UnitValue::RelPct(33.0)
    );
    assert_eq!(
        UnitValue::from_str("  -12.5  vh").unwrap(),
        UnitValue::RelVh(-12.5)
    );
    assert_eq!(
        UnitValue::from_str(" 8.0  VW ").unwrap(),
        UnitValue::RelVw(8.0)
    );
}

#[pagetop::test]
async fn unit_value_whitespace_between_number_and_unit_is_allowed() {
    // There is a space between number and unit (the current implementation allows it).
    assert_eq!(UnitValue::from_str("12 px").unwrap(), UnitValue::Px(12));
    assert_eq!(
        UnitValue::from_str("1.5  rem").unwrap(),
        UnitValue::RelRem(1.5)
    );
    assert_eq!(
        UnitValue::from_str("25 %").unwrap(),
        UnitValue::RelPct(25.0)
    );
}

#[pagetop::test]
async fn unit_value_roundtrip_display_keeps_expected_format() {
    let cases = [
        ("", UnitValue::None, ""),
        ("auto", UnitValue::Auto, "auto"),
        ("0", UnitValue::Zero, "0"),
        ("12px", UnitValue::Px(12), "12px"),
        ("-5pt", UnitValue::Pt(-5), "-5pt"),
        ("7cm", UnitValue::Cm(7), "7cm"),
        ("33%", UnitValue::RelPct(33.0), "33%"),
        ("1.25rem", UnitValue::RelRem(1.25), "1.25rem"),
        ("2em", UnitValue::RelEm(2.0), "2em"),
        ("-0.5vh", UnitValue::RelVh(-0.5), "-0.5vh"),
        ("8vw", UnitValue::RelVw(8.0), "8vw"),
    ];

    for (input, expected_value, expected_display) in cases {
        let parsed = UnitValue::from_str(input).unwrap();
        assert_eq!(
            parsed, expected_value,
            "parsed mismatch for input `{input}`"
        );
        assert_eq!(
            parsed.to_string(),
            expected_display,
            "display mismatch for input `{input}`"
        );
    }
}

#[pagetop::test]
async fn unit_value_percentage_trimming_and_signs() {
    assert_eq!(
        UnitValue::from_str(" 12.5 % ").unwrap(),
        UnitValue::RelPct(12.5)
    );
    assert_eq!(
        UnitValue::from_str("-0.0%").unwrap(),
        UnitValue::RelPct(-0.0)
    );
    assert_eq!(
        UnitValue::from_str("+15%").unwrap(),
        UnitValue::RelPct(15.0)
    );
}

// EXPECTED ERRORS (don't change the messages; is_err() is enough).

#[pagetop::test]
async fn unit_value_errors_missing_unit_for_non_zero() {
    assert!(
        UnitValue::from_str("12").is_err(),
        "non-zero without unit must error"
    );
    assert!(
        UnitValue::from_str("  -3  ").is_err(),
        "non-zero without unit must error"
    );
}

#[pagetop::test]
async fn unit_value_errors_decimals_in_absolute_units() {
    assert!(UnitValue::from_str("1.5px").is_err());
    assert!(UnitValue::from_str("-2.0pt").is_err());
    assert!(UnitValue::from_str("+0.1cm").is_err());
}

#[pagetop::test]
async fn unit_value_errors_unknown_units_or_bad_percentages() {
    // Unsupported unit.
    assert!(UnitValue::from_str("10ch").is_err());
    assert!(UnitValue::from_str("2q").is_err());
    // Missing number.
    assert!(UnitValue::from_str("%").is_err());
    assert!(UnitValue::from_str("  % ").is_err());
}

#[pagetop::test]
async fn unit_value_errors_non_numeric_numbers() {
    assert!(UnitValue::from_str("NaNem").is_err());
    // Decimal not allowed by FromStr.
    assert!(UnitValue::from_str("1,5rem").is_err());
}

#[pagetop::test]
async fn unit_value_serde_deserialize_struct_and_array() {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct BoxStyle {
        width: UnitValue,
        height: UnitValue,
        margin: UnitValue,
    }

    let json = r#"{ "width": "12px", "height": "1.5rem", "margin": "0" }"#;
    let s: BoxStyle = serde_json::from_str(json).unwrap();
    assert_eq!(s.width, UnitValue::Px(12));
    assert_eq!(s.height, UnitValue::RelRem(1.5));
    assert_eq!(s.margin, UnitValue::Zero);

    #[derive(Deserialize, Debug, PartialEq)]
    struct Many {
        values: Vec<UnitValue>,
    }

    let json_arr = r#"{ "values": ["", "auto", "33%", "8vw", "7 cm", "-5pt"] }"#;
    let m: Many = serde_json::from_str(json_arr).unwrap();
    assert_eq!(
        m.values,
        vec![
            UnitValue::None,
            UnitValue::Auto,
            UnitValue::RelPct(33.0),
            UnitValue::RelVw(8.0),
            UnitValue::Cm(7),
            UnitValue::Pt(-5),
        ]
    );
}

#[pagetop::test]
async fn unit_value_accepts_dot5_and_1dot_shorthand_for_relatives() {
    // `.5` and `1.` parse correctly for relative units.
    assert_eq!(UnitValue::from_str(".5em").unwrap(), UnitValue::RelEm(0.5));
    assert_eq!(
        UnitValue::from_str("1.rem").unwrap(),
        UnitValue::RelRem(1.0)
    );
    assert_eq!(UnitValue::from_str("1.vh").unwrap(), UnitValue::RelVh(1.0));
    // Without a unit it must keep failing.
    assert!(UnitValue::from_str("1.").is_err());
}

#[pagetop::test]
async fn unit_value_display_keeps_minus_zero_for_relatives() {
    // Current behavior: f32 Display shows "-0" if the value is -0.0.
    let v = UnitValue::RelEm(-0.0);
    // Either of the two formats is accepted as valid.
    let s = v.to_string();
    assert!(
        s == "-0em" || s == "0em",
        "current Display prints `{s}` for -0.0; both are acceptable in tests"
    );
}

#[pagetop::test]
async fn unit_value_rejects_non_decimal_notations() {
    // Leading zeros (e.g. `"020px"`) are interpreted in **base 10** (`20px`), not octal.
    assert_eq!(UnitValue::from_str("020px").unwrap(), UnitValue::Px(20));
    // Scientific notation and non-decimal bases (e.g., `"1e3vw"`, `"0x10px"`) are not supported.
    assert!(UnitValue::from_str("1e3vw").is_err());
    assert!(UnitValue::from_str("0x10px").is_err());
}
