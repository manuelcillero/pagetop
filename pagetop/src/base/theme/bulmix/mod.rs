use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bulmix.rs"));

pub struct BulmixTheme;

impl ThemeTrait for BulmixTheme {
    fn name(&self) -> &'static str {
        "Bulmix"
    }

    fn fullname(&self) -> String {
        "Bulmix".to_owned()
    }

    fn configure_theme(&self, cfg: &mut app::web::ServiceConfig) {
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
        component: &mut dyn PageComponent,
        _assets: &mut PageAssets
    ) {
        match component.name() {
            "GridRow" => {
                let row = component.as_any().downcast_mut::<grid::Row>().unwrap();
                row.alter_classes("columns", ClassesOp::SetDefault);
            },
            "GridColumn" => {
                let col = component.as_any().downcast_mut::<grid::Column>().unwrap();
                col.alter_classes("column", ClassesOp::SetDefault);
            },
            _ => {},
        }
    }
}
