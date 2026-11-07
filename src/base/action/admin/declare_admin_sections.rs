use crate::prelude::*;

use super::FnActionDeclareAdminSections;

/// Acción para declarar secciones del panel de administración.
///
/// PageTop ya declara las secciones habituales: `"people"`, `"structure"`, `"config"`,
/// `"reports"` y `"help"`. Una sección declarada con la misma clave que otra la sustituye.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn declare_sections(bag: &mut AdminSectionBag) {
///     bag.add(AdminSection::new("tools", Lc::n("Tools")).with_weight(60));
/// }
/// // En Extension::actions():
/// // action::admin::DeclareAdminSections::new(declare_sections)
/// ```
pub struct DeclareAdminSections {
    f: FnActionDeclareAdminSections,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminSections {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminSections {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionDeclareAdminSections`].
    pub fn new(f: FnActionDeclareAdminSections) -> Self {
        DeclareAdminSections { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones sobre la bolsa de secciones.
    #[inline]
    pub(crate) fn dispatch(bag: &mut AdminSectionBag) {
        dispatch_actions(|action: &Self| (action.f)(bag));
    }
}
