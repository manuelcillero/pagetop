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
        page.context()
            .with_favicon(Some(Favicon::new()
                .with_icon("/theme/favicon.png")
            ))
            .add_stylesheet(
                StyleSheet::with_source(
                    "/bulmix/css/bulma.min.css?ver=0.9.4"
                )
                .with_weight(-99)
            )
            .add_jquery();
    }

    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        _context: &mut InContext
    ) {
        match component.handler() {
            ANCHOR_COMPONENT => {
                let a = component_mut::<Anchor>(component);
                a.alter_classes(match a.anchor_type() {
                    AnchorType::Button => "button is-primary",
                    _ => "",
                }, ClassesOp::SetDefault);
            },
            HEADING_COMPONENT => {
                let h = component_mut::<Heading>(component);
                h.alter_classes(match h.display() {
                    HeadingDisplay::XxLarge  => "title is-1",
                    HeadingDisplay::Large    => "title is-2",
                    HeadingDisplay::Medium   => "title is-3",
                    HeadingDisplay::Small    => "title is-4",
                    HeadingDisplay::XxSmall  => "title is-5",
                    HeadingDisplay::Normal   => "title",
                    HeadingDisplay::Subtitle => "subtitle",
                }, ClassesOp::SetDefault);
            },
            PARAGRAPH_COMPONENT => {
                let p = component_mut::<Paragraph>(component);
                p.alter_classes(match p.display() {
                    ParagraphDisplay::XxLarge => "is-size-2",
                    ParagraphDisplay::Large   => "is-size-3",
                    ParagraphDisplay::Medium  => "is-size-4",
                    ParagraphDisplay::Small   => "is-size-5",
                    ParagraphDisplay::XxSmall => "is-size-6",
                    ParagraphDisplay::Normal  => "",
                }, ClassesOp::SetDefault);
            },
            grid::COLUMN_COMPONENT => {
                let col = component_mut::<grid::Column>(component);
                col.alter_classes(concat_string!("column", match col.size() {
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
                }, " content").as_str(), ClassesOp::SetDefault);
            },
            grid::ROW_COMPONENT => {
                let row = component_mut::<grid::Row>(component);
                row.alter_classes("columns", ClassesOp::SetDefault);
            },
            _ => {},
        }
    }

    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        _context: &mut InContext
    ) -> Option<Markup> {
        match component.handler() {
            ICON_COMPONENT => {
                let icon = component_ref::<Icon>(component);
                Some(html! {
                    span class="icon" {
                        i class=[icon.classes().get()] style=[icon.layout().get()] {};
                    }
                })
            },
            _ => None,
        }
    }
}
