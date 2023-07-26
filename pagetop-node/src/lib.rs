use pagetop::prelude::*;

new_handle!(MODULE_NODE);

static_locales!(LOCALES_NODE);

//mod entity;
mod migration;

pub struct Node;

impl ModuleTrait for Node {
    fn handle(&self) -> Handle {
        MODULE_NODE
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_NODE)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_NODE)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/node", service::web::get().to(node));
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(ActionBeforePrepareBody => before_prepare_body, -1)]
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

async fn node(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request).with_title(L10n::n("Nodo")).render()
}

fn before_prepare_body(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-node");
}
