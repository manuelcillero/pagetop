use crate::core::component::{ComponentTrait, Context};
use crate::core::package::PackageTrait;
use crate::html::{html, Favicon, Markup, OptionId};
use crate::locale::L10n;
use crate::response::page::Page;
use crate::{concat_string, config};

pub type ThemeRef = &'static dyn ThemeTrait;

/// Theme built-in classes used by the default page rendering process.
///
/// The [`ThemeTrait`](crate::core::theme::ThemeTrait) default implementation uses these CSS classes
/// in the [`prepare_region()`](crate::core::theme::ThemeTrait::prepare_region) and
/// [`prepare_body()`](crate::core::theme::ThemeTrait::prepare_body) methods to build the HTML code
/// for regions and page body main containers.
///
/// Theme developers can customize the default implementation of
/// [`builtin_classes()`](crate::core::theme::ThemeTrait::builtin_classes) method to return
/// alternative class name or space-separated class names for each variant, without altering the
/// default page rendering process.
pub enum ThemeBuiltInClasses {
    /// Skip to content link. Default is `skip__to_content`.
    SkipToContent,
    /// Main body wrapper. Default is `body__wrapper`.
    BodyWrapper,
    /// Main content wrapper. Default is `content__wrapper`.
    ContentWrapper,
    /// A region container. Default is `region__container`.
    RegionContainer,
    /// The region inner content. Default is `region__content`.
    RegionContent,
}

#[rustfmt::skip]
impl ToString for ThemeBuiltInClasses {
    fn to_string(&self) -> String {
        match self {
            ThemeBuiltInClasses::SkipToContent   => String::from("skip__to_content"),
            ThemeBuiltInClasses::BodyWrapper     => String::from("body__wrapper"),
            ThemeBuiltInClasses::ContentWrapper  => String::from("content__wrapper"),
            ThemeBuiltInClasses::RegionContainer => String::from("region__container"),
            ThemeBuiltInClasses::RegionContent   => String::from("region__content"),
        }
    }
}
/// Los temas deben implementar este "trait".
pub trait ThemeTrait: PackageTrait + Send + Sync {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("header",        L10n::l("header")),
            ("pagetop",       L10n::l("pagetop")),
            ("sidebar_left",  L10n::l("sidebar_left")),
            ("content",       L10n::l("content")),
            ("sidebar_right", L10n::l("sidebar_right")),
            ("footer",        L10n::l("footer")),
        ]
    }

    #[rustfmt::skip]
    /// Return the name of the CSS class or space-separated class names associated with each variant
    /// of [ThemeBuiltInClasses].
    ///
    /// Theme developers can customize the default implementation of this method to return
    /// alternative class name or space-separated class names for each variant, without altering the
    /// default page rendering process.
    fn builtin_classes(&self, builtin: ThemeBuiltInClasses) -> Option<String> {
        Some(builtin.to_string())
    }

    fn prepare_region(&self, page: &mut Page, region_name: &str) -> Markup {
        let render_region = page.components_in(region_name).render(page.context());
        if render_region.is_empty() {
            return html! {};
        }
        html! {
            div
                id=[OptionId::new(region_name).get()]
                class=[self.builtin_classes(ThemeBuiltInClasses::RegionContainer)]
            {
                div class=[self.builtin_classes(ThemeBuiltInClasses::RegionContent)] {
                    (render_region)
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn before_prepare_body(&self, page: &mut Page) {}

    fn prepare_body(&self, page: &mut Page) -> Markup {
        let skip_to = concat_string!("#", page.skip_to().get().unwrap_or("content".to_owned()));

        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                    div class=[self.builtin_classes(ThemeBuiltInClasses::SkipToContent)] {
                        a href=(skip_to) { (skip) }
                    }
                }
                div class=[self.builtin_classes(ThemeBuiltInClasses::BodyWrapper)] {
                    (self.prepare_region(page, "header"))
                    (self.prepare_region(page, "pagetop"))
                    div class=[self.builtin_classes(ThemeBuiltInClasses::ContentWrapper)] {
                        (self.prepare_region(page, "sidebar_left"))
                        (self.prepare_region(page, "content"))
                        (self.prepare_region(page, "sidebar_right"))
                    }
                    (self.prepare_region(page, "footer"))
                }
            }
        }
    }

    fn after_prepare_body(&self, page: &mut Page) {
        if page.favicon().is_none() {
            page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")));
        }
    }

    fn prepare_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @if let Some(title) = page.title() {
                    title { (config::SETTINGS.app.name) (" - ") (title) }
                } @else {
                    title { (config::SETTINGS.app.name) }
                }

                @if let Some(description) = page.description() {
                    meta name="description" content=(description);
                }

                meta name="viewport" content=(viewport);
                @for (name, content) in page.metadata() {
                    meta name=(name) content=(content) {}
                }

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                @for (property, content) in page.properties() {
                    meta property=(property) content=(content) {}
                }

                @if let Some(favicon) = page.favicon() {
                    (favicon.prepare())
                }

                (page.context().prepare_assets())
            }
        }
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn before_prepare_component(
        &self,
        component: &mut dyn ComponentTrait,
        cx: &mut Context,
    ) {
        /*
            Cómo usarlo:

            match component.type_id() {
                t if t == TypeId::of::<Block>() => {
                    if let Some(b) = component_as_mut::<Block>(component) {
                        b.alter_title("New title");
                    }
                },
                _ => {},
            }
        */
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn after_prepare_component(
        &self,
        component: &mut dyn ComponentTrait,
        cx: &mut Context,
    ) {
        /*
            Cómo usarlo:

            match component.type_id() {
                t if t == TypeId::of::<Block>() => {
                    if let Some(b) = component_as_mut::<Block>(component) {
                        b.alter_title("New title");
                    }
                },
                _ => {},
            }
        */
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        cx: &mut Context,
    ) -> Option<Markup> {
        None
        /*
            Cómo usarlo:

            match component.type_id() {
                t if t == TypeId::of::<Block>() => {
                    Some(block_default(block))
                },
                _ => None,
            }
        */
    }
}
