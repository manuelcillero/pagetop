//! Acciones para alterar el contenido de las páginas a renderizar.

use crate::response::page::Page;

/// Tipo de función para manipular una página durante su construcción o renderizado.
///
/// Se emplea en acciones orientadas a modificar o inspeccionar una instancia de [`Page`]
/// directamente, sin acceder a los componentes individuales ni al contexto de renderizado.
///
/// Recibe una referencia mutable (`&mut`) a la página en cuestión.
pub type FnActionWithPage = fn(page: &mut Page);

mod before_render_body;
pub use before_render_body::*;

mod after_render_body;
pub use after_render_body::*;
