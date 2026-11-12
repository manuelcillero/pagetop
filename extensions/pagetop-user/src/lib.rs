/*!
<div align="center">

<h1>PageTop User</h1>

<p>Gestión de usuarios, autenticación, roles y permisos para <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-user?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-user)
[![Crates.io](https://img.shields.io/crates/v/pagetop-user.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-user)
[![Descargas](https://img.shields.io/crates/d/pagetop-user.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-user)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-user#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Descripción general

Añade cuentas de usuario a una aplicación PageTop: registro, inicio de sesión con sesiones en la
base de datos, perfil propio, recuperación de contraseña y verificación del email por correo, y
control de acceso con roles y permisos. Incluye las páginas de administración de usuarios, roles y
permisos en el panel de administración del núcleo, hechas sólo con componentes del núcleo para que
funcionen con cualquier tema.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-user = "..."
```

Los usuarios y las sesiones se guardan con [`pagetop_seaorm`]: la aplicación elige el motor de base
de datos con su *feature* (`postgres`, `mysql` o `sqlite`) y configura la conexión en la sección
`[database]`.

**Declara la extensión** como dependencia de tu aplicación o de la extensión que la requiera
(`&pagetop_user::User`, como en el ejemplo de más abajo). Al arrancar crea sus tablas y, si no hay
ninguna cuenta, el primer administrador, con los datos de `[user.seed]`:

```toml
[user.seed]
admin_username = "admin"
admin_email    = "admin@example.com"
# Sin 'admin_password', se genera una aleatoria y se muestra una sola vez al crearlo.
```

**Protege tus rutas** con su acceso (`Access`): PageTop lo comprueba antes de llegar al *handler*
con los permisos que concede esta extensión, y a un usuario anónimo sin acceso le ofrece el inicio
de sesión. Dentro del *handler*, el usuario actual está en el contexto de cada [`Page`] al crearla
con [`Page::new()`], y funciones del núcleo como [`has_permission()`] lo consultan para las
comprobaciones más concretas:

```rust,no_run
use pagetop::prelude::*;
use pagetop_user::prelude::*;

#[derive(Clone, Copy, Debug)]
enum MyPermission {
    SeeDashboard,
}

impl Permission for MyPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::SeeDashboard => "myapp:see_dashboard".into(),
        }
    }
}

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_user::User]
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        routes.route(
            "/dashboard",
            Access::Permission(&MyPermission::SeeDashboard),
            web::get(dashboard),
        )
    }
}

async fn dashboard(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! { h1 { "Panel" } }))
        .render().await
}
```

## Páginas

| Ruta                      | Uso                                                          |
|---------------------------|--------------------------------------------------------------|
| `/user/login`             | Inicio de sesión, con el nombre de usuario o el email.       |
| `/user/register`          | Registro, si lo permite `allow_registration`.                |
| `/user`                   | Perfil propio: editarlo, cambiar la contraseña o el email.   |
| `/user/password/reset`    | Recuperación de la contraseña por correo.                    |
| `/admin/user/users`       | Administración de usuarios: alta, edición, roles, bloqueo... |
| `/admin/user/roles`       | Administración de roles y de sus permisos.                   |
| `/admin/user/permissions` | Catálogo de los permisos que declaran las extensiones.       |

## Nombres de usuario y email

Cada cuenta tiene un nombre de usuario y un email, obligatorios y únicos; se puede iniciar sesión
con cualquiera de los dos. Qué nombres de usuario se aceptan lo decide `identity_policy` en la
sección `[user]`:

```toml
[user]
identity_policy = "unicode"     # "strict" (por defecto), "unicode" o "permissive".
```

Con cualquier política, un nombre de usuario nunca contiene `@`, no se distinguen mayúsculas (`Ana`
y `ana` son el mismo usuario) y no se admiten nombres que se confunden a simple vista con otro
existente (`rnaria` si existe `maria`, o `аdmin` con una `а` cirílica). Las reglas completas de
cada política, y las del email, están en [`IdentityPolicy`](config::IdentityPolicy).

## Proveedores de autenticación

Otras extensiones pueden comprobar las credenciales del inicio de sesión con un servicio externo,
como un directorio LDAP o Active Directory. Cada una declara su proveedor con la acción
[`DeclareAuthProviders`](provider::DeclareAuthProviders), y la configuración decide con qué
fuentes, y en qué orden, se comprueban las credenciales. Por ejemplo, las dos, primero el
directorio y después la contraseña local:

```toml
[user.login]
sources = ["ldap", "local"]
```

O sólo el directorio, salvo los administradores, que conservan su contraseña local como acceso de
emergencia por si el directorio no responde (`local_fallback` sólo se aplica cuando `"local"` no
está en `sources`):

```toml
[user.login]
sources        = ["ldap"]
local_fallback = "admins"
```

Los límites de intentos, el bloqueo de cuentas, el estado de cada cuenta y las sesiones son los de
`pagetop-user` para todas las fuentes. Una cuenta vinculada a un proveedor sólo se autentica con él,
y lo que su titular puede cambiar (contraseña, email, datos del perfil, roles) lo decide el
proveedor. La contraseña tecleada nunca se guarda ni se escribe en las trazas: el proveedor sólo la
recibe para comprobarla ([`pagetop::util::SecretStr`]).

Las operaciones que crean cuentas, las vinculan con su identidad externa o cambian sus roles no son
funciones públicas: son métodos de un [`ProviderHandle`](provider::ProviderHandle) que sólo
`pagetop-user` construye y que entrega a cada proveedor activo. Así sus reglas viven en un único
sitio: un proveedor sólo actúa sobre sus propias cuentas, una cuenta existente sólo se vincula si un
administrador lo ha permitido en su ficha (y nunca la de un administrador), y nadie puede conceder
el acceso total. Protege de errores en un proveedor, no de código malicioso compilado en la misma
aplicación. Los detalles están en el módulo [`provider`].

## Correos

La recuperación de contraseña, la confirmación de una nueva dirección y, con
`require_email_verification`, la verificación de las cuentas nuevas envían un correo con un enlace
de un solo uso mediante [`pagetop_mail`]. Además, se avisa a la dirección de la cuenta cuando
cambia su contraseña (la cambie el usuario o un administrador) y cuando se pide cambiar su
dirección. Configura el envío en la sección `[mail]`, incluidos el remitente (`mail.from`), que
estos correos no indican, y la URL pública de los enlaces (`mail.base_url`): sin configurar, los
correos se escriben en las trazas en lugar de enviarse.

La recuperación de contraseña sólo se envía a cuentas activas. Mientras una dirección no está
verificada podría ser de otra persona, así que sus correos no incluyen nada que elija quien creó
la cuenta, ni el nombre de usuario ni el nombre visible. Una dirección queda verificada al
confirmarla desde el enlace del correo de verificación (abrirlo no basta: hay que pulsar un botón,
porque los servidores de correo abren los enlaces para analizarlos) o cuando la fija un
administrador.

Para cambiar su dirección, el usuario indica la nueva y su contraseña actual. La nueva no se aplica
hasta que se confirma desde el enlace que se le envía, y entonces queda verificada; mientras tanto,
la cuenta conserva la anterior, que recibe un aviso. Restablecer la contraseña descarta el cambio
pendiente: es lo que el aviso pide hacer a quien no lo ha pedido. También lo descarta un
administrador que fije otra dirección. Cada enlace sirve sólo para la dirección a la que se envió,
y los enlaces sin usar de recuperación y de verificación dejan de valer cuando cambia la dirección
de la cuenta; los de recuperación, también cuando cambia su contraseña.

Cambiar la contraseña o el email exige la contraseña actual, y sus fallos cuentan para el bloqueo
por intentos fallidos (`max_failed_logins`), como los del inicio de sesión.

Para que nadie pueda usar la aplicación para enviar correos a direcciones ajenas, las recuperaciones
de contraseña y los cambios de dirección se limitan por cuenta (`max_password_resets` y
`max_email_changes`), y ellos y los registros, también por dirección IP (`max_ip_requests`).

Los inicios de sesión fallidos se limitan también por dirección IP (`max_ip_failed_logins`), para
frenar a quien prueba unas pocas contraseñas habituales en muchas cuentas sin llegar a bloquear
ninguna. Detrás de un *proxy* inverso, los límites por IP requieren configurar
`server.client_ip_header`.

## Cookies

Al iniciar sesión se crea la *cookie* de sesión (`cookie_name` de `[user.session]`, `pagetop` por
defecto). Es propia, `HttpOnly` y con un identificador aleatorio que la aplicación guarda en la base
de datos. Se borra al cerrar sesión. Con `secure_cookies` de `[server]` activado, como debe estar si
la aplicación se sirve por HTTPS, sólo viaja por HTTPS.

Sin marcar "Recuérdame", el navegador la borra al cerrarse, y la sesión deja de valer tras
`idle_secs` sin uso. Es una *cookie* de autenticación para un servicio que el propio usuario pide,
así que en España y en la Unión Europea no requiere su aceptación: las normas y guías en que se basa
están en [*Cookies*](pagetop::web#cookies), en el módulo `web` de PageTop.

Con "Recuérdame", la sesión se conserva aunque se cierre el navegador, hasta `remember_idle_secs`
sin uso (14 días por defecto). El dictamen 04/2012 del Grupo de Trabajo del Artículo 29 (WP194) no
exime estas *cookies* de inicio de sesión persistente, pero considera suficiente que sea el usuario
quien la pida marcando expresamente la casilla, siempre que sepa que se usa una *cookie*. Por eso el
formulario de acceso lo indica en el texto de ayuda de la casilla, que además desaconseja marcarla
en un equipo compartido. La aplicación sólo tiene que declararla también en su política de
*cookies*.
*/

use pagetop::prelude::*;
use pagetop_seaorm::install_migrations;

use std::sync::LazyLock;

pub mod account;
pub mod actions;
pub mod config;
pub mod error;
pub mod password;
pub mod permission;
pub mod provider;

pub(crate) mod auth;
#[cfg(feature = "demo-data")]
pub(crate) mod demo;
pub(crate) mod entity;
pub(crate) mod handlers;
pub(crate) mod identity;
pub(crate) mod mail;
pub(crate) mod middleware;
pub(crate) mod migration;
pub(crate) mod service;
pub(crate) mod session;
pub(crate) mod throttle;
pub(crate) mod token;

pub mod component;

include_locales!(LOCALES_USER);

pub use account::{Account, UserStatus};

/// Prelude de `pagetop-user`.
pub mod prelude {
    pub use crate::component::LoginForm;
    pub use crate::component::UserName;
    pub use crate::component::{AccountMenu, account_menu};
    pub use crate::error::AuthError;
    pub use crate::{Account, UserStatus};
}

// **< Rutas HTTP (fijas) >*************************************************************************

// Declaradas en la raíz del crate: un ítem privado aquí ya es visible desde cualquier módulo del
// crate (todos son descendientes de la raíz), así que `pub(crate)` sería redundante.

// GET - muestra el formulario de inicio de sesión.
// POST - valida las credenciales y abre la sesión.
const LOGIN_PATH: &str = "/user/login";
// POST - cierra la sesión activa y redirige al formulario de login.
const LOGOUT_PATH: &str = "/user/logout";
// GET - muestra el formulario de registro.
// POST - crea la cuenta de usuario.
const REGISTER_PATH: &str = "/user/register";
// GET - perfil del usuario autenticado; redirige a LOGIN_PATH si no hay sesión activa.
const PROFILE_PATH: &str = "/user";
// Con el sufijo `/{id}`: GET - perfil público del usuario (requiere `user:view_profiles`, salvo el
//                            propio).
// GET - muestra el formulario de edición del perfil propio (requiere `user:edit_own_profile`).
// POST - guarda los cambios.
const PROFILE_EDIT_PATH: &str = "/user/edit";
// GET - muestra el formulario de cambio de la contraseña propia (requiere
//       `user:change_own_password`).
// POST - aplica la nueva contraseña tras comprobar la actual.
const PROFILE_PASSWORD_PATH: &str = "/user/password";
// GET - muestra el formulario de cambio del email propio (requiere `user:edit_own_profile`).
// POST - comprueba la contraseña y envía el enlace de confirmación a la nueva dirección.
// Con el sufijo `/{uid}/{token}`: GET - pide confirmar la nueva dirección.
//                                 POST - la aplica.
const EMAIL_PATH: &str = "/user/email";
// GET - muestra el formulario de solicitud de restablecimiento.
// POST - inicia el flujo (envío del token).
// Con el sufijo `/{uid}/{token}`: GET - muestra el formulario de nueva contraseña.
//                                 POST - la aplica.
const PASSWORD_RESET_PATH: &str = "/user/password/reset";
// Con el sufijo `/{uid}/{token}`: GET - pide confirmar la dirección de email del usuario.
//                                 POST - la confirma.
const VERIFY_PATH: &str = "/user/verify";

// **< Rutas de administración >********************************************************************

// Relativas a la raíz del panel de administración, como se declaran sus páginas, y completas (ver
// `admin_path()`), para montar las rutas y enlazar.

// Listado, alta, edición, asignación de roles, bloqueo/activación y restablecimiento de contraseña
// de usuarios.
const ADMIN_USERS: &str = "user/users";
static ADMIN_USERS_PATH: LazyLock<String> = LazyLock::new(|| admin_path(ADMIN_USERS));
// Listado, alta, edición, eliminación y asignación de permisos de roles.
const ADMIN_ROLES: &str = "user/roles";
static ADMIN_ROLES_PATH: LazyLock<String> = LazyLock::new(|| admin_path(ADMIN_ROLES));
// Catálogo de permisos registrados, agrupado por extensión (solo lectura).
const ADMIN_PERMISSIONS: &str = "user/permissions";

// Ruta del perfil público del usuario `id`: `{PROFILE_PATH}/{id}`.
fn profile_path(id: i32) -> String {
    util::join!(PROFILE_PATH, "/", id.to_string())
}

// Ruta de una acción sobre el usuario `id`: `{ADMIN_USERS_PATH}/{id}/{tail}`.
fn user_path(id: i32, tail: &str) -> String {
    util::join!(ADMIN_USERS_PATH.as_str(), "/", id.to_string(), "/", tail)
}

// Ruta de una acción sobre el rol `id`: `{ADMIN_ROLES_PATH}/{id}/{tail}`.
fn role_path(id: i32, tail: &str) -> String {
    util::join!(ADMIN_ROLES_PATH.as_str(), "/", id.to_string(), "/", tail)
}

// **< Páginas del panel de administración >********************************************************

// Declara las páginas de usuarios, roles y permisos en el panel de administración, bajo la sección
// integrada "people". Las rutas y sus handlers ya están registrados en `configure_router()`; esta
// declaración sólo aporta el metadato (título, sección, permiso) para que aparezcan en el panel.
fn declare_admin_pages(bag: &mut AdminPageBag) {
    bag.add(
        AdminPage::new(
            ADMIN_USERS,
            "people",
            Lc::t("title-admin-users", &LOCALES_USER),
            &permission::UserPermission::AdminUsers,
        )
        .with_description(Lc::t("description-admin-users", &LOCALES_USER)),
    );
    bag.add(
        AdminPage::new(
            ADMIN_ROLES,
            "people",
            Lc::t("title-admin-roles", &LOCALES_USER),
            &permission::UserPermission::AdminRoles,
        )
        .with_description(Lc::t("description-admin-roles", &LOCALES_USER))
        .with_weight(10),
    );
    bag.add(
        AdminPage::new(
            ADMIN_PERMISSIONS,
            "people",
            Lc::t("title-admin-permissions", &LOCALES_USER),
            &permission::UserPermission::AdminPermissions,
        )
        .with_description(Lc::t("description-admin-permissions", &LOCALES_USER))
        .with_weight(20),
    );
}

// **< Roles de sistema (fijos) >*******************************************************************

// Sembrados con id fijo en `migration/m20260629_000002_create_roles.rs` y bloqueados (`locked`);
// no se borran ni cambian de id. Ninguno se asigna en `user_role` (ver `is_system_role()`).

// Implícito para las peticiones sin sesión; sus permisos no se aplican a ninguna cuenta.
const ANONYMOUS_ROLE_ID: i32 = 1;
// Implícito para toda cuenta autenticada; sus permisos se aplican a todas ellas.
const AUTHENTICATED_ROLE_ID: i32 = 2;

// Un rol de sistema no se asigna a ningún usuario: si "anonymous" llegara a `user_role`, esa cuenta
// recibiría los permisos pensados sólo para peticiones sin sesión.
fn is_system_role(role_id: i32) -> bool {
    role_id == ANONYMOUS_ROLE_ID || role_id == AUTHENTICATED_ROLE_ID
}

// Etiqueta visible de un rol. Las de los roles de sistema se traducen: la migración las guarda en
// inglés y, al estar bloqueados, no se pueden editar.
fn role_label(role_id: i32, label: &str) -> Lc {
    match role_id {
        ANONYMOUS_ROLE_ID => Lc::t("role-anonymous", &LOCALES_USER),
        AUTHENTICATED_ROLE_ID => Lc::t("role-authenticated", &LOCALES_USER),
        _ => Lc::n(label.to_owned()),
    }
}

// **< Extension >**********************************************************************************

/// Implementa la extensión `pagetop-user`.
pub struct User;

#[async_trait]
impl Extension for User {
    fn name(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_USER)
    }

    fn description(&self) -> Lc {
        Lc::t("extension_description", &LOCALES_USER)
    }

    fn settings(&self) -> Option<&'static dyn LoadSettings> {
        Some(&config::SETTINGS)
    }

    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_seaorm::SeaORM,
            &pagetop_htmx::Htmx,
            &pagetop_mail::Mail,
        ]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            // Comprueba permisos mediante el modelo RBAC almacenado en BD.
            action::auth::CheckPermission::new(middleware::check_rbac_permission),
            // Ofrece el inicio de sesión a un usuario anónimo sin acceso a una ruta.
            action::auth::AnonymousDenied::new(handlers::account::offer_login),
            // Registra los permisos propios de pagetop-user.
            action::auth::DeclarePermissions::new(permission::declare_builtin_permissions),
            // Declara las páginas de usuarios, roles y permisos en el panel de administración.
            action::admin::DeclareAdminPages::new(declare_admin_pages),
            // Declara las vistas previas de los correos de recuperación y verificación.
            pagetop_mail::DeclareMailPreviews::new(mail::declare_previews),
        ]
    }

    async fn initialize(&self) -> Result<(), BoxError> {
        install_migrations!(
            m20260629_000001_create_users,
            m20260629_000002_create_roles,
            m20260629_000003_create_user_roles,
            m20260629_000004_create_role_permissions,
            m20260629_000005_create_sessions,
            m20260629_000006_create_user_tokens,
            m20261111_000007_create_user_identities,
        )?;
        provider::initialize()?;
        // Los permisos de "anonymous" en memoria sobreviven a una nueva preparación en el mismo
        // proceso (las pruebas lo hacen), que puede usar otra base de datos.
        session::invalidate_anonymous_permissions();
        auth::seed_initial_data().await?;
        #[cfg(feature = "demo-data")]
        demo::seed_demo_data().await;
        Ok(())
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        let edit_own_profile = Access::Permission(&permission::UserPermission::EditOwnProfile);
        let change_own_password =
            Access::Permission(&permission::UserPermission::ChangeOwnPassword);
        routes
            .public_route(
                LOGIN_PATH,
                web::get(handlers::auth::login_get).post(handlers::auth::login_post),
            )
            .public_route(LOGOUT_PATH, web::post(handlers::auth::logout_post))
            .public_route(
                REGISTER_PATH,
                web::get(handlers::auth::register_get).post(handlers::auth::register_post),
            )
            .route(
                PROFILE_PATH,
                Access::Authenticated,
                web::get(handlers::account::profile_get),
            )
            .public_route(
                &format!("{}/{{id}}", PROFILE_PATH),
                web::get(handlers::account::public_profile_get),
            )
            .route(
                PROFILE_EDIT_PATH,
                edit_own_profile,
                web::get(handlers::account::profile_edit_get)
                    .post(handlers::account::profile_edit_post),
            )
            .route(
                PROFILE_PASSWORD_PATH,
                change_own_password,
                web::get(handlers::account::password_get).post(handlers::account::password_post),
            )
            .route(
                EMAIL_PATH,
                edit_own_profile,
                web::get(handlers::account::email_get).post(handlers::account::email_post),
            )
            .public_route(
                &format!("{}/{{uid}}/{{token}}", EMAIL_PATH),
                web::get(handlers::account::email_confirm_get)
                    .post(handlers::account::email_confirm_post),
            )
            .public_route(
                PASSWORD_RESET_PATH,
                web::get(handlers::auth::password_reset_get)
                    .post(handlers::auth::password_reset_post),
            )
            .public_route(
                &format!("{}/{{uid}}/{{token}}", PASSWORD_RESET_PATH),
                web::get(handlers::auth::password_reset_confirm_get)
                    .post(handlers::auth::password_reset_confirm_post),
            )
            .public_route(
                &format!("{}/{{uid}}/{{token}}", VERIFY_PATH),
                web::get(handlers::auth::verify_email_get).post(handlers::auth::verify_email_post),
            )
            .admin_route(
                ADMIN_USERS,
                web::remember_query(web::get(handlers::admin::users::list_get)),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/new"),
                web::get(handlers::admin::users::new_get).post(handlers::admin::users::new_post),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/{{id}}/edit"),
                web::get(handlers::admin::users::edit_get).post(handlers::admin::users::edit_post),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/{{id}}/view"),
                web::get(handlers::admin::users::view_get),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/{{id}}/status"),
                web::post(handlers::admin::users::status_post),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/{{id}}/admin"),
                web::post(handlers::admin::users::admin_post),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/{{id}}/identity"),
                web::post(handlers::admin::users::identity_post),
            )
            .admin_route(
                &format!("{ADMIN_USERS}/{{id}}/password"),
                web::get(handlers::admin::users::password_get)
                    .post(handlers::admin::users::password_post),
            )
            .admin_route(
                ADMIN_ROLES,
                web::remember_query(web::get(handlers::admin::roles::list_get)),
            )
            .admin_route(
                &format!("{ADMIN_ROLES}/new"),
                web::get(handlers::admin::roles::new_get).post(handlers::admin::roles::new_post),
            )
            .admin_route(
                &format!("{ADMIN_ROLES}/{{id}}/edit"),
                web::get(handlers::admin::roles::edit_get).post(handlers::admin::roles::edit_post),
            )
            .admin_route(
                &format!("{ADMIN_ROLES}/{{id}}/view"),
                web::get(handlers::admin::roles::view_get),
            )
            .admin_route(
                &format!("{ADMIN_ROLES}/{{id}}/delete"),
                web::post(handlers::admin::roles::delete_post),
            )
            .admin_route(
                &format!("{ADMIN_ROLES}/{{id}}/delete/confirm"),
                web::get(handlers::admin::roles::delete_confirm_get),
            )
            .admin_route(
                &format!("{ADMIN_ROLES}/{{id}}/permissions"),
                web::get(handlers::admin::roles::permissions_get)
                    .post(handlers::admin::roles::permissions_post),
            )
            .admin_route(
                ADMIN_PERMISSIONS,
                web::get(handlers::admin::permissions::list_get),
            )
    }

    fn configure_middleware(&self, layers: Layers) -> Layers {
        layers.layer(web::middleware::from_fn(middleware::session_middleware))
    }
}
