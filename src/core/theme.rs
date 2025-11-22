//! API para añadir y gestionar nuevos temas.
//!
//! En PageTop un tema es la *piel* de la aplicación. Es responsable último de los estilos,
//! tipografías, espaciados y cualquier otro detalle visual o interactivo (animaciones, scripts de
//! interfaz, etc.).
//!
//! Un tema determina el aspecto final de un documento HTML sin alterar la lógica interna de los
//! componentes ni la estructura del documento, que queda definida por la plantilla
//! ([`Template`](crate::base::component::Template)) utilizada por cada página.
//!
//! Los temas son extensiones que implementan [`Extension`](crate::core::extension::Extension), por
//! lo que se instancian, declaran dependencias y se inician igual que cualquier otra extensión.
//! También deben implementar [`Theme`] y sobrescribir el método
//! [`Extension::theme()`](crate::core::extension::Extension::theme) para que PageTop pueda
//! registrarlos como temas.

mod definition;
pub use definition::{Theme, ThemeRef};

mod regions;
pub(crate) use regions::ChildrenInRegions;
pub use regions::InRegion;

pub(crate) mod all;
