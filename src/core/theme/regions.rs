use crate::core::component::{Child, ChildOp, Children, Component};
use crate::core::theme::{CoreRegions, RegionRef, ThemeRef};
use crate::{AutoDefault, UniqueId, builder_fn};

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

// Lista de prototipos de componentes por nombre de región.
//
// Utiliza Vec en lugar de HashMap. El número de regiones registradas por tema o aplicación es casi
// siempre de un dígito, así que una búsqueda lineal por igualdad de `&str` evita el coste de
// hashear la clave. Además, el trabajo para recorrer regiones vacías es mínimo.
//
// La clave es `&'static str` (lo que ya devuelve `RegionName::name()`) en lugar de `String`. No
// hace falta reservar en el heap una copia de un dato que ya vive de forma estática.
#[derive(AutoDefault)]
struct RegionComponents(Vec<(&'static str, Vec<Arc<dyn Component>>)>);

impl RegionComponents {
    // Devuelve los prototipos registrados para la región indicada, si hay alguno.
    fn get(&self, region_name: &str) -> Option<&Vec<Arc<dyn Component>>> {
        self.0
            .iter()
            .find(|(name, _)| *name == region_name)
            .map(|(_, protos)| protos)
    }

    // Añade un prototipo a la región indicada, creando la entrada si es la primera.
    //
    // Se comparte como `Arc<dyn Component>`. El prototipo no se clona aquí ni al ensamblar la
    // región (sólo se clona el `Arc`, barato). El único clonado real del componente ocurre en
    // `Child::render()`, cuando cada petición necesita su propia copia mutable para pasar por
    // `setup()` desde un estado inicial limpio.
    fn push(&mut self, region_name: &'static str, proto: Arc<dyn Component>) {
        match self.0.iter_mut().find(|(name, _)| *name == region_name) {
            Some((_, protos)) => protos.push(proto),
            None => self.0.push((region_name, vec![proto])),
        }
    }
}

// Regiones globales con prototipos asociados a un tema específico.
static THEME_REGIONS: LazyLock<RwLock<HashMap<UniqueId, RegionComponents>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Regiones globales con prototipos comunes a todos los temas.
static COMMON_REGIONS: LazyLock<RwLock<RegionComponents>> =
    LazyLock::new(|| RwLock::new(RegionComponents::default()));

// *************************************************************************************************

// Contenedor interno de componentes agrupados por región.
#[derive(AutoDefault)]
pub(crate) struct ChildrenInRegions(HashMap<&'static str, Children>);

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
            self.0.insert(region_name, children);
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
            result.add_many(
                global_protos
                    .iter()
                    .map(|proto| Child::from_arc(Arc::clone(proto))),
            );
        }
        // 2. Componentes propios de la página: se mueven, no se clonan.
        if let Some(page_children) = self.0.remove(region_name) {
            result.add_many(page_children);
        }
        // 3. Prototipos del tema activo.
        if let Some(theme_region) = THEME_REGIONS.read().get(&theme.type_id())
            && let Some(theme_protos) = theme_region.get(region_name)
        {
            result.add_many(
                theme_protos
                    .iter()
                    .map(|proto| Child::from_arc(Arc::clone(proto))),
            );
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
/// InRegion::Global(&CoreRegions::Header).add(Html::with(|_| html! { "Publicidad" }));
/// ```
pub enum InRegion {
    /// Región principal de **contenido** por defecto.
    ///
    /// Añade el componente a la región lógica de contenido principal de la aplicación. Internamente
    /// equivale a `InRegion::Global(&CoreRegions::Content)`.
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
    /// InRegion::Global(&CoreRegions::Header).add(Html::with(|_| {
    ///     html! { "Publicidad" }
    /// }));
    ///
    /// // Contenido sólo para la región del pie de página en un tema concreto.
    /// InRegion::ForTheme(&theme::Basic, &CoreRegions::Footer).add(Html::with(|_| {
    ///     html! { "Aviso legal" }
    /// }));
    /// ```
    pub fn add(&self, component: impl Component) -> &Self {
        let proto: Arc<dyn Component> = Arc::new(component);
        match self {
            InRegion::Content => Self::add_to_common(&CoreRegions::Content, proto),
            InRegion::Global(region) => Self::add_to_common(*region, proto),
            InRegion::ForTheme(theme, region) => {
                THEME_REGIONS
                    .write()
                    .entry(theme.type_id())
                    .or_default()
                    .push((*region).name(), proto);
            }
        }
        self
    }

    #[inline]
    fn add_to_common(region: RegionRef, proto: Arc<dyn Component>) {
        COMMON_REGIONS.write().push(region.name(), proto);
    }
}
