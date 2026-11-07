use pagetop::prelude::*;

use global::{AppSettings, LogRolling, LogSettings, ServerSettings};

#[pagetop::test]
async fn enum_values_are_case_insensitive() {
    let log = config::load_from_str::<LogSettings>("[log]\nrolling = \"daily\"\n").unwrap();
    assert_eq!(log.rolling, LogRolling::Daily);
}

#[pagetop::test]
async fn invalid_enum_values_are_rejected() {
    let error = config::load_from_str::<LogSettings>("[log]\nrolling = \"Daly\"\n")
        .expect_err("invalid value accepted")
        .to_string();
    assert!(error.contains("Daly"));
    assert!(error.contains("Daily"));
}

#[pagetop::test]
async fn client_ip_header_must_be_a_valid_header_name() {
    let load = |value: &str| {
        let toml = format!("[server]\nclient_ip_header = \"{value}\"\n");
        config::load_from_str::<ServerSettings>(&toml)
    };
    assert_eq!(load(" X-Real-IP ").unwrap().client_ip_header, "X-Real-IP");
    assert_eq!(load("").unwrap().client_ip_header, "");
    let error = load("X Real IP").expect_err("invalid name accepted");
    assert!(error.to_string().contains("X Real IP"), "{error}");
    // Its values are not plain addresses, so it would never give one.
    assert!(load("Forwarded").is_err());
}

#[pagetop::test]
async fn admin_path_must_be_an_absolute_path_without_a_trailing_slash() {
    let load = |value: &str| {
        let toml = format!("[app]\nadmin_path = \"{value}\"\n");
        config::load_from_str::<AppSettings>(&toml)
    };
    assert_eq!(load(" /manage ").unwrap().admin_path, "/manage");
    assert_eq!(load("/site/admin-2").unwrap().admin_path, "/site/admin-2");
    for invalid in [
        "",
        "/",
        "admin",
        "/admin/",
        "/site//admin",
        "/.",
        "/site/../admin",
        "/{admin}",
        "/ad min",
    ] {
        let error = load(invalid).expect_err(invalid);
        assert!(error.to_string().contains("invalid admin path"), "{error}");
    }
}

#[pagetop::test]
async fn trusted_origins_are_normalized_and_validated() {
    let load = |value: &str| {
        let toml = format!("[server]\ntrusted_origins = [\"{value}\"]\n");
        config::load_from_str::<ServerSettings>(&toml)
    };
    let origins = |value| load(value).unwrap().trusted_origins;
    // Browsers send the Origin header in lowercase and without a trailing slash.
    assert_eq!(
        origins(" HTTPS://App.Example.com/ "),
        ["https://app.example.com"]
    );
    assert_eq!(origins("http://localhost:3000"), ["http://localhost:3000"]);
    // Nor with the default port of the scheme, which they leave out.
    assert_eq!(
        origins("https://app.example.com:443"),
        ["https://app.example.com"]
    );
    assert_eq!(origins("http://[::1]:80"), ["http://[::1]"]);
    assert_eq!(
        origins("http://app.example.com:443"),
        ["http://app.example.com:443"]
    );
    // None of these could ever match an Origin header.
    for invalid in [
        "app.example.com",
        "ftp://app.example.com",
        "https://app.example.com/path",
        "https://app.example.com?x=1",
        "https://user@app.example.com",
        "https://app.example.com#top",
        "",
    ] {
        let error = load(invalid).expect_err(invalid);
        assert!(error.to_string().contains("invalid origin"), "{error}");
    }
}
