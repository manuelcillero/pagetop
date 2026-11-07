use pagetop::prelude::*;

// The administration registry is built once per process, at startup: every declaration of this
// test is in a single extension and the cases are different tests on top of it.

struct ToolsPermission;

impl Permission for ToolsPermission {
    fn key(&self) -> CowStr {
        "tools:use".into()
    }
}

fn declare_sections(bag: &mut AdminSectionBag) {
    bag.add(AdminSection {
        key: "tools".to_owned(),
        path: "tools".to_owned(),
        title: Lc::n("Tools"),
        weight: 35,
    });
    // Replaces the built-in section with the same key.
    bag.add(AdminSection {
        key: "help".to_owned(),
        path: "support".to_owned(),
        title: Lc::n("Support"),
        weight: 5,
    });
}

fn declare_pages(bag: &mut AdminPageBag) {
    for (path, weight) in [("tools/export", 10), ("/tools/import/", 0)] {
        bag.add(AdminPage {
            path: path.to_owned(),
            section: "tools".to_owned(),
            title: Lc::n(path),
            description: Lc::none(),
            weight,
            permission: &ToolsPermission,
        });
    }
}

fn declare_tasks(bag: &mut AdminTaskBag) {
    for (path, weight) in [("tools/export/json", 1), ("tools/export/csv", 0)] {
        bag.add(AdminTask {
            path: path.to_owned(),
            parent_path: "/tools/export".to_owned(),
            title: Lc::n(path),
            weight,
            is_default: weight == 0,
            permission: None,
        });
    }
}

fn declare_actions(bag: &mut AdminActionBag) {
    bag.add(AdminAction {
        url: "tools/export/new/".to_owned(),
        for_path: "tools/export".to_owned(),
        title: Lc::n("New export"),
        weight: 0,
        permission: None,
    });
}

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::admin::DeclareAdminSections::new(declare_sections),
            action::admin::DeclareAdminPages::new(declare_pages),
            action::admin::DeclareAdminTasks::new(declare_tasks),
            action::admin::DeclareAdminActions::new(declare_actions),
        ]
    }
}

async fn start() {
    // Starting more than once in the same process (as tests do) does not duplicate anything.
    let _ = Application::prepare(&Ext).await;
    let _ = Application::prepare(&Ext).await;
}

#[pagetop::test]
async fn built_in_sections_come_first_and_can_be_replaced() {
    start().await;
    let keys: Vec<_> = AdminRegistry::global()
        .ordered_sections()
        .into_iter()
        .map(|s| (s.key.as_str(), s.path.as_str()))
        .collect();
    assert_eq!(
        keys,
        [
            ("help", "support"),
            ("people", "people"),
            ("structure", "structure"),
            ("config", "config"),
            ("tools", "tools"),
            ("reports", "reports"),
        ]
    );
}

#[pagetop::test]
async fn paths_are_kept_relative_without_surrounding_slashes() {
    start().await;
    let registry = AdminRegistry::global();
    let pages: Vec<_> = registry
        .pages_for_section("tools")
        .into_iter()
        .map(|p| p.path.as_str())
        .collect();
    assert_eq!(pages, ["tools/import", "tools/export"]);

    let tasks: Vec<_> = registry
        .tasks_for("tools/export")
        .into_iter()
        .map(|t| t.path.as_str())
        .collect();
    assert_eq!(tasks, ["tools/export/csv", "tools/export/json"]);

    let actions = registry.actions_for("tools/export");
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].url, "tools/export/new");
}

#[pagetop::test]
async fn admin_paths_are_built_from_the_root_and_back() {
    start().await;
    assert_eq!(admin_path(""), "/admin");
    assert_eq!(admin_path("tools/export"), "/admin/tools/export");
    assert_eq!(admin_path("/tools/export/"), "/admin/tools/export");

    assert_eq!(strip_admin_path("/admin"), Some(""));
    assert_eq!(strip_admin_path("/admin/"), Some(""));
    assert_eq!(
        strip_admin_path("/admin/tools/export/"),
        Some("tools/export")
    );
    assert_eq!(strip_admin_path("/administrator"), None);
    assert_eq!(strip_admin_path("/tools"), None);
}

#[pagetop::test]
async fn without_an_authentication_extension_nobody_can_access_the_panel() {
    start().await;
    let request = web::test::TestRequest::get()
        .uri("/admin")
        .to_http_request();
    let cx = Context::new(request);
    assert!(!can_access_admin(&cx));
}
