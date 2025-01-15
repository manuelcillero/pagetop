use pagetop::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Brand {
    id       : OptionId,
    #[default(_code = "global::SETTINGS.app.name.to_owned()")]
    app_name : String,
    slogan   : OptionTranslated,
    logo     : OptionComponent<Image>,
    #[default(_code = "|_| \"/\"")]
    home     : FnContextualPath,
}

impl ComponentTrait for Brand {
    fn new() -> Self {
        Brand::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let logo = self.logo().render(cx);
        let home = self.home()(cx);
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

impl Brand {
    // Brand BUILDER.

    #[fn_builder]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn with_app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = app_name.into();
        self
    }

    #[fn_builder]
    pub fn with_slogan(mut self, slogan: L10n) -> Self {
        self.slogan.alter_value(slogan);
        self
    }

    #[fn_builder]
    pub fn with_logo(mut self, logo: Option<Image>) -> Self {
        self.logo.alter_value(logo);
        self
    }

    #[fn_builder]
    pub fn with_home(mut self, home: FnContextualPath) -> Self {
        self.home = home;
        self
    }

    // Brand GETTERS.

    pub fn app_name(&self) -> &String {
        &self.app_name
    }

    pub fn slogan(&self) -> &OptionTranslated {
        &self.slogan
    }

    pub fn logo(&self) -> &OptionComponent<Image> {
        &self.logo
    }

    pub fn home(&self) -> &FnContextualPath {
        &self.home
    }
}
