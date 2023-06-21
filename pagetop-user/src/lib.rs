use pagetop::prelude::*;
use pagetop_minimal::component::*;

define_handle!(MODULE_USER);

define_locale!(LOCALE_USER, "src/locale");

mod migration;

pub struct User;

impl ModuleTrait for User {
    fn handle(&self) -> Handle {
        MODULE_USER
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALE_USER)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALE_USER)
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&pagetop_minimal::Minimal]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/user/login", service::web::get().to(login));
    }

    fn migrations(&self) -> Vec<MigrationItem> {
        vec![
            migration_item!(m20220312_000001_create_table_role),
            migration_item!(m20220312_000002_create_table_role_permission),
            migration_item!(m20220312_000003_create_table_user),
            migration_item!(m20220312_000004_create_table_user_role),
        ]
    }
}

async fn login(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_title(L10n::n("Identificación del usuario"))
        .with_in(
            "content",
            Container::new()
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
                .with_label(L10n::t("username", &LOCALE_USER))
                .with_help_text(
                    L10n::t("username_help", &LOCALE_USER)
                        .with_arg("app", config::SETTINGS.app.name.to_owned()),
                )
                .with_autofocus(true),
        )
        .with_element(
            form_element::Input::password()
                .with_name("pass")
                .with_label(L10n::t("password", &LOCALE_USER))
                .with_help_text(L10n::t("password_help", &LOCALE_USER)),
        )
        .with_element(form_element::Button::submit(L10n::t("login", &LOCALE_USER)))
}
