//! Constantes para los atributos y valores de HTMX 2.
//!
//! Usar estas constantes en lugar de literales evita errores tipográficos en tiempo de compilación.
//! Una mala declaración como `"hx-gte"` no genera ningún error en tiempo de ejecución, falla
//! silenciosamente. Con constantes, el compilador lo detecta de inmediato.
//!
//! # Atributos estáticos en `html!`
//!
//! Para valores conocidos en tiempo de compilación, los atributos `hx-*` pueden escribirse
//! directamente en la macro `html!` sin necesidad de [`Props`](pagetop::html::Props):
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! let markup = html! {
//!     button hx-get="/api/items" hx-target="#list" hx-swap="outerHTML" { "Load" }
//! };
//! ```
//!
//! # Atributos dinámicos con [`Props`](pagetop::html::Props)
//!
//! Cuando los valores se construyen en tiempo de ejecución o se inyectan desde una extensión,
//! puedes usar [`Props`](pagetop::html::Props) combinado con las constantes de este módulo:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_htmx::prelude::*;
//!
//! let props = Props::new(hx::GET, "/api/items")
//!     .with_prop(PropsOp::set(hx::TARGET, "#list"))
//!     .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML));
//!
//! let markup = html! {
//!     button (props) { "Load" }
//! };
//! ```
//!
//! # Integración en componentes
//!
//! El patrón recomendado es añadir un campo `props: Props` al componente y exponerlo con
//! `with_prop()`. Cualquier extensión puede entonces inyectar atributos HTMX sin que el componente
//! ni el tema necesiten conocer HTMX:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_htmx::prelude::*;
//!
//! #[derive(AutoDefault, Getters)]
//! pub struct MyButton {
//!     props: Props,
//! }
//!
//! impl MyButton {
//!     pub fn new() -> Self { Self::default() }
//!
//!     #[builder_fn]
//!     pub fn with_prop(mut self, op: PropsOp) -> Self {
//!         self.props.alter_prop(op);
//!         self
//!     }
//! }
//!
//! MyButton::new()
//!     .with_prop(PropsOp::set(hx::POST, "/api/save"))
//!     .with_prop(PropsOp::set(hx::TARGET, "#message"))
//!     .with_prop(PropsOp::set(hx::SWAP, hx::swap::INNER_HTML));
//! ```
//!
//! # Eventos en línea
//!
//! Para el atributo `hx-on:*` (cuyo nombre incluye el evento y no puede ser una constante) usa las
//! funciones [`on()`] y [`on_htmx()`]:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_htmx::prelude::*;
//!
//! // Evento nativo del DOM: hx-on:click="..."
//! // Evento propio de HTMX: hx-on::after-swap="..."
//! let props = Props::new(hx::on("click"), "this.classList.toggle('active')")
//!     .with_prop(PropsOp::set(hx::on_htmx("after-swap"), "console.log('done')"));
//! ```

// **< HTTP Methods >*******************************************************************************

/// Realiza una petición GET al servidor y aplica la respuesta al objetivo.
///
/// Es el atributo HTMX más común: carga contenido desde el servidor sin recargar la página.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// let props = Props::new(hx::GET, "/api/search")
///     .with_prop(PropsOp::set(hx::TARGET, "#results"));
/// ```
pub const GET: &str = "hx-get";

/// Realiza una petición POST al servidor.
///
/// Se usa habitualmente para enviar datos de formulario o acciones que modifican el estado del
/// servidor. Para subida de ficheros, combinar con [`ENCODING`] = `"multipart/form-data"`.
pub const POST: &str = "hx-post";

/// Realiza una petición PUT al servidor.
pub const PUT: &str = "hx-put";

/// Realiza una petición PATCH al servidor.
pub const PATCH: &str = "hx-patch";

/// Realiza una petición DELETE al servidor.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// // Al eliminar un elemento, reemplazarlo con respuesta vacía borra el nodo del DOM.
/// let props = Props::new(hx::DELETE, "/api/item/42")
///     .with_prop(PropsOp::set(hx::TARGET, "closest li"))
///     .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML));
/// ```
pub const DELETE: &str = "hx-delete";

// **< Target and Swap >****************************************************************************

/// Selector CSS del elemento que recibirá la respuesta. Por defecto, el elemento mismo.
///
/// Además de selectores CSS estándar, HTMX acepta `"this"` (el elemento), `"closest X"` (ancestro
/// más próximo), `"find X"` (descendiente) y `"next X"` / `"previous X"`.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// let props = Props::new(hx::GET, "/api/detalles")
///     .with_prop(PropsOp::set(hx::TARGET, "closest article"));
/// ```
pub const TARGET: &str = "hx-target";

/// Cómo se inserta la respuesta en el DOM. Por defecto, `innerHTML`.
///
/// Los valores estándar están disponibles en el sub-módulo [`swap`]. Además del modo base, se
/// pueden añadir modificadores separados por espacio: retrasos (`swap:200ms`), tiempo de
/// asentamiento (`settle:300ms`), scroll (`scroll:top`) y foco (`focus-scroll:true`).
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// // Reemplaza el elemento completo con una transición de 300 ms.
/// let props = Props::new(hx::SWAP, "outerHTML swap:300ms");
/// // O usando la constante tipada más los modificadores:
/// let props = Props::new(hx::SWAP, format!("{} swap:300ms", hx::swap::OUTER_HTML));
/// ```
pub const SWAP: &str = "hx-swap";

/// Sustituye fuera de banda el elemento del DOM cuyo `id` coincida con el indicado.
///
/// La respuesta del servidor puede incluir elementos marcados con `hx-swap-oob="true"`: HTMX los
/// extrae y actualiza el DOM en la posición correcta, independientemente del objetivo principal.
pub const SWAP_OOB: &str = "hx-swap-oob";

/// Selector CSS del fragmento de la respuesta que se insertará en el objetivo.
///
/// Permite devolver una página completa desde el servidor y que HTMX extraiga sólo la parte
/// relevante, facilitando la reutilización de rutas existentes.
pub const SELECT: &str = "hx-select";

/// Selector CSS de fragmentos de la respuesta que se sustituyen fuera de banda.
pub const SELECT_OOB: &str = "hx-select-oob";

// **< Trigger >************************************************************************************

/// Evento que activa la petición. Por defecto, `click` en botones y links, `change` en inputs.
///
/// Los valores simples están disponibles en el sub-módulo [`trigger`]. Las expresiones de disparo
/// compuestas se escriben como literales de cadena:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// // Buscar mientras se escribe, con 400 ms de espera y sólo si el valor cambia:
/// let props = Props::new(hx::GET, "/api/search")
///     .with_prop(PropsOp::set(hx::TRIGGER, "keyup changed delay:400ms"))
///     .with_prop(PropsOp::set(hx::TARGET, "#results"));
///
/// // Disparar una vez al cargar la página:
/// let lazy = Props::new(hx::GET, "/api/stats")
///     .with_prop(PropsOp::set(hx::TRIGGER, "load once"));
///
/// // Polling cada 5 segundos:
/// let poll = Props::new(hx::GET, "/api/estado")
///     .with_prop(PropsOp::set(hx::TRIGGER, "every 5s"));
/// ```
pub const TRIGGER: &str = "hx-trigger";

/// Convierte en AJAX todos los enlaces y formularios del elemento y sus descendientes.
///
/// El valor debe ser `"true"` o `"false"`. Cuando vale `"true"`, las respuestas se aplican al
/// `<body>` por defecto; se puede combinar con [`TARGET`] para redirigirlas.
pub const BOOST: &str = "hx-boost";

/// Empuja la URL de la respuesta al historial del navegador.
///
/// Acepta `"true"` (usa la URL de la petición), `"false"` (desactiva) o una URL concreta. Permite
/// navegación con el botón atrás manteniendo el comportamiento SPA.
pub const PUSH_URL: &str = "hx-push-url";

/// Reemplaza la URL actual en el historial sin añadir una nueva entrada.
///
/// Acepta `"true"`, `"false"` o una URL concreta. Útil cuando la petición refina la vista sin que
/// deba ser un paso independiente en el historial.
pub const REPLACE_URL: &str = "hx-replace-url";

/// Sincroniza las peticiones del elemento con las de otros elementos.
///
/// Formato: `"selector:estrategia"`. Estrategias disponibles: `drop` (descarta la nueva si hay una
/// en curso), `abort` (cancela la nueva), `replace` (cancela la anterior), `queue first|last|all`
/// (encola). Ejemplo: `"#form:abort"`.
pub const SYNC: &str = "hx-sync";

// **< Request Data >*******************************************************************************

/// Selector CSS de elementos adicionales cuyos valores se incluyen en la petición.
///
/// Acepta selectores CSS estándar más las extensiones de HTMX: `"this"`, `"closest X"`, `"find X"`.
/// Útil para incluir campos de un formulario padre en una petición de detalle.
pub const INCLUDE: &str = "hx-include";

/// Controla qué parámetros del formulario se envían en la petición.
///
/// - `"*"` - todos (valor por defecto).
/// - `"none"` - ninguno.
/// - Lista de nombres: `"name surname"` - sólo esos.
/// - Exclusión: `"not surname"` - todos excepto los indicados.
pub const PARAMS: &str = "hx-params";

/// Valores extra en JSON que se añaden a los parámetros de la petición.
///
/// Formato: objeto JSON. Los valores sobreescriben parámetros del formulario con el mismo nombre.
/// Admite JavaScript con el prefijo `js:`: `"js:{date: new Date().toISOString()}"`.
pub const VALS: &str = "hx-vals";

/// Cabeceras extra en JSON que se añaden a la petición.
///
/// Formato: objeto JSON. Permiten enviar contexto (token de sesión, versión de API, etc.) sin
/// exponerlo en los parámetros visibles del formulario.
pub const HEADERS: &str = "hx-headers";

/// Codificación de la petición. Por defecto, `"application/x-www-form-urlencoded"`.
///
/// Usar `"multipart/form-data"` para peticiones que incluyan campos de tipo `file`.
pub const ENCODING: &str = "hx-encoding";

// **< Element Behavior >***************************************************************************

/// Selector CSS del indicador de carga que se muestra mientras dura la petición.
///
/// El elemento indicado recibe la clase `htmx-request` durante la petición. Por defecto, si no se
/// especifica, la recibe el propio elemento que realiza la petición.
pub const INDICATOR: &str = "hx-indicator";

/// Selector CSS de elementos que se deshabilitan mientras dura la petición.
///
/// Añade el atributo `disabled` durante la petición y lo elimina al terminar. Evita envíos
/// duplicados al hacer clic varias veces.
pub const DISABLED_ELT: &str = "hx-disabled-elt";

/// Muestra un diálogo de confirmación (`window.confirm`) antes de enviar la petición.
///
/// Si el usuario cancela, la petición no se realiza. El valor es el texto del mensaje.
pub const CONFIRM: &str = "hx-confirm";

/// Muestra un `prompt` de texto y envía el resultado como cabecera `HX-Prompt`.
///
/// Si el usuario cancela el prompt, la petición no se realiza.
pub const PROMPT: &str = "hx-prompt";

/// Activa la validación HTML5 del formulario antes de enviar la petición.
///
/// Si algún campo no supera la validación nativa del navegador, la petición se cancela.
pub const VALIDATE: &str = "hx-validate";

/// Preserva el elemento entre respuestas HTMX.
///
/// El elemento debe tener un `id` único. HTMX no lo destruye ni lo recrea al aplicar la respuesta,
/// manteniendo su estado interno (p. ej. posición de reproducción de un vídeo).
pub const PRESERVE: &str = "hx-preserve";

// **< Config and Extensions >**********************************************************************

/// Activa una o varias extensiones HTMX en el elemento y sus descendientes.
///
/// Las extensiones se identifican por nombre y se separan con comas. Extensiones comunes:
/// - `"ws"` - soporte WebSocket.
/// - `"sse"` - soporte Server-Sent Events.
/// - `"json-enc"` - codifica la petición como JSON en lugar de form-urlencoded.
/// - `"loading-states"` - gestión avanzada de estados de carga.
///
/// `pagetop-htmx` sólo integra el *core* de HTMX: usar cualquiera de estas extensiones (ver el
/// [catálogo oficial](https://htmx.org/extensions/)) requiere añadir su script correspondiente por
/// separado, por ejemplo con [`JavaScript::defer()`](pagetop::html::JavaScript::defer) en
/// [`dependencies()`](pagetop::core::extension::Extension::dependencies).
pub const EXT: &str = "hx-ext";

/// Atributos HTMX que los elementos descendientes NO heredarán de este elemento.
///
/// Acepta una lista de atributos separados por comas o `"*"` para bloquear toda herencia.
pub const DISINHERIT: &str = "hx-disinherit";

/// Atributos HTMX que los elementos descendientes SÍ heredarán (anula [`DISINHERIT`]).
pub const INHERIT: &str = "hx-inherit";

/// Opciones de configuración de la petición en JSON.
///
/// Claves disponibles: `timeout` (ms), `credentials` (`"include"`, `"omit"`...), `noHeaders`
/// (bool), `getWithBody` (bool).
pub const REQUEST: &str = "hx-request";

/// Controla si este elemento participa en el historial del navegador.
///
/// Con el valor `"false"`, las peticiones de este elemento no se guardan en el historial aunque
/// [`PUSH_URL`] esté activo en un elemento padre.
pub const HISTORY: &str = "hx-history";

/// Designa el elemento como contenedor del historial del navegador.
///
/// HTMX guarda y restaura el contenido de este elemento al navegar hacia atrás/adelante. Sólo debe
/// haber un elemento con este atributo en la página.
pub const HISTORY_ELT: &str = "hx-history-elt";

/// Desactiva el procesamiento HTMX en el elemento y todos sus descendientes.
///
/// Útil para aislar zonas del DOM gestionadas por otra librería o para desactivar HTMX en secciones
/// de contenido generado dinámicamente donde no debe intervenir.
pub const DISABLE: &str = "hx-disable";

// **< Inline Events (hx-on) >**********************************************************************

/// Genera `hx-on:{event}` para escuchar eventos nativos del DOM en línea.
///
/// Es la alternativa de HTMX a los handlers `on*` de HTML (`onclick`, `onmouseenter`, ...). La
/// diferencia clave está en cómo los trata el navegador bajo una política CSP (*Content Security
/// Policy*): los atributos `on*` son JavaScript en línea y quedan bloqueados si la CSP no incluye
/// `'unsafe-inline'`; en cambio, `hx-on:*` es un atributo de datos que HTMX lee e interpreta desde
/// su propio código ya autorizado, por lo que la CSP no lo bloquea.
///
/// La CSP puede definirla el servidor en la cabecera HTTP `Content-Security-Policy`, o la
/// aplicación en una etiqueta `<meta http-equiv="Content-Security-Policy">` en el `<head>` del
/// documento. En la práctica, pocas aplicaciones configuran una CSP estricta, pero es una buena
/// práctica de seguridad que conviene tener en cuenta.
///
/// El valor es código JavaScript que se ejecuta cuando el evento se dispara; `event` contiene el
/// objeto del evento.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// let props = Props::new(hx::on("click"), "this.classList.toggle('active')")
///     .with_prop(PropsOp::set(hx::on("mouseenter"), "this.style.opacity='0.8'"));
/// ```
pub fn on(event: &str) -> String {
    format!("hx-on:{event}")
}

/// Genera `hx-on::{event}` para escuchar eventos propios de HTMX en línea.
///
/// Los eventos de HTMX usan un doble carácter dos-puntos (`hx-on::evento`). El ciclo de vida
/// completo incluye `before-request`, `after-request`, `before-swap`, `after-swap`,
/// `before-settle`, `after-settle`, `after-on-load`, `history-restore`, entre otros.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// let props = Props::new(hx::on_htmx("before-request"), "console.log('enviando...')")
///     .with_prop(PropsOp::set(hx::on_htmx("after-swap"), "initTooltips()"));
/// ```
pub fn on_htmx(event: &str) -> String {
    format!("hx-on::{event}")
}

// **< HTMX Request Headers >***********************************************************************

/// Nombres de las cabeceras que HTMX envía con cada petición AJAX.
///
/// Están en minúsculas porque así las normaliza el módulo `http`. Se pueden usar con
/// [`HttpRequest::headers()`](pagetop::web::HttpRequest::headers) para leer sus valores
/// directamente, aunque lo habitual es usar el trait
/// [`HtmxRequestExt`](crate::request::HtmxRequestExt).
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// async fn handler(request: HttpRequest) {
///     if let Some(target) = request.headers().get(hx::request::TARGET) {
///         // El elemento objetivo tenía este id.
///     }
/// }
/// ```
pub mod request {
    /// Siempre `"true"` en peticiones HTMX. Permite distinguirlas de navegaciones directas.
    pub const REQUEST: &str = "hx-request";
    /// `"true"` si la petición viene de un enlace o formulario con `hx-boost`.
    pub const BOOSTED: &str = "hx-boosted";
    /// URL de la página activa en el navegador cuando se realizó la petición.
    pub const CURRENT_URL: &str = "hx-current-url";
    /// `"true"` si la petición es una restauración del historial del navegador.
    pub const HISTORY_RESTORE_REQUEST: &str = "hx-history-restore-request";
    /// Texto introducido por el usuario en un diálogo `hx-prompt`.
    pub const PROMPT: &str = "hx-prompt";
    /// Valor del atributo `id` del elemento objetivo de la petición.
    pub const TARGET: &str = "hx-target";
    /// Valor del atributo `id` del elemento que disparó la petición.
    pub const TRIGGER: &str = "hx-trigger";
    /// Valor del atributo `name` del elemento que disparó la petición.
    pub const TRIGGER_NAME: &str = "hx-trigger-name";
}

// **< HTMX Response Headers >**********************************************************************

/// Cabeceras de respuesta HTTP para HTMX.
///
/// Se pueden usar con [`HeaderMap`](pagetop::web::http::HeaderMap) para construir respuestas
/// manualmente, aunque lo habitual es usar el constructor
/// [`HtmxResponse`](crate::response::HtmxResponse).
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// let mut headers = web::http::HeaderMap::new();
/// headers.insert(
///     hx::response::TRIGGER.parse::<web::http::HeaderName>().unwrap(),
///     web::http::HeaderValue::from_static("itemAdded"),
/// );
/// ```
pub mod response {
    /// Redirige mediante AJAX a la URL o configuración JSON indicada. Ver
    /// [`HtmxResponse::location()`](crate::response::HtmxResponse::location) y
    /// [`HtmxResponse::location_json()`](crate::response::HtmxResponse::location_json).
    pub const LOCATION: &str = "HX-Location";
    /// Empuja la URL indicada al historial del navegador. Ver
    /// [`HtmxResponse::push_url()`](crate::response::HtmxResponse::push_url).
    pub const PUSH_URL: &str = "HX-Push-Url";
    /// Provoca una redirección completa del navegador. Ver
    /// [`HtmxResponse::redirect()`](crate::response::HtmxResponse::redirect).
    pub const REDIRECT: &str = "HX-Redirect";
    /// Provoca una recarga completa de la página. Ver
    /// [`HtmxResponse::refresh()`](crate::response::HtmxResponse::refresh).
    pub const REFRESH: &str = "HX-Refresh";
    /// Reemplaza la URL actual en el historial. Ver
    /// [`HtmxResponse::replace_url()`](crate::response::HtmxResponse::replace_url).
    pub const REPLACE_URL: &str = "HX-Replace-Url";
    /// Anula el `hx-swap` del elemento. Ver
    /// [`HtmxResponse::reswap()`](crate::response::HtmxResponse::reswap).
    pub const RESWAP: &str = "HX-Reswap";
    /// Anula el `hx-target` del elemento. Ver
    /// [`HtmxResponse::retarget()`](crate::response::HtmxResponse::retarget).
    pub const RETARGET: &str = "HX-Retarget";
    /// Anula el `hx-select` del elemento. Ver
    /// [`HtmxResponse::reselect()`](crate::response::HtmxResponse::reselect).
    pub const RESELECT: &str = "HX-Reselect";
    /// Dispara eventos JavaScript al completar la respuesta. Ver
    /// [`HtmxResponse::trigger()`](crate::response::HtmxResponse::trigger).
    pub const TRIGGER: &str = "HX-Trigger";
    /// Dispara eventos tras la fase *settle*. Ver
    /// [`HtmxResponse::trigger_after_settle()`](crate::response::HtmxResponse::trigger_after_settle).
    pub const TRIGGER_AFTER_SETTLE: &str = "HX-Trigger-After-Settle";
    /// Dispara eventos tras el *swap*. Ver
    /// [`HtmxResponse::trigger_after_swap()`](crate::response::HtmxResponse::trigger_after_swap).
    pub const TRIGGER_AFTER_SWAP: &str = "HX-Trigger-After-Swap";
}

// **< hx-swap Values >*****************************************************************************

/// Valores estándar del atributo [`SWAP`] (`hx-swap`).
///
/// Se pueden combinar con modificadores separados por espacio:
/// - `swap:Xms` - tiempo de espera antes de realizar el intercambio.
/// - `settle:Xms` - tiempo de espera antes de quitar las clases de transición.
/// - `scroll:top` / `scroll:bottom` - desplaza el objetivo tras el intercambio.
/// - `show:top` / `show:bottom` - hace visible el objetivo tras el intercambio.
/// - `focus-scroll:true` - sigue al elemento enfocado.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// // Reemplaza el elemento con una transición de 200 ms y desplaza al inicio:
/// let props = Props::new(hx::SWAP, format!("{} swap:200ms scroll:top", hx::swap::OUTER_HTML));
/// ```
pub mod swap {
    /// Reemplaza el contenido interior del objetivo (valor por defecto de HTMX).
    pub const INNER_HTML: &str = "innerHTML";
    /// Reemplaza el elemento objetivo completo.
    pub const OUTER_HTML: &str = "outerHTML";
    /// Reemplaza el elemento objetivo completo y desplaza la vista al borde superior tras el
    /// intercambio (combina [`OUTER_HTML`] con el modificador `scroll:top`).
    ///
    /// Pensado para listados paginados/ordenables cuyo `hx-swap` sustituye un contenedor completo
    /// (tabla + paginador) al variar el número de filas entre páginas. Sin este anclaje la posición
    /// del scroll tras el intercambio queda desajustada.
    pub const OUTER_HTML_SCROLL_TOP: &str = "outerHTML scroll:top";
    /// Inserta la respuesta antes de la etiqueta de apertura del objetivo.
    pub const BEFORE_BEGIN: &str = "beforebegin";
    /// Inserta la respuesta al inicio del contenido del objetivo.
    pub const AFTER_BEGIN: &str = "afterbegin";
    /// Inserta la respuesta al final del contenido del objetivo.
    pub const BEFORE_END: &str = "beforeend";
    /// Inserta la respuesta después de la etiqueta de cierre del objetivo.
    pub const AFTER_END: &str = "afterend";
    /// Elimina el elemento objetivo independientemente de la respuesta.
    pub const DELETE: &str = "delete";
    /// No realiza ningún intercambio; útil cuando sólo importan las cabeceras de respuesta.
    pub const NONE: &str = "none";
}

// **< hx-trigger Values >**************************************************************************

/// Eventos comunes del atributo [`TRIGGER`] (`hx-trigger`).
///
/// Estos valores cubren los disparadores más simples. Las expresiones de disparo compuestas deben
/// escribirse como literales de cadena. Modificadores disponibles:
/// - `once` - se dispara sólo la primera vez.
/// - `changed` - sólo si el valor del elemento ha cambiado.
/// - `delay:Xms` - espera antes de disparar (se cancela si el evento vuelve a ocurrir).
/// - `throttle:Xms` - limita la frecuencia máxima de disparo.
/// - `from:selector` - escucha el evento en otro elemento.
/// - `target:selector` - sólo si el evento viene del selector indicado.
/// - `consume` - evita que el evento se propague a otros elementos HTMX.
/// - `queue:first|last|all|none` - política de cola cuando llegan eventos consecutivos.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_htmx::prelude::*;
/// // Búsqueda progresiva: petición 400 ms después de que el usuario deje de escribir.
/// let search = Props::new(hx::TRIGGER, "keyup changed delay:400ms");
///
/// // Carga diferida al entrar en el viewport, una sola vez.
/// let lazy = Props::new(hx::TRIGGER, "intersect once");
///
/// // Polling: actualiza cada 10 segundos mientras el elemento esté en el DOM.
/// let poll = Props::new(hx::TRIGGER, "every 10s");
///
/// // Múltiples eventos: clic o pulsación de Enter en el campo.
/// let multi = Props::new(hx::TRIGGER, "click, keyup[key=='Enter']");
///
/// // Escucha un evento personalizado emitido desde otro elemento.
/// let custom = Props::new(hx::TRIGGER, "itemAdded from:body");
/// ```
pub mod trigger {
    /// Se dispara al hacer clic (valor por defecto en la mayoría de elementos interactivos).
    pub const CLICK: &str = "click";
    /// Se dispara cuando el valor del elemento cambia (valor por defecto en `input`/`select`).
    pub const CHANGE: &str = "change";
    /// Se dispara al enviar un formulario.
    pub const SUBMIT: &str = "submit";
    /// Se dispara al soltar una tecla.
    pub const KEYUP: &str = "keyup";
    /// Se dispara cuando la página termina de cargarse.
    pub const LOAD: &str = "load";
    /// Se dispara cuando el elemento entra en el área visible del *viewport* al hacer scroll.
    pub const REVEALED: &str = "revealed";
    /// Se dispara cuando el elemento intersecta con el *viewport* (Intersection Observer API).
    pub const INTERSECT: &str = "intersect";
}
