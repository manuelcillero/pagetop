use pagetop::prelude::*;

// The administration registry is built once per process: this test needs its own binary.

struct ToolsPermission;

impl Permission for ToolsPermission {
    fn key(&self) -> CowStr {
        "tools:use".into()
    }
}

fn declare_pages(bag: &mut AdminPageBag) {
    bag.add(AdminPage {
        path: "config/export".to_owned(),
        section: "config".to_owned(),
        title: Lc::n("Export"),
        description: Lc::none(),
        weight: 0,
        permission: &ToolsPermission,
    });
}

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![action::admin::DeclareAdminPages::new(declare_pages)]
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        let tools = Access::Permission(&ToolsPermission);
        routes
            // Valid: hangs from a declared page, whose permission it requires.
            .admin_route("config/export/{id}", web::get(unchecked))
            // Does not hang from any declared page.
            .admin_route("undeclared", web::get(unchecked))
            // A route of the panel mounted without `admin_route()`, even with the right permission.
            .route(&admin_path("config/export/new"), tools, web::get(unchecked))
    }
}

async fn unchecked() -> &'static str {
    "unchecked"
}

#[pagetop::test]
#[should_panic(expected = "declared page: /admin/undeclared, /admin/config/export/new")]
async fn a_route_of_the_panel_mounted_wrongly_stops_the_application() {
    let _ = Application::prepare(&Ext).await.test();
}
