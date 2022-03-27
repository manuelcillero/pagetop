use pagetop::prelude::*;

localize!("src/locales");

//mod entity;
mod migration;

pub struct NodeModule;

impl ModuleTrait for NodeModule {
    fn name(&self) -> &'static str {
        "Node"
    }

    fn fullname(&self) -> String {
        l("module_fullname")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.route("/node", app::web::get().to(node));
    }

    fn migrations(&self) -> Vec<Box<dyn db::migration::MigrationTrait>> {
        vec![
            boxed_migration!(m20220316_000001_create_table_node_type),
            boxed_migration!(m20220316_000002_create_table_node),
            boxed_migration!(m20220316_000003_create_table_node_access),
            boxed_migration!(m20220316_000004_create_table_node_revision),
        ]
    }
}

async fn node() -> app::Result<Markup> {
    Page::new()
        .with_title(
            "Nodo"
        )
        .render()
}
