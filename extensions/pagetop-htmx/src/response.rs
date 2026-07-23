//! Implementación de [`HtmxResponse`] e [`IntoResponse`](pagetop::web::IntoResponse) para HTMX.

use pagetop::prelude::*;

// **< HtmxResponse >*******************************************************************************

/// Generador de respuestas HTML parciales con cabeceras HTMX.
///
/// En una aplicación HTMX, los handlers del servidor devuelven con frecuencia fragmentos HTML
/// parciales acompañados de cabeceras especiales que instruyen al cliente sobre qué hacer con la
/// respuesta: actualizar la URL del historial, disparar eventos JavaScript, redirigir, etc.
///
/// Implementa [`IntoResponse`](pagetop::web::IntoResponse), por lo que puede devolverse
/// directamente desde cualquier handler.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// async fn add_item(request: HttpRequest) -> impl IntoResponse {
///     let new_item = html! { li #item-42 { "New item" } };
///
///     HtmxResponse::new(new_item)
///         .retarget("#list")
///         .reswap(hx::swap::BEFORE_END)
///         .push_url("/items")
///         .trigger("itemAdded")
/// }
/// ```
///
/// # Respuestas de sólo cabeceras
///
/// Cuando la respuesta no lleva cuerpo HTML (por ejemplo, una redirección o un refresco), usa
/// [`HtmxResponse::empty()`](Self::empty):
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// async fn delete_item() -> impl IntoResponse {
///     HtmxResponse::empty().redirect("/items")
/// }
/// ```
///
/// # Construcción
///
/// - [`HtmxResponse::new(markup)`](Self::new), con el fragmento HTML.
/// - [`HtmxResponse::empty()`](Self::empty), sin cuerpo, sólo cabeceras.
///
/// # Cabeceras disponibles
///
/// Los nombres de cabecera como constantes están en [`crate::hx::response`].
///
/// # Múltiples eventos en `trigger`
///
/// Para disparar varios eventos en una sola llamada, pasa una cadena con comas o un objeto JSON:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// // Dos eventos sin datos:
/// HtmxResponse::empty().trigger("itemAdded, listUpdated");
///
/// // Evento con datos en JSON:
/// HtmxResponse::empty().trigger(r#"{"itemAdded": {"id": 42}}"#);
/// ```
#[must_use]
pub struct HtmxResponse {
    markup: Markup,
    headers: web::http::HeaderMap,
}

impl HtmxResponse {
    /// Crea una respuesta con el fragmento HTML indicado.
    pub fn new(markup: Markup) -> Self {
        Self {
            markup,
            headers: web::http::HeaderMap::new(),
        }
    }

    /// Crea una respuesta sin cuerpo HTML, útil para respuestas de sólo cabeceras.
    pub fn empty() -> Self {
        Self::new(html! {})
    }

    // **< HtmxResponse BUILDER >*******************************************************************

    /// Hace que HTMX realice una navegación AJAX a la URL indicada sin recargar la página.
    ///
    /// A diferencia de [`redirect()`](Self::redirect), la navegación usa HTMX y actualiza sólo el
    /// objetivo definido por el destino. Para personalizar `target`, `swap`, `select` o `values`,
    /// usa [`location_json()`](Self::location_json).
    ///
    /// Usa [`Context::route()`](pagetop::core::component::Context::route) en lugar de un literal
    /// para que la URL preserve el parámetro `lang` cuando corresponda:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// # fn build_response(cx: &Context) -> HtmxResponse {
    /// HtmxResponse::empty().location(cx.route("/items"))
    /// # }
    /// ```
    pub fn location(self, url: impl Into<RoutePath>) -> Self {
        self.set_header(b"hx-location", url.into().to_string())
    }

    /// Hace que HTMX realice una navegación AJAX personalizada, con un objeto JSON de configuración
    /// en lugar de una URL simple.
    ///
    /// Acepta un objeto JSON con las claves `path`, `target`, `swap`, `select` y `values` (ver la
    /// [documentación de HTMX](https://htmx.org/reference/#response_headers) para el detalle de
    /// cada una). Al no ser una URL, no admite `Context::route()`: si `path` necesita el parámetro
    /// `lang`, hay que componerlo a mano antes de construir el JSON. Para una navegación simple sin
    /// estas opciones, usa [`location()`](Self::location).
    ///
    /// Si `json` no es sintácticamente válido, la cabecera se descarta y se registra un aviso; el
    /// resto de la respuesta no se ve afectado. Esta comprobación sólo valida la sintaxis JSON, no
    /// que las claves sean las que espera HTMX.
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// HtmxResponse::empty()
    ///     .location_json(r##"{"path": "/items", "target": "#content"}"##);
    /// ```
    ///
    /// Si algún valor se calcula en tiempo de ejecución, constrúyelo con [`serde_json::json!`] en
    /// lugar de interpolarlo a mano con `format!()`: evita comillas u otros caracteres sin escapar
    /// que romperían la estructura del JSON.
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// let item_name = "Alice's item"; // Contiene una comilla: no es seguro interpolarlo a mano.
    ///
    /// let json = serde_json::json!({
    ///     "path": "/items",
    ///     "values": { "name": item_name },
    /// })
    /// .to_string();
    ///
    /// HtmxResponse::empty().location_json(json);
    /// ```
    pub fn location_json(self, json: impl Into<String>) -> Self {
        let json = json.into();
        if let Err(error) = serde_json::from_str::<serde_json::Value>(&json) {
            trace::warn!(
                json = %json,
                %error,
                "HtmxResponse: invalid JSON in location_json(), header discarded",
            );
            return self;
        }
        self.set_header(b"hx-location", json)
    }

    /// Empuja la URL indicada al historial del navegador.
    ///
    /// El usuario podrá navegar hacia atrás hasta esa URL. Usar `"false"` para desactivar el empuje
    /// aunque esté habilitado por el atributo `hx-push-url` del elemento.
    ///
    /// Usa [`Context::route()`](pagetop::core::component::Context::route) en lugar de un literal
    /// para que la URL preserve el parámetro `lang` cuando corresponda:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// # fn build_response(cx: &Context) -> HtmxResponse {
    /// HtmxResponse::empty().push_url(cx.route("/items"))
    /// # }
    /// ```
    pub fn push_url(self, url: impl Into<RoutePath>) -> Self {
        self.set_header(b"hx-push-url", url.into().to_string())
    }

    /// Reemplaza la URL actual en el historial sin añadir una nueva entrada.
    ///
    /// Usar `"false"` para desactivar el reemplazo. Usa
    /// [`Context::route()`](pagetop::core::component::Context::route) en lugar de un literal para
    /// que la URL preserve el parámetro `lang` cuando corresponda.
    pub fn replace_url(self, url: impl Into<RoutePath>) -> Self {
        self.set_header(b"hx-replace-url", url.into().to_string())
    }

    /// Provoca una redirección completa del navegador a la URL indicada.
    ///
    /// A diferencia de [`location()`](Self::location), esta redirección recarga la página por
    /// completo, como un `window.location.href = url` en JavaScript.
    ///
    /// Usa [`Context::route()`](pagetop::core::component::Context::route) en lugar de un literal
    /// para que la URL preserve el parámetro `lang` cuando corresponda:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// # fn build_response(cx: &Context) -> HtmxResponse {
    /// HtmxResponse::empty().redirect(cx.route("/items"))
    /// # }
    /// ```
    pub fn redirect(self, url: impl Into<RoutePath>) -> Self {
        self.set_header(b"hx-redirect", url.into().to_string())
    }

    /// Provoca una recarga completa de la página actual.
    ///
    /// Equivale a `window.location.reload()` en JavaScript.
    pub fn refresh(self) -> Self {
        self.set_header(b"hx-refresh", "true")
    }

    /// Anula el `hx-target` del elemento y redirige la respuesta al selector CSS indicado.
    ///
    /// Útil cuando el servidor necesita actualizar un elemento distinto al que realizó la petición,
    /// sin modificar el HTML del cliente.
    pub fn retarget(self, selector: impl Into<String>) -> Self {
        self.set_header(b"hx-retarget", selector)
    }

    /// Anula el `hx-swap` del elemento e impone la estrategia de sustitución indicada.
    ///
    /// Acepta los mismos valores que el atributo `hx-swap`, incluidos modificadores (`swap:200ms`,
    /// `scroll:top`, ...). Los valores tipados están en [`crate::hx::swap`].
    pub fn reswap(self, strategy: impl Into<String>) -> Self {
        self.set_header(b"hx-reswap", strategy)
    }

    /// Anula el `hx-select` del elemento y selecciona el fragmento CSS indicado de la respuesta
    /// para insertarlo en el objetivo.
    pub fn reselect(self, selector: impl Into<String>) -> Self {
        self.set_header(b"hx-reselect", selector)
    }

    /// Dispara uno o varios eventos JavaScript en el cliente al completar la respuesta.
    ///
    /// Los eventos se disparan inmediatamente tras procesar la respuesta. Para disparar eventos con
    /// datos o después de otras fases del ciclo HTMX, ver
    /// [`trigger_after_settle()`](Self::trigger_after_settle) y
    /// [`trigger_after_swap()`](Self::trigger_after_swap).
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// // Evento simple:
    /// HtmxResponse::empty().trigger("itemAdded");
    ///
    /// // Múltiples eventos sin datos:
    /// HtmxResponse::empty().trigger("itemAdded, listUpdated");
    ///
    /// // Evento con datos en JSON:
    /// HtmxResponse::empty().trigger(r#"{"itemAdded": {"id": 42, "name": "Example"}}"#);
    /// ```
    ///
    /// Si el dato del evento se calcula en tiempo de ejecución, constrúyelo con
    /// [`serde_json::json!`] en lugar de interpolarlo a mano con `format!()`, para evitar comillas
    /// u otros caracteres sin escapar que romperían la estructura del JSON:
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    /// use pagetop_htmx::prelude::*;
    ///
    /// let item_name = "Alice's item"; // Contiene una comilla: no es seguro interpolarlo a mano.
    ///
    /// let json = serde_json::json!({ "itemAdded": { "name": item_name } }).to_string();
    ///
    /// HtmxResponse::empty().trigger(json);
    /// ```
    pub fn trigger(self, event: impl Into<String>) -> Self {
        self.set_header(b"hx-trigger", event)
    }

    /// Dispara eventos JavaScript después de que HTMX haya aplicado la respuesta al DOM y haya
    /// completado la fase de *settle* (animaciones CSS).
    ///
    /// Acepta los mismos formatos que [`trigger()`](Self::trigger).
    pub fn trigger_after_settle(self, event: impl Into<String>) -> Self {
        self.set_header(b"hx-trigger-after-settle", event)
    }

    /// Dispara eventos JavaScript después de que HTMX haya aplicado la respuesta al DOM, pero antes
    /// de la fase de *settle*.
    ///
    /// Acepta los mismos formatos que [`trigger()`](Self::trigger).
    pub fn trigger_after_swap(self, event: impl Into<String>) -> Self {
        self.set_header(b"hx-trigger-after-swap", event)
    }

    // Inserta o reemplaza una cabecera. Los nombres deben ser bytes ASCII en minúsculas.
    fn set_header(mut self, name: &[u8], value: impl Into<String>) -> Self {
        let value = value.into();
        if let (Ok(n), Ok(v)) = (
            web::http::HeaderName::from_bytes(name),
            web::http::HeaderValue::from_str(&value),
        ) {
            self.headers.insert(n, v);
        } else {
            trace::warn!(value = %value, "HtmxResponse: invalid header value, header discarded");
        }
        self
    }
}

impl web::IntoResponse for HtmxResponse {
    fn into_response(self) -> Response {
        let mut headers = self.headers;
        headers.insert(
            web::http::header::CONTENT_TYPE,
            web::http::HeaderValue::from_static("text/html; charset=utf-8"),
        );
        (headers, self.markup.into_string()).into_response()
    }
}
