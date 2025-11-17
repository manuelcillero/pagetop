use crate::core::component::{Child, ChildOp, Children};
use crate::core::theme::ThemeRef;
use crate::locale::L10n;
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

/// Define la interfaz mínima que describe una **región de renderizado** dentro de una página.
///
/// Una *región* representa una zona del documento HTML (por ejemplo: `"header"`, `"content"` o
/// `"sidebar-left"`), en la que se pueden incluir y renderizar componentes dinámicamente.
///
/// Este `trait` abstrae los metadatos básicos de cada región, esencialmente:
///
/// - su **clave interna** (`key()`), que la identifica de forma única dentro de la página, y
/// - su **etiqueta localizada** (`label()`), que se usa como texto accesible (por ejemplo en
///   `aria-label` o en descripciones semánticas del contenedor).
///
/// Las implementaciones típicas son *enumeraciones estáticas* declaradas por cada tema (ver como
/// ejemplo [`DefaultRegions`](crate::core::theme::DefaultRegions)), de modo que las claves y
/// etiquetas permanecen inmutables y fácilmente referenciables.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// pub enum MyThemeRegions {
///     Header,
///     Content,
///     Footer,
/// }
///
/// impl Region for MyThemeRegions {
///     fn key(&self) -> &str {
///         match self {
///             Self::Header => "header",
///             Self::Content => "content",
///             Self::Footer => "footer",
///         }
///     }
///
///     fn label(&self) -> L10n {
///         L10n::l(join!("region__", self.key()))
///     }
/// }
/// ```
pub trait Region: Send + Sync {
    /// Devuelve la **clave interna** que identifica de forma única una región.
    ///
    /// La clave se utiliza para asociar los componentes de la región con su contenedor HTML
    /// correspondiente. Por convención, se emplean nombres en minúsculas y con guiones (`"header"`,
    /// `"main"`, `"sidebar-right"`, etc.), y la región `"content"` es **obligatoria** en todos los
    /// temas.
    fn key(&self) -> &str;

    /// Devuelve la **etiqueta localizada** (`L10n`) asociada a la región.
    ///
    /// Esta etiqueta se evalúa en el idioma activo de la página y se utiliza principalmente para
    /// accesibilidad, como el valor de `aria-label` en el contenedor generado por
    /// [`ThemePage::render_region()`](crate::core::theme::ThemePage::render_region).
    fn label(&self) -> L10n;
}

/// Referencia estática a una región.
pub type RegionRef = &'static dyn Region;

// Contenedor interno de componentes agrupados por región.
#[derive(AutoDefault)]
pub struct ChildrenInRegions(HashMap<&'static str, Children>);

impl ChildrenInRegions {
    pub fn with(region_key: &'static str, child: Child) -> Self {
        ChildrenInRegions::default().with_child_in(region_key, ChildOp::Add(child))
    }

    #[builder_fn]
    pub fn with_child_in(mut self, region_key: &'static str, op: ChildOp) -> Self {
        if let Some(region) = self.0.get_mut(region_key) {
            region.alter_child(op);
        } else {
            self.0.insert(region_key, Children::new().with_child(op));
        }
        self
    }

    pub fn merge_all_components(&self, theme_ref: ThemeRef, region_key: &'static str) -> Children {
        let common = COMMON_REGIONS.read();
        if let Some(r) = THEME_REGIONS.read().get(&theme_ref.type_id()) {
            Children::merge(&[
                common.0.get(region_key),
                self.0.get(region_key),
                r.0.get(region_key),
            ])
        } else {
            Children::merge(&[common.0.get(region_key), self.0.get(region_key)])
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
    Content,
    /// Región identificada por la clave proporcionado.
    Key(&'static str),
    /// Región identificada por una clave para un tema concreto.
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
    /// InRegion::Content.add(Child::with(Html::with(|_|
    ///     html! { ("🎉 ¡Bienvenido!") }
    /// )));
    ///
    /// // Texto en la región "sidebar".
    /// InRegion::Key("sidebar").add(Child::with(Html::with(|_|
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
            InRegion::Key(region_key) => {
                COMMON_REGIONS
                    .write()
                    .alter_child_in(region_key, ChildOp::Add(child));
            }
            InRegion::OfTheme(region_key, theme_ref) => {
                let mut regions = THEME_REGIONS.write();
                if let Some(r) = regions.get_mut(&theme_ref.type_id()) {
                    r.alter_child_in(region_key, ChildOp::Add(child));
                } else {
                    regions.insert(
                        theme_ref.type_id(),
                        ChildrenInRegions::with(region_key, child),
                    );
                }
            }
        }
        self
    }
}
