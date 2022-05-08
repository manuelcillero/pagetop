use crate::prelude::*;

pub const BULMIX_THEME: &str = "pagetop::theme::bulmix";

include!(concat!(env!("OUT_DIR"), "/bulmix.rs"));

pub struct Bulmix;

impl ThemeTrait for Bulmix {
    fn handler(&self) -> &'static str {
        BULMIX_THEME
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        theme_static_files!(cfg, "/bulmix");
    }

    fn before_render_page(&self, page: &mut Page) {
        page.assets()
            .with_favicon(
                Favicon::new()
                    .with_icon("/theme/favicon.png")
            )
            .add_stylesheet(
                StyleSheet::source(
                    "/bulmix/css/bulma.min.css?ver=0.9.3"
                )
                .with_weight(-99)
            )
            .add_jquery();
    }

    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        _assets: &mut Assets
    ) {
        match component.handler() {
            grid::ROW_COMPONENT => {
                let row = component_mut::<grid::Row>(component);
                row.alter_classes("columns", ClassesOp::SetDefault);
            },
            grid::COLUMN_COMPONENT => {
                let col = component_mut::<grid::Column>(component);
                col.alter_classes("column", ClassesOp::SetDefault);
            },
            _ => {},
        }
    }
}
