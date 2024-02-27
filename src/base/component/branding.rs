use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Branding {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    #[default(_code = "config::SETTINGS.app.name.to_owned()")]
    app_name  : String,
    slogan    : OptionTranslated,
    logo      : OptionComponent<Image>,
    #[default(_code = "|_| \"/\"")]
    frontpage : FnContextualPath,
}

impl ComponentTrait for Branding {
    fn new() -> Self {
        Branding::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let logo = self.logo().render(cx);
        let title = L10n::l("site_home").using(cx.langid());
        PrepareMarkup::With(html! {
            div id=[self.id()] class="pt-branding" {
                div class="pt-branding__wrapper" {
                    @if !logo.is_empty() {
                        div class="pt-branding__logo" { (logo) }
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

    #[fn_with]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_with]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_with]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_with]
    pub fn alter_app_name(&mut self, app_name: impl Into<String>) -> &mut Self {
        self.app_name = app_name.into();
        self
    }

    #[fn_with]
    pub fn alter_slogan(&mut self, slogan: L10n) -> &mut Self {
        self.slogan.alter_value(slogan);
        self
    }

    #[fn_with]
    pub fn alter_logo(&mut self, logo: Option<Image>) -> &mut Self {
        self.logo.alter_value(logo);
        self
    }

    #[fn_with]
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

    pub fn logo(&self) -> &OptionComponent<Image> {
        &self.logo
    }

    pub fn frontpage(&self) -> &FnContextualPath {
        &self.frontpage
    }
}
