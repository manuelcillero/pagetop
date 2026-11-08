// Verifies that the links to a route declared with `Endpoint` are hidden as well in the menus and
// buttons that the theme renders on its own.

use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

struct Granted;

impl Permission for Granted {
    fn key(&self) -> CowStr {
        "test:granted".into()
    }
}

struct Denied;

impl Permission for Denied {
    fn key(&self) -> CowStr {
        "test:denied".into()
    }
}

fn grant(_: &HttpRequest, perm: PermissionRef, granted: &mut bool) {
    if perm.key() == "test:granted" {
        *granted = true;
    }
}

#[derive(Clone, Copy)]
enum TestRoute {
    Allowed,
    Forbidden,
}

impl Endpoint for TestRoute {
    fn path(&self) -> CowStr {
        match self {
            Self::Allowed => "/allowed".into(),
            Self::Forbidden => "/forbidden".into(),
        }
    }

    fn access(&self) -> Access {
        match self {
            Self::Allowed => Access::Permission(&Granted),
            Self::Forbidden => Access::Permission(&Denied),
        }
    }
}

fn menu() -> Nav {
    Nav::new()
        .with_item(bs::nav::Item::link(
            Lc::n("nav-allowed"),
            TestRoute::Allowed,
        ))
        .with_item(bs::nav::Item::link(
            Lc::n("nav-forbidden"),
            TestRoute::Forbidden,
        ))
        .with_item(bs::nav::Item::dropdown(
            Dropdown::new()
                .with_title(Lc::n("dropdown-empty"))
                .with_item(dropdown::Item::link(Lc::n("x"), TestRoute::Forbidden)),
        ))
        .with_item(bs::nav::Item::dropdown(
            Dropdown::new()
                .with_title(Lc::n("dropdown-partial"))
                .with_item(dropdown::Item::header(Lc::n("header-empty")))
                .with_item(dropdown::Item::link(Lc::n("x"), TestRoute::Forbidden))
                .with_item(dropdown::Item::divider())
                .with_item(dropdown::Item::link(
                    Lc::n("in-allowed"),
                    TestRoute::Allowed,
                ))
                .with_item(dropdown::Item::link(
                    Lc::n("in-forbidden"),
                    TestRoute::Forbidden,
                )),
        ))
}

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![action::auth::CheckPermission::new(grant)]
    }
}

async fn render(mut component: impl Component + 'static) -> String {
    let _app = Application::prepare(&Ext).await.test();
    let mut cx = Context::new(web::test::TestRequest::get().to_http_request())
        .with_theme(&pagetop_bootsier::Bootsier);
    component.render(&mut cx).await.into_string()
}

fn assert_only_allowed_links(html: &str) {
    for shown in ["nav-allowed", "dropdown-partial", "in-allowed"] {
        assert!(html.contains(shown), "`{shown}` is missing in {html}");
    }
    for hidden in [
        "nav-forbidden",
        "dropdown-empty",
        "in-forbidden",
        "header-empty",
    ] {
        assert!(!html.contains(hidden), "`{hidden}` is shown in {html}");
    }
    assert!(
        !html.contains("dropdown-divider"),
        "a divider is shown in {html}"
    );
}

#[pagetop::test]
async fn nav_only_shows_the_allowed_links() {
    assert_only_allowed_links(&render(menu()).await);
}

#[pagetop::test]
async fn navbar_only_shows_the_allowed_links() {
    let navbar = Navbar::simple().with_item(bs::navbar::Item::nav(menu()));
    let html = render(navbar).await;
    assert!(
        html.contains("navbar-nav"),
        "not rendered by the theme: {html}"
    );
    assert_only_allowed_links(&html);
}

#[pagetop::test]
async fn buttons_only_show_the_allowed_links() {
    let html = render(Button::anchor(Lc::n("button-allowed"), TestRoute::Allowed)).await;
    assert!(html.contains("button-allowed"), "{html}");
    let html = render(Button::anchor(
        Lc::n("button-forbidden"),
        TestRoute::Forbidden,
    ))
    .await;
    assert!(html.is_empty(), "{html}");
}
