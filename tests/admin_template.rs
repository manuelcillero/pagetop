use pagetop::prelude::*;

// The administration registry is built once per process, at startup: every declaration of this
// test is in a single extension and the cases are different tests on top of it.

struct ToolsPermission;

impl Permission for ToolsPermission {
    fn key(&self) -> CowStr {
        "tools:use".into()
    }
}

// Not granted by `grant_tools()`.
struct AuditPermission;

impl Permission for AuditPermission {
    fn key(&self) -> CowStr {
        "tools:audit".into()
    }
}

// Grants `tools:use` to every request, as an authentication extension would.
fn grant_tools(_: &HttpRequest, perm: PermissionRef, granted: &mut bool) {
    if perm.key() == "tools:use" {
        *granted = true;
    }
}

fn declare_pages(bag: &mut AdminPageBag) {
    bag.add(
        AdminPage::new("config/export", "config", Lc::n("Export"), &ToolsPermission)
            .with_icon(Icon::with(icon::Core::Plus)),
    );
    bag.add(AdminPage::new(
        "reports/audit",
        "reports",
        Lc::n("Audit"),
        &AuditPermission,
    ));
}

fn declare_tasks(bag: &mut AdminTaskBag) {
    bag.add(AdminTask::new(
        "config/export",
        "config/export",
        Lc::n("All"),
    ));
    bag.add(AdminTask::new("config/export/csv", "config/export", Lc::n("CSV")).with_weight(1));
    bag.add(
        AdminTask::new("config/export/log", "config/export", Lc::n("Log"))
            .with_weight(2)
            .with_permission(&AuditPermission),
    );
}

fn declare_actions(bag: &mut AdminActionBag) {
    bag.add(AdminAction::new(
        "config/export/new",
        "config/export",
        Lc::n("New export"),
    ));
    bag.add(
        AdminAction::new(
            "config/export/purge",
            "config/export",
            Lc::n("Purge exports"),
        )
        .with_weight(1)
        .with_permission(&AuditPermission),
    );
}

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::auth::CheckPermission::new(grant_tools),
            action::admin::DeclareAdminPages::new(declare_pages),
            action::admin::DeclareAdminTasks::new(declare_tasks),
            action::admin::DeclareAdminActions::new(declare_actions),
        ]
    }

    // None of these handlers checks any permission: each route requires the one of its page.
    fn configure_router(&self, routes: Routes) -> Routes {
        routes
            .admin_route("config/export", web::get(unchecked))
            .admin_route("config/export/{id}/edit", web::get(unchecked))
            .admin_route("reports/audit", web::get(unchecked))
            .admin_route("reports/audit/{id}", web::get(unchecked))
    }
}

async fn unchecked() -> &'static str {
    "unchecked"
}

async fn render(page: Page) -> String {
    let mut page = page.with_child(Html::with(|_| html! { p { "Body of the page." } }));
    page.render().await.unwrap().into_string()
}

fn request(uri: &str) -> HttpRequest {
    web::test::TestRequest::get().uri(uri).to_http_request()
}

#[pagetop::test]
async fn the_admin_template_frames_the_content_of_the_page() {
    let _ = Application::prepare(&Ext).await;
    let html =
        render(Page::admin(request("/admin/config/export/csv")).with_title(Lc::n("CSV"))).await;

    assert!(html.contains(r#"class="wrapper wrapper-admin""#), "{html}");
    // The template does not paint the navigation of the panel: the application places it.
    assert!(!html.contains("nav-admin"), "{html}");
    // Breadcrumbs up to the nearest declared page, then the page title.
    assert!(html.contains(r#"href="/admin/config""#), "{html}");
    assert!(
        html.contains(r#"<h1 class="admin-page-title">CSV</h1>"#),
        "{html}"
    );
    // Tasks of the parent page, the current one active, without the one not granted.
    assert!(
        html.contains(
            r#"<li class="admin-task admin-task-active"><a href="/admin/config/export/csv">"#
        ),
        "{html}"
    );
    assert!(!html.contains("config/export/log"), "{html}");
    // The content of the page, inside the frame.
    assert!(html.contains(r#"<div class="admin-content">"#), "{html}");
    assert!(html.contains("Body of the page."), "{html}");
}

#[pagetop::test]
async fn a_declared_page_shows_its_actions_and_its_declared_title() {
    let _ = Application::prepare(&Ext).await;
    let html = render(Page::admin(request("/admin/config/export"))).await;

    assert!(
        html.contains(r#"<h1 class="admin-page-title">Export</h1>"#),
        "{html}"
    );
    assert!(
        html.contains(r#"href="/admin/config/export/new""#),
        "{html}"
    );
    // Without the permission of the action, its button is not shown.
    assert!(!html.contains("config/export/purge"), "{html}");
    // The declared title also reaches the <title> of the document.
    assert!(html.contains("<title>"), "{html}");
    assert!(html.contains(" | Export</title>"), "{html}");
}

#[pagetop::test]
async fn the_standard_template_has_no_admin_frame() {
    let _ = Application::prepare(&Ext).await;
    let html = render(Page::new(request("/admin/config/export")).with_title(Lc::n("Export"))).await;

    assert!(!html.contains("wrapper-admin"), "{html}");
    assert!(!html.contains("nav-admin"), "{html}");
    assert!(!html.contains("admin-content"), "{html}");
    assert!(html.contains("Body of the page."), "{html}");
}

// Navigation of the panel in its own bar.
fn admin_navbar() -> Navbar {
    Navbar::simple().with_item(navbar::Item::nav(Nav::admin()))
}

#[pagetop::test]
async fn the_admin_menu_goes_where_the_application_places_it() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::admin(request("/admin/config/export/csv"))
        .with_child_in(&CoreRegions::Header, admin_navbar());
    let html = render(page).await;

    // Only the sections with an accessible page.
    let header = &html[html.find(r#"id="header""#).unwrap()..html.find("admin-content").unwrap()];
    assert!(header.contains("nav-admin"), "{header}");
    assert!(header.contains("navbar-nav"), "{header}");
    assert!(
        header.contains(r#"href="/admin/config/export""#),
        "{header}"
    );
    assert!(
        !header.contains(r#"href="/admin/reports/audit""#),
        "{header}"
    );
}

#[pagetop::test]
async fn the_admin_menu_shows_the_icons_of_the_dashboard_sections_and_pages() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::admin(request("/admin/config/export/csv"))
        .with_child_in(&CoreRegions::Header, admin_navbar());
    let html = render(page).await;

    // The dashboard, the "config" section (the only one with an accessible page) and its page.
    let header = &html[html.find(r#"id="header""#).unwrap()..html.find("admin-content").unwrap()];
    assert_eq!(
        header.matches(r#"class="icon icon-start""#).count(),
        3,
        "{header}"
    );
}

#[pagetop::test]
async fn the_admin_menu_only_shows_in_the_panel() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::new(request("/home")).with_child_in(&CoreRegions::Header, admin_navbar());
    let html = render(page).await;

    assert!(!html.contains("nav-admin"), "{html}");
    assert!(!html.contains(r#"href="/admin"#), "{html}");
}

#[pagetop::test]
async fn the_admin_menu_does_not_repeat_its_links_when_rendered_again() {
    let _ = Application::prepare(&Ext).await;
    // The same prototype for every page, as with `InRegion`.
    let navbar = admin_navbar();
    for _ in 0..2 {
        let page = Page::admin(request("/admin/config/export"))
            .with_child_in(&CoreRegions::Header, navbar.clone());
        let html = render(page).await;
        // Links of the menu only (the breadcrumbs also link to the dashboard).
        let links = r#"class="nav-link" href="/admin""#;
        assert_eq!(html.matches(links).count(), 1, "{html}");
    }
}

// Main navbar of an application, with its own menu and the navigation of the panel.
fn main_navbar() -> Navbar {
    Navbar::simple()
        .with_item(navbar::Item::nav(
            Nav::new().with_item(nav::Item::link(Lc::n("Home"), "/home")),
        ))
        .with_item(navbar::Item::nav(Nav::admin()))
}

#[pagetop::test]
async fn the_admin_menu_can_go_in_the_main_navbar() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::admin(request("/admin/config/export/csv"))
        .with_child_in(&CoreRegions::Header, main_navbar());
    let html = render(page).await;

    // The navigation of the panel goes in the main navbar, with the accessible pages only.
    let header = &html[html.find(r#"id="header""#).unwrap()..html.find("admin-content").unwrap()];
    assert!(header.contains(r#"href="/home""#), "{header}");
    assert!(header.contains(r#"href="/admin""#), "{header}");
    assert!(
        header.contains(r#"href="/admin/config/export""#),
        "{header}"
    );
    assert!(
        !header.contains(r#"href="/admin/reports/audit""#),
        "{header}"
    );
}

#[pagetop::test]
async fn the_admin_nav_keeps_its_layout_in_the_main_navbar() {
    let _ = Application::prepare(&Ext).await;
    let navbar = Navbar::simple().with_item(navbar::Item::nav(
        Nav::admin()
            .with_id("admin-nav")
            .with_layout(nav::Layout::Center),
    ));
    let page =
        Page::admin(request("/admin/config/export")).with_child_in(&CoreRegions::Header, navbar);
    let html = render(page).await;

    let start = html
        .find(r#"id="admin-nav""#)
        .expect("the admin nav is rendered");
    let nav = &html[html[..start].rfind('<').unwrap()..];
    let nav = &nav[..nav.find('>').unwrap()];
    assert!(nav.contains("nav-center"), "{nav}");
    assert!(nav.contains("nav-admin"), "{nav}");
    assert!(html.contains(r#"href="/admin/config/export""#), "{html}");
}

// Main navbar whose own menu leaves its place to the navigation of the panel.
fn swapping_navbar() -> Navbar {
    Navbar::simple()
        .with_item(navbar::Item::nav(
            Nav::new()
                .with_item(nav::Item::link(Lc::n("Home"), "/home"))
                .with_renderable(|cx| !is_admin_panel(cx)),
        ))
        .with_item(navbar::Item::nav(Nav::admin()))
}

#[pagetop::test]
async fn the_admin_nav_takes_the_place_of_the_main_menu_in_the_panel() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::admin(request("/admin/config/export"))
        .with_child_in(&CoreRegions::Header, swapping_navbar());
    let html = render(page).await;

    assert!(html.contains(r#"href="/admin/config/export""#), "{html}");
    assert!(!html.contains(r#"href="/home""#), "{html}");
}

#[pagetop::test]
async fn the_main_menu_shows_outside_the_panel() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::new(request("/home")).with_child_in(&CoreRegions::Header, swapping_navbar());
    let html = render(page).await;

    assert!(html.contains(r#"href="/home""#), "{html}");
    assert!(!html.contains(r#"href="/admin"#), "{html}");
}

#[pagetop::test]
async fn the_admin_menu_of_the_main_navbar_only_shows_in_the_panel() {
    let _ = Application::prepare(&Ext).await;
    let page = Page::new(request("/home")).with_child_in(&CoreRegions::Header, main_navbar());
    let html = render(page).await;

    assert!(html.contains(r#"href="/home""#), "{html}");
    assert!(!html.contains(r#"href="/admin"#), "{html}");
}

async fn get(uri: &str) -> (u16, String) {
    let app = Application::prepare(&Ext).await.test();
    let request = web::test::TestRequest::get().uri(uri).to_request();
    let response = web::test::send_request(&app, request).await;
    let status = response.status().as_u16();
    (status, web::test::read_body_text(response).await)
}

#[pagetop::test]
async fn the_dashboard_lists_the_accessible_pages_by_section() {
    let (status, html) = get("/admin").await;
    assert_eq!(status, 200, "{html}");
    assert!(
        html.contains(r#"<a href="/admin/config/export">Export</a>"#),
        "{html}"
    );
    assert!(!html.contains("Audit"), "{html}");
}

#[pagetop::test]
async fn a_section_page_is_mounted_only_for_sections_with_pages() {
    let (status, html) = get("/admin/config").await;
    assert_eq!(status, 200, "{html}");
    assert!(
        html.contains(r#"<a href="/admin/config/export">Export</a>"#),
        "{html}"
    );
    // Its only page needs a permission that nobody has.
    assert_eq!(get("/admin/reports").await.0, 403);
    // No page declared in it.
    assert_eq!(get("/admin/people").await.0, 404);
}

#[pagetop::test]
async fn each_route_of_the_panel_requires_the_permission_of_its_page() {
    // Granted permission, also on a route below the page.
    assert_eq!(get("/admin/config/export").await.0, 200);
    assert_eq!(get("/admin/config/export/5/edit").await.0, 200);
    // Permission not granted, also on a route below the page.
    assert_eq!(get("/admin/reports/audit").await.0, 403);
    assert_eq!(get("/admin/reports/audit/5").await.0, 403);
    // Routes that are not mounted are not affected.
    assert_eq!(get("/admin/config/export/5/missing").await.0, 404);
}
