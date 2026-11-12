//! Proveedores de autenticación externos.
//!
//! Permite a otras extensiones comprobar credenciales y gestionar cuentas de `pagetop-user` (un
//! directorio LDAP, un proveedor OAuth...).
//!
//! # Cómo funciona
//!
//! 1. La extensión implementa [`AuthProvider`] y lo declara con la acción
//!    [`DeclareAuthProviders`].
//! 2. Quien configura la aplicación decide en
//!    [`[user.login] sources`](crate::config::LoginSettings::sources) con qué fuentes, y en qué
//!    orden, se comprueban las credenciales del formulario de inicio de sesión: la contraseña
//!    guardada en PageTop (`"local"`) y los proveedores por su [`id()`](AuthProvider::id).
//! 3. Al arrancar, `pagetop-user` entrega a cada proveedor que figura en `sources` su
//!    [`ProviderHandle`] ([`AuthProvider::attach()`]): la única vía para crear cuentas, vincularlas
//!    con su identidad externa, cambiar sus datos y roles o cerrar sus sesiones.
//! 4. En cada inicio de sesión, `pagetop-user` aplica los límites de intentos, recorre las fuentes
//!    en orden ([`AuthProvider::authenticate()`]) y, si una autentica, comprueba el estado de la
//!    cuenta resultante y abre la sesión. Las comprobaciones del estado, del bloqueo por intentos
//!    fallidos y de los límites son siempre las de `pagetop-user`, para todas las fuentes.
//!
//! Una cuenta vinculada a un proveedor sólo se autentica con ese proveedor (y con `"local"` si
//! `sources` o [`local_fallback`](crate::config::LoginSettings::local_fallback) lo permiten), y su
//! contraseña local deja de valer al vincularla. Lo que el usuario puede cambiar de su cuenta lo
//! decide el proveedor con [`AuthProvider::account_policy()`].
//!
//! # La contraseña
//!
//! El proveedor recibe la contraseña tecleada como [`pagetop::util::SecretStr`] y sólo debe usarla
//! para comprobarla: nunca guardarla, registrarla ni pasarla a tareas que sobrevivan a la petición.
//!
//! `SecretStr` la borra de la memoria al descartarse, pero el cuerpo del formulario pasa antes por
//! búferes del servidor web que PageTop no controla: se garantiza que la contraseña no se guarda ni
//! se registra, no que no quede ningún rastro en la memoria del proceso.

mod handle;
mod policy;

pub use handle::{ExternalIdentity, ExternalUserData, ExternalUserUpdate, ProviderHandle};
pub use handle::{UpdateReport, find_role_by_machine_name, find_user_by_login};
pub use handle::{validate_email, validate_username};

pub(crate) use handle::{allow_linking, cancel_linking, identities_of, pending_identity};
pub use policy::UserField;
pub use policy::{AccountPolicy, FieldPolicy, HelpLink, LoginHints, ResetLink, ResetPolicy};

use pagetop::prelude::*;
use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, dbconn};

use crate::account::UserStatus;
use crate::config::{LocalFallback, SETTINGS};
use crate::entity::{user, user_identity};
use crate::error::AuthError;

use std::net::IpAddr;
use std::sync::OnceLock;

/// Identificador de la fuente de credenciales que es la contraseña guardada en PageTop.
pub const LOCAL_SOURCE: &str = "local";

// **< UserRef >************************************************************************************

/// Vista de sólo lectura de una cuenta, la que reciben los proveedores.
#[derive(Clone, Debug, Getters)]
pub struct UserRef {
    /// Identificador de la cuenta.
    #[getters(copy)]
    id: i32,
    /// Nombre de usuario.
    username: String,
    /// Email.
    email: String,
    /// Estado de la cuenta.
    #[getters(copy)]
    status: UserStatus,
    /// Si es una cuenta de administrador (`is_admin`).
    #[getters(copy)]
    is_admin: bool,
    /// Hasta cuándo está bloqueada por intentos fallidos, si lo está o lo estuvo.
    #[getters(copy)]
    locked_until: Option<DateTime<Utc>>,
}

impl From<&user::Model> for UserRef {
    fn from(user: &user::Model) -> Self {
        UserRef {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            status: UserStatus::from_i16(user.status),
            is_admin: user.is_admin,
            locked_until: user.locked_until,
        }
    }
}

// **< LoginAttempt >*******************************************************************************

/// Un intento de inicio de sesión, tal como lo recibe [`AuthProvider::authenticate()`].
///
/// Cuando llega al proveedor ya ha superado los límites de intentos de `pagetop-user` (por
/// dirección IP y por identificador) y los topes de longitud: un identificador de hasta 255
/// caracteres sin caracteres de control y una contraseña de hasta 1024 bytes sin NUL.
pub struct LoginAttempt<'a> {
    identifier: &'a str,
    password: &'a util::SecretStr,
    local: Option<&'a UserRef>,
    client_ip: Option<IpAddr>,
}

impl<'a> LoginAttempt<'a> {
    pub(crate) fn new(
        identifier: &'a str,
        password: &'a util::SecretStr,
        local: Option<&'a UserRef>,
        client_ip: Option<IpAddr>,
    ) -> Self {
        LoginAttempt {
            identifier,
            password,
            local,
            client_ip,
        }
    }

    /// Lo tecleado como usuario: un nombre de usuario o un email, sin normalizar.
    pub fn identifier(&self) -> &'a str {
        self.identifier
    }

    /// La contraseña tecleada.
    pub fn password(&self) -> &'a util::SecretStr {
        self.password
    }

    /// La cuenta de PageTop que corresponde a lo tecleado (por email si lleva `@`, por nombre de
    /// usuario si no), si existe.
    pub fn local(&self) -> Option<&'a UserRef> {
        self.local
    }

    /// Dirección IP del cliente, si se conoce.
    pub fn client_ip(&self) -> Option<IpAddr> {
        self.client_ip
    }
}

// **< Resultados >*********************************************************************************

/// Resultado de [`AuthProvider::authenticate()`].
#[derive(Debug)]
pub enum ProviderOutcome {
    /// El proveedor no conoce a la persona, o no le corresponde comprobarla.
    NotFound,
    /// La contraseña no es válida para una persona que el proveedor conoce. `user_id` es su cuenta
    /// de PageTop, si la tiene: en ella se anota el intento fallido.
    InvalidCredentials {
        /// Cuenta de PageTop de la persona, si la tiene.
        user_id: Option<i32>,
    },
    /// Credenciales válidas, y ésta es la cuenta de PageTop (creada, vinculada o ya vinculada).
    Authenticated {
        /// Cuenta de PageTop con la que se inicia la sesión.
        user_id: i32,
    },
    /// Credenciales **válidas**, pero el proveedor deniega el acceso. Detiene la cadena: no se
    /// prueba ninguna otra fuente.
    ///
    /// Sólo se puede devolver tras comprobar que la contraseña es correcta: si no, el motivo
    /// revelaría a cualquiera qué cuentas existen y en qué situación están.
    Rejected(RejectReason),
    /// El proveedor no está disponible (red, TLS, plazo agotado...).
    Unavailable,
}

/// Motivo de un [`ProviderOutcome::Rejected`], que se muestra al usuario.
#[derive(Clone, Debug)]
pub enum RejectReason {
    /// El proveedor no permite el acceso a esta persona.
    Disallowed,
    /// La persona no tiene cuenta en la aplicación y no se crean automáticamente.
    NotRegistered,
    /// La cuenta se ha creado y espera a que un administrador la apruebe.
    AwaitingApproval,
    /// No se puede asociar a ninguna cuenta sin ambigüedad. El mensaje es genérico; el motivo
    /// concreto debe ir a las trazas.
    Conflict,
    /// La contraseña ha caducado y debe cambiarse fuera de la aplicación.
    PasswordExpired {
        /// Enlace a la página donde se cambia, si lo hay.
        help: Option<HelpLink>,
    },
}

/// Resultado de [`AuthProvider::reauthenticate()`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReauthOutcome {
    /// La contraseña es correcta.
    Valid,
    /// La contraseña no es correcta.
    Invalid,
    /// No se ha podido comprobar.
    Unavailable,
}

/// Cómo ha entrado una cuenta vinculada a un proveedor.
///
/// Lo recibe [`AuthProvider::after_login()`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoginVia {
    /// Con su contraseña local (ver
    /// [`local_fallback`](crate::config::LoginSettings::local_fallback)).
    Local,
    /// Con el propio proveedor.
    Provider,
}

// **< AuthProvider >*******************************************************************************

/// Un proveedor de autenticación externo. Ver la [documentación del módulo](self).
#[async_trait]
pub trait AuthProvider: Send + Sync + 'static {
    /// Identificador estable del proveedor (`"ldap"`), el que se escribe en
    /// [`sources`](crate::config::LoginSettings::sources) y se guarda en cada identidad vinculada.
    /// No debe cambiar nunca: las cuentas vinculadas quedarían sin proveedor.
    fn id(&self) -> &'static str;

    /// Nombre visible del proveedor («Directorio corporativo»).
    fn label(&self) -> Lc;

    /// Recibe la API privilegiada de `pagetop-user`. Se llama una sola vez, al arrancar, y sólo si
    /// el proveedor figura en `sources`.
    ///
    /// El proveedor debe guardar el [`ProviderHandle`] en un lugar privado (un `OnceLock` de su
    /// módulo) y no compartirlo: es la única vía para crear y modificar las cuentas que gestiona.
    /// Ver la sección *Seguridad* de [`ProviderHandle`].
    fn attach(&self, handle: ProviderHandle);

    /// Si el proveedor renuncia a gestionar la cuenta, que entonces puede entrar con su contraseña
    /// local aunque `"local"` no esté en `sources` (ver
    /// [`LocalFallback::Released`]). Por defecto, no.
    async fn releases(&self, _user: &UserRef) -> bool {
        false
    }

    /// Comprueba las credenciales de un intento de inicio de sesión.
    ///
    /// Con la contraseña correcta, el proveedor resuelve la cuenta de PageTop con su
    /// [`ProviderHandle`]: la ya vinculada a la persona
    /// ([`find_user()`](ProviderHandle::find_user)), una que un administrador haya permitido
    /// vincular ([`find_pending()`](ProviderHandle::find_pending) y
    /// [`link_identity()`](ProviderHandle::link_identity)) o una nueva
    /// ([`create_user()`](ProviderHandle::create_user)), y devuelve
    /// [`Authenticated`](ProviderOutcome::Authenticated). Si la persona existe pero la contraseña
    /// no vale, [`InvalidCredentials`](ProviderOutcome::InvalidCredentials); si no la conoce,
    /// [`NotFound`](ProviderOutcome::NotFound); si no puede comprobarla,
    /// [`Unavailable`](ProviderOutcome::Unavailable). [`Rejected`](ProviderOutcome::Rejected)
    /// sólo tras comprobar que la contraseña es correcta.
    async fn authenticate(&self, attempt: &LoginAttempt<'_>) -> ProviderOutcome;

    /// Comprueba de nuevo la contraseña de una cuenta ya vinculada a este proveedor, cuando el
    /// usuario confirma con ella una operación sensible (p. ej. cambiar su email).
    async fn reauthenticate(&self, user: &UserRef, password: &util::SecretStr) -> ReauthOutcome;

    /// Qué se puede cambiar de una cuenta vinculada a este proveedor, o `None` si no la gestiona.
    async fn account_policy(&self, _user: &UserRef) -> Option<AccountPolicy> {
        None
    }

    /// Si admite «Recuérdame» para sus cuentas. Si no, la sesión dura lo que el navegador
    /// abierto, aunque el usuario marque la casilla.
    fn allows_remember(&self) -> bool {
        true
    }

    /// Lo que quiere mostrar en el formulario de inicio de sesión.
    fn login_hints(&self) -> Option<LoginHints> {
        None
    }

    /// Se llama tras un inicio de sesión correcto de una cuenta vinculada a este proveedor, antes
    /// de responder. Sus errores no impiden el inicio de sesión, y `pagetop-user` deja de esperarlo
    /// pasados unos segundos.
    async fn after_login(&self, _user_id: i32, _via: LoginVia) {}
}

// **< DeclareAuthProviders >***********************************************************************

/// Tipo de función que declara proveedores en un [`AuthProviders`].
pub type FnDeclareAuthProviders = fn(providers: &mut AuthProviders);

/// Proveedores declarados por las extensiones. Ver [`DeclareAuthProviders`].
#[derive(AutoDefault)]
pub struct AuthProviders {
    providers: Vec<&'static dyn AuthProvider>,
}

impl AuthProviders {
    /// Añade un proveedor.
    pub fn add(&mut self, provider: &'static dyn AuthProvider) {
        self.providers.push(provider);
    }
}

/// Acción para declarar proveedores de autenticación.
///
/// Se despacha una sola vez, al inicializar `pagetop-user`.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_user::provider::*;
///
/// use std::sync::OnceLock;
///
/// static HANDLE: OnceLock<ProviderHandle> = OnceLock::new();
///
/// struct Sso;
///
/// #[async_trait]
/// impl AuthProvider for Sso {
///     fn id(&self) -> &'static str {
///         "sso"
///     }
///
///     fn label(&self) -> Lc {
///         Lc::n("Single sign-on")
///     }
///
///     fn attach(&self, handle: ProviderHandle) {
///         let _ = HANDLE.set(handle);
///     }
///
///     async fn authenticate(&self, _attempt: &LoginAttempt<'_>) -> ProviderOutcome {
///         ProviderOutcome::NotFound
///     }
///
///     async fn reauthenticate(
///         &self,
///         _user: &UserRef,
///         _password: &util::SecretStr,
///     ) -> ReauthOutcome {
///         ReauthOutcome::Invalid
///     }
/// }
///
/// fn declare_providers(providers: &mut AuthProviders) {
///     providers.add(&Sso);
/// }
///
/// struct MyApp;
///
/// #[async_trait]
/// impl Extension for MyApp {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![&pagetop_user::User]
///     }
///
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![DeclareAuthProviders::new(declare_providers)]
///     }
/// }
/// ```
pub struct DeclareAuthProviders {
    f: FnDeclareAuthProviders,
    weight: Weight,
}

impl ActionDispatcher for DeclareAuthProviders {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAuthProviders {
    /// Registra una función que declara proveedores.
    pub fn new(f: FnDeclareAuthProviders) -> Self {
        DeclareAuthProviders { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< Registro de fuentes >************************************************************************

// Una fuente de credenciales de `sources`.
#[derive(Clone, Copy)]
pub(crate) enum Source {
    Local,
    Provider(&'static dyn AuthProvider),
}

// Fuentes activas, en el orden de `sources`, resueltas una sola vez al arrancar. Arrancar de nuevo
// en el mismo proceso (las pruebas lo hacen) conserva las primeras, como las propias acciones.
static SOURCES: OnceLock<Vec<Source>> = OnceLock::new();

// Recoge los proveedores declarados, valida `sources` y entrega su `ProviderHandle` a los que
// figuran en ella. Se llama desde `User::initialize()`.
pub(crate) fn initialize() -> Result<(), AuthError> {
    if SOURCES.get().is_some() {
        return Ok(());
    }
    let mut declared = AuthProviders::default();
    dispatch_actions(|action: &DeclareAuthProviders| (action.f)(&mut declared));
    let sources = resolve_sources(&SETTINGS.login.sources, &declared.providers)?;

    for provider in &declared.providers {
        if !sources
            .iter()
            .any(|s| matches!(s, Source::Provider(p) if p.id() == provider.id()))
        {
            trace::warn!(
                "Authentication provider \"{}\" is not in [user.login] sources: it will not \
                 authenticate anyone",
                provider.id()
            );
        }
    }
    let has_providers = sources.iter().any(|s| matches!(s, Source::Provider(_)));
    if has_providers && SETTINGS.allow_registration {
        trace::warn!(
            "Local registration is open (user.allow_registration) while an authentication \
             provider is active: anyone can create a local account, and a name taken here can \
             block the account of the person with that name in the provider"
        );
    }
    if SOURCES.set(sources).is_ok() {
        for source in active_sources() {
            if let Source::Provider(provider) = source {
                provider.attach(ProviderHandle::issue(provider.id()));
            }
        }
    }
    Ok(())
}

// Comprueba la lista `sources` contra los proveedores declarados y la devuelve resuelta.
fn resolve_sources(
    ids: &[String],
    declared: &[&'static dyn AuthProvider],
) -> Result<Vec<Source>, AuthError> {
    if ids.is_empty() {
        return Err(AuthError::NoLoginSources);
    }
    let mut sources = Vec::with_capacity(ids.len());
    for (i, id) in ids.iter().enumerate() {
        if ids[..i].contains(id) {
            return Err(AuthError::DuplicateLoginSource(id.clone()));
        }
        if id == LOCAL_SOURCE {
            sources.push(Source::Local);
        } else {
            let provider = declared
                .iter()
                .find(|p| p.id() == id)
                .ok_or_else(|| AuthError::UnknownLoginSource(id.clone()))?;
            sources.push(Source::Provider(*provider));
        }
    }
    Ok(sources)
}

// Fuentes activas, o sólo `"local"` fuera de una aplicación (o antes de arrancar).
pub(crate) fn active_sources() -> &'static [Source] {
    SOURCES.get().map_or(&[Source::Local], Vec::as_slice)
}

/// Proveedores activos (los de `sources`), en su orden.
pub fn active_providers() -> impl Iterator<Item = &'static dyn AuthProvider> {
    active_sources().iter().filter_map(|source| match source {
        Source::Provider(provider) => Some(*provider),
        Source::Local => None,
    })
}

/// Proveedor activo con ese identificador.
pub fn active_provider(id: &str) -> Option<&'static dyn AuthProvider> {
    active_providers().find(|provider| provider.id() == id)
}

// Si `"local"` está en `sources`.
pub(crate) fn local_is_source() -> bool {
    active_sources().iter().any(|s| matches!(s, Source::Local))
}

// Cadena de fuentes de un intento. Si `"local"` no está en `sources` y la cuenta existe, se añade
// al principio cuando `local_fallback` lo permite: para un administrador con `Admins`, o para una
// cuenta que algún proveedor libera (`released`) con `Released` o `Admins`.
pub(crate) fn login_chain(
    sources: &[Source],
    fallback: LocalFallback,
    local: Option<&UserRef>,
    released: bool,
) -> Vec<Source> {
    let mut chain = Vec::with_capacity(sources.len() + 1);
    let has_local = sources.iter().any(|s| matches!(s, Source::Local));
    if !has_local && let Some(user) = local {
        let admin = fallback == LocalFallback::Admins && user.is_admin();
        let freed = fallback != LocalFallback::None && released;
        if admin || freed {
            chain.push(Source::Local);
        }
    }
    chain.extend_from_slice(sources);
    chain
}

// Si hace falta preguntar a los proveedores si liberan la cuenta para construir la cadena.
pub(crate) fn needs_release_check(fallback: LocalFallback, local: Option<&UserRef>) -> bool {
    !local_is_source()
        && fallback != LocalFallback::None
        && local.is_some_and(|user| !(fallback == LocalFallback::Admins && user.is_admin()))
}

// Proveedores (por identificador) a los que está vinculada la cuenta; vacío si es local.
pub(crate) async fn linked_providers(user_id: i32) -> Result<Vec<String>, AuthError> {
    let rows = user_identity::Entity::find()
        .filter(user_identity::Column::UserId.eq(user_id))
        .filter(user_identity::Column::Pending.eq(false))
        .all(dbconn())
        .await?;
    Ok(rows.into_iter().map(|row| row.provider).collect())
}

// Proveedor activo que gestiona la cuenta, si está vinculada a alguno.
pub(crate) async fn managing_provider(
    user_id: i32,
) -> Result<Option<&'static dyn AuthProvider>, AuthError> {
    if active_providers().next().is_none() {
        return Ok(None);
    }
    let linked = linked_providers(user_id).await?;
    Ok(active_providers().find(|p| linked.iter().any(|id| id == p.id())))
}

// Política de la cuenta, si la gestiona un proveedor activo que devuelve una.
pub(crate) async fn account_policy(user: &user::Model) -> Option<AccountPolicy> {
    match managing_provider(user.id).await {
        Ok(Some(provider)) => provider.account_policy(&UserRef::from(user)).await,
        Ok(None) => None,
        Err(err) => {
            trace::error!("Cannot load the identities of user {}: {err}", user.id);
            None
        }
    }
}

// Pistas para el formulario de inicio de sesión: la nota de cada proveedor activo y el enlace de
// recuperación (el local si `"local"` es una fuente; si no, el primero que indique un proveedor).
pub(crate) fn login_hints() -> (Vec<Lc>, ResetLink) {
    let mut notes = Vec::new();
    let mut reset = None;
    for provider in active_providers() {
        if let Some(hints) = provider.login_hints() {
            if !hints.note().is_none() {
                notes.push(hints.note().clone());
            }
            reset.get_or_insert_with(|| hints.reset().clone());
        }
    }
    let reset = if local_is_source() {
        ResetLink::Local
    } else {
        reset.unwrap_or(ResetLink::Hidden)
    };
    (notes, reset)
}

// Si alguna fuente admite "Recuérdame".
pub(crate) fn any_allows_remember() -> bool {
    active_sources().iter().any(|source| match source {
        Source::Local => true,
        Source::Provider(provider) => provider.allows_remember(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake;

    #[async_trait]
    impl AuthProvider for Fake {
        fn id(&self) -> &'static str {
            "fake"
        }
        fn label(&self) -> Lc {
            Lc::n("Fake")
        }
        fn attach(&self, _handle: ProviderHandle) {}
        async fn authenticate(&self, _attempt: &LoginAttempt<'_>) -> ProviderOutcome {
            ProviderOutcome::NotFound
        }
        async fn reauthenticate(
            &self,
            _user: &UserRef,
            _password: &util::SecretStr,
        ) -> ReauthOutcome {
            ReauthOutcome::Invalid
        }
    }

    static FAKE: Fake = Fake;

    fn ids(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| (*v).to_owned()).collect()
    }

    fn names(chain: &[Source]) -> Vec<&'static str> {
        chain
            .iter()
            .map(|s| match s {
                Source::Local => LOCAL_SOURCE,
                Source::Provider(p) => p.id(),
            })
            .collect()
    }

    fn user(is_admin: bool) -> UserRef {
        UserRef {
            id: 1,
            username: "ana".into(),
            email: "ana@example.com".into(),
            status: UserStatus::Active,
            is_admin,
            locked_until: None,
        }
    }

    #[pagetop::test]
    async fn sources_must_be_known_unique_and_not_empty() {
        let declared: [&'static dyn AuthProvider; 1] = [&FAKE];
        assert!(resolve_sources(&ids(&["local", "fake"]), &declared).is_ok());
        assert!(matches!(
            resolve_sources(&ids(&[]), &declared),
            Err(AuthError::NoLoginSources)
        ));
        assert!(matches!(
            resolve_sources(&ids(&["fake", "fake"]), &declared),
            Err(AuthError::DuplicateLoginSource(_))
        ));
        assert!(matches!(
            resolve_sources(&ids(&["local", "ldap"]), &declared),
            Err(AuthError::UnknownLoginSource(_))
        ));
    }

    #[pagetop::test]
    async fn local_is_only_added_when_the_fallback_allows_it() {
        let declared: [&'static dyn AuthProvider; 1] = [&FAKE];
        let only_fake = resolve_sources(&ids(&["fake"]), &declared).unwrap();
        let both = resolve_sources(&ids(&["fake", "local"]), &declared).unwrap();
        let admin = user(true);
        let plain = user(false);
        use LocalFallback::*;

        // "local" already in the sources: the chain is the configured one.
        assert_eq!(
            names(&login_chain(&both, None, Some(&admin), true)),
            ["fake", "local"]
        );
        // No account: nothing to fall back to.
        assert_eq!(
            names(&login_chain(&only_fake, Admins, Option::None, true)),
            ["fake"]
        );
        // `none` never falls back.
        assert_eq!(
            names(&login_chain(&only_fake, None, Some(&admin), true)),
            ["fake"]
        );
        // `released` only for released accounts, administrators included.
        assert_eq!(
            names(&login_chain(&only_fake, Released, Some(&admin), false)),
            ["fake"]
        );
        assert_eq!(
            names(&login_chain(&only_fake, Released, Some(&plain), true)),
            ["local", "fake"]
        );
        // `admins` adds any administrator, and released accounts too.
        assert_eq!(
            names(&login_chain(&only_fake, Admins, Some(&admin), false)),
            ["local", "fake"]
        );
        assert_eq!(
            names(&login_chain(&only_fake, Admins, Some(&plain), false)),
            ["fake"]
        );
        assert_eq!(
            names(&login_chain(&only_fake, Admins, Some(&plain), true)),
            ["local", "fake"]
        );
    }
}
