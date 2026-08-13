use crate::prelude::*;

/// Fila de datos (`<tr>`) de una [`Table`], formada por una lista de celdas ([`table::Cell`]).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let row = table::Row::new()
///     .with_cell("Julia")
///     .with_cell("ana@example.com");
/// assert_eq!(row.cells().len(), 2);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Row {
    /// Devuelve identificador, clases CSS y atributos HTML de la fila (`<tr>`).
    props: Props,
    /// Devuelve las celdas de la fila, en orden de aparición.
    cells: Vec<table::Cell>,
}

impl Row {
    /// Crea una fila vacía.
    pub fn new() -> Self {
        Self::default()
    }

    // **< Row BUILDER >****************************************************************************

    /// Establece el identificador único de la fila.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML de la fila.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Añade una celda al final de la fila.
    ///
    /// Acepta directamente un `&str`, un `String` o un [`Lc`] (equivalen a `table::Cell::new(...)`
    /// con el contenido indicado), o un [`table::Cell`] ya construido (por ejemplo para asignarle
    /// clases o atributos propios, o para contener otros componentes).
    #[builder_fn]
    pub fn with_cell(mut self, cell: impl Into<table::Cell>) -> Self {
        self.cells.push(cell.into());
        self
    }
}
