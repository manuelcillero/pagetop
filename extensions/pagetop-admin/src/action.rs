//! Acción para declarar páginas de ajustes que `pagetop-admin` pinta y guarda por sí solo.

use pagetop::prelude::*;

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::settings::SettingsSchema;

// **< ConfigFormPage >*****************************************************************************

/// Página de ajustes del panel: una [`AdminPage`] y el esquema de su formulario.
///
/// `pagetop-admin` la añade al registro del panel como cualquier otra página, monta su ruta y
/// responde a su `GET` (el formulario con los valores guardados) y a su `POST` (los guarda en la
/// base de datos). La extensión que la declara no registra ninguna ruta.
#[derive(Clone)]
pub struct ConfigFormPage {
    /// Página del panel, con su ruta relativa a la raíz del panel.
    pub page: AdminPage,
    /// Esquema del formulario.
    pub schema: SettingsSchema,
}

/// Bolsa de páginas de ajustes para [`DeclareConfigForms`].
pub struct ConfigFormBag {
    forms: Vec<ConfigFormPage>,
}

impl ConfigFormBag {
    /// Añade una página de ajustes.
    pub fn add(&mut self, form: ConfigFormPage) {
        self.forms.push(form);
    }
}

// **< DeclareConfigForms >*************************************************************************

/// Tipo de función para declarar páginas de ajustes.
pub type FnActionDeclareConfigForms = fn(bag: &mut ConfigFormBag);

/// Acción para declarar páginas de ajustes que gestiona `pagetop-admin` (ver [`ConfigFormPage`]).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_admin::prelude::*;
///
/// #[derive(Clone, Copy, Debug)]
/// enum MyPermission {
///     Config,
/// }
///
/// impl Permission for MyPermission {
///     fn key(&self) -> CowStr {
///         "myapp:config".into()
///     }
/// }
///
/// fn declare_forms(bag: &mut ConfigFormBag) {
///     bag.add(ConfigFormPage {
///         page: AdminPage::new("config/myapp", "config", Lc::n("My App"), &MyPermission::Config)
///             .with_description(Lc::n("Configure My App.")),
///         schema: SettingsSchema::new("myapp.config").with_field(
///             SettingField::text("site_name", Lc::n("Site name")).with_required(true),
///         ),
///     });
/// }
/// // En Extension::actions():
/// // DeclareConfigForms::new(declare_forms)
/// ```
pub struct DeclareConfigForms {
    f: FnActionDeclareConfigForms,
    weight: Weight,
}

impl ActionDispatcher for DeclareConfigForms {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareConfigForms {
    /// Registra una nueva acción para declarar páginas de ajustes.
    pub fn new(f: FnActionDeclareConfigForms) -> Self {
        DeclareConfigForms { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< Registro de las páginas de ajustes >*********************************************************

// Esquemas de las páginas de ajustes, por su ruta relativa a la raíz del panel.
static CONFIG_FORMS: OnceLock<BTreeMap<String, SettingsSchema>> = OnceLock::new();

// Añade las páginas de ajustes al registro del panel. `pagetop-admin` lo registra como una acción
// `DeclareAdminPages`, así que se ejecuta una sola vez, cuando el núcleo construye el registro.
pub(crate) fn declare_config_form_pages(bag: &mut AdminPageBag) {
    let mut forms = ConfigFormBag { forms: Vec::new() };
    dispatch_actions(|action: &DeclareConfigForms| (action.f)(&mut forms));
    let mut schemas = BTreeMap::new();
    for form in forms.forms {
        schemas.insert(form.page.path().to_owned(), form.schema);
        bag.add(form.page);
    }
    let _ = CONFIG_FORMS.set(schemas);
}

// Esquemas de las páginas de ajustes, por su ruta relativa a la raíz del panel.
pub(crate) fn config_forms() -> &'static BTreeMap<String, SettingsSchema> {
    static EMPTY: BTreeMap<String, SettingsSchema> = BTreeMap::new();
    CONFIG_FORMS.get().unwrap_or(&EMPTY)
}
