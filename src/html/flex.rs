//! Definiciones para el posicionamiento de componentes con [Flexbox].
//!
//! Por un lado está [`Flex`], el **componente contenedor** disponible en
//! [`pagetop::base::component`] que permite estructurar un conjunto de componentes hijo como un
//! grupo Flexbox sobre el que se aplican propiedades de presentación (dirección, ajuste de línea,
//! alineación, espaciado), con sus propios constructores (`new()`, `at()`, `inline()`,
//! `inline_at()`) y builders (`with_direction()`, `with_gap()`, etc.).
//!
//! Por otro, aquí se define [`FlexItem`], que es la **configuración** que se aplica sobre un único
//! elemento respecto a su contenedor flex padre (crecimiento, reducción, alineación individual,
//! orden, ancho y desplazamiento). No tiene un builder propio ya que podría usarse sobre cualquier
//! componente. Por eso se aplica pasándolo directamente al `with_prop()` que normalmente ya expone
//! cualquier componente, gracias a su implementación `From` hacia [`PropsOp`].
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
//! # Alineación y espaciado: ver [`align`]
//!
//! `align`/`align_content`/`gap` en [`Flex`] y `align_self` en [`FlexItem`] no tienen tipos propios
//! de este módulo. Usan [`align::Items`], [`align::Content`], [`align::Gap`] y [`align::ItemSelf`],
//! tipos declarados en [`pagetop::html::align`] con las propiedades que Flexbox y CSS Grid
//! resuelven de manera idéntica.
//!
//! [Flexbox]: https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Flexible_box_layout
//! [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style
//! [`pagetop::html::align`]: crate::html::align
//! [`align`]: crate::html::align
//! [`align::Items`]: crate::html::align::Items
//! [`align::Content`]: crate::html::align::Content
//! [`align::Gap`]: crate::html::align::Gap
//! [`align::ItemSelf`]: crate::html::align::ItemSelf
//! [`PropsOp`]: crate::html::props::PropsOp
//! [`pagetop::base::component`]: crate::base::component
//! [`Flex`]: crate::base::component::Flex

mod props_container;
pub use props_container::{Behavior, ContentJustify, Direction};

mod props_item;
pub use props_item::{ItemGrow, ItemOffset, ItemOrder, ItemShrink, ItemSize};

mod item;
pub use item::FlexItem;
