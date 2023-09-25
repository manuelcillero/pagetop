use pagetop::prelude::*;

new_handle!(MODULE_USER);

static_locales!(LOCALES_USER);

mod migration;

pub struct User;

impl ModuleTrait for User {
    fn handle(&self) -> Handle {
        MODULE_USER
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_USER)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_USER)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/user/login", service::web::get().to(login));
    }

    fn migrations(&self) -> Vec<MigrationItem> {
        migrations![
            m20220312_000001_create_table_role,
            m20220312_000002_create_table_role_permission,
            m20220312_000003_create_table_user,
            m20220312_000004_create_table_user_role,
        ]
    }
}

async fn login(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_title(L10n::n("Identificación del usuario"))
        .with_in(
            "content",
            Wrapper::new()
                .with_id("welcome")
                .with_component(form_login()),
        )
        .render()
}

fn form_login() -> Form {
    Form::new()
        .with_id("user-login")
        .with_element(
            form_element::Input::textfield()
                .with_name("name")
                .with_label(L10n::t("username", &LOCALES_USER))
                .with_help_text(
                    L10n::t("username_help", &LOCALES_USER)
                        .with_arg("app", config::SETTINGS.app.name.to_owned()),
                )
                .with_autofocus(true),
        )
        .with_element(
            form_element::Input::password()
                .with_name("pass")
                .with_label(L10n::t("password", &LOCALES_USER))
                .with_help_text(L10n::t("password_help", &LOCALES_USER)),
        )
        .with_element(form_element::Button::submit(L10n::t(
            "login",
            &LOCALES_USER,
        )))
}
