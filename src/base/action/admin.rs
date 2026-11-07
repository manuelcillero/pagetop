//! Acciones para declarar las piezas que componen el panel de administración.
//!
//! Cada extensión aporta sus secciones, páginas, pestañas y botones al panel de administración con
//! estas acciones, sin depender de ninguna otra extensión, y PageTop las reúne al arrancar en el
//! registro del panel ([`AdminRegistry`](crate::admin::AdminRegistry)). El resto del panel está en
//! el módulo [`admin`](crate::admin).

use crate::admin::{AdminActionBag, AdminPageBag, AdminSectionBag, AdminTaskBag};

/// Tipo de función para declarar secciones del panel de administración.
pub type FnActionDeclareAdminSections = fn(bag: &mut AdminSectionBag);

/// Tipo de función para declarar páginas del panel de administración.
pub type FnActionDeclareAdminPages = fn(bag: &mut AdminPageBag);

/// Tipo de función para declarar tareas (pestañas) locales de las páginas del panel.
pub type FnActionDeclareAdminTasks = fn(bag: &mut AdminTaskBag);

/// Tipo de función para declarar acciones locales (botones) de las páginas del panel.
pub type FnActionDeclareAdminActions = fn(bag: &mut AdminActionBag);

mod declare_admin_sections;
pub use declare_admin_sections::*;

mod declare_admin_pages;
pub use declare_admin_pages::*;

mod declare_admin_tasks;
pub use declare_admin_tasks::*;

mod declare_admin_actions;
pub use declare_admin_actions::*;
