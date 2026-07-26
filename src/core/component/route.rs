use crate::core::component::Context;
use crate::html::RoutePath;
use crate::util;

use std::fmt;
use std::sync::Arc;

/// Encapsula una función que **resuelve una ruta URL** según el contexto de renderizado.
///
/// `Route` envuelve un closure (`Arc<dyn Fn>`) que le permite capturar valores de su entorno,
/// normalmente un identificador u otro dato dinámico, para construir la ruta. También puede
/// clonarse a bajo coste (sólo el `Arc` interno) y compartirse entre hilos.
///
/// Los componentes que acepten una `Route` la invocan con [`resolve()`](Self::resolve) durante el
/// renderizado para obtener la [`RoutePath`] final, que se asigna al atributo HTML correspondiente
/// (normalmente `href` o `action`).
///
/// # ¿Cuál usar, `RoutePath`, `Route` o `Context::route()`?
///
/// [`RoutePath`] es el valor común. Incluye un *path* más una lista de parámetros de consulta, sin
/// ninguna noción de [`Context`] ni de idioma. Es lo que devuelven [`Context::route()`] y
/// [`resolve()`](Self::resolve), y lo que reciben directamente funciones que ya se llaman con `cx`
/// a mano y consumen el resultado de inmediato, como
/// [`Waypoint::append_to()`](crate::response::Waypoint::append_to) /
/// [`Waypoint::or()`](crate::response::Waypoint::or), [`Redirect::*`](crate::response::Redirect) o
/// [`SortLink`](crate::base::component::table::SortLink). Al no depender de `Context`, también
/// sirve fuera del ciclo de renderizado, por ejemplo, para construir la URL de una API externa.
///
/// Entre `Route` y `Context::route()`, cuál usar depende de una sola pregunta: ¿el valor es una
/// propiedad de un componente, que se resolverá más tarde en su propio
/// [`prepare()`](crate::core::component::Component::prepare), o se va a resolver desde una función
/// que ya tiene `cx: &Context`?
///
/// - Los componentes suelen usar `Route` en sus propiedades. Su contenido llega en bruto y se
///   resuelve más tarde, normalmente en la ejecución de su `prepare()`, cuando ya existe un
///   `Context` concreto para esa petición.
///
///   Un literal (`"/path"`) o un `String` se enrutan automáticamente mediante conversión implícita
///   a `Route`, así que no hace falta invocarlo a mano salvo que quieras añadir parámetros
///   adicionales.
///
/// - Si tienes `cx: &Context` (o `&mut Context`) a tu disposición, ya sea en un handler HTTP,
///   dentro de un [`Html::with(|cx| ...)`](crate::base::component::Html::with) o cualquier otra
///   función que reciba `cx`, puedes usar `Context::route()` para obtener el `RoutePath` de
///   inmediato, seguir componiéndolo con `.with_param()`/`.with_flag()`, o insertarlo directamente
///   en el marcado.
///
///   A diferencia de `Route`, un literal pasado directamente a las funciones anteriores **no**
///   reconoce `Context` ni idioma, por lo que habría que llamar a `cx.route(...)` a mano antes de
///   invocarlas (ver el ejemplo con `Waypoint::append_to()` más abajo).
///
/// # Detección automática de URLs externas
///
/// La conversión implícita desde `&str`/`String` reconoce si el texto **parece** una URL externa
/// (ver [`util::url_looks_external()`](crate::util::url_looks_external)) y la trata como tal. En
/// ese caso se comporta como [`Route::external()`], sin pasar por `Context::route()`. Una URL
/// externa nunca debe llevar `lang`, porque no pertenece al espacio de rutas de la aplicación.
///
/// Esta protección no depende sólo de `Route`: [`Context::route()`] también comprueba si el
/// resultado parece externo antes de añadir `lang`, así que llamarlo directamente (por ejemplo, con
/// una URL dinámica que no ha pasado por `Route`) tampoco añade el parámetro.
///
/// Esta detección es una comodidad, no una garantía: se basa en el prefijo del texto. Usa
/// [`Route::external()`] explícitamente cuando quieras dejar clara la intención sin ambigüedad, o
/// para esquemas que la heurística aplicada no reconozca.
///
/// # Ejemplos
///
/// El caso más común es una ruta relativa dependiente del contexto, normalmente usando
/// [`Context::route()`] para que el enlace preserve el parámetro `lang` cuando corresponda:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let route = Route::with(|cx| cx.route("/path/to/page"));
/// ```
///
/// Un literal o un `String` se convierten directamente, y también pasan por [`Context::route()`]:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let route: Route = "/path/to/page".into();
/// ```
///
/// Una ruta dinámica puede capturar valores del entorno, algo que un simple puntero a función no
/// permite:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let user_id = 42;
/// let route = Route::with(move |cx| cx.route(format!("/users/{user_id}")));
/// ```
///
/// Si ya tienes un [`RoutePath`] resuelto -- por ejemplo, combinando
/// [`Context::route()`] con [`Waypoint::append_to()`](crate::response::Waypoint::append_to) --
/// se convierte directamente, sin volver a procesarlo:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # let cx = Context::new(None);
/// # let waypoint = Waypoint::default();
/// let route: Route = waypoint.append_to(cx.route("/items")).into();
/// ```
///
/// Una URL externa fija, que no debe llevar `lang`, se detecta automáticamente por su prefijo al
/// convertir un literal, y se comporta como [`Route::external()`]:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let route: Route = "https://www.example.com".into();
/// ```
///
/// [`Route::external()`] sigue siendo útil para dejar la intención explícita, o para esquemas que
/// la heurística no reconozca (por ejemplo una URL construida dinámicamente que no empieza
/// literalmente por uno de los prefijos detectados):
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let route = Route::external("ftp://files.example.com");
/// ```
#[derive(Clone)]
pub struct Route(Arc<dyn Fn(&Context) -> RoutePath + Send + Sync>);

impl Route {
    /// Crea una `Route` a partir de un closure que resuelve la ruta según el contexto.
    pub fn with<F>(f: F) -> Self
    where
        F: Fn(&Context) -> RoutePath + Send + Sync + 'static,
    {
        Route(Arc::new(f))
    }

    /// Crea una `Route` para una URL externa fija.
    ///
    /// Una URL externa nunca debe llevar el parámetro `lang` ya que no pertenece al espacio de
    /// rutas de la aplicación. Un literal o un `String` con pinta de URL externa (ver "Detección
    /// automática de URLs externas" más arriba) ya se comportan así al convertirse a `Route`; usa
    /// `external()` explícitamente para dejar la intención clara sin ambigüedad, o cuando el
    /// heurístico no reconozca el esquema.
    pub fn external(url: impl Into<RoutePath>) -> Self {
        url.into().into()
    }

    /// Invoca el closure interno para obtener la [`RoutePath`] final.
    pub fn resolve(&self, cx: &Context) -> RoutePath {
        (self.0)(cx)
    }

    /// Como [`resolve()`](Self::resolve), pero devuelve `None` cuando la ruta calculada está vacía.
    ///
    /// Útil para atributos HTML opcionales (por ejemplo `href`) que no deben renderizarse si la
    /// ruta resultante no tiene contenido.
    pub fn try_resolve(&self, cx: &Context) -> Option<RoutePath> {
        let route = self.resolve(cx);
        (!route.is_empty()).then_some(route)
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Route")
            .field(&"Fn(&Context) -> RoutePath")
            .finish()
    }
}

impl Default for Route {
    fn default() -> Self {
        Route::with(|_| RoutePath::default())
    }
}

/// Envuelve un [`RoutePath`] que ya tienes construido, sin volver a procesarlo.
///
/// Es el caso de un `RoutePath` obtenido con [`Context::route()`], solo o combinado con
/// [`Waypoint::append_to()`](crate::response::Waypoint::append_to) o
/// [`Waypoint::or()`](crate::response::Waypoint::or): el idioma (`lang`) ya se aplicó ahí, y esta
/// conversión evita volver a aplicarlo.
///
/// Ojo: esta conversión asume que el `RoutePath` ya pasó por `Context::route()`. Si construyes uno
/// a mano (por ejemplo con `RoutePath::new(...)` directamente, sin pasar por `cx.route()`), esta
/// conversión no añadirá `lang` por ti -- simplemente envuelve el valor tal cual, igual que
/// [`Route::external()`].
impl From<RoutePath> for Route {
    fn from(path: RoutePath) -> Self {
        Route::with(move |_| path.clone())
    }
}

impl From<&'static str> for Route {
    fn from(path: &'static str) -> Self {
        if util::url_looks_external(path) {
            Route::external(path)
        } else {
            Route::with(move |cx| cx.route(path))
        }
    }
}

impl From<String> for Route {
    fn from(path: String) -> Self {
        if util::url_looks_external(&path) {
            Route::external(path)
        } else {
            Route::with(move |cx| cx.route(path.clone()))
        }
    }
}
