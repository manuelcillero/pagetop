use crate::core::component::{Child, ChildOp, Children, Component};
use crate::core::theme::{CoreRegion, RegionRef, ThemeRef};
use crate::{AutoDefault, UniqueId, builder_fn};

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

// Mapea cada nombre de región con su lista de prototipos de componentes.
//
// Se comparten como `Arc<dyn Component>`. El prototipo no se clona al registrarse ni al ensamblar
// la región (sólo se clona el `Arc`, barato). En cambio, sí se realiza un clonado del componente en
// `Child::render()`, cuando cada petición necesita su propia copia mutable para pasar por `setup()`
// desde un estado inicial limpio.
type RegionComponents = HashMap<String, Vec<Arc<dyn Component>>>;

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
    pub fn with(region: RegionRef, child: Child) -> Self {
        Self::default().with_child_in(region, child)
    }

    #[builder_fn]
    pub fn with_child_in(mut self, region: RegionRef, op: impl Into<ChildOp>) -> Self {
        let child = op.into();
        let region_name = region.name();
        if let Some(region) = self.0.get_mut(region_name) {
            region.alter_child(child);
        } else {
            let children = Children::new().with_child(child);
            self.0.insert(region_name.to_owned(), children);
        }
        self
    }

    /// Ensambla los componentes frescos de la región indicada.
    ///
    /// Se recogen desde tres fuentes disponibles, en el siguiente orden:
    ///
    /// 1. Prototipos globales comunes, disponibles en cualquier tema. Se comparten como `Arc` (sin
    ///    clonar el componente); `Child::render()` obtiene su propia copia mutable más adelante,
    ///    para que `setup()` parta siempre de un estado inicial limpio.
    /// 2. Componentes propios de la página, registrados para esta petición concreta. Se mueven en
    ///    lugar de clonarse, ya que son de un único uso.
    /// 3. Prototipos del tema activo, exclusivos del tema en curso. Se comparten igual que los
    ///    comunes.
    pub fn assemble_region(&mut self, theme: ThemeRef, region: RegionRef) -> Children {
        let mut result = Children::new();

        let region_name = region.name();

        // 1. Prototipos globales comunes.
        if let Some(global_protos) = COMMON_REGIONS.read().get(region_name) {
            for proto in global_protos {
                result.add(Child::from_arc(Arc::clone(proto)));
            }
        }
        // 2. Componentes propios de la página: se mueven, no se clonan.
        if let Some(page_children) = self.0.remove(region_name) {
            for child in page_children {
                result.add(child);
            }
        }
        // 3. Prototipos del tema activo.
        if let Some(theme_region) = THEME_REGIONS.read().get(&theme.type_id())
            && let Some(theme_protos) = theme_region.get(region_name)
        {
            for proto in theme_protos {
                result.add(Child::from_arc(Arc::clone(proto)));
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
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// // Banner global en la región de contenido.
/// InRegion::Content.add(Html::with(|_| html! { "🎉 ¡Bienvenido!" }));
///
/// // Texto en la cabecera, visible en todos los temas.
/// InRegion::Global(&CoreRegion::Header).add(Html::with(|_| html! { "Publicidad" }));
/// ```
pub enum InRegion {
    /// Región principal de **contenido** por defecto.
    ///
    /// Añade el componente a la región lógica de contenido principal de la aplicación. Internamente
    /// equivale a `InRegion::Global(&CoreRegion::Content)`.
    Content,
    /// Región global compartida por todos los temas.
    ///
    /// Los componentes añadidos aquí se asocian al nombre de la región indicado por [`RegionRef`],
    /// es decir, al valor devuelto por
    /// [`RegionName::name()`](crate::core::theme::RegionName::name) para esa región. Se mostrarán
    /// en cualquier tema que renderice la región que devuelva ese nombre.
    Global(RegionRef),
    /// Región asociada a un tema concreto.
    ///
    /// Los componentes sólo se renderizarán cuando el documento se procese exactamente con el tema
    /// indicado (no sirve un tema hijo que lo herede), y se utilice la región referenciada. A
    /// diferencia del resto de comportamiento de `Theme`, este registro no sigue la cadena
    /// `parent()`. Resulta útil para añadir contenido específico en un tema sin afectar a otros.
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
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// // Banner global en la región por defecto.
    /// InRegion::Content.add(Html::with(|_| {
    ///     html! { "🎉 ¡Bienvenido!" }
    /// }));
    ///
    /// // Texto en la cabecera.
    /// InRegion::Global(&CoreRegion::Header).add(Html::with(|_| {
    ///     html! { "Publicidad" }
    /// }));
    ///
    /// // Contenido sólo para la región del pie de página en un tema concreto.
    /// InRegion::ForTheme(&theme::Basic, &CoreRegion::Footer).add(Html::with(|_| {
    ///     html! { "Aviso legal" }
    /// }));
    /// ```
    pub fn add(&self, component: impl Component) -> &Self {
        let proto: Arc<dyn Component> = Arc::new(component);
        match self {
            InRegion::Content => Self::add_to_common(&CoreRegion::Content, proto),
            InRegion::Global(region) => Self::add_to_common(*region, proto),
            InRegion::ForTheme(theme, region) => {
                THEME_REGIONS
                    .write()
                    .entry(theme.type_id())
                    .or_default()
                    .entry((*region).name().to_owned())
                    .or_default()
                    .push(proto);
            }
        }
        self
    }

    #[inline]
    fn add_to_common(region: RegionRef, proto: Arc<dyn Component>) {
        COMMON_REGIONS
            .write()
            .entry(region.name().to_owned())
            .or_default()
            .push(proto);
    }
}
