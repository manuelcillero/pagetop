//! Handlers HTTP de `pagetop-user`.

pub(crate) mod account;
pub(crate) mod admin;
pub(crate) mod auth;

use pagetop::prelude::*;

// Página con un único mensaje.
pub(crate) async fn message_page(request: HttpRequest, title: Lc, message: Lc) -> Response {
    Page::new(request)
        .with_title(title)
        .with_child(Html::with(move |cx| html! { p { (message.using(cx)) } }))
        .render()
        .await
        .into_response()
}

// Página con un mensaje y un botón que envía un formulario a la propia URL, que incluye el
// `{uid}/{token}`: los servidores de correo abren por su cuenta los enlaces de los correos para
// analizarlos, así que el token sólo se consume al pulsar el botón.
pub(crate) async fn confirm_page(
    request: HttpRequest,
    title: Lc,
    message: Lc,
    action: Lc,
) -> Response {
    // Sin `with_action()`: se envía a la propia URL.
    let form = Form::new()
        .with_id("user-confirm-form")
        .with_method(form::Method::Post)
        .with_child(Html::with(move |cx| html! { p { (message.using(cx)) } }))
        .with_child(Button::submit(action).with_style(button::Style::Solid(Intent::Primary)));
    Page::new(request)
        .with_title(title)
        .with_child(form)
        .render()
        .await
        .into_response()
}

// Añade `Referrer-Policy: no-referrer` a la respuesta de una página con un token en la URL: si el
// tema carga recursos de otro sitio, o el usuario sigue un enlace externo, el navegador no le
// enviará la URL con el token en la cabecera `Referer`.
pub(crate) fn no_referrer(mut response: Response) -> Response {
    response.headers_mut().insert(
        web::http::header::REFERRER_POLICY,
        web::http::HeaderValue::from_static("no-referrer"),
    );
    response
}
