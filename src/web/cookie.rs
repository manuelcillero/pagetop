use crate::global;

use std::fmt::Write;

/// Devuelve el valor de una cabecera `Set-Cookie` con los atributos comunes de PageTop.
///
/// Toda *cookie* lleva `HttpOnly`, para que el código JavaScript de la página no pueda leerla, y
/// `SameSite=Lax`, para que el navegador no la envíe en las peticiones que otros sitios hacen a la
/// aplicación, salvo cuando llevan al usuario a una de sus páginas con `GET` (al seguir un enlace,
/// por ejemplo). Lleva además `Secure` si está activado
/// [`secure_cookies`](crate::global::ServerSettings::secure_cookies) en `[server]`.
///
/// - `name` y `value` sólo pueden contener caracteres válidos en una *cookie* (RFC 6265), y `path`
///   no puede contener `;`. La función no los comprueba ni los codifica: un `;` añadiría atributos
///   a la *cookie*.
/// - `path` es la ruta a partir de la cual el navegador envía la *cookie*, `"/"` para toda la
///   aplicación.
/// - `max_age` son los segundos que dura. Con `None` es una *cookie* de sesión, que el navegador
///   borra al cerrarse; con `Some(0)` borra la que tenga el navegador con el mismo nombre y ruta.
///
/// Es la función que usan las *cookies* de PageTop y de sus extensiones propias, para que todas
/// sigan el mismo criterio. Cualquier *cookie* nueva debe cumplir además los criterios de
/// [*Cookies*](crate::web#cookies) para no requerir la aceptación del usuario.
///
/// ```rust
/// # use pagetop::prelude::*;
/// let cookie = web::cookie_header("theme", "dark", "/", Some(3600));
/// assert!(cookie.starts_with("theme=dark; Path=/; HttpOnly; SameSite=Lax"));
/// assert!(cookie.ends_with("; Max-Age=3600"));
/// ```
pub fn cookie_header(name: &str, value: &str, path: &str, max_age: Option<u64>) -> String {
    build(
        name,
        value,
        path,
        max_age,
        global::SETTINGS.server.secure_cookies,
    )
}

fn build(name: &str, value: &str, path: &str, max_age: Option<u64>, secure: bool) -> String {
    let mut cookie = String::with_capacity(name.len() + value.len() + path.len() + 64);
    cookie.push_str(name);
    cookie.push('=');
    cookie.push_str(value);
    cookie.push_str("; Path=");
    cookie.push_str(path);
    cookie.push_str("; HttpOnly; SameSite=Lax");
    if secure {
        cookie.push_str("; Secure");
    }
    if let Some(secs) = max_age {
        // Escribir en un `String` nunca falla.
        let _ = write!(cookie, "; Max-Age={secs}");
    }
    cookie
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_cookie_without_secure() {
        assert_eq!(
            build("pagetop_list_x", "q=a", "/items", None, false),
            "pagetop_list_x=q=a; Path=/items; HttpOnly; SameSite=Lax"
        );
    }

    #[test]
    fn secure_goes_before_max_age() {
        assert_eq!(
            build("pagetop", "sid", "/", Some(1_209_600), true),
            "pagetop=sid; Path=/; HttpOnly; SameSite=Lax; Secure; Max-Age=1209600"
        );
    }

    #[test]
    fn expired_cookie_keeps_secure() {
        assert_eq!(
            build("pagetop_flash", "", "/", Some(0), true),
            "pagetop_flash=; Path=/; HttpOnly; SameSite=Lax; Secure; Max-Age=0"
        );
    }
}
