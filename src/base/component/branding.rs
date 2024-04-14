use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Branding {
    id        : OptionId,
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

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let logo = self.logo().render(cx);
        let home = self.frontpage()(cx);
        let title = &L10n::l("site_home").using(cx.langid());
        PrepareMarkup::With(html! {
            div id=[self.id()] class="branding__container" {
                div class="branding__content" {
                    @if !logo.is_empty() {
                        a class="branding__logo" href=(home) title=[title] rel="home" {
                            (logo)
                        }
                    }
                    div class="branding__text" {
                        a class="branding__name" href=(home) title=[title] rel="home" {
                            (self.app_name())
                        }
                        @if let Some(slogan) = self.slogan().using(cx.langid()) {
                            div class="branding__slogan" {
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
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
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
    pub fn alter_logo(&mut self, logo: Option<Image>) -> &mut Self {
        self.logo.alter_value(logo);
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

    pub fn logo(&self) -> &OptionComponent<Image> {
        &self.logo
    }

    pub fn frontpage(&self) -> &FnContextualPath {
        &self.frontpage
    }
}
