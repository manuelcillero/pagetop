//! API para añadir y gestionar nuevos temas.
//!
//! En `PageTop` un tema es la *piel* de la aplicación, decide cómo se muestra cada documento HTML,
//! especialmente las páginas de contenido ([`Page`](crate::response::page::Page)), sin alterar la
//! lógica interna de sus componentes.
//!
//! Un tema **declara las regiones** (*cabecera*, *barra lateral*, *pie*, etc.) que estarán
//! disponibles para colocar contenido. Los temas son responsables últimos de los estilos,
//! tipografías, espaciados y cualquier otro detalle visual o de comportamiento (como animaciones,
//! *scripts* de interfaz, etc.).
//!
//! Es una extensión más (implementando [`Extension`](crate::core::extension::Extension)). Se
//! instala, activa y declara dependencias igual que el resto de extensiones; y se señala a sí misma
//! como tema (implementando [`theme()`](crate::core::extension::Extension::theme) y [`Theme`]).

mod definition;
pub use definition::{Theme, ThemeRef};

mod regions;
pub(crate) use regions::ChildrenInRegions;
pub use regions::InRegion;

pub(crate) mod all;

/// Nombre de la región por defecto: `content`.
pub const CONTENT_REGION_NAME: &str = "content";
