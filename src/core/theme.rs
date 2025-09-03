//! API para añadir y gestionar nuevos temas.
//!
//! En PageTop un tema es la *piel* de la aplicación, decide cómo se muestra cada documento HTML,
//! especialmente las páginas de contenido ([`Page`](crate::response::page::Page)), sin alterar la
//! lógica interna de sus componentes.
//!
//! Un tema **declara las regiones** (*cabecera*, *barra lateral*, *pie*, etc.) que estarán
//! disponibles para colocar contenido. Los temas son responsables últimos de los estilos,
//! tipografías, espaciados y cualquier otro detalle visual o de comportamiento (como animaciones,
//! *scripts* de interfaz, etc.).
//!
//! Los temas son extensiones que implementan [`Extension`](crate::core::extension::Extension); por
//! lo que se instancian, declaran sus dependencias y se inician igual que el resto de extensiones;
//! pero serán temas si además implementan [`theme()`](crate::core::extension::Extension::theme) y
//! [`Theme`].

mod definition;
pub use definition::{Theme, ThemePage, ThemeRef};

mod regions;
pub(crate) use regions::{ChildrenInRegions, REGION_CONTENT};
pub use regions::{InRegion, Region};

pub(crate) mod all;
