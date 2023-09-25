use crate::prelude::*;
use crate::LOCALES_PAGETOP;

new_handle!(COMPONENT_BRANDING);

type SiteSlogan = TypedComponent<L10n>;
type SiteLogo = TypedComponent<Image>;

#[rustfmt::skip]
pub struct SiteBranding {
    weight    : Weight,
    renderable: Renderable,
    name      : String,
    slogan    : SiteSlogan,
    logo      : SiteLogo,
    frontpage : FnContextualPath,
}

#[rustfmt::skip]
impl Default for SiteBranding {
    fn default() -> Self {
        SiteBranding {
            weight    : Weight::default(),
            renderable: Renderable::default(),
            name      : config::SETTINGS.app.name.to_owned(),
            slogan    : SiteSlogan::default(),
            logo      : SiteLogo::default(),
            frontpage : |_| "/",
        }
    }
}

impl ComponentTrait for SiteBranding {
    fn new() -> Self {
        SiteBranding::default()
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

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let title = L10n::t("site_home", &LOCALES_PAGETOP).prepare(cx);
        let slogan = self.slogan().prepare(cx);
        PrepareMarkup::With(html! {
            div id=[self.id()] {
                div class="site-branding-wrapper" {
                    div class="site-branding-logo" {
                        (self.logo().prepare(cx))
                    }
                    div class="site-branding-text" {
                        div class="site-branding-name" {
                            a href=(self.frontpage()(cx)) title=(title) rel="home" { (self.name()) }
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
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
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

    #[fn_builder]
    pub fn alter_frontpage(&mut self, frontpage: FnContextualPath) -> &mut Self {
        self.frontpage = frontpage;
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

    pub fn frontpage(&self) -> &FnContextualPath {
        &self.frontpage
    }
}
