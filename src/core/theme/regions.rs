use crate::core::component::{Child, ChildOp, Children};
use crate::core::theme::{DefaultRegion, RegionRef, ThemeRef};
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
    pub fn with(region_ref: RegionRef, child: Child) -> Self {
        Self::default().with_child_in(region_ref, ChildOp::Add(child))
    }

    #[builder_fn]
    pub fn with_child_in(mut self, region_ref: RegionRef, op: ChildOp) -> Self {
        if let Some(region) = self.0.get_mut(region_ref.name()) {
            region.alter_child(op);
        } else {
            self.0
                .insert(region_ref.name().to_owned(), Children::new().with_child(op));
        }
        self
    }

    pub fn children_for(&self, theme_ref: ThemeRef, region_ref: RegionRef) -> Children {
        let name = region_ref.name();
        let common = COMMON_REGIONS.read();
        let themed = THEME_REGIONS.read();

        if let Some(r) = themed.get(&theme_ref.type_id()) {
            Children::merge(&[common.0.get(name), self.0.get(name), r.0.get(name)])
        } else {
            Children::merge(&[common.0.get(name), self.0.get(name)])
        }
    }
}

/// Añade componentes a regiones globales o específicas de un tema.
///
/// Cada variante indica la región en la que se añade el componente usando [`Self::add()`]. Los
/// componentes añadidos se mantienen durante toda la ejecución y se inyectan automáticamente al
/// renderizar los documentos HTML que utilizan esas regiones, como las páginas de contenido
/// ([`Page`](crate::response::page::Page)).
pub enum InRegion {
    /// Región principal de **contenido** por defecto.
    ///
    /// Añade el componente a la región lógica de contenido principal de la aplicación. Por
    /// convención, esta región corresponde a [`DefaultRegion::Content`], cuyo nombre es
    /// `"content"`. Cualquier tema que renderice esa misma región de contenido, ya sea usando
    /// directamente [`DefaultRegion::Content`] o cualquier otra implementación de
    /// [`Region`](crate::core::theme::Region) que devuelva ese mismo nombre, mostrará los
    /// componentes registrados aquí, aunque lo harán según su propio método de renderizado
    /// ([`Region::render()`](crate::core::theme::Region::render)).
    Content,
    /// Región global compartida por todos los temas.
    ///
    /// Los componentes añadidos aquí se asocian al nombre de la región indicado por [`RegionRef`],
    /// es decir, al valor devuelto por [`Region::name()`](crate::core::theme::Region::name) para
    /// esa región. Se mostrarán en cualquier tema cuya plantilla renderice una región que devuelva
    /// ese mismo nombre.
    Global(RegionRef),
    /// Región asociada a un tema concreto.
    ///
    /// Los componentes sólo se renderizarán cuando el documento se procese con el tema indicado y
    /// se utilice la región referenciada. Resulta útil para añadir contenido específico en un tema
    /// sin afectar a otros.
    ForTheme(ThemeRef, RegionRef),
}

impl InRegion {
    /// Añade un componente a la región indicada por la variante.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// // Banner global en la región por defecto.
    /// InRegion::Content.add(Child::with(Html::with(|_| {
    ///     html! { "🎉 ¡Bienvenido!" }
    /// })));
    ///
    /// // Texto en la cabecera.
    /// InRegion::Global(&DefaultRegion::Header).add(Child::with(Html::with(|_| {
    ///     html! { "Publicidad" }
    /// })));
    ///
    /// // Contenido sólo para la región del pie de página en un tema concreto.
    /// InRegion::ForTheme(&theme::Basic, &DefaultRegion::Footer).add(Child::with(Html::with(|_| {
    ///     html! { "Aviso legal" }
    /// })));
    /// ```
    pub fn add(&self, child: Child) -> &Self {
        match self {
            InRegion::Content => Self::add_to_common(&DefaultRegion::Content, child),
            InRegion::Global(region_ref) => Self::add_to_common(*region_ref, child),
            InRegion::ForTheme(theme_ref, region_ref) => {
                let mut regions = THEME_REGIONS.write();
                if let Some(r) = regions.get_mut(&theme_ref.type_id()) {
                    r.alter_child_in(*region_ref, ChildOp::Add(child));
                } else {
                    regions.insert(
                        theme_ref.type_id(),
                        ChildrenInRegions::with(*region_ref, child),
                    );
                }
            }
        }
        self
    }

    #[inline]
    fn add_to_common(region_ref: RegionRef, child: Child) {
        COMMON_REGIONS
            .write()
            .alter_child_in(region_ref, ChildOp::Add(child));
    }
}
