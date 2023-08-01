use pagetop::prelude::*;

use crate::component::Image;
use crate::Minimal;
use crate::LOCALES_MINIMAL;

new_handle!(COMPONENT_BRANDING);

type SiteSlogan = OneComponent<L10n>;
type SiteLogo = OneComponent<Image>;

#[rustfmt::skip]
#[derive(Default)]
pub struct SiteBranding {
    weight    : Weight,
    renderable: Renderable,
    name      : String,
    slogan    : SiteSlogan,
    logo      : SiteLogo,
}

impl ComponentTrait for SiteBranding {
    fn new() -> Self {
        SiteBranding {
            name: config::SETTINGS.app.name.to_owned(),
            ..Default::default()
        }
    }

    fn handle(&self) -> Handle {
        COMPONENT_BRANDING
    }

    fn id(&self) -> Option<String> {
        Some("site-branding".to_owned())
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        Minimal.load_assets(cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let title = L10n::t("site_home", &LOCALES_MINIMAL).prepare(cx);
        let slogan = self.slogan().prepare(cx);
        PrepareMarkup::With(html! {
            div id=[self.id()] {
                div class="site-branding-wrapper" {
                    div class="site-branding-logo" {
                        (self.logo().prepare(cx))
                    }
                    div class="site-branding-text" {
                        div class="site-branding-name" {
                            a href="/" title=(title) rel="home" { (self.name()) }
                        }
                        @if !slogan.is_empty() {
                            div class="site-branding-slogan" {
                                (slogan)
                            }
                        }
                    }
                }
            }
        })
    }
}

impl SiteBranding {
    // SiteBranding BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    #[fn_builder]
    pub fn alter_slogan(&mut self, slogan: L10n) -> &mut Self {
        self.slogan = SiteSlogan::with(slogan);
        self
    }

    #[fn_builder]
    pub fn alter_logo(&mut self, logo: Image) -> &mut Self {
        self.logo.set(logo);
        self
    }

    // SiteBranding GETTERS.

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn slogan(&self) -> &SiteSlogan {
        &self.slogan
    }

    pub fn logo(&self) -> &SiteLogo {
        &self.logo
    }
}
