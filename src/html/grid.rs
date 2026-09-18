//! Definiciones para el posicionamiento de componentes con [CSS Grid].
//!
//! Al igual que ocurre con [`Flex`]/[`FlexItem`], por un lado está [`Grid`], el **componente
//! contenedor** de [`pagetop::base::component`], que define una rejilla de filas y columnas sobre
//! la que se aplican pistas, alineación y espaciado, con sus propios constructores (`new()`,
//! `at()`, `inline()`, `inline_at()`) y builders (`with_columns()`, `with_gap()`, etc.).
//!
//! Y por otro, aquí se define [`GridItem`], que es la **configuración** que se aplica sobre un
//! único elemento dentro de la rejilla de su padre (columna, fila, alineación individual). No tiene
//! un builder propio ya que podría usarse sobre cualquier componente. Por eso se aplica pasándolo
//! directamente al `with_prop()` que normalmente ya expone cualquier componente, gracias a su
//! implementación `From` hacia [`PropsOp`].
//!
//! # Un entorno nativo autosuficiente
//!
//! `Grid`/`GridItem` reutilizan el mismo esquema que [`Flex`]/[`FlexItem`] para generar clases CSS
//! dinámicamente, independientes de cualquier tema o framework CSS, registradas vía
//! [`AssetsOp::add_responsive_style()`] y renderizadas en el `<head>` del documento. El nombre de
//! cada clase se deriva de la propiedad y el valor que representa, así que dos elementos con la
//! misma configuración comparten la misma regla generada en lugar de duplicarla.
//!
//! # Qué se comparte con Flex, y qué no
//!
//! Ambos sistemas comparten varias propiedades con el mismo catálogo de valores, `align-items`,
//! `align-content`, `align-self`, `gap`, disponibles en [`pagetop::html::align`].
//!
//! No se comparte nada para `justify-items`/`justify-self` porque no hay equivalencia en Flexbox
//! (sólo tiene un eje principal y uno transversal, intercambiables con [`Direction`], no dos ejes
//! independientes como Grid) ni para `justify-content` ya que aunque el nombre de la propiedad
//! coincide con el de Flex, Grid admite `stretch` (un valor sin efecto en Flexbox que
//! [`flex::ContentJustify`] no modela), así que [`grid::ContentJustify`] es un catálogo propio de
//! este módulo, no compartido.
//!
//! [CSS Grid]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_grid_layout
//! [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style
//! [`pagetop::html::align`]: crate::html::align
//! [`FlexItem`]: crate::html::flex::FlexItem
//! [`flex::ContentJustify`]: crate::html::flex::ContentJustify
//! [`grid::ContentJustify`]: crate::html::grid::ContentJustify
//! [`Direction`]: crate::html::flex::Direction
//! [`PropsOp`]: crate::html::props::PropsOp
//! [`pagetop::base::component`]: crate::base::component
//! [`Flex`]: crate::base::component::Flex
//! [`Grid`]: crate::base::component::Grid

mod props_container;
pub use props_container::MAX_TRACKS;
pub use props_container::{AutoFlow, AxisTrack, ContentJustify, DefaultJustify, Tracks};

mod props_item;
pub use props_item::{ItemJustify, ItemPlacement};

mod item;
pub use item::GridItem;
