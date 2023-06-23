use pagetop::prelude::*;

use_handle!(MODULE_NODE);

use_locale!(LOCALE_NODE, "src/locale");

//mod entity;
mod migration;

pub struct Node;

impl ModuleTrait for Node {
    fn handle(&self) -> Handle {
        MODULE_NODE
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALE_NODE)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALE_NODE)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/node", service::web::get().to(node));
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(ActionBeforeRenderPage => before_render_page, -1)]
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

fn before_render_page(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-node");
}
