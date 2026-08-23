use pagetop::prelude::*;

// **< Waypoint - only accepts local paths >********************************************************

#[pagetop::test]
async fn waypoint_accepts_local_paths() {
    for path in ["/", "/admin/users", "/admin/users?page=2"] {
        let w = Waypoint::from(path.to_owned());
        assert_eq!(w.as_deref(), Some(path));
    }
}

#[pagetop::test]
async fn waypoint_rejects_open_redirect_targets() {
    // Targets that a malicious client could slip into `?waypoint=...` to redirect off-site: they
    // must be discarded just as if no waypoint had been provided.
    for target in [
        "",
        "https://evil.example",
        "//evil.example",
        "/\\evil.example",
        "javascript:alert(1)",
    ] {
        let w = Waypoint::from(target.to_owned());
        assert_eq!(w.as_deref(), None, "expected {target:?} to be rejected");
    }
}

#[pagetop::test]
async fn waypoint_deserialize_rejects_open_redirect_targets() {
    // Reproduces the real entry point (`web::Query<Waypoint>`): the value arrives via
    // deserialization, not through `Waypoint::from(String)` as in the previous test.
    let w: Waypoint = serde_json::from_str(r#"{"waypoint":"https://evil.example"}"#).unwrap();
    assert_eq!(w.as_deref(), None);
}

// **< Waypoint::or() >*****************************************************************************

#[pagetop::test]
async fn waypoint_or_returns_transported_local_path() {
    let w = Waypoint::from("/admin/users?page=2".to_owned());
    assert_eq!(w.or("/fallback").to_string(), "/admin/users?page=2");
}

#[pagetop::test]
async fn waypoint_or_falls_back_when_target_is_not_local() {
    let w = Waypoint::from("https://evil.example".to_owned());
    assert_eq!(w.or("/admin/users").to_string(), "/admin/users");
}

// **< Waypoint::append_to() >**********************************************************************

#[pagetop::test]
async fn waypoint_append_to_adds_param_only_when_present() {
    let w = Waypoint::from("/admin/users".to_owned());
    assert_eq!(
        w.append_to("/items").to_string(),
        "/items?waypoint=%2Fadmin%2Fusers"
    );

    let empty = Waypoint::default();
    assert_eq!(empty.append_to("/items").to_string(), "/items");
}
