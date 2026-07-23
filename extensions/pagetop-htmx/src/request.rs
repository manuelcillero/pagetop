//! Implementación de [`HtmxRequestExt`] para [`pagetop::web::HttpRequest`].

use pagetop::prelude::*;

// **< HtmxRequestExt >*****************************************************************************

/// Extiende [`HttpRequest`](pagetop::web::HttpRequest) con métodos para detectar y leer peticiones
/// HTMX.
///
/// HTMX añade cabeceras especiales a cada petición AJAX. Este trait permite acceder a ellas de
/// forma expresiva, sin manipular [`pagetop::web::http::HeaderMap`] directamente.
///
/// El patrón más común es devolver una respuesta distinta según si la petición viene de HTMX
/// (fragmento parcial) o de una navegación directa (página completa). Cuando las dos ramas retornan
/// tipos distintos, se usa [`pagetop::web::Response`] como tipo común:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// async fn list_items(request: HttpRequest) -> Response {
///     if request.is_htmx() {
///         // Fragmento parcial con cabeceras HTMX opcionales.
///         HtmxResponse::new(html! { ul { li { "Item 1" } li { "Item 2" } } })
///             .into_response()
///     } else {
///         // Página completa para navegación directa.
///         Page::new(request)
///             .with_child(Html::with(|_| html! {
///                 ul { li { "Item 1" } li { "Item 2" } }
///             }))
///             .render().await
///             .into_response()
///     }
/// }
/// ```
///
/// Los nombres de cabecera como constantes están en [`crate::hx::request`].
pub trait HtmxRequestExt {
    /// Devuelve `true` si la petición proviene de HTMX (`HX-Request: true`).
    ///
    /// Es la comprobación principal para distinguir una petición HTMX de una navegación directa del
    /// navegador.
    fn is_htmx(&self) -> bool;

    /// Devuelve `true` si la petición proviene de un enlace o formulario con `hx-boost`.
    ///
    /// Las peticiones boosted son HTMX pero conservan la semántica de navegación completa: el
    /// objetivo por defecto es `<body>`. Puede ser útil para no devolver un fragmento parcial en
    /// este caso.
    fn is_boosted(&self) -> bool;

    /// Devuelve `true` si la petición es una restauración del historial del navegador.
    ///
    /// Ocurre cuando el usuario navega hacia atrás o adelante y HTMX necesita restaurar el estado
    /// de la página. En este caso conviene devolver la página completa.
    fn is_history_restore(&self) -> bool;

    /// URL de la página activa en el navegador en el momento de la petición (`HX-Current-URL`).
    ///
    /// Útil para redirigir o actualizar la URL del historial en función de dónde estaba el usuario.
    fn hx_current_url(&self) -> Option<&str>;

    /// Valor del atributo `id` del elemento objetivo de la petición (`HX-Target`).
    ///
    /// Si el elemento objetivo no tiene `id`, esta cabecera no se envía.
    fn hx_target(&self) -> Option<&str>;

    /// Valor del atributo `id` del elemento que disparó la petición (`HX-Trigger`).
    ///
    /// Si el elemento disparador no tiene `id`, esta cabecera no se envía. Ver también
    /// [`hx_trigger_name()`](Self::hx_trigger_name).
    fn hx_trigger_id(&self) -> Option<&str>;

    /// Valor del atributo `name` del elemento que disparó la petición (`HX-Trigger-Name`).
    ///
    /// Especialmente útil en formularios, donde el elemento disparador puede tener `name` pero no
    /// `id`.
    fn hx_trigger_name(&self) -> Option<&str>;

    /// Texto introducido por el usuario en un diálogo `hx-prompt` (`HX-Prompt`).
    ///
    /// Sólo presente si el elemento tiene el atributo `hx-prompt` y el usuario no canceló el
    /// diálogo.
    fn hx_prompt(&self) -> Option<&str>;
}

impl HtmxRequestExt for HttpRequest {
    fn is_htmx(&self) -> bool {
        header_equals(self.headers(), "hx-request", "true")
    }

    fn is_boosted(&self) -> bool {
        header_equals(self.headers(), "hx-boosted", "true")
    }

    fn is_history_restore(&self) -> bool {
        header_equals(self.headers(), "hx-history-restore-request", "true")
    }

    fn hx_current_url(&self) -> Option<&str> {
        header_str(self.headers(), "hx-current-url")
    }

    fn hx_target(&self) -> Option<&str> {
        header_str(self.headers(), "hx-target")
    }

    fn hx_trigger_id(&self) -> Option<&str> {
        header_str(self.headers(), "hx-trigger")
    }

    fn hx_trigger_name(&self) -> Option<&str> {
        header_str(self.headers(), "hx-trigger-name")
    }

    fn hx_prompt(&self) -> Option<&str> {
        header_str(self.headers(), "hx-prompt")
    }
}

fn header_equals(headers: &web::http::HeaderMap, name: &str, expected: &str) -> bool {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|v| v == expected)
        .unwrap_or(false)
}

fn header_str<'a>(headers: &'a web::http::HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}
