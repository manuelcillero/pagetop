use pagetop::prelude::*;

localize!("src/locales");

mod migration;

pub struct UserModule;

impl ModuleTrait for UserModule {
    fn name(&self) -> &'static str {
        "User"
    }

    fn fullname(&self) -> String {
        l("module_fullname")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.route("/user/login", app::web::get().to(login));
    }

    fn migrations(&self) -> Vec<Box<dyn db::migration::MigrationTrait>> {
        vec![
            boxed_migration!(m20220312_000001_create_table_role),
            boxed_migration!(m20220312_000002_create_table_role_permission),
            boxed_migration!(m20220312_000003_create_table_user),
            boxed_migration!(m20220312_000004_create_table_user_role),
        ]
    }
}

fn form_login() -> Form {
    Form::new()
        .with_id("user-login")
        .add(form::Input::textfield()
            .with_name("name")
            .with_label(l("username").as_str())
            .with_help_text(t("username_help", &args![
                "app" => SETTINGS.app.name.to_owned()
            ]).as_str())
            .with_autofocus(true)
        )
        .add(form::Input::password()
            .with_name("pass")
            .with_label(l("password").as_str())
            .with_help_text(l("password_help").as_str())
        )
        .add(form::Button::submit(l("login").as_str()))
}

async fn login() -> app::Result<Markup> {
    Page::new()
        .with_title(
            "Identificación del usuario"
        )
        .add_to("content", Container::new()
            .with_id("welcome")
            .add(form_login())
        )
        .render()
}
