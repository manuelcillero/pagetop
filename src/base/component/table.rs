//! Definiciones para representar tablas de datos ([`Table`]).

mod props;
pub use props::SortLink;

mod component;
pub use component::Table;

mod column;
pub use column::Column;

mod row;
pub use row::Row;

mod cell;
pub use cell::Cell;
