//! Definiciones para el posicionamiento de componentes con [Flexbox].
//!
//! [`Flex`] configura un contenedor y sus hijos como un grupo sobre el que se aplican propiedades
//! de presentación (dirección, ajuste de línea, alineación, espaciado). Lo usan componentes que
//! ofrecen su propio `with_flex()`, como [`Container`] o [`Navbar`].
//!
//! [`FlexItem`] configura, en cambio, un único elemento en relación con el contenedor flex de su
//! padre (crecimiento, reducción, alineación individual, orden, ancho y desplazamiento). No tiene
//! un builder propio ya que puede acabar aplicándose sobre cualquier componente (no sólo los que
//! ofrecen `with_flex()`). Por eso se aplica pasándolo directamente al `with_prop()` que
//! normalmente ya expone cualquier componente, gracias a su `From` hacia [`PropsOp`].
//!
//! # Un entorno nativo autosuficiente
//!
//! Toda la configuración de `Flex`/`FlexItem` se resuelve generando clases CSS dinámicamente y de
//! manera independiente a cualquier tema o framework CSS. El nombre interno de cada clase se deriva
//! de la propiedad y el valor que representa, así que dos elementos con la misma configuración
//! comparten la misma regla en vez de duplicarla. Las declaraciones correspondientes se registran
//! vía [`AssetsOp::add_responsive_style()`] y se renderizan como reglas en el `<head>` del
//! documento. Funciona igual conviva con quien conviva en la misma página, sin necesidad de
//! coordinar nombres de clase ni orden alguno en la carga de hojas de estilo.
//!
//! [Flexbox]: https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Flexible_box_layout
//! [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style
//! [`PropsOp`]: crate::html::props::PropsOp
//! [`Container`]: crate::base::component::Container
//! [`Navbar`]: crate::base::component::Navbar

mod props_container;
pub use props_container::{Align, AlignContent, Behavior, ContentJustify, Direction, Gap};

mod props_item;
pub use props_item::{ItemAlign, ItemGrow, ItemOffset, ItemOrder, ItemShrink, ItemSize};

mod container;
pub use container::Flex;

mod item;
pub use item::FlexItem;
