use pagetop::prelude::*;
use pagetop_hljs::prelude::*;

struct HljsSample;

impl PackageTrait for HljsSample {
    fn dependencies(&self) -> Vec<PackageRef> {
        vec![&pagetop_hljs::HighlightJS]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            // Switch the HighlightJS theme once the body page is ready.
            action::page::AfterRenderBody::new(|page: &mut Page| page
                .context()
                .set_hljs_theme(&HljsTheme::Sunburst))
        ]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/snippet", service::web::get().to(hljs_sample));
    }
}

async fn hljs_sample(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component(Snippet::with(
            HljsLang::Rust,
            r###"
use pagetop::prelude::*;

struct HelloWorld;

impl Package for HelloWorld {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component(Html::with(html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).run()?.await
}
"###,
        ))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HljsSample).run()?.await
}
