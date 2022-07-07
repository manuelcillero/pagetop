use pagetop::prelude::*;

pub const USER_MODULE: &str = "pagetop-user::module::user";

localize!("src/locales");

mod migration;

pub struct User;

impl ModuleTrait for User {
    fn handler(&self) -> &'static str {
        USER_MODULE
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.route("/user/login", app::web::get().to(login));
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

async fn login() -> app::Result<Markup> {
    Page::new()
        .with_title(
            "Identificación del usuario"
        )
        .add_to("content", Container::new()
            .with_id("welcome")
            .with_component(form_login())
        )
        .render()
}

fn form_login() -> Form {
    Form::new()
        .with_id("user-login")
        .with_element(form::Input::textfield()
            .with_name("name")
            .with_label(l("username").as_str())
            .with_help_text(t("username_help", &args![
                "app" => SETTINGS.app.name.to_owned()
            ]).as_str())
            .with_autofocus(true)
        )
        .with_element(form::Input::password()
            .with_name("pass")
            .with_label(l("password").as_str())
            .with_help_text(l("password_help").as_str())
        )
        .with_element(form::Button::submit(l("login").as_str()))
}
