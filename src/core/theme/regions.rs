use crate::core::component::{Child, ChildOp, Children};
use crate::core::theme::{ThemeRef, CONTENT_REGION_NAME};
use crate::{builder_fn, AutoDefault, UniqueId};

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::LazyLock;

// Regiones globales con componentes para un tema dado.
static THEME_REGIONS: LazyLock<RwLock<HashMap<UniqueId, ChildrenInRegions>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Regiones globales con componentes para cualquier tema.
static COMMON_REGIONS: LazyLock<RwLock<ChildrenInRegions>> =
    LazyLock::new(|| RwLock::new(ChildrenInRegions::default()));

// Estructura interna para mantener los componentes de una región.
#[derive(AutoDefault)]
pub struct ChildrenInRegions(HashMap<&'static str, Children>);

impl ChildrenInRegions {
    pub fn with(region_name: &'static str, child: Child) -> Self {
        ChildrenInRegions::default().with_child_in_region(region_name, ChildOp::Add(child))
    }

    #[builder_fn]
    pub fn with_child_in_region(mut self, region_name: &'static str, op: ChildOp) -> Self {
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

/// Permite añadir componentes a regiones globales o regiones de temas concretos.
///
/// Dada una región, según la variante seleccionada, se le podrán añadir ([`add()`](Self::add))
/// componentes que se mantendrán durante la ejecución de la aplicación.
///
/// Estas estructuras de componentes se renderizarán automáticamente al procesar los documentos HTML
/// que las usan, como las páginas de contenido ([`Page`](crate::response::page::Page)), por
/// ejemplo.
pub enum InRegion {
    /// Representa la región por defecto en la que se pueden añadir componentes.
    Content,
    /// Representa la región con el nombre del argumento.
    Named(&'static str),
    /// Representa la región con el nombre y del tema especificado en los argumentos.
    OfTheme(&'static str, ThemeRef),
}

impl InRegion {
    /// Permite añadir un componente en la región de la variante seleccionada.
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
                    .alter_child_in_region(CONTENT_REGION_NAME, ChildOp::Add(child));
            }
            InRegion::Named(name) => {
                COMMON_REGIONS
                    .write()
                    .alter_child_in_region(name, ChildOp::Add(child));
            }
            InRegion::OfTheme(region_name, theme_ref) => {
                let mut regions = THEME_REGIONS.write();
                if let Some(r) = regions.get_mut(&theme_ref.type_id()) {
                    r.alter_child_in_region(region_name, ChildOp::Add(child));
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
