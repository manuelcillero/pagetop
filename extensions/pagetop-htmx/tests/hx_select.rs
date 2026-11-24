use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

fn dropdown(name: &str) -> form::select::Field {
    form::select::Field::new()
        .with_name(name)
        .with_item(form::select::Item::new("", Lc::n("-")))
}

async fn render(mut field: form::select::Field) -> String {
    field.render(&mut Context::default()).await.into_string()
}

// **< cascade() >**********************************************************************************

// Every dropdown but the last asks for the options with its own value and the names that follow it,
// and expects them out of band, without touching the page around it.
#[pagetop::test]
async fn every_dropdown_but_the_last_reloads_the_following_ones() {
    let fields = hx_select::cascade(
        [dropdown("country"), dropdown("province"), dropdown("town")],
        "/regions?for=form",
    );
    let mut html = Vec::new();
    for field in fields {
        html.push(render(field).await);
    }

    for (i, chain) in [(0, "country,province,town"), (1, "province,town")] {
        let field = &html[i];
        assert!(field.contains(r#"hx-get="/regions?for=form""#), "{field}");
        assert!(field.contains(r#"hx-trigger="change""#), "{field}");
        assert!(field.contains(r#"hx-include="find select""#), "{field}");
        assert!(field.contains(r#"hx-swap="none""#), "{field}");
        assert!(
            field.contains(&format!(
                r#"hx-vals="{{&quot;hx_cascade&quot;:&quot;{chain}&quot;}}""#
            )),
            "{field}"
        );
        // Inside a list filter, a change must not apply it nor change the address.
        assert!(field.contains(r#"hx-push-url="false""#), "{field}");
        assert!(field.contains(r#"hx-sync="this:replace""#), "{field}");
    }
    assert!(html[0].contains(r##"hx-indicator="#edit-province-select""##));
    assert!(html[1].contains(r##"hx-indicator="#edit-town-select""##));
    assert!(!html[2].contains("hx-"), "{}", html[2]);
}

// A name may carry characters that mean something in a CSS selector; the indicator escapes them so
// that it still points to the next `<select>`.
#[pagetop::test]
async fn the_indicator_escapes_the_name_of_the_next_dropdown() {
    let fields = hx_select::cascade([dropdown("place"), dropdown("site.room[1]")], "/places");
    let html = render(fields.into_iter().next().unwrap()).await;
    assert!(
        html.contains(r##"hx-indicator="#edit-site\.room\[1\]-select""##),
        "{html}"
    );
}

// A chain that cannot work fails while developing, instead of losing the options silently.
#[pagetop::test]
#[should_panic(expected = "dropdown 2 has no name")]
async fn a_dropdown_without_a_name_breaks_the_cascade() {
    hx_select::cascade(
        [dropdown("country"), form::select::Field::new()],
        "/regions",
    );
}

#[pagetop::test]
#[should_panic(expected = "dropdown `province` has its own id")]
async fn a_dropdown_with_its_own_id_breaks_the_cascade() {
    hx_select::cascade(
        [dropdown("country"), dropdown("province").with_id("where")],
        "/regions",
    );
}

#[pagetop::test]
#[should_panic(expected = "dropdown name `country` is repeated")]
async fn a_repeated_name_breaks_the_cascade() {
    hx_select::cascade([dropdown("country"), dropdown("country")], "/regions");
}

#[pagetop::test]
#[should_panic(expected = "dropdown `a,b` has a comma in its name")]
async fn a_comma_in_a_name_breaks_the_cascade() {
    hx_select::cascade([dropdown("a,b"), dropdown("province")], "/regions");
}

// **< Cascade >************************************************************************************

async fn options(web::Query(cascade): web::Query<hx_select::Cascade>) -> String {
    format!(
        "{:?} {:?} {:?} {:?} {:?}",
        cascade.changed(),
        cascade.value(),
        cascade.next(),
        cascade.rest().collect::<Vec<_>>(),
        cascade.param("for"),
    )
}

struct Regions;

#[async_trait]
impl Extension for Regions {
    fn configure_router(&self, routes: Routes) -> Routes {
        routes.public_route("/regions", web::get(options))
    }
}

async fn ask(uri: &str) -> String {
    let app = web::test::init_router(Application::prepare(&Regions).await.test());
    let request = web::test::TestRequest::get().uri(uri).to_request();
    web::test::read_body_text(web::test::send_request(&app, request).await).await
}

// The request of a dropdown carries what changed, its value, what follows and the route's own
// parameters.
#[pagetop::test]
async fn the_request_tells_what_changed_and_what_follows() {
    assert_eq!(
        ask("/regions?for=form&country=es&hx_cascade=country,province,town").await,
        r#"Some("country") Some("es") Some("province") ["town"] Some("form")"#
    );
    assert_eq!(
        ask("/regions?province=&hx_cascade=province,town").await,
        r#"Some("province") Some("") Some("town") [] None"#
    );
    // Without the chain (not a cascade request), nothing changed.
    assert_eq!(ask("/regions").await, "None None None [] None");
}

// **< Options >************************************************************************************

// Each dropdown to update gets its options out of band, by the id of its `<select>`, carried in a
// multiple-choice dropdown so that the browser keeps the chosen option as sent.
#[pagetop::test]
async fn the_response_replaces_the_options_of_each_dropdown() {
    let response = hx_select::Options::new()
        .with_options(
            "province",
            [
                form::select::Item::new("", Lc::n("-")),
                form::select::Item::new("gr", Lc::n("Granada")).with_selected(true),
                form::select::Item::new("al", Lc::n("Almería")).with_disabled(true),
            ],
        )
        .with_options(
            "town",
            [form::select::Item::new(
                "",
                Lc::n("Choose a province first"),
            )],
        )
        .response(&Context::default());
    let body = web::test::read_body_text(response.into_response()).await;

    assert!(
        body.contains(
            r#"<select id="edit-province-select" multiple hx-swap-oob="innerHTML"><option value="">-</option><option value="gr" selected>Granada</option><option value="al" disabled>Almería</option></select>"#
        ),
        "{body}"
    );
    assert!(
        body.contains(
            r#"<select id="edit-town-select" multiple hx-swap-oob="innerHTML"><option value="">Choose a province first</option></select>"#
        ),
        "{body}"
    );
}
