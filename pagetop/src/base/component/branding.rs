use crate::prelude::*;

new_handle!(COMPONENT_BASE_BRANDING);

type SiteLogo = TypedComponent<Image>;

#[rustfmt::skip]
pub struct Branding {
    weight    : Weight,
    renderable: Renderable,
    app_name  : String,
    slogan    : OptionTranslated,
    logo      : SiteLogo,
    frontpage : FnContextualPath,
}

#[rustfmt::skip]
impl Default for Branding {
    fn default() -> Self {
        Branding {
            weight    : Weight::default(),
            renderable: Renderable::default(),
            app_name  : config::SETTINGS.app.name.to_owned(),
            slogan    : OptionTranslated::default(),
            logo      : SiteLogo::default(),
            frontpage : |_| "/",
        }
    }
}

impl ComponentTrait for Branding {
    fn new() -> Self {
        Branding::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_BRANDING
    }

    fn id(&self) -> Option<String> {
        Some("pt-branding".to_owned())
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let title = L10n::l("site_home").using(cx.langid());
        PrepareMarkup::With(html! {
            div id=[self.id()] {
                div class="pt-branding__wrapper" {
                    div class="pt-branding__logo" {
                        (self.logo().render(cx))
                    }
                    div class="pt-branding__text" {
                        div class="pt-branding__name" {
                            a href=(self.frontpage()(cx)) title=[title] rel="home" {
                                (self.app_name())
                            }
                        }
                        @if let Some(slogan) = self.slogan().using(cx.langid()) {
                            div class="pt-branding__slogan" {
                                (slogan)
                            }
                        }
                    }
                }
            }
        })
    }
}

impl Branding {
    // Branding BUILDER.

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
    pub fn alter_app_name(&mut self, app_name: impl Into<String>) -> &mut Self {
        self.app_name = app_name.into();
        self
    }

    #[fn_builder]
    pub fn alter_slogan(&mut self, slogan: L10n) -> &mut Self {
        self.slogan.alter_value(slogan);
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

    // Branding GETTERS.

    pub fn app_name(&self) -> &String {
        &self.app_name
    }

    pub fn slogan(&self) -> &OptionTranslated {
        &self.slogan
    }

    pub fn logo(&self) -> &SiteLogo {
        &self.logo
    }

    pub fn frontpage(&self) -> &FnContextualPath {
        &self.frontpage
    }
}
