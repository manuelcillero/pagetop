use crate::base::component::Region;
use crate::core::component::{Child, ChildOp, Children};
use crate::core::theme::ThemeRef;
use crate::{builder_fn, AutoDefault, UniqueId};

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::LazyLock;

// Conjunto de regiones globales asociadas a un tema específico.
static THEME_REGIONS: LazyLock<RwLock<HashMap<UniqueId, ChildrenInRegions>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Conjunto de regiones globales comunes a todos los temas.
static COMMON_REGIONS: LazyLock<RwLock<ChildrenInRegions>> =
    LazyLock::new(|| RwLock::new(ChildrenInRegions::default()));

// Contenedor interno de componentes agrupados por región.
#[derive(AutoDefault)]
pub(crate) struct ChildrenInRegions(HashMap<String, Children>);

impl ChildrenInRegions {
    pub fn with(region_name: impl AsRef<str>, child: Child) -> Self {
        Self::default().with_child_in(region_name, ChildOp::Add(child))
    }

    #[builder_fn]
    pub fn with_child_in(mut self, region_name: impl AsRef<str>, op: ChildOp) -> Self {
        let name = region_name.as_ref();
        if let Some(region) = self.0.get_mut(name) {
            region.alter_child(op);
        } else {
            self.0
                .insert(name.to_owned(), Children::new().with_child(op));
        }
        self
    }

    pub fn children_for(&self, theme_ref: ThemeRef, region_name: impl AsRef<str>) -> Children {
        let name = region_name.as_ref();
        let common = COMMON_REGIONS.read();
        let themed = THEME_REGIONS.read();

        if let Some(r) = themed.get(&theme_ref.type_id()) {
            Children::merge(&[common.0.get(name), self.0.get(name), r.0.get(name)])
        } else {
            Children::merge(&[common.0.get(name), self.0.get(name)])
        }
    }
}

/// Permite añadir componentes a regiones globales o específicas de un tema.
///
/// Según la variante, se pueden añadir componentes ([`add()`](Self::add)) que permanecerán
/// disponibles durante toda la ejecución.
///
/// Estos componentes se renderizarán automáticamente al procesar los documentos HTML que incluyen
/// estas regiones, como las páginas de contenido ([`Page`](crate::response::page::Page)).
pub enum InRegion {
    /// Región de contenido por defecto.
    Default,
    /// Región identificada por el nombre proporcionado.
    Named(&'static str),
    /// Región identificada por su nombre para un tema concreto.
    OfTheme(&'static str, ThemeRef),
}

impl InRegion {
    /// Añade un componente a la región indicada por la variante.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// // Banner global, en la región por defecto de cualquier página.
    /// InRegion::Default.add(Child::with(Html::with(|_|
    ///     html! { ("🎉 ¡Bienvenido!") }
    /// )));
    ///
    /// // Texto en la región "sidebar".
    /// InRegion::Named("sidebar").add(Child::with(Html::with(|_|
    ///     html! { ("Publicidad") }
    /// )));
    /// ```
    pub fn add(&self, child: Child) -> &Self {
        match self {
            InRegion::Default => Self::add_to_common(Region::DEFAULT, child),
            InRegion::Named(region_name) => Self::add_to_common(region_name, child),
            InRegion::OfTheme(region_name, theme_ref) => {
                let mut regions = THEME_REGIONS.write();
                if let Some(r) = regions.get_mut(&theme_ref.type_id()) {
                    r.alter_child_in(region_name, ChildOp::Add(child));
                } else {
                    regions.insert(
                        theme_ref.type_id(),
                        ChildrenInRegions::with(region_name, child),
                    );
                }
            }
        }
        self
    }

    #[inline]
    fn add_to_common(region_name: &str, child: Child) {
        COMMON_REGIONS
            .write()
            .alter_child_in(region_name, ChildOp::Add(child));
    }
}
