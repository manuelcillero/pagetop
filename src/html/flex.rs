//! Definiciones para el posicionamiento de componentes con [Flexbox].
//!
//! [`Flex`] configura un contenedor y sus hijos como un grupo sobre el que se aplican propiedades
//! de presentación (dirección, ajuste de línea, alineación, espaciado). Lo usan componentes que
//! ofrecen su propio `with_flex()`, como [`Container`] o [`Navbar`].
//!
//! [`FlexItem`] configura, en cambio, un único elemento en relación con el contenedor flex de su
//! padre (crecimiento, reducción, alineación individual, orden, ancho y desplazamiento). Al poder
//! acabar aplicándose sobre cualquier componente (no sólo los que ofrecen `with_flex()`), no tiene
//! un builder propio: se aplica con [`PropsOp::flex_item()`] sobre el `with_prop()` que ya expone
//! cualquier componente.
//!
//! # Un entorno autosuficiente
//!
//! Toda la configuración de `Flex`/`FlexItem` se resuelve con estilos en línea (`style="..."`),
//! nunca como clases CSS (consulta el propio [`Flex`] para ver el porqué). Los estilos en línea
//! tienen la especificidad más alta que existe en CSS, salvo `!important`, así que ningún framework
//! CSS de terceros, ni el CSS de la propia aplicación, puede sobrescribirlo por accidente. Funciona
//! igual conviva con quien conviva en la misma página, sin necesidad de coordinar nombres de clase
//! ni orden alguno en la carga de hojas de estilo.
//!
//! [Flexbox]: https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Flexible_box_layout
//! [`Container`]: crate::base::component::Container
//! [`Navbar`]: crate::base::component::Navbar
//! [`PropsOp::flex_item()`]: crate::html::props::PropsOp::flex_item

mod props_container;
pub use props_container::{Align, AlignContent, Behavior, ContentJustify, Direction, Gap};

mod props_item;
pub use props_item::{ItemAlign, ItemGrow, ItemOffset, ItemOrder, ItemShrink, ItemSize};

mod container;
pub use container::Flex;

mod item;
pub use item::FlexItem;
