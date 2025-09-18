use crate::prelude::*;

/// Página de bienvenida predeterminada de PageTop.
///
/// Esta extensión se instala por defecto y muestra una página en la ruta raíz (`/`) cuando no se ha
/// configurado ninguna página de inicio personalizada. Permite confirmar que el servidor está
/// funcionando correctamente.
pub struct Welcome;

impl Extension for Welcome {
    fn name(&self) -> L10n {
        L10n::l("welcome_extension_name")
    }

    fn description(&self) -> L10n {
        L10n::l("welcome_extension_description")
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(homepage));
    }
}

async fn homepage(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    let app = &global::SETTINGS.app.name;

    Page::new(request)
        .with_theme("basic")
        .with_layout("intro")
        .with_title(L10n::l("welcome_title"))
        .with_description(L10n::l("welcome_intro").with_arg("app", app))
        .with_param("intro_button_text", L10n::l("welcome_powered"))
        .with_param("intro_button_link", "https://pagetop.cillero.es".to_string())
        .with_assets(AssetsOp::AddJavaScript(JavaScript::on_load_async("welcome-js", |cx|
            util::indoc!(r#"
            try {
                const resp = await fetch("https://crates.io/api/v1/crates/pagetop");
                const data = await resp.json();
                const date = new Date(data.versions[0].created_at);
                const formatted = date.toLocaleDateString("LANGID", { year: "numeric", month: "2-digit", day: "2-digit" });
                document.getElementById("welcome-release").src = `https://img.shields.io/badge/Release%20date-${encodeURIComponent(formatted)}-blue?label=LABEL&style=for-the-badge`;
                document.getElementById("welcome-badges").style.display = "block";
            } catch (e) {
                console.error("Failed to fetch release date from crates.io:", e);
            }
            "#)
            .replace("LANGID", cx.langid().to_string().as_str())
            .replace("LABEL", L10n::l("welcome_release_label").using(cx).as_str())
            .to_string(),
        )))
        .add_component(Html::with(|cx| html! {
            p { (L10n::l("welcome_text1").using(cx)) }
            div id="welcome-badges" style="display: none; margin-bottom: 1.1rem;" {
                img
                    src="https://img.shields.io/crates/v/pagetop.svg?label=PageTop&style=for-the-badge"
                    alt=[L10n::l("welcome_pagetop_label").lookup(cx)] {} (" ")
                img
                    id="welcome-release"
                    alt=[L10n::l("welcome_release_label").lookup(cx)] {} (" ")
                img
                    src=(format!(
                        "https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label={}&style=for-the-badge",
                        L10n::l("welcome_license_label").lookup(cx).unwrap_or_default()
                    ))
                    alt=[L10n::l("welcome_license_label").lookup(cx)] {}
            }
            p { (L10n::l("welcome_text2").using(cx)) }
        }))
        .add_component(
            Block::new()
                .with_title(L10n::l("welcome_notice_title"))
                .add_component(Html::with(move |cx| html! {
                    p { (L10n::l("welcome_notice_1").using(cx)) }
                    p { (L10n::l("welcome_notice_2").using(cx)) }
                    p { (L10n::l("welcome_notice_3").using(cx)) }
                    p { (L10n::l("welcome_notice_4").with_arg("app", app).using(cx)) }
                })),
        )
        .render()
}
