//! Soporte HTMX al componente [`form::select::Field`] para desplegables en cascada.
//!
//! En una cadena de desplegables, cada uno ofrece sólo lo que corresponde a lo elegido en el
//! anterior. Es el caso de los datos jerárquicos (país, provincia y municipio; categoría y
//! subcategoría; centro, ubicación y puesto...). Al elegir un valor en un desplegable, se piden al
//! servidor las opciones del siguiente, y los que van detrás de éste vuelven a su estado inicial
//! (normalmente una opción que pide elegir primero el anterior). Las etiquetas, las ayudas y si el
//! campo es obligatorio no cambian: sólo se sustituyen las opciones.
//!
//! Son tres piezas:
//!
//! - [`cascade()`] prepara los desplegables de la página, ya construidos como cualquier
//!   [`form::select::Field`].
//! - [`Cascade`] recoge en el servidor lo que envían: qué desplegable cambió, su valor y cuáles van
//!   detrás.
//! - [`Options`] responde con las opciones nuevas de cada desplegable que haya que actualizar.
//!
//! La aplicación decide qué opciones corresponden a cada valor y monta la ruta que las sirve, con
//! el acceso que convenga. Sin JavaScript, los desplegables se pintan igual pero no se actualizan:
//! quien recibe el formulario debe comprobar siempre que lo elegido es coherente, porque el
//! navegador puede enviar cualquier valor.
//!
//! # Ejemplo
//!
//! País y provincia en un formulario de alta:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_htmx::prelude::*;
//!
//! // Los desplegables de la página. La provincia espera a que se elija un país.
//! fn region_fields() -> Vec<form::select::Field> {
//!     let country = form::select::Field::new()
//!         .with_name("country")
//!         .with_label(Lc::n("Country"))
//!         .with_item(form::select::Item::new("", Lc::n("-")))
//!         .with_item(form::select::Item::new("es", Lc::n("Spain")))
//!         .with_item(form::select::Item::new("pt", Lc::n("Portugal")));
//!     let province = form::select::Field::new()
//!         .with_name("province")
//!         .with_label(Lc::n("Province"))
//!         .with_item(form::select::Item::new("", Lc::n("Choose a country first")));
//!     hx_select::cascade([country, province], "/regions")
//! }
//!
//! // GET /regions: las opciones del desplegable siguiente al que cambió.
//! async fn regions(
//!     request: HttpRequest,
//!     web::Query(cascade): web::Query<hx_select::Cascade>,
//! ) -> Response {
//!     let provinces: &[&str] = match cascade.value() {
//!         Some("es") => &["Almería", "Granada", "Sevilla"],
//!         Some("pt") => &["Lisboa", "Oporto"],
//!         _ => &[],
//!     };
//!     let items = if provinces.is_empty() {
//!         vec![form::select::Item::new("", Lc::n("Choose a country first"))]
//!     } else {
//!         std::iter::once(form::select::Item::new("", Lc::n("-")))
//!             .chain(provinces.iter().map(|p| form::select::Item::new(*p, Lc::n(*p))))
//!             .collect()
//!     };
//!     let cx = Context::new(request);
//!     hx_select::Options::new()
//!         .with_options(cascade.next().unwrap_or_default(), items)
//!         .response(&cx)
//!         .into_response()
//! }
//! ```
//!
//! Con más niveles, la respuesta añade con [`Options::with_options()`] el estado inicial de cada
//! desplegable de [`Cascade::rest()`].

use std::collections::HashMap;

use pagetop::prelude::*;
use serde::Deserialize;

use crate::hx;
use crate::response::HtmxResponse;

// Parámetro con los nombres de los desplegables desde el que cambia hasta el último, separados por
// comas. Lo añade `cascade()` en `hx-vals` y lo lee `Cascade`.
const CHAIN_PARAM: &str = "hx_cascade";

/// Prepara una cadena de desplegables para que cada uno actualice los que van detrás.
///
/// Recibe los desplegables en orden, ya construidos (etiqueta, ayuda, opciones, el valor elegido,
/// si son obligatorios...), y los devuelve con los atributos HTMX añadidos, listos para incluirlos
/// en un formulario o en un [`ListFilter`]. Al cambiar uno de ellos se pide `url` con su valor (con
/// su nombre como parámetro) y el [`Cascade`] que lo describe; la respuesta, construida con
/// [`Options`], sustituye las opciones de los desplegables siguientes. El último no pide nada.
///
/// `url` es la ruta ya resuelta que sirve las opciones (p. ej. `cx.route(...).to_string()`), y
/// puede llevar sus propios parámetros. Cada desplegable debe tener nombre, que es el que identifica
/// su `<select>` en la respuesta, y no debe tener un identificador propio (ver
/// [`form::select::Field::select_id()`]). Por la misma razón, una página no puede tener dos
/// desplegables con el mismo nombre.
///
/// Una cadena que no cumple esto (algún desplegable sin nombre, con identificador propio, con una
/// coma en el nombre o con el nombre de otro) provoca `panic!` en desarrollo. En producción se
/// devuelven los desplegables sin cambios, como desplegables normales, y se avisa en las trazas.
///
/// Dentro de un [`ListFilter`], los cambios no aplican el filtro ni cambian la dirección del
/// navegador: los atributos que el filtro pasa a sus campos se anulan aquí. Con
/// [`ListFilter::with_auto_submit()`], cambiar un desplegable aplica además el filtro, como
/// cualquier otro campo.
pub fn cascade(
    fields: impl IntoIterator<Item = form::select::Field>,
    url: impl Into<CowStr>,
) -> Vec<form::select::Field> {
    let url: CowStr = url.into();
    let fields: Vec<form::select::Field> = fields.into_iter().collect();
    let names: Vec<String> = fields
        .iter()
        .map(|field| field.name().as_deref().unwrap_or_default().to_owned())
        .collect();
    // Sin esto, la cascada pediría opciones y las perdería, o las daría a otro desplegable.
    if let Some(reason) = invalid_chain(&fields, &names) {
        if cfg!(debug_assertions) {
            panic!("invalid dropdown cascade: {reason}");
        }
        trace::warn!("Dropdown cascade left as plain dropdowns: {reason}");
        return fields;
    }
    let selects: Vec<Option<String>> = fields.iter().map(form::select::Field::select_id).collect();

    let last = fields.len().saturating_sub(1);
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            if i == last {
                return field;
            }
            let chain = serde_json::json!({ CHAIN_PARAM: names[i..].join(",") }).to_string();
            let mut field = field
                .with_prop(PropsOp::set(hx::GET, url.clone()))
                .with_prop(PropsOp::set(hx::TRIGGER, "change"))
                // El contenedor del campo envía el valor de su propio `<select>`.
                .with_prop(PropsOp::set(hx::INCLUDE, "find select"))
                .with_prop(PropsOp::set(hx::VALS, chain))
                // Las opciones llegan fuera de banda (`Options`), cada una a su desplegable.
                .with_prop(PropsOp::set(hx::SWAP, hx::swap::NONE))
                // Un `ListFilter` pasa a sus campos atributos que cambiarían la dirección, atenuarían
                // el listado o cancelarían sus peticiones.
                .with_prop(PropsOp::set(hx::PUSH_URL, "false"))
                .with_prop(PropsOp::set(hx::SYNC, "this:replace"));
            if let Some(next) = &selects[i + 1] {
                field = field.with_prop(PropsOp::set(hx::INDICATOR, id_selector(next)));
            }
            field
        })
        .collect()
}

// Motivo por el que la cadena no puede funcionar, si lo hay.
fn invalid_chain(fields: &[form::select::Field], names: &[String]) -> Option<String> {
    for (i, (field, name)) in fields.iter().zip(names).enumerate() {
        if name.is_empty() {
            return Some(format!("dropdown {} has no name", i + 1));
        }
        if name.contains(',') {
            return Some(format!("dropdown `{name}` has a comma in its name"));
        }
        if field.id().is_some() {
            return Some(format!("dropdown `{name}` has its own id"));
        }
        if names[..i].contains(name) {
            return Some(format!("dropdown name `{name}` is repeated"));
        }
    }
    None
}

// Selector CSS del identificador `id`. Un nombre puede llevar caracteres como `.`, `[` o `:`, que en
// un selector sin escapar apuntarían a otro elemento o lo harían inválido.
fn id_selector(id: &str) -> String {
    let mut selector = String::with_capacity(id.len() + 1);
    selector.push('#');
    for c in id.chars() {
        if c.is_ascii() && !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            selector.push('\\');
        }
        selector.push(c);
    }
    selector
}

/// Lo que envía un desplegable de una cadena preparada con [`cascade()`] al cambiar.
///
/// Se recibe con `web::Query<hx_select::Cascade>`. Da el nombre del desplegable que cambió, su valor y los nombres de los que
/// van detrás, en orden. Los demás parámetros de la ruta se leen con [`param()`](Self::param).
///
/// Los valores llegan del navegador: hay que comprobarlos como cualquier otro dato del usuario.
#[derive(AutoDefault, Clone, Debug, Deserialize)]
pub struct Cascade {
    #[serde(rename = "hx_cascade", default)]
    chain: String,
    #[serde(flatten)]
    params: HashMap<String, String>,
}

impl Cascade {
    /// Devuelve el nombre del desplegable que cambió.
    pub fn changed(&self) -> Option<&str> {
        self.names().next()
    }

    /// Devuelve el valor elegido en el desplegable que cambió (vacío si se eligió la opción sin
    /// valor).
    pub fn value(&self) -> Option<&str> {
        self.changed().and_then(|name| self.param(name))
    }

    /// Devuelve el nombre del desplegable siguiente, el que recibe las opciones nuevas.
    pub fn next(&self) -> Option<&str> {
        self.names().nth(1)
    }

    /// Devuelve los nombres de los desplegables que van detrás del siguiente, que deben volver a su
    /// estado inicial.
    pub fn rest(&self) -> impl Iterator<Item = &str> {
        self.names().skip(2)
    }

    /// Devuelve el valor de otro parámetro de la petición, como los que lleve la propia ruta.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(String::as_str)
    }

    fn names(&self) -> impl Iterator<Item = &str> {
        self.chain.split(',').filter(|name| !name.is_empty())
    }
}

/// Respuesta con las opciones nuevas de los desplegables de [`cascade()`].
///
/// Cada llamada a [`with_options()`](Self::with_options) sustituye todas las opciones de un
/// desplegable, identificado por su nombre: las del siguiente al que cambió y, normalmente, el
/// estado inicial de los que van detrás. Marca con [`form::select::Item::with_selected()`] la que
/// deba quedar elegida; sin ninguna, el navegador elige la primera.
#[derive(AutoDefault)]
pub struct Options {
    updates: Vec<(String, Vec<form::select::Item>)>,
}

impl Options {
    /// Crea una respuesta sin opciones.
    pub fn new() -> Self {
        Self::default()
    }

    /// Añade las opciones que sustituirán a las del desplegable `name`.
    pub fn with_options(
        mut self,
        name: impl AsRef<str>,
        items: impl IntoIterator<Item = form::select::Item>,
    ) -> Self {
        self.updates
            .push((name.as_ref().to_owned(), items.into_iter().collect()));
        self
    }

    /// Construye la respuesta HTMX, con los textos traducidos al idioma de `cx`.
    pub fn response(&self, cx: &Context) -> HtmxResponse {
        HtmxResponse::new(html! {
            @for (name, items) in &self.updates {
                @if let Some(id) = form::select::Field::new().with_name(name).select_id() {
                    // Las opciones se mueven una a una al desplegable de la página. Un `<select>`
                    // de selección única marcaría como elegida la siguiente cada vez que sale una,
                    // y la última llegaría elegida; uno múltiple las deja como están.
                    select id=(id) multiple hx-swap-oob="innerHTML" {
                        @for item in items {
                            option
                                value=(item.value().as_deref().unwrap_or(""))
                                selected[*item.selected()]
                                disabled[*item.disabled()]
                            {
                                (item.label().using(cx))
                            }
                        }
                    }
                }
            }
        })
    }
}
