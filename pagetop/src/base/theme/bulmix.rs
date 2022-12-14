use crate::prelude::*;

pub_handle!(THEME_BULMIX);

include!(concat!(env!("OUT_DIR"), "/bulmix.rs"));

pub struct Bulmix;

impl ThemeTrait for Bulmix {
    fn handle(&self) -> Handle {
        THEME_BULMIX
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        configure_service_for_static_files!(cfg, "/bulmix", bundle_bulmix);
    }

    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_resource(ResourceOp::AddStyleSheet(
                StyleSheet::located("/bulmix/css/bulma.min.css")
                    .with_version("0.9.4")
                    .with_weight(-99),
            ))
            .alter_resource(ResourceOp::AddJQuery);
    }

    #[rustfmt::skip]
    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        _rsx: &mut RenderResources,
    ) {
        match component.handle() {
            COMPONENT_ANCHOR => {
                let a = component_mut::<Anchor>(component);
                a.alter_classes(
                    ClassesOp::SetDefault,
                    match a.anchor_type() {
                        AnchorType::Button => "button is-primary",
                        _ => "",
                    },
                );
            }
            COMPONENT_HEADING => {
                let h = component_mut::<Heading>(component);
                h.alter_classes(
                    ClassesOp::SetDefault,
                    match h.display() {
                        HeadingDisplay::XxLarge  => "title is-1",
                        HeadingDisplay::Large    => "title is-2",
                        HeadingDisplay::Medium   => "title is-3",
                        HeadingDisplay::Small    => "title is-4",
                        HeadingDisplay::XxSmall  => "title is-5",
                        HeadingDisplay::Normal   => "title",
                        HeadingDisplay::Subtitle => "subtitle",
                    },
                );
            }
            COMPONENT_PARAGRAPH => {
                let p = component_mut::<Paragraph>(component);
                p.alter_classes(
                    ClassesOp::SetDefault,
                    match p.display() {
                        ParagraphDisplay::XxLarge => "is-size-2",
                        ParagraphDisplay::Large   => "is-size-3",
                        ParagraphDisplay::Medium  => "is-size-4",
                        ParagraphDisplay::Small   => "is-size-5",
                        ParagraphDisplay::XxSmall => "is-size-6",
                        ParagraphDisplay::Normal  => "",
                    },
                );
            }
            grid::COMPONENT_COLUMN => {
                let col = component_mut::<grid::Column>(component);
                col.alter_classes(
                    ClassesOp::SetDefault,
                    concat_string!(
                        "column",
                        match col.size() {
                            grid::ColumnSize::Default  => "",
                            grid::ColumnSize::Is1of12  => " is-1",
                            grid::ColumnSize::Is2of12  => " is-2",
                            grid::ColumnSize::Is3of12  => " is-3",
                            grid::ColumnSize::Is4of12  => " is-4",
                            grid::ColumnSize::Is5of12  => " is-5",
                            grid::ColumnSize::Is6of12  => " is-6",
                            grid::ColumnSize::Is7of12  => " is-7",
                            grid::ColumnSize::Is8of12  => " is-8",
                            grid::ColumnSize::Is9of12  => " is-9",
                            grid::ColumnSize::Is10of12 => " is-10",
                            grid::ColumnSize::Is11of12 => " is-11",
                            grid::ColumnSize::IsFull   => " is-12",
                        },
                        " content"
                    )
                    .as_str(),
                );
            }
            grid::COMPONENT_ROW => {
                let row = component_mut::<grid::Row>(component);
                row.alter_classes(ClassesOp::SetDefault, "columns");
            }
            _ => {}
        }
    }

    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        _rsx: &mut RenderResources,
    ) -> Option<Markup> {
        match component.handle() {
            COMPONENT_ICON => {
                let icon = component_ref::<Icon>(component);
                Some(html! {
                    span class="icon" {
                        i class=[icon.classes().get()] {};
                    }
                })
            }
            _ => None,
        }
    }
}
