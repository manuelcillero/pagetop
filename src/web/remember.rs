use std::borrow::Cow;

use crate::web::http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header};
use crate::web::{IntoResponse, MethodRouter, Request, Response};

use axum::extract::OriginalUri;
use axum::middleware::{self, Next};

// Prefijo del nombre de las *cookies*; cada listado añade el *hash* de su ruta.
const COOKIE_PREFIX: &str = "pagetop_list_";

// Longitud del nombre de las *cookies*: el prefijo y 16 cifras hexadecimales.
const NAME_LEN: usize = COOKIE_PREFIX.len() + 16;

const SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");

// Atributos comunes. Sin `Max-Age` es una *cookie* de sesión.
const COOKIE_ATTRS: &str = "; HttpOnly; SameSite=Lax";

// Estado más largo que esto no se recuerda: un listado no necesita tanto y la *cookie* viaja en
// cada petición a su ruta y a las que cuelgan de ella.
const MAX_STATE_LEN: usize = 1024;

// Parámetro que olvida el estado guardado. Lo añade el enlace "Limpiar" de `ListFilter`.
const RESET_PARAM: &str = "reset";

/// Hace que un listado recuerde su último estado (filtros, orden y página) mientras dure la sesión
/// del navegador.
///
/// Envuelve el `MethodRouter` de la ruta de un listado. Cada vez que la página se pide con
/// parámetros en la *query string* (al filtrar, ordenar o paginar, también desde HTMX) y responde
/// con éxito, los guarda. Cuando se vuelve a pedir **sin parámetros**, como hacen los enlaces de
/// vuelta, las redirecciones tras guardar un formulario o el menú, redirige (`303`) a la misma
/// ruta con el estado guardado. Así no hay que transportar el estado del listado por cada pantalla
/// intermedia, como haría [`Waypoint`].
///
/// Si todos los parámetros llegan vacíos (`?q=&status=`, un filtro enviado sin rellenar), equivale
/// a no filtrar: no se guarda nada y se olvida el estado anterior.
///
/// Con el parámetro `reset` (`?reset`), que añade el enlace "Limpiar" de [`ListFilter`], olvida el
/// estado y redirige a la ruta sin parámetros. El parámetro `lang` no se guarda nunca: se conserva
/// el de la petición en curso. Las peticiones de HTMX se redirigen igual: HTMX sigue la redirección
/// y, si actualiza el historial del navegador, guarda la URL final, con el estado.
///
/// Si el estado guardado deja de dar una página válida (por ejemplo, un error `400` porque un
/// parámetro ha cambiado en una versión posterior de la aplicación), se olvida.
///
/// El estado guardado sólo cambia con las peticiones que salen de la propia aplicación o que se
/// escriben a mano (según la cabecera `Sec-Fetch-Site` que envían los navegadores): un enlace desde
/// otro sitio no puede fijar ni borrar los filtros que recuerda quien lo sigue.
///
/// Sólo actúa en las peticiones `GET`; las demás llegan al *handler* sin cambios.
///
/// # Qué necesita el listado
///
/// Basta con montar su ruta con esta función: los enlaces de vuelta, los botones de cancelar y las
/// redirecciones tras guardar no cambian, siguen apuntando a la ruta del listado sin parámetros.
/// Para que funcione:
///
/// - El estado del listado debe ir en la *query string*. Así lo hacen [`ListFilter`] (un
///   formulario `GET`), [`Pager`] y los enlaces de orden de [`Table`], también con HTMX. Un estado
///   que no esté en la URL (filtros enviados por `POST`, sólo en JavaScript) no se recuerda.
/// - Un enlace a la ruta con parámetros propios es un estado nuevo y no restaura el guardado.
/// - Cada ruta tiene su propio estado: `/projects/5/tasks` y `/projects/6/tasks` se recuerdan por
///   separado.
/// - Un formulario de filtros propio, en lugar de [`ListFilter`], debe enlazar su "Limpiar" a la
///   ruta con `?reset` (con [`RoutePath::with_flag("reset")`][with_flag]); si no, volvería a
///   restaurar el filtro que se quería quitar.
///
/// # *Cookies*
///
/// El estado se guarda en el navegador, en una *cookie* por listado (`pagetop_list_` seguido de un
/// código derivado de la ruta, limitada a esa ruta con `Path`), sin nada en el servidor. Es una
/// *cookie* técnica de sesión: sólo se crea cuando el usuario filtra, ordena o pagina, sólo
/// contiene los parámetros del listado y el navegador la borra al cerrarse (salvo los que restauran
/// la sesión anterior al abrirse de nuevo, que la conservan). Como *cookie* de personalización de
/// la interfaz que el propio usuario solicita, suele quedar exenta de consentimiento, aunque
/// conviene declararla en la política de *cookies* de la aplicación. Las ventanas y pestañas del
/// navegador comparten el mismo estado. No se recuerda un estado de más de 1024 caracteres ni uno
/// con caracteres que una *cookie* no admite (los navegadores ya los codifican en la URL).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # async fn list_items() -> &'static str { "" }
/// fn configure_router(routes: web::Routes) -> web::Routes {
///     routes.public_route("/items", web::remember_query(web::get(list_items)))
/// }
/// ```
///
/// [`ListFilter`]: crate::base::component::ListFilter
/// [`Pager`]: crate::base::component::Pager
/// [`Table`]: crate::base::component::Table
/// [`Waypoint`]: crate::response::Waypoint
/// [with_flag]: crate::html::RoutePath::with_flag
pub fn remember_query(method_router: MethodRouter) -> MethodRouter {
    method_router.layer(middleware::from_fn(remember))
}

async fn remember(request: Request, next: Next) -> Response {
    if request.method() != Method::GET {
        return next.run(request).await;
    }
    // Copia de la URI (sólo una cuenta de referencias), para que la ruta y la *query* sigan
    // disponibles tras pasar la petición al *handler*. Dentro de un `Router` anidado la URI llega
    // sin el prefijo; la *cookie* necesita la ruta real.
    let uri = request
        .extensions()
        .get::<OriginalUri>()
        .map_or_else(|| request.uri().clone(), |uri| uri.0.clone());
    let raw_query = uri.query().unwrap_or_default();
    let path = uri.path();
    // Un `;` en la ruta rompería el atributo `Path` de la *cookie*. Y una ruta que empiece por `//`
    // o `/\` (posible si el listado se monta en una ruta comodín) haría de la redirección una URL
    // de otro sitio para el navegador.
    if path.contains(';') || path.starts_with("//") || path.starts_with("/\\") {
        return next.run(request).await;
    }
    let name = cookie_name(path);
    // El estado guardado se vuelve a separar: una *cookie* manipulada podría traer `lang`, `reset`
    // o nada útil (`&`), y redirigir con ella repetiría la redirección sin fin. Una con caracteres
    // no válidos queda sin estado, como la que no trae nada útil, y se borra igual.
    let saved = cookie_value(request.headers(), &name).map(|v| match str::from_utf8(v) {
        Ok(v) if is_valid_state(v) => Params::split(v).state.into_owned(),
        _ => String::new(),
    });
    // Sin parámetros ni estado guardado no hay nada que hacer.
    if raw_query.is_empty() && saved.is_none() {
        return next.run(request).await;
    }
    let query = Params::split(raw_query);
    let writable = is_own_request(request.headers());

    if query.reset {
        let mut response = see_other(query.location(path, None));
        if writable {
            set_cookie(&mut response, &name, "", path, true);
        }
        return response;
    }
    if query.state.is_empty() {
        match saved {
            Some(saved) if !is_blank(&saved) => {
                return see_other(query.location(path, Some(&saved)));
            }
            Some(_) => {
                let mut response = next.run(request).await;
                if writable {
                    set_cookie(&mut response, &name, "", path, true);
                }
                return response;
            }
            None => return next.run(request).await,
        }
    }

    let state = &query.state;
    let unchanged = saved.as_deref() == Some(state.as_ref());
    let mut response = next.run(request).await;
    if !writable {
        return response;
    }
    if !response.status().is_success() {
        // Un estado guardado que ya no da una página válida (parámetros de una versión anterior de
        // la aplicación, un *handler* que redirige...) se olvida: si no, cada vuelta al listado
        // repetiría el error, o la redirección, hasta cerrar el navegador.
        if unchanged {
            set_cookie(&mut response, &name, "", path, true);
        }
        return response;
    }
    if is_blank(state) {
        if saved.is_some() {
            set_cookie(&mut response, &name, "", path, true);
        }
    } else if !unchanged && state.len() <= MAX_STATE_LEN && is_valid_state(state) {
        set_cookie(&mut response, &name, state, path, false);
    }
    response
}

// Redirección `303` a `location`, sin pasar por `Redirect`, que copia la cadena. La ruta sale de la
// URI de la petición y el estado está validado, así que siempre es una cabecera válida; si no lo
// fuera, mejor un `500`, como haría `Redirect`, que una redirección sin destino.
fn see_other(location: String) -> Response {
    match HeaderValue::try_from(location) {
        Ok(location) => (StatusCode::SEE_OTHER, [(header::LOCATION, location)]).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// Sólo cambian el estado guardado las peticiones de la propia aplicación o escritas a mano
// (`Sec-Fetch-Site` igual a `same-origin` o `none`): un enlace o una redirección desde otro sitio,
// o desde otro subdominio, no puede fijar ni borrar los filtros de quien lo sigue. Sin la cabecera
// (navegadores antiguos, otros clientes) se admite, con el mismo criterio que la protección CSRF.
fn is_own_request(headers: &HeaderMap) -> bool {
    headers
        .get(SEC_FETCH_SITE)
        .is_none_or(|site| matches!(site.as_bytes(), b"same-origin" | b"none"))
}

// Un estado con todos los valores vacíos (`q=&status=`, un filtro enviado sin rellenar) equivale a
// no filtrar: no merece una *cookie*.
fn is_blank(state: &str) -> bool {
    state.split('&').all(|pair| {
        pair.split_once('=')
            .is_none_or(|(_, value)| value.is_empty())
    })
}

// Parámetros de la petición separados en estado del listado, `lang` y la marca `reset`. Sin `lang`
// ni `reset`, lo habitual, el estado es la propia *query* y no se copia.
struct Params<'a> {
    state: Cow<'a, str>,
    lang: String,
    reset: bool,
}

impl<'a> Params<'a> {
    fn split(query: &'a str) -> Self {
        let mut pairs = query.split('&');
        if pairs.all(|pair| !pair.is_empty() && !matches!(key_of(pair), "lang" | RESET_PARAM)) {
            return Params {
                state: Cow::Borrowed(query),
                lang: String::new(),
                reset: false,
            };
        }
        let mut state = String::with_capacity(query.len());
        let mut split = Params {
            state: Cow::Borrowed(""),
            lang: String::new(),
            reset: false,
        };
        for pair in query.split('&').filter(|pair| !pair.is_empty()) {
            let target = match key_of(pair) {
                "lang" => &mut split.lang,
                RESET_PARAM => {
                    split.reset = true;
                    continue;
                }
                _ => &mut state,
            };
            if !target.is_empty() {
                target.push('&');
            }
            target.push_str(pair);
        }
        split.state = Cow::Owned(state);
        split
    }

    // Ruta del listado con el estado indicado y el `lang` de la petición en curso.
    fn location(&self, path: &str, state: Option<&str>) -> String {
        let state = state.unwrap_or_default();
        let mut location = String::with_capacity(path.len() + state.len() + self.lang.len() + 2);
        location.push_str(path);
        for part in [state, self.lang.as_str()] {
            if !part.is_empty() {
                location.push(if location.len() == path.len() {
                    '?'
                } else {
                    '&'
                });
                location.push_str(part);
            }
        }
        location
    }
}

fn key_of(pair: &str) -> &str {
    pair.split_once('=').map_or(pair, |(key, _)| key)
}

// Nombre de la *cookie* de un listado, en la pila: el prefijo y el *hash* FNV-1a de 64 bits de la
// ruta en hexadecimal. Es estable entre procesos e instancias, a diferencia de `RandomState`, y
// basta para distinguir las rutas de una aplicación.
fn cookie_name(path: &str) -> [u8; NAME_LEN] {
    let hash = path.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    });
    let mut name = [0; NAME_LEN];
    name[..COOKIE_PREFIX.len()].copy_from_slice(COOKIE_PREFIX.as_bytes());
    for (i, digit) in name[COOKIE_PREFIX.len()..].iter_mut().enumerate() {
        *digit = b"0123456789abcdef"[(hash >> (60 - 4 * i) & 0xf) as usize];
    }
    name
}

// Sólo caracteres admitidos en el valor de una *cookie* (RFC 6265: ASCII visible salvo `"`, `,`,
// `;` y `\`). Una *query string* que tenga otros no se recuerda; uno guardado que los tenga (una
// *cookie* manipulada) se ignora. Así el valor también es seguro en la cabecera `Location`.
fn is_valid_state(state: &str) -> bool {
    !state.is_empty()
        && state
            .bytes()
            .all(|b| b.is_ascii_graphic() && !matches!(b, b'"' | b',' | b';' | b'\\'))
}

// Valor de la *cookie* `name`, si la petición la trae, sin validar toda la cabecera. Con HTTP/2 el
// navegador puede repartir las *cookies* en varias cabeceras `cookie`.
fn cookie_value<'a>(headers: &'a HeaderMap, name: &[u8]) -> Option<&'a [u8]> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .flat_map(|value| value.as_bytes().split(|&b| b == b';'))
        .filter_map(|part| part.trim_ascii().strip_prefix(name)?.strip_prefix(b"="))
        .find(|value| !value.is_empty())
}

fn set_cookie(response: &mut Response, name: &[u8], value: &str, path: &str, expired: bool) {
    let mut cookie = String::with_capacity(NAME_LEN + value.len() + path.len() + 40);
    // El nombre es ASCII: el prefijo y cifras hexadecimales.
    cookie.extend(name.iter().map(|&b| char::from(b)));
    cookie.push('=');
    cookie.push_str(value);
    cookie.push_str("; Path=");
    cookie.push_str(path);
    cookie.push_str(COOKIE_ATTRS);
    if expired {
        cookie.push_str("; Max-Age=0");
    }
    if let Ok(cookie) = HeaderValue::try_from(cookie) {
        response.headers_mut().append(header::SET_COOKIE, cookie);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_and_reset_are_kept_apart_from_the_state() {
        let query = Params::split("q=acer&lang=es&page=2&reset");
        assert_eq!(query.state, "q=acer&page=2");
        assert_eq!(query.lang, "lang=es");
        assert!(query.reset);
    }

    #[test]
    fn a_query_without_lang_or_reset_is_not_copied() {
        assert!(matches!(
            Params::split("q=acer&page=2").state,
            Cow::Borrowed("q=acer&page=2")
        ));
        assert!(matches!(Params::split("q=a&lang=es").state, Cow::Owned(_)));
    }

    #[test]
    fn location_joins_path_state_and_lang() {
        let query = Params::split("lang=es");
        assert_eq!(query.location("/items", Some("q=a")), "/items?q=a&lang=es");
        assert_eq!(query.location("/items", None), "/items?lang=es");
        assert_eq!(
            Params::split("").location("/items", Some("q=a")),
            "/items?q=a"
        );
    }

    #[test]
    fn cookie_names_are_stable_and_distinct_per_path() {
        // Changing the hash would silently drop the state every browser has saved.
        assert_eq!(&cookie_name("/items"), b"pagetop_list_fca267fbaf9eb12a");
        assert_ne!(cookie_name("/items"), cookie_name("/items/archived"));
    }

    #[test]
    fn a_state_is_blank_only_when_every_value_is_empty() {
        assert!(is_blank("q=&status="));
        assert!(is_blank("q"));
        assert!(!is_blank("q=&status=active"));
        assert!(!is_blank("page=1"));
    }

    #[test]
    fn states_with_characters_not_allowed_in_cookies_are_rejected() {
        assert!(is_valid_state("q=a%20b&sort=name&dir=asc"));
        assert!(!is_valid_state("q=a,b"));
        assert!(!is_valid_state("q=\"a\""));
        assert!(!is_valid_state(""));
    }
}
