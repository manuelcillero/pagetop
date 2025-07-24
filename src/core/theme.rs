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
//! Es una extensión más (implementando [`ExtensionTrait`](crate::core::extension::ExtensionTrait)).
//! Se instala, activa y declara dependencias igual que el resto de extensiones; y se señala a sí
//! misma como tema (implementando [`theme()`](crate::core::extension::ExtensionTrait::theme)
//! y [`ThemeTrait`]).

mod definition;
pub use definition::{ThemeRef, ThemeTrait};

mod regions;
pub(crate) use regions::ChildrenInRegions;
pub use regions::InRegion;

pub(crate) mod all;
