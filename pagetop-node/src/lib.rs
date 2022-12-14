use pagetop::prelude::*;

pub_handle!(MODULE_NODE);

pub_locale!("src/locales");

//mod entity;
mod migration;

pub struct Node;

impl ModuleTrait for Node {
    fn handle(&self) -> Handle {
        MODULE_NODE
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.route("/node", server::web::get().to(node));
    }

    fn actions(&self) -> Vec<HookAction> {
        vec![hook_action!(BeforeRenderPageHook => before_render_page, -1)]
    }

    fn migrations(&self) -> Vec<MigrationItem> {
        vec![
            migration_item!(m20220316_000001_create_table_node_type),
            migration_item!(m20220316_000002_create_table_node),
            migration_item!(m20220316_000003_create_table_node_access),
            migration_item!(m20220316_000004_create_table_node_revision),
        ]
    }
}

async fn node() -> ResultPage<Markup, FatalError> {
    Page::new().with_title("Nodo").render()
}

fn before_render_page(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-node");
}
