//! Servidor web y rutas de la aplicación (basado en [Axum](https://docs.rs/axum)).
//!
//! Define rutas y handlers: el [`Router`], las operaciones HTTP ([`get`], [`post`], [`put`],
//! [`delete`], [`patch`]), los extractores ([`Path`], [`Query`], [`Form`], [`RawForm`],
//! [`Multipart`], [`Request`], [`ConnectInfo`]) e [`IntoResponse`], el módulo [`middleware`] para
//! *middlewares* de función, y re-exporta el módulo `http` para tipos de bajo nivel como
//! `StatusCode`, `HeaderName` o `Method`. También ofrece utilidades para servir archivos estáticos,
//! [`ServeDir`] y [`ServeEmbedded`], para recibir y guardar los archivos subidos en formularios,
//! [`Upload`], y para servirlos después con control de acceso, [`Download`].
//!
//! Los handlers son las funciones asíncronas que el servidor invoca al recibir una petición HTTP en
//! una ruta concreta.
//!
//! # Protección frente a CSRF
//!
//! Todas las rutas están protegidas frente a CSRF (*Cross-Site Request Forgery*), sin *tokens* en
//! los formularios: una petición que puede cambiar estado (cualquier método salvo `GET`, `HEAD` y
//! `OPTIONS`) se rechaza con `403 Forbidden` si el navegador indica que llega desde otro origen
//! (cabeceras `Sec-Fetch-Site` y `Origin`), incluido otro subdominio del mismo sitio. Funciona
//! igual con formularios, HTMX o `fetch`, y no afecta a las peticiones que no hace un navegador
//! (`curl`, otros servidores...), que no llevan esas cabeceras.
//!
//! Dos consecuencias para las aplicaciones:
//!
//! - **Un handler `GET` nunca debe cambiar estado** (borrar, cerrar sesión, confirmar...): esas
//!   acciones van siempre con `POST` u otro método no seguro.
//! - Si otra aplicación propia, servida desde otro origen, envía peticiones a ésta, su origen debe
//!   añadirse a [`trusted_origins`](crate::global::ServerSettings::trusted_origins) de `[server]`.
//!
//! Detrás de un *proxy* inverso, éste debe conservar la cabecera `Host` original (p. ej.
//! `proxy_set_header Host $host;` en nginx): los navegadores que no envían `Sec-Fetch-Site` (o las
//! conexiones HTTP sin cifrar fuera de `localhost`) se comprueban comparando `Origin` con `Host`.
//!
//! Ninguna protección frente a CSRF resiste un fallo de XSS en la propia aplicación.
//!
//! # *Cookies*
//!
//! PageTop sólo crea dos *cookies*, ambas propias (de la aplicación, no de terceros) y únicamente
//! cuando las provoca una acción del usuario:
//!
//! | *Cookie*         | Para qué                                                       | Duración                                                        |
//! |------------------|----------------------------------------------------------------|-----------------------------------------------------------------|
//! | `pagetop_flash`  | Mensajes para la página siguiente ([`ResponseExt`])            | Un minuto como máximo; se borra al mostrar los mensajes         |
//! | `pagetop_list_*` | Filtros, orden y página de un listado ([`remember_query()`])   | La sesión del navegador; se borra al limpiar los filtros        |
//!
//! Las extensiones pueden añadir las suyas; `pagetop-user`, por ejemplo, la de la sesión iniciada.
//!
//! Las de PageTop y sus extensiones propias se crean con [`cookie_header()`], que les da los
//! mismos atributos: `HttpOnly`, `SameSite=Lax` y, si la aplicación se sirve por HTTPS y así se
//! indica en [`secure_cookies`](crate::global::ServerSettings::secure_cookies) de `[server]`,
//! `Secure`.
//!
//! ## Sin aceptación de *cookies*
//!
//! PageTop y sus extensiones propias (las que se publican con él, como `pagetop-user`) están
//! pensados para que una aplicación no tenga que pedir la aceptación de *cookies* sólo por usarlos.
//! Para ello, todas las *cookies* que crean cumplen los criterios que las principales normas de
//! privacidad del mundo tratan como necesarias para el servicio:
//!
//! - Son propias de la aplicación; ninguna es de terceros.
//! - Sólo existen para prestar algo que el usuario pide, y se crean por una acción suya.
//! - No identifican al usuario fuera de la aplicación ni sirven para seguirlo entre sitios. No se
//!   usan para analítica, publicidad ni perfiles, y no se venden ni se comparten.
//! - Duran lo imprescindible para su función.
//!
//! Las normas que exigen consentimiento previo (las de la Unión Europea y otras de inspiración
//! europea) eximen expresamente las *cookies* necesarias para el servicio que pide el usuario. Las
//! que sólo obligan a ofrecer al usuario la posibilidad de oponerse (las de varios estados de
//! Estados Unidos, por ejemplo) se dirigen a la venta o el intercambio de datos y a la publicidad
//! dirigida, que estas *cookies* no hacen. En unas y otras, lo que sí se exige casi siempre es
//! informar: describirlas en la política de *cookies* o de privacidad de la aplicación (nombre,
//! finalidad, duración y si son propias), y la tabla anterior sirve para ello.
//!
//! La única que no queda exenta por sí sola es la de la sesión de `pagetop-user` cuando el usuario
//! marca "Recuérdame", porque se conserva aunque cierre el navegador. Pero es él quien la pide, y
//! marcar la casilla sabiendo que usa una *cookie* es ya su consentimiento. Es el criterio para
//! cualquier función que necesite una *cookie* que el usuario deba aceptar: el consentimiento se
//! pide en el propio gesto que la activa, nunca con un aviso aparte.
//!
//! La exención depende del uso: si una aplicación aprovechara estas *cookies* para otra cosa, o
//! alargara su duración, dejaría de valer. Las que añada cada aplicación (analítica, publicidad...)
//! sí suelen necesitar consentimiento.
//!
//! ## Unión Europea y España
//!
//! Es el marco más exigente y el que se ha tomado como referencia. La norma europea es una
//! directiva: no se aplica directamente, sino que cada Estado miembro (y también Islandia,
//! Liechtenstein y Noruega, del Espacio Económico Europeo) la incorpora a su propia ley, y la
//! autoridad de protección de datos de cada país publica sus propias guías, de modo que los
//! detalles pueden variar de un país a otro. Aquí se analizan la ley española y la guía de la
//! autoridad española.
//!
//! Guardar o leer información en el dispositivo del usuario exige su consentimiento previo, salvo
//! cuando es estrictamente necesario para prestar un servicio que el propio usuario ha pedido
//! (artículo 5.3 de la Directiva 2002/58/CE, la «Directiva ePrivacy», modificada por la Directiva
//! 2009/136/CE; en España, artículo 22.2 de la Ley 34/2002, de servicios de la sociedad de la
//! información, LSSI). El dictamen 04/2012 del Grupo de Trabajo del Artículo 29 sobre la exención
//! del requisito de consentimiento de las *cookies* (WP194), criterio que mantiene el Comité
//! Europeo de Protección de Datos, y la *Guía sobre el uso de las cookies* de la Agencia Española
//! de Protección de Datos (AEPD) concretan esa excepción. Entre sus ejemplos de *cookies* exentas
//! están las que recuerdan lo que el usuario ha hecho durante la sesión y las de personalización de
//! la interfaz que él mismo elige.
//!
//! Las dos de PageTop encajan en esa excepción, así que **no requieren la aceptación del usuario**
//! ni un aviso de *cookies* que la pida:
//!
//! - `pagetop_flash` completa una acción que el usuario acaba de pedir (guardar un formulario,
//!   iniciar sesión...): lleva el aviso del resultado a la página siguiente y desaparece en cuanto
//!   se muestra.
//! - `pagetop_list_*` es una personalización de la interfaz que el usuario activa al filtrar,
//!   ordenar o paginar un listado, y dura lo que la sesión del navegador.
//!
//! El texto de un filtro puede contener datos personales (un nombre buscado), pero queda sólo en el
//! navegador de quien lo escribió. La AEPD recomienda describir también las *cookies* exentas en la
//! política de *cookies* de la aplicación.
//!
//! ## Otros países
//!
//! Fuera de la Unión Europea, los criterios anteriores se basan en lo que comparten las normas más
//! extendidas, no en un análisis país por país. Antes de desplegar en un país concreto conviene
//! contrastarlos con su normativa, que además cambia con frecuencia.
//!
//! Es el criterio de las normas y guías citadas, no un asesoramiento jurídico: en un despliegue con
//! exigencias legales concretas, el responsable del tratamiento debe confirmarlo.
//!
//! [`ResponseExt`]: crate::response::ResponseExt

use crate::{StaticFile, global};

#[doc(inline)]
pub use axum::http;

// Infraestructura del router.
pub use axum::Router;

// Extractores de petición.
pub use axum::extract::{ConnectInfo, Form, Multipart, Path, Query, RawForm, Request};

// Partes y errores del extractor `Multipart`.
pub use axum::extract::multipart;

// Para implementar respuestas.
pub use axum::response::{IntoResponse, Response};

// Operaciones HTTP para registrar rutas.
pub use axum::routing::{MethodRouter, delete, get, patch, post, put};

// Middlewares de función (`from_fn`) y tipos asociados (`Next`).
pub use axum::middleware;

// Guardado de los archivos subidos en formularios `multipart/form-data`.
mod upload;
pub(crate) use upload::push_percent_encoded;
pub use upload::{Upload, UploadError, UploadedFile, UploadedForm};

// Respuestas con los archivos subidos.
mod download;
pub use download::Download;

// Protección frente a CSRF, aplicada a todas las rutas por `Application`.
mod csrf;
pub(crate) use csrf::reject_cross_origin;

// Rutas de las extensiones con su acceso, capas globales y archivos estáticos.
mod routes;
pub use routes::{Layers, Routes, StaticFiles};

// Rutas declaradas junto con su acceso, para montarlas y enlazarlas sin repetirlo.
mod endpoint;
pub use endpoint::Endpoint;

// Cabecera `Set-Cookie` común a todas las *cookies* de PageTop y sus extensiones.
mod cookie;
pub use cookie::cookie_header;

// Estado de los listados (filtros, orden, página) recordado entre pantallas.
mod remember;
pub use remember::remember_query;

use axum::body::Body;
use axum::extract::FromRequestParts;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::task::{Context, Poll};

// **< HttpRequest >********************************************************************************

/// Representa una petición HTTP.
///
/// Almacena los datos necesarios para negociar el idioma y renderizar las páginas de error,
/// incluyendo la URI completa, las cabeceras de la petición original y las extensiones de tipo que
/// los *middlewares* pueden inyectar antes de que el handler se ejecute.
///
/// Puede declararse directamente como parámetro en un handler para pasarlo al
/// [`Context`](crate::core::component::Context) de renderizado y a las variantes de
/// [`ErrorPage`](crate::response::ErrorPage):
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     // ...
///     # todo!()
/// }
/// ```
///
/// Las extensiones inyectadas por middleware son accesibles vía [`extension`](Self::extension). Por
/// ejemplo, desde un handler, tras añadir `MyData` en un middleware:
///
/// ```rust,ignore
/// if let Some(data) = request.extension::<MyData>() {
///     // ...
/// }
/// ```
///
/// `HttpRequest` no consume las extensiones de la petición, por lo que el resto de extractores del
/// handler que también las necesiten (como `Path<T>`, `Extension<T>`, etc.) las siguen viendo
/// intactas, sea cual sea la posición en la que se declare `HttpRequest`. Por convención, se
/// declara como primer parámetro del handler. El único orden que sigue siendo obligatorio es el que
/// impone Axum: un extractor que consuma el cuerpo de la petición (`Form<T>`, `RawForm`, etc.) debe
/// ir siempre el último.
#[derive(Clone, Debug)]
pub struct HttpRequest(Arc<RequestParts>);

// Datos de la petición. Se guardan tras un `Arc` para que `HttpRequest` ocupe un puntero y clonarlo
// sea una simple cuenta de referencias: viaja dentro de `ErrorPage` y de los `Result<_, ErrorPage>`
// de los handlers, donde un tipo grande se copiaría en cada retorno.
#[derive(Debug)]
struct RequestParts {
    method: http::Method,
    uri: http::Uri,
    headers: http::HeaderMap,
    extensions: http::Extensions,
}

impl HttpRequest {
    /// Devuelve el método HTTP de la petición.
    pub fn method(&self) -> &http::Method {
        &self.0.method
    }

    /// Devuelve la URI completa de la petición, incluyendo la *query string* si la hay.
    pub fn uri(&self) -> &str {
        self.0
            .uri
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/")
    }

    /// Devuelve la ruta (*path*) de la petición, sin la *query string*.
    pub fn path(&self) -> &str {
        self.0.uri.path()
    }

    /// Devuelve la cadena de consulta (*query string*) de la petición, sin el carácter `?`.
    ///
    /// Devuelve una cadena vacía si la petición no tiene *query string*.
    pub fn query_string(&self) -> &str {
        self.0.uri.query().unwrap_or("")
    }

    /// Devuelve las cabeceras HTTP de la petición.
    pub fn headers(&self) -> &http::HeaderMap {
        &self.0.headers
    }

    /// Accede a un valor inyectado por middleware en las extensiones de la petición.
    ///
    /// Devuelve una referencia al tipo `T` si algún middleware lo insertó antes de que el handler
    /// recibiera la petición, o `None` en caso contrario.
    ///
    /// El tipo debe implementar `Send + Sync + 'static` (requisito de [`http::Extensions`]).
    pub fn extension<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.0.extensions.get::<T>()
    }

    /// Devuelve la dirección IP del cliente que hace la petición, si se conoce.
    ///
    /// Por defecto es la dirección desde la que llega la conexión. Detrás de un *proxy* inverso o
    /// de un balanceador, esa dirección es siempre la del *proxy*: entonces hay que indicar en
    /// [`client_ip_header`](crate::global::ServerSettings::client_ip_header) de `[server]` la
    /// cabecera en la que el *proxy* escribe la dirección del cliente.
    ///
    /// Las direcciones IPv4 se devuelven siempre como tales, también las que llegan en forma IPv6
    /// (`::ffff:192.0.2.1`), como ocurre al escuchar en `"::"`: así una misma dirección es siempre
    /// el mismo valor.
    ///
    /// Devuelve `None` si la cabecera configurada falta o no contiene una dirección válida (nunca
    /// la de la conexión, que sería la del *proxy*), y en las pruebas, que no abren conexiones
    /// (salvo que la petición incluya [`ConnectInfo`]).
    pub fn client_ip(&self) -> Option<IpAddr> {
        let header = global::SETTINGS.server.client_ip_header.as_str();
        let ip = if header.is_empty() {
            self.extension::<ConnectInfo<SocketAddr>>()
                .map(|ConnectInfo(addr)| addr.ip())
        } else {
            ip_from_header(&self.0.headers, header)
        };
        ip.map(|ip| ip.to_canonical())
    }
}

// Dirección IP de la cabecera `name`. Si la cabecera se repite o tiene varias direcciones separadas
// por comas (como `X-Forwarded-For`), vale la última: la que añadió el último *proxy*, el de la
// propia aplicación. Las anteriores las envía el cliente y puede inventarlas.
fn ip_from_header(headers: &http::HeaderMap, name: &str) -> Option<IpAddr> {
    let value = headers.get_all(name).iter().next_back()?.to_str().ok()?;
    let last = value.rsplit(',').next()?.trim();
    last.parse().ok().or_else(|| {
        // Una dirección IPv4 o IPv6 con puerto (`203.0.113.7:4711`, `[2001:db8::1]:4711`).
        last.parse::<SocketAddr>().ok().map(|addr| addr.ip())
    })
}

impl<S: Send + Sync> FromRequestParts<S> for HttpRequest {
    type Rejection = Infallible;

    // Clona (no toma) las extensiones inyectadas por middleware, para que otros extractores del
    // handler (`Path<T>`, `Extension<T>`, etc.) las sigan viendo intactas sin importar en qué
    // posición se declare `HttpRequest`. Todo se guarda tras un `Arc` compartido para que
    // `HttpRequest` sea `Clone` a coste mínimo en el resto de su ciclo de vida.
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(HttpRequest::from_parts(parts))
    }
}

impl HttpRequest {
    // También la usa el *middleware* que comprueba el acceso de las rutas, antes del *handler*.
    pub(crate) fn from_parts(parts: &http::request::Parts) -> Self {
        HttpRequest(Arc::new(RequestParts {
            method: parts.method.clone(),
            uri: parts.uri.clone(),
            headers: parts.headers.clone(),
            extensions: parts.extensions.clone(),
        }))
    }
}

// **< ServeDir >***********************************************************************************

// Servicio para archivos estáticos en disco.
pub use tower_http::services::ServeDir;

// Valor de `Cache-Control` para los archivos estáticos: el navegador puede guardarlos, pero debe
// revalidarlos antes de reutilizarlos. Las versiones de las URL (`?v=...`) no siguen el contenido
// de cada archivo, así que una caché con caducidad podría servir recursos obsoletos tras desplegar.
const STATIC_CACHE_CONTROL: http::HeaderValue = http::HeaderValue::from_static("no-cache");

/// Servicio para archivos estáticos en disco con la política de caché de PageTop.
///
/// Lo usa la macro [`serve_static_files!`](crate::serve_static_files). [`ServeDir`] ya envía
/// `Last-Modified` y `ETag`, y responde `304 Not Modified` cuando corresponde; este servicio añade
/// `Cache-Control: no-cache` para que el navegador revalide los archivos en lugar de reutilizarlos
/// sin preguntar.
#[doc(hidden)]
pub fn serve_dir(
    dir: impl AsRef<std::path::Path>,
) -> tower_http::set_header::SetResponseHeader<ServeDir, http::HeaderValue> {
    tower_http::set_header::SetResponseHeader::if_not_present(
        ServeDir::new(dir),
        http::header::CACHE_CONTROL,
        STATIC_CACHE_CONTROL,
    )
}

// **< ServeEmbedded >******************************************************************************

/// Servicio para archivos estáticos embebidos en el binario.
///
/// Creado por la macro [`serve_static_files!`](crate::serve_static_files) en los modos que incluyen
/// recursos embebidos. Estos recursos se identifican por su ruta relativa sin la barra inicial
/// (p. ej. `"css/style.css"`). Si se solicita la raíz o una ruta que termina en `/`, el servicio
/// devuelve el `index.html` raíz si existe; no busca por subdirectorio.
///
/// Cada respuesta incluye un `ETag` calculado sobre el contenido del recurso y
/// `Cache-Control: no-cache`, de modo que el navegador guarda el recurso pero lo revalida antes de
/// reutilizarlo; si no ha cambiado, el servicio responde `304 Not Modified` sin cuerpo.
///
/// Implementa [`Clone`] para clonar el servicio por petición, pero internamente comparte el mapa de
/// recursos con un [`Arc`] para evitar copias innecesarias.
#[derive(Clone)]
pub struct ServeEmbedded {
    files: Arc<HashMap<&'static str, EmbeddedFile>>,
}

// Recurso embebido con su `ETag`, calculado una sola vez al crear el servicio.
struct EmbeddedFile {
    data: &'static [u8],
    mime_type: &'static str,
    etag: http::HeaderValue,
}

impl ServeEmbedded {
    /// Crea un nuevo servicio a partir del mapa de recursos embebidos generado por `build.rs`.
    pub fn new(files: HashMap<&'static str, StaticFile>) -> Self {
        let files = files
            .into_iter()
            .map(|(path, file)| {
                let embedded = EmbeddedFile {
                    data: file.data,
                    mime_type: file.mime_type,
                    etag: etag_for(file.data),
                };
                (path, embedded)
            })
            .collect();
        Self {
            files: Arc::new(files),
        }
    }
}

// `ETag` de un recurso a partir de su contenido. `DefaultHasher::new()` usa siempre las mismas
// claves, así que el valor sólo cambia si cambia el contenido (o el algoritmo, al actualizar Rust,
// lo que únicamente obliga a descargar de nuevo los recursos).
fn etag_for(data: &[u8]) -> http::HeaderValue {
    use std::hash::{DefaultHasher, Hasher};
    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    http::HeaderValue::try_from(format!("\"{:016x}\"", hasher.finish()))
        .expect("a quoted hexadecimal string is always a valid header value")
}

// Si alguna etiqueta de `If-None-Match` coincide con `etag`. La comparación es débil, como exige
// RFC 9110 para `If-None-Match`: se ignora el prefijo `W/`.
fn matches_etag(req: &http::Request<Body>, etag: &http::HeaderValue) -> bool {
    req.headers()
        .get_all(http::header::IF_NONE_MATCH)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|tags| tags.split(','))
        .map(str::trim)
        .any(|tag| {
            tag == "*" || tag.strip_prefix("W/").unwrap_or(tag).as_bytes() == etag.as_bytes()
        })
}

impl tower::Service<http::Request<Body>> for ServeEmbedded {
    type Response = http::Response<Body>;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        use axum::http::header;

        // Axum elimina el prefijo de montaje: la ruta restante puede o no comenzar con '/'.
        let path = req.uri().path().trim_start_matches('/');

        // Busca la ruta exacta; si es raíz o directorio, intenta index.html.
        let resource = self.files.get(path).or_else(|| {
            if path.is_empty() || path.ends_with('/') {
                self.files.get("index.html")
            } else {
                None
            }
        });

        let response = match resource {
            Some(file) if matches_etag(&req, &file.etag) => http::Response::builder()
                .status(http::StatusCode::NOT_MODIFIED)
                .header(header::ETAG, file.etag.clone())
                .header(header::CACHE_CONTROL, STATIC_CACHE_CONTROL)
                .body(Body::empty())
                .unwrap(),
            Some(file) => http::Response::builder()
                .header(header::CONTENT_TYPE, file.mime_type)
                .header(header::ETAG, file.etag.clone())
                .header(header::CACHE_CONTROL, STATIC_CACHE_CONTROL)
                .body(Body::from(file.data))
                .unwrap(),
            None => http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
        };

        std::future::ready(Ok(response))
    }
}

// **< serve_static_files! >************************************************************************

/// Configura el servidor web para publicar archivos estáticos.
///
/// Se usa normalmente en el método [`configure_static_files()`] de las extensiones, para que estas
/// rutas no pasen por su middleware.
///
/// La macro añade los archivos a los [`StaticFiles`] del primer argumento, siempre públicos, usando
/// uno de los tres modos posibles:
///
/// - **Sistema de ficheros o embebido** (`[$dir, $bundle]`): intenta servir los archivos desde el
///   directorio `$dir`; si está vacío, no existe o no es un directorio, usa el conjunto de recursos
///   `$bundle` embebido.
/// - **Sólo embebido** (`[$bundle]`): sirve siempre desde el conjunto de recursos embebido en el
///   binario.
/// - **Sólo sistema de ficheros** (`$dir`): sin corchetes, sirve únicamente desde el directorio si
///   existe.
///
/// # Argumentos
///
/// * `$router` - Variable de tipo [`StaticFiles`] donde registrar los archivos.
/// * `$dir`    - Ruta al directorio local con los archivos estáticos.
/// * `$bundle` - Nombre del conjunto de recursos embebidos generado por `build.rs`.
/// * `$path`   - Prefijo URL bajo el que se publicarán los archivos.
///
/// # Ejemplos
///
/// ```rust,ignore
/// # use pagetop::prelude::*;
/// pub struct MyExtension;
///
/// #[async_trait]
/// impl Extension for MyExtension {
///     fn configure_static_files(&self, files: StaticFiles) -> StaticFiles {
///         // Forma 1) Sistema de ficheros o embebido.
///         serve_static_files!(files, ["/var/www/static", assets] => "/public");
///
///         // Forma 2) Siempre embebido.
///         serve_static_files!(files, [assets] => "/public");
///
///         // Forma 3) Sólo sistema de ficheros (no requiere `assets`).
///         serve_static_files!(files, "/var/www/static" => "/public");
///
///         files
///     }
/// }
/// ```
///
/// [`configure_static_files()`]: crate::core::extension::Extension::configure_static_files
#[macro_export]
macro_rules! serve_static_files {
    // Forma 1: primero intenta servir desde el sistema de ficheros; si falla, sirve embebido.
    ( $router:ident, [$dir:expr, $bundle:ident] => $path:expr $(,)? ) => {
        let $router = {
            let _span = $crate::trace::debug_span!(
                "serve_static_files",
                mode = "filesystem_or_embedded",
                route = $path,
            )
            .entered();
            let mut __r = $router;
            let mut served_from_fs = false;
            if !::std::path::Path::new(&$dir).as_os_str().is_empty() {
                if let Ok(absolute) = $crate::util::resolve_absolute_dir($dir) {
                    __r = __r.nest_service($path, $crate::web::serve_dir(absolute));
                    served_from_fs = true;
                }
            }
            if !served_from_fs {
                $crate::util::paste! {
                    mod [<static_files_ $bundle>] {
                        include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                    }
                    __r = __r.nest_service(
                        $path,
                        $crate::web::ServeEmbedded::new(
                            [<static_files_ $bundle>]::$bundle(),
                        ),
                    );
                }
            }
            __r
        };
    };
    // Forma 2: sirve siempre embebido.
    ( $router:ident, [$bundle:ident] => $path:expr $(,)? ) => {
        let $router = {
            let _span = $crate::trace::debug_span!(
                "serve_static_files",
                mode = "embedded_only",
                route = $path,
            )
            .entered();
            $crate::util::paste! {
                mod [<static_files_ $bundle>] {
                    include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                }
                $router.nest_service(
                    $path,
                    $crate::web::ServeEmbedded::new(
                        [<static_files_ $bundle>]::$bundle(),
                    ),
                )
            }
        };
    };
    // Forma 3: intenta servir desde el sistema de ficheros.
    ( $router:ident, $dir:expr => $path:expr $(,)? ) => {
        let $router = {
            let _span = $crate::trace::debug_span!(
                "serve_static_files",
                mode = "filesystem_only",
                route = $path,
            )
            .entered();
            match $crate::util::resolve_absolute_dir($dir) {
                Ok(absolute) => $router.nest_service($path, $crate::web::serve_dir(absolute)),
                Err(e) => {
                    $crate::trace::warn!(
                        "Static dir not found or invalid for route `{}`: {} ({e})",
                        $path,
                        $dir,
                    );
                    $router
                }
            }
        };
    };
}

// **< Utilidades de test >*************************************************************************

/// Utilidades para escribir pruebas de integración con PageTop sobre Axum.
#[doc(hidden)]
pub mod test {
    use axum::Router;
    use axum::body::Body;
    use axum::http;
    use tower::ServiceExt;

    /// Devuelve el router tal como se recibe, listo para usarse en pruebas de integración.
    pub fn init_router(router: Router) -> Router {
        router
    }

    /// Constructor de peticiones HTTP para pruebas.
    pub struct TestRequest {
        method: http::Method,
        uri: String,
        headers: http::HeaderMap,
        extensions: http::Extensions,
        body: String,
    }

    impl TestRequest {
        /// Crea una petición GET.
        pub fn get() -> Self {
            Self::with_method(http::Method::GET)
        }

        /// Crea una petición POST.
        pub fn post() -> Self {
            Self::with_method(http::Method::POST)
        }

        /// Crea una petición PUT.
        pub fn put() -> Self {
            Self::with_method(http::Method::PUT)
        }

        /// Crea una petición PATCH.
        pub fn patch() -> Self {
            Self::with_method(http::Method::PATCH)
        }

        /// Crea una petición DELETE.
        pub fn delete() -> Self {
            Self::with_method(http::Method::DELETE)
        }

        fn with_method(method: http::Method) -> Self {
            Self {
                method,
                uri: "/".to_owned(),
                headers: http::HeaderMap::new(),
                extensions: http::Extensions::new(),
                body: String::new(),
            }
        }

        /// Establece la URI de la petición.
        pub fn uri(mut self, uri: impl Into<String>) -> Self {
            self.uri = uri.into();
            self
        }

        /// Añade una cabecera a la petición.
        ///
        /// Falla si `name` o `value` no son válidos como cabecera HTTP, para que una prueba no pase
        /// sin la cabecera que cree enviar (compilando con *debug assertions*; sin ellas, se
        /// descarta).
        pub fn header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
            let (name, value) = (name.as_ref(), value.as_ref());
            let parsed = http::HeaderName::from_bytes(name.as_bytes()).ok();
            let parsed = parsed.zip(http::HeaderValue::from_str(value).ok());
            debug_assert!(parsed.is_some(), "invalid header `{name}: {value}`");
            if let Some((name, value)) = parsed {
                self.headers.insert(name, value);
            }
            self
        }

        /// Envía como cuerpo los campos de un formulario, ya codificados (p. ej.
        /// `"title=Plant+lettuces&done=on"`), con la cabecera
        /// `Content-Type: application/x-www-form-urlencoded`, como lo envía un navegador.
        pub fn form(mut self, fields: impl Into<String>) -> Self {
            self.body = fields.into();
            self.header("content-type", "application/x-www-form-urlencoded")
        }

        /// Envía como cuerpo de la petición el texto indicado, tal cual.
        ///
        /// Combinado con [`header()`](Self::header) para fijar `Content-Type`, permite probar
        /// cuerpos con otros formatos, como los formularios `multipart/form-data` con archivos.
        pub fn body(mut self, body: impl Into<String>) -> Self {
            self.body = body.into();
            self
        }

        /// Inserta un valor en las extensiones de la petición.
        ///
        /// Útil para simular lo que un middleware haría en producción antes de que el handler
        /// reciba la petición. El tipo debe implementar `Clone + Send + Sync + 'static`.
        pub fn with_extension<T: Clone + Send + Sync + 'static>(mut self, value: T) -> Self {
            self.extensions.insert(value);
            self
        }

        /// Construye la petición HTTP de Axum (para enviar al router en tests de integración).
        pub fn to_request(self) -> http::Request<Body> {
            let mut req = http::Request::builder()
                .method(self.method)
                .uri(self.uri)
                .body(Body::from(self.body))
                .unwrap();
            *req.headers_mut() = self.headers;
            *req.extensions_mut() = self.extensions;
            req
        }

        /// Construye un [`HttpRequest`](super::HttpRequest) listo para pasarlo a
        /// [`Context::new`](crate::core::component::Context::new) en tests unitarios de
        /// componentes.
        pub fn to_http_request(self) -> super::HttpRequest {
            let uri = self.uri.parse().unwrap();
            super::HttpRequest(std::sync::Arc::new(super::RequestParts {
                method: self.method,
                uri,
                headers: self.headers,
                extensions: self.extensions,
            }))
        }
    }

    /// Envía una petición al router y devuelve la respuesta.
    pub async fn send_request(router: &Router, req: http::Request<Body>) -> http::Response<Body> {
        router.clone().oneshot(req).await.unwrap()
    }

    /// Devuelve el cuerpo de una respuesta como texto UTF-8, para comprobaciones en tests.
    pub async fn read_body_text(response: http::Response<Body>) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(values: &[&str]) -> http::HeaderMap {
        let mut headers = http::HeaderMap::new();
        for value in values {
            headers.append("x-forwarded-for", value.parse().unwrap());
        }
        headers
    }

    fn ip(value: &str) -> Option<IpAddr> {
        Some(value.parse().unwrap())
    }

    #[test]
    fn the_last_address_of_the_header_wins() {
        let from = |values| ip_from_header(&headers(values), "X-Forwarded-For");
        assert_eq!(from(&["203.0.113.7"]), ip("203.0.113.7"));
        // Earlier addresses are sent by the client and may be forged.
        assert_eq!(from(&["10.0.0.1, 203.0.113.7"]), ip("203.0.113.7"));
        assert_eq!(from(&["10.0.0.1", "203.0.113.7"]), ip("203.0.113.7"));
        assert_eq!(from(&["2001:db8::1"]), ip("2001:db8::1"));
        assert_eq!(from(&["203.0.113.7:4711"]), ip("203.0.113.7"));
        assert_eq!(from(&["[2001:db8::1]:4711"]), ip("2001:db8::1"));
    }

    #[test]
    fn a_missing_or_invalid_header_gives_no_address() {
        assert_eq!(ip_from_header(&headers(&[]), "X-Forwarded-For"), None);
        assert_eq!(
            ip_from_header(&headers(&["203.0.113.7, unknown"]), "X-Forwarded-For"),
            None
        );
    }

    #[test]
    fn without_a_header_the_connection_address_is_used() {
        let request = test::TestRequest::get()
            .with_extension(ConnectInfo(SocketAddr::from(([198, 51, 100, 2], 4711))))
            .to_http_request();
        assert_eq!(request.client_ip(), ip("198.51.100.2"));
        assert_eq!(test::TestRequest::get().to_http_request().client_ip(), None);
    }

    #[test]
    fn ipv4_addresses_seen_through_ipv6_are_returned_as_ipv4() {
        // A dual-stack socket ("::") sees IPv4 clients as IPv4-mapped IPv6 addresses.
        let mapped: SocketAddr = "[::ffff:198.51.100.2]:4711".parse().unwrap();
        let request = test::TestRequest::get()
            .with_extension(ConnectInfo(mapped))
            .to_http_request();
        assert_eq!(request.client_ip(), ip("198.51.100.2"));
        let request = test::TestRequest::get()
            .with_extension(ConnectInfo(SocketAddr::from((
                [0x2001, 0xdb8, 0, 0, 0, 0, 0, 1],
                80,
            ))))
            .to_http_request();
        assert_eq!(request.client_ip(), ip("2001:db8::1"));
    }
}
