//! Mensajes para la página siguiente (mensajes *flash*).
//!
//! Viajan en una *cookie* de vida corta, sin guardar nada en el servidor ni necesitar sesión: la
//! respuesta que los añade la crea, la página siguiente los muestra y su respuesta la borra.

use crate::core::component::{Context, MessageLevel, StatusMessage};
use crate::locale::Lc;
use crate::response::ErrorPage;
use crate::trace;
use crate::web::{Request, Response, http, push_percent_encoded};

use axum::middleware::Next;

const COOKIE_NAME: &str = "pagetop_flash";

// Atributos de la *cookie*. Basta con un minuto: es el tiempo de seguir una redirección.
const COOKIE_ATTRS: &str = "; Path=/; HttpOnly; SameSite=Lax; Max-Age=60";
const EXPIRED_COOKIE: &str = "pagetop_flash=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0";

// Los navegadores admiten al menos 4096 bytes por *cookie*, contando nombre y atributos.
const MAX_VALUE_LEN: usize = 3800;

// Separa los mensajes en el valor de la *cookie*: no aparece en el texto codificado.
const SEPARATOR: char = '|';

// Mensajes añadidos a una respuesta con `with_message()`, pendientes de guardar en la *cookie*.
#[derive(Clone, Default)]
struct Pending(Vec<(MessageLevel, String)>);

/// Añade a una respuesta mensajes para mostrar en la página siguiente.
///
/// Es la forma de avisar del resultado de una acción tras redirigir a otra página (cuenta creada,
/// cambios guardados...), como los mensajes *flash* de otros *frameworks*. Los mensajes viajan en
/// una *cookie* de vida corta, ya traducidos al idioma de `cx`, y la página siguiente los añade a
/// su [`Context`] como si se hubieran acumulado con [`Context::push_message()`]. Los muestra el
/// componente [`Messages`] que encabeza la región [`CoreRegions::Content`], o uno propio de la
/// página.
///
/// Si la respuesta siguiente es otra redirección, los mensajes esperan a la página posterior.
///
/// La *cookie* (`pagetop_flash`) sólo se crea cuando una respuesta usa este método; una aplicación
/// que no lo usa no envía ninguna. Es una *cookie* técnica: surge de una acción del usuario, dura
/// como mucho un minuto, se borra en cuanto se muestran los mensajes y no contiene datos que lo
/// identifiquen. Como *cookie* estrictamente necesaria para el servicio que el usuario ha pedido,
/// suele quedar exenta de consentimiento, aunque conviene declararla en la política de *cookies* de
/// la aplicación.
///
/// El texto se muestra siempre como texto plano: el marcado que tenga una traducción aparecería
/// tal cual. Cada mensaje debe ser breve; los que no quepan en la *cookie* (unos 3,8 KB entre
/// todos) se descartan con un aviso en las trazas.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// async fn save(request: HttpRequest) -> Response {
///     let cx = Context::new(request);
///     // ... guarda los cambios ...
///     Redirect::see_other(cx.route("/settings"))
///         .with_message(&cx, MessageLevel::Info, Lc::n("Settings saved."))
/// }
/// ```
///
/// [`Context::push_message()`]: crate::core::component::Contextual::push_message
/// [`CoreRegions::Content`]: crate::core::theme::CoreRegions::Content
/// [`Messages`]: crate::base::component::Messages
pub trait ResponseExt {
    /// Añade un mensaje para la página siguiente, traducido con el idioma del contexto.
    fn with_message(self, cx: &Context, level: MessageLevel, text: Lc) -> Response;
}

impl ResponseExt for Response {
    fn with_message(mut self, cx: &Context, level: MessageLevel, text: Lc) -> Response {
        if let Some(text) = text.lookup(cx).filter(|text| !text.is_empty()) {
            self.extensions_mut()
                .get_or_insert_default::<Pending>()
                .0
                .push((level, text));
        }
        self
    }
}

// Lee los mensajes de la *cookie* de la petición, para el `Context` de la página. Con un valor
// manipulado no hace más que mostrar texto plano, que `Lc::n()` escapa al renderizarse.
pub(crate) fn read_flash_messages(headers: &http::HeaderMap) -> Vec<StatusMessage> {
    let Some(value) = cookie_value(headers) else {
        return Vec::new();
    };
    value
        .split(SEPARATOR)
        .filter_map(|entry| {
            let level = match entry.as_bytes().first()? {
                b'i' => MessageLevel::Info,
                b'w' => MessageLevel::Warning,
                b'e' => MessageLevel::Error,
                _ => return None,
            };
            let text = percent_decode(&entry[1..])?;
            Some(StatusMessage::new(level, Lc::n(text)))
        })
        .collect()
}

// Guarda en la *cookie* los mensajes que añade la respuesta y la borra cuando la página siguiente
// ya los ha mostrado. Sin *cookie* en la petición ni mensajes nuevos, no hace nada más.
pub(crate) async fn keep_flash_messages(request: Request, next: Next) -> Response {
    let previous = cookie_value(request.headers()).map(str::to_owned);
    let mut response = next.run(request).await;
    let pending = response.extensions_mut().remove::<Pending>();
    if previous.is_none() && pending.is_none() {
        return response;
    }
    // Una página (también de error) ya ha mostrado los mensajes anteriores; una redirección, o
    // algo que no sea una página (un archivo, JSON...), los conserva para la siguiente.
    let shown = !is_redirect(&response) && is_page(&response);
    let mut value = match previous {
        Some(previous) if !shown => {
            if pending.is_none() {
                return response;
            }
            previous
        }
        _ => String::new(),
    };
    for (level, text) in pending.map(|pending| pending.0).unwrap_or_default() {
        push_message(&mut value, level, &text);
    }
    let cookie = if value.is_empty() {
        EXPIRED_COOKIE.to_owned()
    } else {
        crate::util::join!(COOKIE_NAME, "=", value, COOKIE_ATTRS)
    };
    if let Ok(cookie) = http::HeaderValue::from_str(&cookie) {
        response
            .headers_mut()
            .append(http::header::SET_COOKIE, cookie);
    }
    response
}

fn is_redirect(response: &Response) -> bool {
    response.status().is_redirection()
        || response.headers().contains_key("hx-redirect")
        || response.headers().contains_key("hx-location")
}

fn is_page(response: &Response) -> bool {
    response.extensions().get::<ErrorPage>().is_some()
        || response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|value| value.as_bytes().get(..9))
            .is_some_and(|essence| essence.eq_ignore_ascii_case(b"text/html"))
}

// Añade un mensaje al valor de la *cookie*, si cabe.
fn push_message(value: &mut String, level: MessageLevel, text: &str) {
    let mut entry = String::with_capacity(text.len() + 1);
    entry.push(match level {
        MessageLevel::Info => 'i',
        MessageLevel::Warning => 'w',
        MessageLevel::Error => 'e',
    });
    push_percent_encoded(&mut entry, text);
    let separator = usize::from(!value.is_empty());
    if value.len() + separator + entry.len() > MAX_VALUE_LEN {
        trace::warn!("Flash message discarded, it does not fit in the cookie: {text:?}");
        return;
    }
    if separator == 1 {
        value.push(SEPARATOR);
    }
    value.push_str(&entry);
}

// Valor de la *cookie* de mensajes, si la petición la trae. Con HTTP/2 el navegador puede repartir
// las *cookies* en varias cabeceras `cookie`.
fn cookie_value(headers: &http::HeaderMap) -> Option<&str> {
    headers
        .get_all(http::header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|cookies| cookies.split(';'))
        .filter_map(|part| part.trim().strip_prefix(COOKIE_NAME)?.strip_prefix('='))
        .find(|value| !value.is_empty())
}

// Decodifica `%XX`; devuelve `None` si la secuencia no es válida o el resultado no es UTF-8.
fn percent_decode(encoded: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(encoded.len());
    let mut input = encoded.bytes();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let hex = [input.next()?, input.next()?];
            bytes.push(u8::from_str_radix(std::str::from_utf8(&hex).ok()?, 16).ok()?);
        } else {
            bytes.push(byte);
        }
    }
    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(cookie: &str) -> http::HeaderMap {
        let mut headers = http::HeaderMap::new();
        headers.insert(http::header::COOKIE, cookie.parse().unwrap());
        headers
    }

    fn texts(cookie: &str) -> Vec<(MessageLevel, String)> {
        read_flash_messages(&headers(cookie))
            .iter()
            .map(|m| (*m.level(), m.text().get().unwrap()))
            .collect()
    }

    #[test]
    fn messages_round_trip_through_the_cookie_value() {
        let mut value = String::new();
        push_message(
            &mut value,
            MessageLevel::Info,
            "Cuenta creada | ¡bienvenida!",
        );
        push_message(&mut value, MessageLevel::Error, "a;b=c, \"d\"");
        assert!(
            value
                .bytes()
                .all(|b| b.is_ascii_graphic() && !b"; ,\"\\".contains(&b))
        );
        let cookie = format!("sid=abc; {COOKIE_NAME}={value}");
        assert_eq!(
            texts(&cookie),
            [
                (
                    MessageLevel::Info,
                    "Cuenta creada | ¡bienvenida!".to_owned()
                ),
                (MessageLevel::Error, "a;b=c, \"d\"".to_owned()),
            ]
        );
    }

    #[test]
    fn invalid_entries_are_skipped() {
        let cookie = format!("{COOKIE_NAME}=iok|xbad|e%E2%28|w%4|i%41b");
        assert_eq!(
            texts(&cookie),
            [
                (MessageLevel::Info, "ok".to_owned()),
                (MessageLevel::Info, "Ab".to_owned()),
            ]
        );
        assert!(texts("other=1").is_empty());
    }

    #[test]
    fn messages_that_do_not_fit_are_discarded() {
        let mut value = String::new();
        let long = "x".repeat(MAX_VALUE_LEN - 10);
        push_message(&mut value, MessageLevel::Info, &long);
        push_message(&mut value, MessageLevel::Info, "does not fit");
        assert_eq!(value.len(), MAX_VALUE_LEN - 9);
    }
}
