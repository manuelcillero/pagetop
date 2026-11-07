//! Acciones predefinidas para intervenir en el funcionamiento de PageTop.
//!
//! Se agrupan por ámbito, para el renderizado de páginas ([`page`]) y componentes ([`component`]),
//! para extender el tratamiento de usuarios y permisos ([`auth`]) y mantener el panel de
//! administración ([`admin`]). Todas se declaran igual, con su constructor `new()` en
//! [`Extension::actions()`](crate::core::extension::Extension::actions), y se escriben con la ruta
//! de su ámbito, por ejemplo `action::page::BeforeRenderBody` o `action::auth::CheckPermission`.

pub mod component;

pub mod page;

pub mod auth;

pub mod admin;
