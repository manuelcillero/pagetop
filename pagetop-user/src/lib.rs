use pagetop::prelude::*;

pub_const_handler!(MODULE_USER);

localize!("src/locales");

mod migration;

pub struct User;

impl ModuleTrait for User {
    fn handler(&self) -> Handler {
        MODULE_USER
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

async fn login() -> ResultPage<Markup, FatalError> {
    Page::new()
        .with_title("Identificación del usuario")
        .add_to(
            "region-content",
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
                .with_label(l("username").as_str())
                .with_help_text(
                    t(
                        "username_help",
                        &args![
                            "app" => config::get("app.name").to_owned()
                        ],
                    )
                    .as_str(),
                )
                .with_autofocus(true),
        )
        .with_element(
            form_element::Input::password()
                .with_name("pass")
                .with_label(l("password").as_str())
                .with_help_text(l("password_help").as_str()),
        )
        .with_element(form_element::Button::submit(l("login").as_str()))
}
