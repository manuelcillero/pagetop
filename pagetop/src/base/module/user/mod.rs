use crate::prelude::*;

localize!("en-US", "src/base/module/user/locales");

mod entity;
mod migration;

pub struct UserModule;

impl ModuleTrait for UserModule {
    fn name(&self) -> &'static str {
        "user"
    }

    fn fullname(&self) -> String {
        l("module_fullname")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.route("/user/login", server::web::get().to(login));
    }

    fn migrations(&self, dbconn: &db::DbConn) -> Result<(), db::DbErr> {
        db_migrations!(dbconn)
    }
}

fn form_login() -> impl PageComponent {
    Form::prepare()
        .with_id("user-login")
        .add(form::Input::textfield()
            .with_name("name")
            .with_label(l("username").as_str())
            .with_help_text(t("username_help", &args![
                "app" => SETTINGS.app.name.to_owned()
            ]).as_str())
            .autofocus(true)
        )
        .add(form::Input::password()
            .with_name("pass")
            .with_label(l("password").as_str())
            .with_help_text(l("password_help").as_str())
        )
        .add(form::Button::submit(l("login").as_str()))
}

async fn login() -> server::Result<Markup> {
    Page::prepare()
        .with_title(
            "Identificación del usuario"
        )
        .add_to("content", Container::prepare()
            .with_id("welcome")
            .add(form_login())
        )
        .render()
}
