//! Identidad del usuario y sistema de autorización extensible.
//!
//! Define el tipo [`CurrentUser`] que PageTop inyecta en el [`Context`] con la información mínima
//! sobre el usuario que ejecuta la petición actual ([`HttpRequest`]).
//!
//! Las extensiones se conectan con este sistema mediante las acciones de [`action::auth`]: con
//! [`CheckPermission`] implementan su propio modelo de permisos. Define también [`Access`], el
//! acceso con el que se monta cada ruta: PageTop lo comprueba antes de llegar al *handler* y
//! responde a quien no lo cumple con [`deny_access()`], la página de acceso denegado (403). A un
//! usuario anónimo le ofrece en su lugar el inicio de sesión sólo si una extensión de autenticación
//! lo prepara con [`AnonymousDenied`]. Dentro del *handler*, [`has_permission()`] y
//! [`require_permission()`] sirven para lo que el acceso de la ruta no expresa: mostrar u ocultar
//! partes de la página, o exigir un permiso más concreto.
//!
//! Las extensiones declaran además sus permisos con la acción [`DeclarePermissions`], y PageTop los
//! reúne al arrancar en un catálogo ([`PermissionRegistry`]) para que una extensión de
//! autenticación pueda mostrarlos, por ejemplo, en una pantalla de asignación de permisos a roles.
//! El catálogo sólo dice qué permisos existen; a quién se conceden lo decide la extensión.
//!
//! La resolución concreta del usuario (sesión en BD, LDAP, OAuth, ...) y la lógica de permisos
//! (RBAC, grupos LDAP, ...) son responsabilidad de las extensiones de autenticación. Un concepto
//! como "administrador" que tiene todos los permisos no es responsabilidad de PageTop: cada
//! extensión decide si existe y, si es así, lo aplica dentro de su propio handler
//! [`CheckPermission`].
//!
//! [`Context`]: crate::core::component::Context
//! [`action::auth`]: crate::base::action::auth

use crate::base::action::auth::{AnonymousDenied, CheckPermission, DeclarePermissions};
use crate::core::component::Context;
use crate::core::theme::{ThemeRef, theme_by_short_name};
use crate::datetime::{Timezone, Tz};
use crate::locale::{LanguageIdentifier, Lc, Locale};
use crate::response::ErrorPage;
use crate::web::{HttpRequest, IntoResponse, Response};
use crate::{AutoDefault, CowStr, Getters, builder_impl, global, trace};

use std::sync::OnceLock;

// **< CurrentUser >********************************************************************************

/// Identidad mínima del usuario que ejecuta la petición actual.
///
/// Se almacena automáticamente en el [`Context`] a partir de la petición HTTP. La identidad se
/// extrae de las extensiones de la petición, que una extensión de autenticación inyecta mediante su
/// middleware. Sin extensión de autenticación, o si ésta no inyecta ninguna identidad, el usuario
/// es anónimo ([`CurrentUser::anonymous()`], que también es el valor por defecto).
///
/// Se accede usando [`Contextual::current_user()`].
///
/// Los usuarios pueden tener idioma, zona horaria y tema preferidos. Se asignan con su valor en
/// bruto y se validan al asignarlos: un idioma no soportado, una zona horaria desconocida o un tema
/// no habilitado en la aplicación se descartan y el dato queda sin valor, como si el usuario no
/// tuviera ninguno. Así, las preferencias de un `CurrentUser` son siempre válidas.
///
/// Los datos extendidos del usuario autenticado (roles, permisos, cuenta completa, ...) son
/// responsabilidad de la extensión de autenticación y se obtienen a través de
/// [`HttpRequest::extension`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let user = CurrentUser::authenticated(42, "Alice")
///     .with_language("es-ES")
///     .with_timezone("Europe/Madrid");
/// ```
///
/// [`Context`]: crate::core::component::Context
/// [`Contextual::current_user()`]: crate::core::component::Contextual::current_user
/// [`HttpRequest::extension`]: crate::web::HttpRequest::extension
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct CurrentUser {
    /// Devuelve el identificador del usuario, o `None` si es anónimo.
    #[getters(copy)]
    id: Option<i32>,
    // Siempre `Some` en un usuario autenticado y `None` en uno anónimo, igual que `id`.
    #[getters(skip)]
    display_name: Option<String>,
    /// Devuelve el idioma preferido del usuario, o `None` si no tiene ninguno.
    ///
    /// Lo tiene en cuenta [`RequestLocale`](crate::locale::RequestLocale) al decidir el idioma de
    /// la petición.
    #[getters(copy)]
    language: Option<&'static LanguageIdentifier>,
    // Ver `timezone()`, que devuelve la zona horaria efectiva.
    #[getters(skip)]
    timezone: Option<Tz>,
    /// Devuelve el tema preferido del usuario, o `None` si no tiene ninguno.
    ///
    /// Lo tiene en cuenta el [`Context`](crate::core::component::Context) de la petición al elegir
    /// el tema con el que se renderiza.
    #[getters(copy)]
    theme: Option<ThemeRef>,
}

#[builder_impl]
impl CurrentUser {
    /// Crea un usuario anónimo, sin idioma, zona horaria ni tema preferidos.
    pub fn anonymous() -> Self {
        Self::default()
    }

    /// Crea un usuario autenticado, sin idioma, zona horaria ni tema preferidos.
    pub fn authenticated(id: i32, display_name: impl Into<String>) -> Self {
        CurrentUser {
            id: Some(id),
            display_name: Some(display_name.into()),
            ..Self::default()
        }
    }

    // **< CurrentUser BUILDER >********************************************************************

    /// Asigna el idioma preferido a partir de su identificador (p. ej. `"es-ES"` o `"es"`).
    ///
    /// Se resuelve con [`Locale::resolve()`](crate::locale::Locale::resolve); si el idioma no está
    /// soportado por la aplicación, o es `None`, el usuario queda sin idioma preferido.
    pub fn with_language<'a>(mut self, language: impl Into<Option<&'a str>>) -> Self {
        self.language = language
            .into()
            .and_then(|language| Locale::resolve(language).as_option());
        self
    }

    /// Asigna la zona horaria a partir de su nombre IANA (p. ej. `"Europe/Madrid"`).
    ///
    /// Si el nombre no corresponde a ninguna zona horaria conocida, o es `None`, el usuario queda
    /// sin zona horaria propia.
    pub fn with_timezone<'a>(mut self, timezone: impl Into<Option<&'a str>>) -> Self {
        self.timezone = timezone.into().and_then(|timezone| timezone.parse().ok());
        self
    }

    /// Asigna el tema preferido a partir de su nombre corto (p. ej. `"basic"`).
    ///
    /// Se busca con [`theme_by_short_name()`](crate::core::theme::theme_by_short_name); si el tema
    /// no está habilitado en la aplicación, o es `None`, el usuario queda sin tema preferido.
    pub fn with_theme<'a>(mut self, theme: impl Into<Option<&'a str>>) -> Self {
        self.theme = theme.into().and_then(theme_by_short_name);
        self
    }

    // **< CurrentUser GETTERS >********************************************************************

    /// Devuelve `true` si el usuario no está autenticado.
    pub fn is_anonymous(&self) -> bool {
        self.id.is_none()
    }

    /// Devuelve `true` si el usuario está autenticado.
    pub fn is_authenticated(&self) -> bool {
        self.id.is_some()
    }

    /// Devuelve el nombre visible del usuario, o `None` si es anónimo.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Devuelve la zona horaria efectiva del usuario.
    ///
    /// Devuelve su zona horaria si tiene una y [`global::SETTINGS.app.timezone_per_user`] lo
    /// permite; en otro caso, devuelve [`Timezone::default_tz()`].
    ///
    /// Normalmente se resuelve una sola vez, al construir el [`Context`] de la petición. A partir
    /// de ese momento el renderizado del documento no vuelve a llamarlo porque usa el valor ya
    /// resuelto vía [`Contextual::timezone()`].
    ///
    /// [`global::SETTINGS.app.timezone_per_user`]: crate::global::AppSettings::timezone_per_user
    /// [`Context`]: crate::core::component::Context
    /// [`Contextual::timezone()`]: crate::core::component::Contextual::timezone
    pub fn timezone(&self) -> Tz {
        match self.timezone {
            Some(tz) if global::SETTINGS.app.timezone_per_user => tz,
            _ => Timezone::default_tz(),
        }
    }
}

// **< Permission >*********************************************************************************

/// Clave tipada de un permiso de acceso.
///
/// Cada extensión que lo requiera puede definir su propio enum de permisos e implementar este trait
/// para obtener la clave textual que finalmente se compara contra su modelo de permisos (RBAC en
/// base de datos, grupos LDAP, ...).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// #[derive(Clone, Copy, Debug)]
/// pub enum MyPermission {
///     EditPosts,
///     DeletePosts,
/// }
///
/// impl Permission for MyPermission {
///     fn key(&self) -> CowStr {
///         match self {
///             Self::EditPosts => "my_extension:edit_posts".into(),
///             Self::DeletePosts => "my_extension:delete_posts".into(),
///         }
///     }
/// }
/// ```
pub trait Permission: Send + Sync {
    /// Clave única del permiso. Por convención, el nombre de la extensión y el del permiso
    /// separados por `:` (p. ej. `"my_extension:edit_posts"`), para que no coincida con la de otra
    /// extensión.
    fn key(&self) -> CowStr;

    /// Descripción breve para humanos (p. ej. en una pantalla de asignación de permisos a roles).
    ///
    /// Por defecto devuelve la propia clave; una extensión que declare sus permisos en el catálogo
    /// ([`DeclarePermissions`]) debería sobrescribirlo con un texto traducible.
    fn label(&self) -> Lc {
        Lc::n(self.key())
    }

    /// Identificador estable de la categoría del permiso, usado para agrupar en un catálogo (p.
    /// ej. `"administration"`). Por defecto no pertenece a ningún grupo.
    fn group(&self) -> &'static str {
        ""
    }

    /// Título traducible de [`group()`](Self::group), mostrado en la UI de administración.
    ///
    /// Por defecto reutiliza el propio identificador del grupo como texto fijo.
    fn group_label(&self) -> Lc {
        Lc::n(self.group())
    }
}

/// Referencia estática a un permiso de acceso.
///
/// Es el tipo que recorre toda la API de autorización ([`has_permission()`],
/// [`require_permission()`] o [`CheckPermission`]).
pub type PermissionRef = &'static dyn Permission;

// **< has_permission >*****************************************************************************

/// Comprueba si el usuario actual tiene el permiso indicado.
///
/// Despacha la acción [`CheckPermission`]: cualquier extensión registrada puede conceder el permiso
/// asignando `granted = true` en su handler. Si no hay extensiones de autenticación activas,
/// devuelve `false` para cualquier usuario, incluido el anónimo.
///
/// La decisión de conceder o denegar permisos al usuario anónimo también es responsabilidad de cada
/// extensión.
///
/// Sirve para mostrar u ocultar lo que no se puede usar. Una ruta se protege con el acceso que se
/// indica al montarla ([`Access`]), y un enlace a ella se oculta solo si se declara con
/// [`Endpoint`](crate::web::Endpoint); para un permiso más concreto dentro del *handler* está
/// [`require_permission()`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(Clone, Copy, Debug)]
/// # enum MyPermission { Edit }
/// # impl Permission for MyPermission {
/// #     fn key(&self) -> CowStr { "myapp:edit".into() }
/// # }
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     let can_edit = has_permission(&request, &MyPermission::Edit);
///     let mut page = Page::new(request);
///     if can_edit {
///         page.push_message(MessageLevel::Info, Lc::n("You can edit this page."));
///     }
///     page.render().await
/// }
/// ```
pub fn has_permission(request: &HttpRequest, perm: PermissionRef) -> bool {
    CheckPermission::dispatch(request, perm)
}

// **< require_permission >*************************************************************************

/// Comprueba un permiso y devuelve `Err(ErrorPage::AccessDenied)` si se deniega.
///
/// Ejecuta [`has_permission()`] para detener un *handler* con una respuesta 403 en cuanto falta el
/// permiso, sin repetir el `if`/`return` en cada punto de comprobación. El acceso de la ruta
/// ([`Access`]) ya se ha comprobado antes de llegar al *handler*; esta función es para un permiso
/// más concreto que ése (p. ej. una acción que no todos los que ven la página pueden hacer). Se
/// llama directamente sobre la petición, antes de construir ni ejecutar nada (`Context`, `Page`,
/// consultas a datos, etc.), para no hacer ningún trabajo si la petición no está autorizada.
///
/// La denegación queda en las trazas igual que en [`deny_access()`].
///
/// Si la aplicación necesita ocultar la existencia del recurso a quien no tiene permiso (devolver
/// un 404 en vez de un 403), no se puede reutilizar esta función: hay que llamar a
/// `has_permission()` directamente, como en su propio ejemplo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(Clone, Copy, Debug)]
/// # enum MyPermission { Edit }
/// # impl Permission for MyPermission {
/// #     fn key(&self) -> CowStr { "myapp:edit".into() }
/// # }
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     // La ruta ya exige su acceso; este permiso es más concreto.
///     require_permission(&request, &MyPermission::Edit)?;
///
///     // Ejecuta las instrucciones propias de la petición.
///     Page::new(request)
///         .with_child(Html::with(|_| html! { p { "You have permission!" } }))
///         .render()
///         .await
/// }
/// ```
pub fn require_permission(request: &HttpRequest, perm: PermissionRef) -> Result<(), ErrorPage> {
    if has_permission(request, perm) {
        Ok(())
    } else {
        trace_denied(request);
        Err(ErrorPage::AccessDenied(Some(request.clone())))
    }
}

// **< Access >*************************************************************************************

/// Quién puede acceder a una ruta.
///
/// Toda ruta se monta con el suyo ([`Routes::route()`](crate::web::Routes::route)), y PageTop lo
/// comprueba antes de llegar al *handler*, que sólo comprueba permisos más concretos, si los hay
/// (p. ej. que el usuario sea el autor de lo que edita). A quien no tiene acceso se le responde
/// con [`deny_access()`].
///
/// El acceso con el que se monta una ruta no se puede consultar después. Para que sus enlaces lo
/// conozcan sin repetirlo, ruta y acceso se declaran juntos con [`Endpoint`](crate::web::Endpoint).
#[derive(Clone, Copy)]
pub enum Access {
    /// Cualquiera, también un usuario anónimo.
    ///
    /// Es el acceso de las páginas públicas y de las rutas que se protegen por otra vía, que su
    /// *handler* comprueba (p. ej. un enlace con un *token* de un solo uso).
    Public,
    /// Cualquier usuario autenticado, sin exigir ningún permiso.
    Authenticated,
    /// Quien tenga el permiso indicado (ver [`has_permission()`]).
    Permission(PermissionRef),
}

impl Access {
    /// Devuelve `true` si la petición cumple este acceso.
    pub fn allows(&self, request: &HttpRequest) -> bool {
        match self {
            Self::Public => true,
            Self::Authenticated => is_authenticated(request),
            Self::Permission(perm) => has_permission(request, *perm),
        }
    }
}

// Sin una extensión de autenticación, la petición no lleva `CurrentUser` y el usuario es anónimo.
fn is_authenticated(request: &HttpRequest) -> bool {
    request
        .extension::<CurrentUser>()
        .is_some_and(CurrentUser::is_authenticated)
}

// **< deny_access >********************************************************************************

/// Responde a una petición sin acceso a la ruta pedida.
///
/// A un usuario anónimo le responde la primera acción [`AnonymousDenied`] que lo haga (p. ej. con
/// el inicio de sesión); a uno autenticado, o si ninguna acción responde, la página de acceso
/// denegado (403). Es lo que hace PageTop con una ruta cuyo [`Access`] no se cumple, y lo puede
/// usar un *handler* que compruebe el acceso por su cuenta.
///
/// Deja constancia en las trazas con el método y la ruta: en el nivel *info*, con el identificador
/// del usuario, si está autenticado, y en *debug* si es anónimo.
pub fn deny_access(request: HttpRequest) -> Response {
    trace_denied(&request);
    if !is_authenticated(&request)
        && let Some(response) = AnonymousDenied::dispatch(&request)
    {
        return response;
    }
    ErrorPage::AccessDenied(Some(request)).into_response()
}

// Los anónimos van en `debug` porque bots y sesiones caducadas los hacen frecuentes y poco
// significativos. Sin la *query string*, donde pueden viajar datos personales.
fn trace_denied(request: &HttpRequest) {
    let (method, path) = (request.method(), request.path());
    match request.extension::<CurrentUser>().and_then(CurrentUser::id) {
        Some(id) => trace::info!("Access denied: {method} {path} (user {id})"),
        None => trace::debug!("Access denied: {method} {path} (anonymous)"),
    }
}

// **< PermissionRegistry >*************************************************************************

/// Catálogo de los permisos declarados por las extensiones.
///
/// Se construye una sola vez al arrancar con las acciones [`DeclarePermissions`] y se consulta con
/// [`PermissionRegistry::global()`]. Vive sólo en memoria: a quién se concede cada permiso lo
/// guarda, si hace falta, la extensión de autenticación.
///
/// Conserva el orden de registro. Para mostrar los grupos en pantalla,
/// [`groups_sorted()`](Self::groups_sorted) los ordena alfabéticamente según el idioma activo.
#[derive(AutoDefault)]
pub struct PermissionRegistry {
    permissions: Vec<PermissionRef>,
    // `(identificador de grupo, título traducible)`, en orden de primer registro.
    groups: Vec<(&'static str, Lc)>,
}

static PERMISSIONS: OnceLock<PermissionRegistry> = OnceLock::new();

static EMPTY_PERMISSIONS: PermissionRegistry = PermissionRegistry {
    permissions: Vec::new(),
    groups: Vec::new(),
};

impl PermissionRegistry {
    /// Devuelve el catálogo global de permisos.
    ///
    /// Fuera de una aplicación, o antes de que ésta registre las acciones de sus extensiones,
    /// devuelve un catálogo vacío.
    pub fn global() -> &'static PermissionRegistry {
        PERMISSIONS.get().unwrap_or(&EMPTY_PERMISSIONS)
    }

    /// Registra un permiso. Si ya hay uno con la misma clave se ignora, con un aviso en las trazas.
    pub fn register(&mut self, perm: PermissionRef) {
        if self.has(perm) {
            trace::warn!(
                "Permission \"{}\" declared more than once, keeping the first one",
                perm.key()
            );
            return;
        }
        let group = perm.group();
        if !self.groups.iter().any(|(g, _)| *g == group) {
            self.groups.push((group, perm.group_label()));
        }
        self.permissions.push(perm);
    }

    /// Comprueba si un permiso está en el catálogo.
    pub fn has(&self, perm: PermissionRef) -> bool {
        self.has_key(&perm.key())
    }

    /// Comprueba si una clave textual (p. ej. procedente de un formulario) corresponde a un
    /// permiso del catálogo.
    pub fn has_key(&self, key: &str) -> bool {
        self.permissions.iter().any(|p| p.key() == key)
    }

    /// Todos los permisos, en orden de registro.
    pub fn all(&self) -> impl Iterator<Item = PermissionRef> + '_ {
        self.permissions.iter().copied()
    }

    /// Grupos en orden de primer registro, sin traducir ni ordenar alfabéticamente.
    ///
    /// Listado de bajo nivel sobre el que se apoya [`groups_sorted()`](Self::groups_sorted); úsalo
    /// en su lugar sólo cuando el orden de registro sea justo lo que se necesita (por ejemplo, para
    /// inspeccionar el catálogo en pruebas).
    pub fn groups(&self) -> &[(&'static str, Lc)] {
        &self.groups
    }

    /// Grupos ordenados alfabéticamente por su título traducido al idioma del contexto, no sensible
    /// a mayúsculas y, en general, a los acentos (ver [`Lc::collation_key()`] para el criterio
    /// exacto y sus límites).
    pub fn groups_sorted(&self, cx: &Context) -> Vec<&(&'static str, Lc)> {
        let mut groups: Vec<_> = self
            .groups
            .iter()
            .map(|g| (g.1.collation_key(cx), g))
            .collect();
        groups.sort_by(|a, b| a.0.cmp(&b.0));
        groups.into_iter().map(|(_, g)| g).collect()
    }

    /// Permisos de un grupo, en orden de registro.
    pub fn by_group<'a>(&'a self, group: &'a str) -> impl Iterator<Item = PermissionRef> + 'a {
        self.permissions
            .iter()
            .copied()
            .filter(move |p| p.group() == group)
    }
}

// Construye el catálogo global despachando las acciones `DeclarePermissions`. Se llama al
// arrancar, justo después de registrar las acciones de las extensiones; si ya está construido (p.
// ej. por otra llamada a `Application::prepare()` en el mismo proceso, como hacen las pruebas), no
// hace nada.
pub(crate) fn build_permission_registry() {
    PERMISSIONS.get_or_init(|| {
        let mut registry = PermissionRegistry::default();
        DeclarePermissions::dispatch(&mut registry);
        registry
    });
}
