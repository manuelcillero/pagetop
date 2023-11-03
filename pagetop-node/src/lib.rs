use pagetop::prelude::*;

new_static_locales!(LOCALES_NODE);

//mod entity;
mod migration;

pub struct Node;

impl_handle!(MODULE_NODE for Node);

impl ModuleTrait for Node {
    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_NODE)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_NODE)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/node", service::web::get().to(node));
    }

    fn actions(&self) -> Vec<Action> {
        actions![action::page::BeforePrepareBody::with(before_prepare_body).with_weight(-1)]
    }

    fn migrations(&self) -> Vec<MigrationItem> {
        migrations![
            m20220316_000001_create_table_node_type,
            m20220316_000002_create_table_node,
            m20220316_000003_create_table_node_access,
            m20220316_000004_create_table_node_revision,
        ]
    }
}

async fn node(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request).with_title(L10n::n("Nodo")).render()
}

fn before_prepare_body(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-node");
}
