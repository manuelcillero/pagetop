use pagetop::prelude::*;

use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Brand {
    id       : AttrId,
    #[default(_code = "global::SETTINGS.app.name.to_owned()")]
    app_name : String,
    slogan   : AttrL10n,
    logo     : Typed<Image>,
    #[default(_code = "|_| \"/\"")]
    home     : FnPathByContext,
}

impl Component for Brand {
    fn new() -> Self {
        Brand::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let logo = self.logo().render(cx);
        let home = self.home()(cx);
        let title = &L10n::l("site_home").lookup(cx);
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
                        @if let Some(slogan) = self.slogan().lookup(cx) {
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

    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[builder_fn]
    pub fn with_app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = app_name.into();
        self
    }

    #[builder_fn]
    pub fn with_slogan(mut self, slogan: L10n) -> Self {
        self.slogan.alter_value(slogan);
        self
    }

    #[builder_fn]
    pub fn with_logo(mut self, logo: Option<Image>) -> Self {
        self.logo.alter_component(logo);
        self
    }

    #[builder_fn]
    pub fn with_home(mut self, home: FnPathByContext) -> Self {
        self.home = home;
        self
    }

    // Brand GETTERS.

    pub fn app_name(&self) -> &String {
        &self.app_name
    }

    pub fn slogan(&self) -> &AttrL10n {
        &self.slogan
    }

    pub fn logo(&self) -> &Typed<Image> {
        &self.logo
    }

    pub fn home(&self) -> &FnPathByContext {
        &self.home
    }
}
