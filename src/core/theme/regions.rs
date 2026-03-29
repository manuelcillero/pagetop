use crate::core::component::{Child, ChildOp, Children, Component};
use crate::core::theme::{DefaultRegion, RegionRef, ThemeRef};
use crate::{builder_fn, AutoDefault, UniqueId};

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

// Permite almacenar un componente como prototipo en regiones globales.
//
// Se implementa automáticamente para todo tipo que implemente [`Component`] y [`Clone`]. En cada
// llamada a [`as_child`](Self::as_child) produce un clon fresco del estado original, de modo que
// cada página renderiza el componente desde su estado inicial sin acumular mutaciones de peticiones
// anteriores.
trait ComponentGlobal: Send + Sync {
    // Devuelve un nuevo [`Child`] con una copia independiente del componente original.
    fn as_child(&self) -> Child;
}

impl<T: Component + Clone + 'static> ComponentGlobal for T {
    #[inline]
    fn as_child(&self) -> Child {
        Child::with(self.clone())
    }
}

// Mapa de nombre de región a lista de prototipos de componentes.
type RegionComponents = HashMap<String, Vec<Arc<dyn ComponentGlobal>>>;

// Regiones globales con prototipos asociados a un tema específico.
static THEME_REGIONS: LazyLock<RwLock<HashMap<UniqueId, RegionComponents>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Regiones globales con prototipos comunes a todos los temas.
static COMMON_REGIONS: LazyLock<RwLock<RegionComponents>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// *************************************************************************************************

// Contenedor interno de componentes agrupados por región.
#[derive(AutoDefault)]
pub(crate) struct ChildrenInRegions(HashMap<String, Children>);

impl ChildrenInRegions {
    pub fn with(region_ref: RegionRef, child: Child) -> Self {
        Self::default().with_child_in(region_ref, child)
    }

    #[builder_fn]
    pub fn with_child_in(mut self, region_ref: RegionRef, op: impl Into<ChildOp>) -> Self {
        let child = op.into();
        if let Some(region) = self.0.get_mut(region_ref.name()) {
            region.alter_child(child);
        } else {
            self.0.insert(
                region_ref.name().to_owned(),
                Children::new().with_child(child),
            );
        }
        self
    }

    /// Construye una lista de componentes frescos para la región indicada.
    ///
    /// El orden es: prototipos globales comunes → children propios de la página →
    /// prototipos específicos del tema activo.
    ///
    /// Los prototipos globales se clonan en cada llamada (clon profundo gracias a
    /// [`ComponentClone`]), garantizando que `setup()` siempre parte del estado
    /// inicial. Los children propios de la página se mueven (son por petición y no necesitan
    /// clonarse).
    ///
    /// [`ComponentClone`]: crate::core::component::ComponentClone
    pub fn children_for(&mut self, theme_ref: ThemeRef, region_ref: RegionRef) -> Children {
        let name = region_ref.name();
        let common = COMMON_REGIONS.read();
        let themed = THEME_REGIONS.read();

        let mut result = Children::new();

        // 1. Prototipos globales comunes — clon fresco por cada página.
        if let Some(protos) = common.get(name) {
            for proto in protos {
                result.add(proto.as_child());
            }
        }
        // 2. Children propios de la página — se mueven (son por petición, no requieren clonado).
        if let Some(page_children) = self.0.remove(name) {
            for child in page_children {
                result.add(child);
            }
        }
        // 3. Prototipos del tema activo — clon fresco por cada página.
        if let Some(theme_map) = themed.get(&theme_ref.type_id()) {
            if let Some(protos) = theme_map.get(name) {
                for proto in protos {
                    result.add(proto.as_child());
                }
            }
        }

        result
    }
}

// *************************************************************************************************

/// Añade componentes a regiones globales o específicas de un tema.
///
/// Los componentes se almacenan como **prototipos**: cada página recibe un clon fresco en el
/// momento del renderizado, de modo que `setup()` se ejecuta siempre sobre un
/// estado inicial limpio sin acumular mutaciones de peticiones anteriores.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Banner global en la región de contenido.
/// InRegion::Content.add(Html::with(|_| html! { "🎉 ¡Bienvenido!" }));
///
/// // Texto en la cabecera, visible en todos los temas.
/// InRegion::Global(&DefaultRegion::Header).add(Html::with(|_| html! { "Publicidad" }));
/// ```
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
    /// Añade un componente como prototipo en la región indicada por la variante.
    ///
    /// El componente se almacena internamente como prototipo. Cada vez que se renderiza una página,
    /// se genera un clon fresco del estado original, garantizando que `setup()` no
    /// acumula estado entre peticiones.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// // Banner global en la región por defecto.
    /// InRegion::Content.add(Html::with(|_| {
    ///     html! { "🎉 ¡Bienvenido!" }
    /// }));
    ///
    /// // Texto en la cabecera.
    /// InRegion::Global(&DefaultRegion::Header).add(Html::with(|_| {
    ///     html! { "Publicidad" }
    /// }));
    ///
    /// // Contenido sólo para la región del pie de página en un tema concreto.
    /// InRegion::ForTheme(&theme::Basic, &DefaultRegion::Footer).add(Html::with(|_| {
    ///     html! { "Aviso legal" }
    /// }));
    /// ```
    pub fn add(&self, component: impl Component + Clone + 'static) -> &Self {
        let proto: Arc<dyn ComponentGlobal> = Arc::new(component);
        match self {
            InRegion::Content => Self::add_to_common(&DefaultRegion::Content, proto),
            InRegion::Global(region_ref) => Self::add_to_common(*region_ref, proto),
            InRegion::ForTheme(theme_ref, region_ref) => {
                THEME_REGIONS
                    .write()
                    .entry(theme_ref.type_id())
                    .or_default()
                    .entry((*region_ref).name().to_owned())
                    .or_default()
                    .push(proto);
            }
        }
        self
    }

    #[inline]
    fn add_to_common(region_ref: RegionRef, proto: Arc<dyn ComponentGlobal>) {
        COMMON_REGIONS
            .write()
            .entry(region_ref.name().to_owned())
            .or_default()
            .push(proto);
    }
}
