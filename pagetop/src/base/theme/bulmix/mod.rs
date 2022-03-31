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

    fn render_component(
        &self,
        component: &dyn PageComponent,
        assets: &mut PageAssets
    ) -> Option<Markup> {
        match component.name() {
            "GridRow" => {
                let row = component.downcast_ref::<grid::Row>().unwrap();
                Some(html! {
                    div id=[row.id()] class=[row.classes("columns")] {
                        (row.columns().render(assets))
                    }
                })
            },
            "GridColumn" => {
                let col = component.downcast_ref::<grid::Column>().unwrap();
                Some(html! {
                    div id=[col.id()] class=[col.classes("column")] {
                        (col.components().render(assets))
                    }
                })
            },
            _ => None
        }
    }
}
