//! API para añadir y gestionar nuevos temas.
//!
//! En PageTop un tema es la *piel* de la aplicación, decide cómo se muestra cada documento HTML,
//! especialmente las páginas de contenido ([`Page`](crate::response::page::Page)), sin alterar la
//! lógica interna de sus componentes.
//!
//! Un tema **declara las regiones** (*cabecera*, *barra lateral*, *pie*, etc.) que estarán
//! disponibles para colocar contenido. Los temas son responsables últimos de los estilos,
//! tipografías, espaciados y cualquier otro detalle visual o de comportamiento (como animaciones,
//! scripts de interfaz, etc.).
//!
//! Los temas son extensiones que implementan [`Extension`](crate::core::extension::Extension), por
//! lo que se instancian, declaran dependencias y se inician igual que cualquier otra extensión.
//! También deben implementar [`Theme`] y sobrescribir el método
//! [`Extension::theme()`](crate::core::extension::Extension::theme) para que PageTop pueda
//! registrarlos como temas

mod definition;
pub use definition::{Theme, ThemePage, ThemeRef, DefaultRegions};

mod regions;
pub(crate) use regions::{ChildrenInRegions, REGION_CONTENT};
pub use regions::{InRegion, Region, RegionRef};

pub(crate) mod all;
