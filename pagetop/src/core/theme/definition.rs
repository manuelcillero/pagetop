use crate::app;
use crate::app::SETTINGS;
use crate::base::component::{Container, Html};
use crate::concat_string;
use crate::core::component::ComponentTrait;
use crate::html::{html, Favicon, Markup};
use crate::response::page::{Page, PageContext, PageOp};
use crate::util::{single_type_name, Handle};

pub type ThemeStaticRef = &'static dyn ThemeTrait;

pub trait BaseTheme {
    fn single_name(&self) -> &'static str;
}

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: BaseTheme + Send + Sync {
    fn handle(&self) -> Handle;

    fn name(&self) -> String {
        self.single_name().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {}

    #[allow(unused_variables)]
    fn before_render_page(&self, page: &mut Page) {
        page.alter_context(PageOp::AddFavicon(
            Favicon::new().with_icon("/theme/favicon.png"),
        ));
    }

    fn render_page_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @match page.title().get() {
                    Some(t) => title {
                        (concat_string!(SETTINGS.app.name, " | ", t))
                    },
                    None => title { (SETTINGS.app.name) }
                }

                @if let Some(d) = page.description().get() {
                    meta name="description" content=(d);
                }
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content=(viewport);

                (page.context().render())
            }
        }
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body class=[page.body_classes().get()] {
                @match page.template() {
                    "admin" => {
                        @for region in &["top-menu", "side-menu", "region-content"] {
                            @if let Some(content) = page.render_region(region) {
                                #(region) { (content) }
                            }
                        }
                    },
                    _ => {
                        @for region in &["region-content"] {
                            @if let Some(content) = page.render_region(region) {
                                #(region) { (content) }
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        context: &mut PageContext,
    ) {
        /*
            Cómo usarlo:

            match component.handle() {
                BLOCK_COMPONENT => {
                    let block = component_mut::<Block>(component);
                    block.alter_title("New title");
                },
                _ => {},
            }
        */
    }

    #[allow(unused_variables)]
    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        context: &mut PageContext,
    ) -> Option<Markup> {
        None
        /*
            Cómo usarlo:

            match component.handle() {
                BLOCK_COMPONENT => {
                    let block = component_ref::<Block>(component);
                    match block.template() {
                        "default" => Some(block_default(block)),
                        _ => None,
                    }
                },
                _ => None,
            }
        */
    }

    fn error_404_not_found(&self) -> Container {
        Container::new().with_component(Html::with(html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
            }
        }))
    }

    fn error_403_access_denied(&self) -> Container {
        Container::new().with_component(Html::with(html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        }))
    }
}

impl<T: ?Sized + ThemeTrait> BaseTheme for T {
    fn single_name(&self) -> &'static str {
        single_type_name::<Self>()
    }
}
