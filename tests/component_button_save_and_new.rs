use pagetop::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct ItemInput {
    name: String,
    #[serde(flatten)]
    save_and_new: button::SaveAndNew,
}

async fn create(web::Form(input): web::Form<ItemInput>) -> String {
    format!("{} {}", input.name, input.save_and_new.is_requested())
}

struct Items;

#[async_trait]
impl Extension for Items {
    fn configure_router(&self, routes: Routes) -> Routes {
        routes.public_route("/items/new", web::post(create))
    }
}

async fn send(body: &str) -> String {
    let app = web::test::init_router(Application::prepare(&Items).await.test());
    let request = web::test::TestRequest::post()
        .uri("/items/new")
        .form(body)
        .to_request();
    web::test::read_body_text(web::test::send_request(&app, request).await).await
}

// The button sends the field that `SaveAndNew` reads; the plain save button sends nothing.
#[pagetop::test]
async fn the_button_is_recognised_through_a_flattened_form() {
    let html = Button::save_and_new()
        .render(&mut Context::default())
        .await
        .into_string();
    assert!(html.contains(r#"type="submit""#), "{html}");
    assert!(html.contains(r#"name="save_and_new""#), "{html}");
    assert!(html.contains(r#"value="1""#), "{html}");
    assert!(html.contains("Save & New"), "{html}");

    assert_eq!(send("name=a&save_and_new=1").await, "a true");
    assert_eq!(send("name=a").await, "a false");
}

#[pagetop::test]
async fn the_notice_is_translated() {
    let es = Locale::resolve("es-ES");
    assert_eq!(
        button::SaveAndNew::notice().lookup(&es).as_deref(),
        Some("Guardado. Puedes crear otro registro.")
    );
}
