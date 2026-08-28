/*!
<div align="center">

<h1>PageTop User</h1>

<p>Gestión de usuarios, autenticación, roles y permisos para <strong>PageTop</strong>.</p>

</div>

## Guía rápida

Declara la dependencia en tu `Cargo.toml`, activando en `pagetop-seaorm` el motor de base de
datos que vayas a usar:

```toml
[dependencies]
pagetop-user = "..."
pagetop-seaorm = { version = "...", features = ["postgres"] }
```

Añade `&pagetop_user::User` a las dependencias de tu extensión. El usuario actual se inyecta
automáticamente en el contexto de cada [`Page`] al crearla con [`Page::new()`], sin necesidad de
llamadas adicionales. Usa las helpers del core para acceder a él:

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
            Self::SeeDashboard => "myapp.see_dashboard".into(),
        }
    }
}

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_user::User]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/dashboard", web::get(dashboard))
    }
}

async fn dashboard(request: HttpRequest) -> Result<Markup, ErrorPage> {
    if !has_permission(&request, &MyPermission::SeeDashboard) {
        return Err(ErrorPage::NotFound(Some(request)));
    }
    Page::new(request)
        .with_child(Html::with(|_| html! { h1 { "Panel" } }))
        .render().await
}
```
*/

use pagetop::prelude::*;
use pagetop_admin::prelude::*;
use pagetop_seaorm::install_migrations;

pub mod account;
pub mod config;
pub mod error;
pub mod password;
pub mod permission;

pub(crate) mod auth;
#[cfg(feature = "demo-data")]
pub(crate) mod demo;
pub(crate) mod entity;
pub(crate) mod handlers;
pub(crate) mod middleware;
pub(crate) mod migration;
pub(crate) mod service;
pub(crate) mod session;
pub(crate) mod token;

pub mod component;

include_locales!(LOCALES_USER);

pub use account::{Account, UserStatus};
pub use permission::{DeclarePermissions, PermissionRegistry};

/// Prelude de `pagetop-user`.
pub mod prelude {
    pub use crate::component::{LoginForm, UserBlock};
    pub use crate::error::AuthError;
    pub use crate::{Account, DeclarePermissions, UserStatus};
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
// GET - muestra el formulario de solicitud de restablecimiento.
// POST - inicia el flujo (envío del token).
// Con el sufijo `/{uid}/{token}`: GET - muestra el formulario de nueva contraseña.
//                                 POST - la aplica.
const PASSWORD_RESET_PATH: &str = "/user/password/reset";
// Con el sufijo `/{uid}/{token}`: confirma la dirección de email del usuario.
const VERIFY_PATH: &str = "/user/verify";

// **< Rutas de administración (fijas) >************************************************************

// Listado, alta, edición, asignación de roles, bloqueo/activación y restablecimiento de contraseña
// de usuarios.
const ADMIN_USERS_PATH: &str = "/admin/user/users";
// Listado, alta, edición, eliminación y asignación de permisos de roles.
const ADMIN_ROLES_PATH: &str = "/admin/user/roles";
// Catálogo de permisos registrados, agrupado por extensión (solo lectura).
const ADMIN_PERMISSIONS_PATH: &str = "/admin/user/permissions";

// **< Registro en pagetop-admin (fijo) >***********************************************************

// Registra las páginas de usuarios, roles y permisos en el portal de `pagetop-admin`, bajo la
// sección integrada "people". Las rutas y sus handlers ya están registrados en
// `configure_router()`; este registro sólo aporta el metadato (título, sección, permiso) para que
// aparezcan en la portada `/admin`.
fn declare_admin_pages(bag: &mut PageBag) {
    bag.add(AdminPage {
        path: ADMIN_USERS_PATH.to_owned(),
        section: "people".to_owned(),
        title: Lc::t("title-admin-users", &LOCALES_USER),
        description: Some(Lc::t("description-admin-users", &LOCALES_USER)),
        weight: 0,
        permission: Some(&permission::UserPermission::AdminUsers),
        kind: AdminPageKind::View,
    });
    bag.add(AdminPage {
        path: ADMIN_ROLES_PATH.to_owned(),
        section: "people".to_owned(),
        title: Lc::t("title-admin-roles", &LOCALES_USER),
        description: Some(Lc::t("description-admin-roles", &LOCALES_USER)),
        weight: 10,
        permission: Some(&permission::UserPermission::AdminRoles),
        kind: AdminPageKind::View,
    });
    bag.add(AdminPage {
        path: ADMIN_PERMISSIONS_PATH.to_owned(),
        section: "people".to_owned(),
        title: Lc::t("title-admin-permissions", &LOCALES_USER),
        description: Some(Lc::t("description-admin-permissions", &LOCALES_USER)),
        weight: 20,
        permission: Some(&permission::UserPermission::AdminPermissions),
        kind: AdminPageKind::View,
    });
}

// **< Roles de sistema (fijos) >********************************************************************

// Sembrados con id fijo en `migration/m20260629_000002_create_roles.rs` y bloqueados (`locked`);
// no se borran ni cambian de id.
const ANONYMOUS_ROLE_ID: i32 = 1;
// Se asigna automáticamente a toda cuenta en el alta (ver `auth::assign_role`).
const AUTHENTICATED_ROLE_ID: i32 = 2;

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

    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_admin::Admin,
            &pagetop_seaorm::SeaORM,
            &pagetop_htmx::Htmx,
        ]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            // Comprueba permisos mediante el modelo RBAC almacenado en BD.
            CheckPermission::new(middleware::check_rbac_permission),
            // Registra los permisos propios de pagetop-user.
            DeclarePermissions::new(permission::declare_builtin_permissions),
            // Registra las páginas de usuarios, roles y permisos en el portal de pagetop-admin.
            DeclareAdminPages::new(declare_admin_pages),
        ]
    }

    async fn initialize(&self) {
        install_migrations!(
            m20260629_000001_create_users,
            m20260629_000002_create_roles,
            m20260629_000003_create_user_roles,
            m20260629_000004_create_role_permissions,
            m20260629_000005_create_sessions,
            m20260629_000006_create_user_tokens,
        );
        permission::build_registry();
        auth::seed_initial_data().await;
        #[cfg(feature = "demo-data")]
        demo::seed_demo_data().await;
    }

    fn configure_router(&self, router: Router) -> Router {
        router
            .route(
                LOGIN_PATH,
                web::get(handlers::auth::login_get).post(handlers::auth::login_post),
            )
            .route(LOGOUT_PATH, web::post(handlers::auth::logout_post))
            .route(
                REGISTER_PATH,
                web::get(handlers::auth::register_get).post(handlers::auth::register_post),
            )
            .route(PROFILE_PATH, web::get(handlers::account::profile_get))
            .route(
                PASSWORD_RESET_PATH,
                web::get(handlers::auth::password_reset_get)
                    .post(handlers::auth::password_reset_post),
            )
            .route(
                &format!("{}/{{uid}}/{{token}}", PASSWORD_RESET_PATH),
                web::get(handlers::auth::password_reset_confirm_get)
                    .post(handlers::auth::password_reset_confirm_post),
            )
            .route(
                &format!("{}/{{uid}}/{{token}}", VERIFY_PATH),
                web::get(handlers::auth::verify_email_get),
            )
            .route(ADMIN_USERS_PATH, web::get(handlers::admin::users::list_get))
            .route(
                &format!("{}/new", ADMIN_USERS_PATH),
                web::get(handlers::admin::users::new_get).post(handlers::admin::users::new_post),
            )
            .route(
                &format!("{}/{{id}}/edit", ADMIN_USERS_PATH),
                web::get(handlers::admin::users::edit_get).post(handlers::admin::users::edit_post),
            )
            .route(
                &format!("{}/{{id}}/view", ADMIN_USERS_PATH),
                web::get(handlers::admin::users::view_get),
            )
            .route(
                &format!("{}/{{id}}/roles", ADMIN_USERS_PATH),
                web::get(handlers::admin::users::roles_get)
                    .post(handlers::admin::users::roles_post),
            )
            .route(
                &format!("{}/{{id}}/status", ADMIN_USERS_PATH),
                web::post(handlers::admin::users::status_post),
            )
            .route(
                &format!("{}/{{id}}/admin", ADMIN_USERS_PATH),
                web::post(handlers::admin::users::admin_post),
            )
            .route(
                &format!("{}/{{id}}/password", ADMIN_USERS_PATH),
                web::get(handlers::admin::users::password_get)
                    .post(handlers::admin::users::password_post),
            )
            .route(ADMIN_ROLES_PATH, web::get(handlers::admin::roles::list_get))
            .route(
                &format!("{}/new", ADMIN_ROLES_PATH),
                web::get(handlers::admin::roles::new_get).post(handlers::admin::roles::new_post),
            )
            .route(
                &format!("{}/{{id}}/edit", ADMIN_ROLES_PATH),
                web::get(handlers::admin::roles::edit_get).post(handlers::admin::roles::edit_post),
            )
            .route(
                &format!("{}/{{id}}/view", ADMIN_ROLES_PATH),
                web::get(handlers::admin::roles::view_get),
            )
            .route(
                &format!("{}/{{id}}/delete", ADMIN_ROLES_PATH),
                web::post(handlers::admin::roles::delete_post),
            )
            .route(
                &format!("{}/{{id}}/delete/confirm", ADMIN_ROLES_PATH),
                web::get(handlers::admin::roles::delete_confirm_get),
            )
            .route(
                &format!("{}/{{id}}/permissions", ADMIN_ROLES_PATH),
                web::get(handlers::admin::roles::permissions_get)
                    .post(handlers::admin::roles::permissions_post),
            )
            .route(
                ADMIN_PERMISSIONS_PATH,
                web::get(handlers::admin::permissions::list_get),
            )
    }

    fn configure_middleware(&self, router: Router) -> Router {
        router.layer(web::middleware::from_fn(middleware::session_middleware))
    }
}
