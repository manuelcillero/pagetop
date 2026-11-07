use pagetop::prelude::*;

// The permission catalog is built once per process, at startup: every declaration of this test is
// in a single extension and the cases are different tests on top of it.

#[derive(Clone, Copy, Debug)]
enum Perm {
    Read,
    Write,
    Audit,
}

impl Permission for Perm {
    fn key(&self) -> CowStr {
        match self {
            Self::Read => "test:read".into(),
            Self::Write => "test:write".into(),
            Self::Audit => "test:audit".into(),
        }
    }

    fn group(&self) -> &'static str {
        match self {
            Self::Read | Self::Write => "content",
            Self::Audit => "security",
        }
    }
}

// Same key as `Perm::Read`, declared again by another action.
struct Duplicate;

impl Permission for Duplicate {
    fn key(&self) -> CowStr {
        "test:read".into()
    }

    fn group(&self) -> &'static str {
        "duplicated"
    }
}

fn declare_audit(registry: &mut PermissionRegistry) {
    registry.register(&Perm::Audit);
}

fn declare_content(registry: &mut PermissionRegistry) {
    registry.register(&Perm::Read);
    registry.register(&Perm::Write);
    registry.register(&Duplicate);
}

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::auth::DeclarePermissions::new(declare_audit),
            action::auth::DeclarePermissions::new(declare_content).with_weight(-1),
        ]
    }
}

async fn start() {
    // Starting more than once in the same process (as tests do) does not duplicate the catalog.
    let _ = Application::prepare(&Ext).await;
    let _ = Application::prepare(&Ext).await;
}

fn keys(permissions: impl Iterator<Item = PermissionRef>) -> Vec<String> {
    permissions.map(|p| p.key().into_owned()).collect()
}

#[pagetop::test]
async fn declarations_are_registered_by_weight_then_registration_order() {
    start().await;
    let registry = PermissionRegistry::global();
    assert_eq!(
        keys(registry.all()),
        ["test:read", "test:write", "test:audit"]
    );
    let groups: Vec<_> = registry.groups().iter().map(|(g, _)| *g).collect();
    assert_eq!(groups, ["content", "security"]);
}

#[pagetop::test]
async fn a_repeated_key_is_ignored_with_its_group() {
    start().await;
    let registry = PermissionRegistry::global();
    assert!(registry.has(&Duplicate));
    assert!(registry.has_key("test:read"));
    assert!(!registry.has_key("test:unknown"));
    assert!(!registry.groups().iter().any(|(g, _)| *g == "duplicated"));
}

#[pagetop::test]
async fn permissions_are_listed_by_group() {
    start().await;
    let registry = PermissionRegistry::global();
    assert_eq!(
        keys(registry.by_group("content")),
        ["test:read", "test:write"]
    );
    assert_eq!(keys(registry.by_group("security")), ["test:audit"]);
    assert!(registry.by_group("missing").next().is_none());
}

#[pagetop::test]
async fn declaring_a_permission_does_not_grant_it() {
    start().await;
    let request = web::test::TestRequest::get().uri("/").to_http_request();
    assert!(!has_permission(&request, &Perm::Read));
}
