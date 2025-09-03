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

/// Nombre de la región de contenido por defecto (`"content"`).
pub const REGION_CONTENT: &str = "content";

/// Identificador de una región de página.
///
/// Incluye una **clave estática** ([`key()`](Self::key)) que identifica la región en el tema, y un
/// **nombre normalizado** ([`name()`](Self::name)) en minúsculas para su uso en atributos HTML
/// (p.ej., clases `region__{name}`).
///
/// Se utiliza para declarar las regiones que componen una página en un tema (ver
/// [`page_regions()`](crate::core::theme::Theme::page_regions)).
pub struct Region {
    key: &'static str,
    name: String,
}

impl Default for Region {
    #[inline]
    fn default() -> Self {
        Self {
            key: REGION_CONTENT,
            name: String::from(REGION_CONTENT),
        }
    }
}

impl Region {
    /// Declara una región a partir de su clave estática.
    ///
    /// Genera además un nombre normalizado de la clave, eliminando espacios iniciales y finales,
    /// convirtiendo a minúsculas y sustituyendo los espacios intermedios por guiones (`-`).
    ///
    /// Esta clave se usará para añadir componentes a la región; por ello se recomiendan nombres
    /// sencillos, limitando los caracteres a `[a-z0-9-]` (p.ej., `"sidebar"` o `"main-menu"`), cuyo
    /// nombre normalizado coincidirá con la clave.
    #[inline]
    pub fn declare(key: &'static str) -> Self {
        Self {
            key,
            name: key.trim().to_ascii_lowercase().replace(' ', "-"),
        }
    }

    /// Devuelve la clave estática asignada a la región.
    #[inline]
    pub fn key(&self) -> &'static str {
        self.key
    }

    /// Devuelve el nombre normalizado de la región (para atributos y búsquedas).
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

// Contenedor interno de componentes agrupados por región.
#[derive(AutoDefault)]
pub struct ChildrenInRegions(HashMap<&'static str, Children>);

impl ChildrenInRegions {
    pub fn with(region_name: &'static str, child: Child) -> Self {
        ChildrenInRegions::default().with_child_in(region_name, ChildOp::Add(child))
    }

    #[builder_fn]
    pub fn with_child_in(mut self, region_name: &'static str, op: ChildOp) -> Self {
        if let Some(region) = self.0.get_mut(region_name) {
            region.alter_child(op);
        } else {
            self.0.insert(region_name, Children::new().with_child(op));
        }
        self
    }

    pub fn merge_all_components(&self, theme_ref: ThemeRef, region_name: &'static str) -> Children {
        let common = COMMON_REGIONS.read();
        if let Some(r) = THEME_REGIONS.read().get(&theme_ref.type_id()) {
            Children::merge(&[
                common.0.get(region_name),
                self.0.get(region_name),
                r.0.get(region_name),
            ])
        } else {
            Children::merge(&[common.0.get(region_name), self.0.get(region_name)])
        }
    }
}

/// Punto de acceso para añadir componentes a regiones globales o específicas de un tema.
///
/// Según la variante, se pueden añadir componentes ([`add()`](Self::add)) que permanecerán
/// disponibles durante toda la ejecución.
///
/// Estos componentes se renderizarán automáticamente al procesar los documentos HTML que incluyen
/// estas regiones, como las páginas de contenido ([`Page`](crate::response::page::Page)).
pub enum InRegion {
    /// Región de contenido por defecto.
    Content,
    /// Región identificada por el nombre proporcionado.
    Named(&'static str),
    /// Región identificada por un nombre y asociada a un tema concreto.
    OfTheme(&'static str, ThemeRef),
}

impl InRegion {
    /// Añade un componente a la región indicada por la variante.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// // Banner global, en la región por defecto de cualquier página.
    /// InRegion::Content.add(Child::with(Html::with(|_|
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
            InRegion::Content => {
                COMMON_REGIONS
                    .write()
                    .alter_child_in(REGION_CONTENT, ChildOp::Add(child));
            }
            InRegion::Named(region_name) => {
                COMMON_REGIONS
                    .write()
                    .alter_child_in(region_name, ChildOp::Add(child));
            }
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
}
