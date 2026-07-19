//! Parámetro `waypoint`, URL de destino transportada entre pantallas.

use serde::Deserialize;

use crate::AutoDefault;
use crate::html::RoutePath;

/// URL de destino transportada por el parámetro `waypoint`.
///
/// Cualquier pantalla alcanzada desde un listado (alta, edición, confirmación, o una cadena de
/// varias de ellas) puede recibir en su *query string* un parámetro `waypoint` con la URL de la
/// página a la que ir para continuar.
///
/// `Waypoint` transporta ese valor a través de la cadena de pantallas. Se extrae de la petición
/// entrante con [`web::Query`](crate::web::Query), igual que cualquier otro parámetro, y se repone
/// en cada enlace o acción de formulario intermedia con [`append_to()`](Self::append_to) para que
/// sobreviva a la siguiente petición. Se resuelve con [`or()`](Self::or) al decidir el destino de
/// un enlace de vuelta o de una redirección.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// const ITEMS_PATH: &str = "/items";
///
/// /// GET /items/{id}/edit - Formulario de edición, alcanzado desde el listado.
/// async fn edit_get(
///     web::Path(id): web::Path<i32>,
///     web::Query(waypoint): web::Query<Waypoint>,
/// ) -> Markup {
///     // El listado de origen (si lo hay) viaja en la acción del formulario, para volver después.
///     let action = waypoint.append_to(format!("{ITEMS_PATH}/{id}/edit"));
///     html! { form action=(action) method="post" { /* ... */ } }
/// }
///
/// /// POST /items/{id}/edit - Guarda los cambios y vuelve al listado de origen, o a `ITEMS_PATH`.
/// async fn edit_post(
///     web::Path(id): web::Path<i32>,
///     web::Query(waypoint): web::Query<Waypoint>,
/// ) -> Response {
///     Redirect::see_other(waypoint.or(ITEMS_PATH))
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Deserialize)]
#[serde(from = "RawWaypoint")]
pub struct Waypoint {
    waypoint: Option<String>,
}

impl Waypoint {
    /// Crea un valor explícito.
    ///
    /// Sólo acepta una ruta local que empiece por `/`, pero no por `//` ni `/\` que el navegador
    /// interpretaría como una URL *protocol-relative*. Una cadena vacía, una URL absoluta
    /// (`https://...`), un esquema arbitrario (`javascript:...`) o cualquier otro valor que no
    /// cumpla esa forma se trata igual que `None`.
    ///
    /// Este filtro protege contra *open redirect*; el `waypoint` viaja en la *query string* de la
    /// petición, así que un cliente malicioso lo controla por completo. Por eso se aplica siempre,
    /// también al deserializar con [`web::Query`](crate::web::Query), el punto de entrada habitual
    /// donde el valor procede de fuera de la aplicación.
    pub fn new(waypoint: impl Into<Option<String>>) -> Self {
        Self {
            waypoint: waypoint.into().filter(|d| {
                // Sólo acepta `/algo`. Rechaza cadenas vacías, esquemas (`https:`, `javascript:`) y
                // las URLs *protocol-relative* (`//evil.example`, `/\evil.example`).
                let mut chars = d.chars();
                match chars.next() {
                    Some('/') => !matches!(chars.next(), Some('/') | Some('\\')),
                    _ => false,
                }
            }),
        }
    }

    /// Devuelve la URL de destino, si se proporcionó una y es una ruta local válida.
    pub fn as_str(&self) -> Option<&str> {
        self.waypoint.as_deref()
    }

    /// Añade `?waypoint=<url codificada>` a `route`, si hay una URL de destino propia. Si no,
    /// devuelve `route` sin modificar.
    ///
    /// Es la operación habitual para que un enlace o la acción de un formulario transporten el
    /// waypoint a la siguiente pantalla. Igual que [`or()`](Self::or), devuelve un [`RoutePath`]
    /// convertible directamente en [`Route`](crate::core::component::Route) sin volver a
    /// procesarse. A diferencia de [`or()`](Self::or), nunca sustituye `route`; úsalo para propagar
    /// el waypoint a un enlace intermedio, no para resolver un destino final.
    ///
    /// Aparece en dos momentos típicos: el `action` de un formulario, para que un envío no pierda
    /// el waypoint (ver el ejemplo del módulo), y el `href` de los enlaces que un listado genera
    /// hacia otras pantallas (ver el ejemplo).
    ///
    /// Preserva cualquier *query string* ya presente en `route` sin romperla (como `lang=...` si
    /// `route` se construyó con [`Context::route()`](crate::core::component::Context::route)).
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// const ITEMS_PATH: &str = "/items";
    ///
    /// fn edit_href(cx: &Context, waypoint: &Waypoint, item_id: i32) -> RoutePath {
    ///     waypoint.append_to(cx.route(format!("{ITEMS_PATH}/{item_id}/edit")))
    /// }
    /// ```
    pub fn append_to(&self, route: impl Into<RoutePath>) -> RoutePath {
        let mut route = route.into();
        if let Some(d) = self.as_str() {
            route.alter_param("waypoint", d);
        }
        route
    }

    /// Devuelve la URL de destino, o `fallback` si no se proporcionó ninguna.
    ///
    /// Es la operación habitual al decidir el destino de un enlace de vuelta o de una redirección:
    /// el waypoint transportado si lo hay, o la URL del listado por defecto si se llegó a esta
    /// pantalla sin pasar por ninguno. A diferencia de [`append_to()`](Self::append_to), sustituye
    /// por completo el destino. Úsalo cuando necesites un único `RoutePath` final, no para propagar
    /// el waypoint a otro enlace intermedio.
    ///
    /// Aparece en dos momentos típicos de una misma pantalla: el `href` del enlace "volver" al
    /// renderizarla (ver el ejemplo) y el destino de `Redirect::see_other(...)` tras guardar con
    /// éxito en un POST, como en el ejemplo del módulo.
    ///
    /// Devuelve un [`RoutePath`], no una cadena ya renderizada, para que el resultado pueda
    /// convertirse directamente en una [`Route`](crate::core::component::Route) (por ejemplo al
    /// pasarlo a [`Form::with_action()`](crate::base::component::Form::with_action)) sin volver a
    /// procesarse. `fallback` acepta cualquier tipo convertible a `RoutePath` (un literal, un
    /// `String`, o un `RoutePath` ya construido); preservando los parámetros que correspondan.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// const ITEMS_PATH: &str = "/items";
    ///
    /// fn back_href(cx: &Context, waypoint: &Waypoint) -> RoutePath {
    ///     waypoint.or(cx.route(ITEMS_PATH))
    /// }
    /// ```
    pub fn or(&self, fallback: impl Into<RoutePath>) -> RoutePath {
        match self.as_str() {
            Some(d) => RoutePath::new(d.to_owned()),
            None => fallback.into(),
        }
    }
}

impl From<Option<String>> for Waypoint {
    fn from(waypoint: Option<String>) -> Self {
        Self::new(waypoint)
    }
}

impl From<String> for Waypoint {
    fn from(waypoint: String) -> Self {
        Self::new(Some(waypoint))
    }
}

// Valor tal y como llega en la deserialización (p. ej. desde `web::Query`), antes de ser filtrado.
#[derive(Deserialize)]
struct RawWaypoint {
    #[serde(default)]
    waypoint: Option<String>,
}

impl From<RawWaypoint> for Waypoint {
    fn from(raw: RawWaypoint) -> Self {
        Self::new(raw.waypoint)
    }
}
