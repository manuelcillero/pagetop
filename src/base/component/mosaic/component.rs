use crate::prelude::*;

/// Componente que reparte sus hijos en un **mosaico** de columnas.
///
/// Pensado para bloques de alturas distintas (tarjetas, resúmenes, paneles): cada uno se coloca en
/// la columna más corta, de modo que no quedan espacios vacíos bajo los bloques más bajos como
/// pasaría en una rejilla por filas. Por defecto, el número de columnas depende del ancho del
/// propio mosaico y del ancho mínimo de columna, y en pantallas estrechas queda una sola; con
/// [`with_columns()`](Self::with_columns) y [`with_columns_at()`](Self::with_columns_at) se fija en
/// cambio el número de columnas a partir de cada punto de corte del tema.
///
/// Aporta sus propios recursos, sea cual sea el tema. Con JavaScript, los bloques se colocan en el
/// orden en que se añaden siguiendo el algoritmo de `display: grid-lanes`, el mosaico nativo de CSS
/// que todavía no admiten todos los navegadores: cada uno va a la columna más corta, pero las que
/// quedan como mucho a una tolerancia de la más corta (`--mosaic-tolerance`, por defecto `1em`) se
/// consideran empatadas, y entre ellas gana la primera a partir del bloque anterior. Así un bloque
/// no salta a otra columna por unos píxeles y el orden de lectura se parece al de colocación; con
/// una tolerancia muy grande (p. ej. `9999px`), los bloques se colocan estrictamente en orden, una
/// columna tras otra. Se vuelven a repartir cuando cambia el tamaño de alguno, cuando cambia el
/// ancho de la ventana y cuando se añaden o quitan bloques, y tras cada reparto el mosaico emite el
/// evento `mosaic:layout`. Sin JavaScript, y al
/// imprimir, se ven en columnas CSS, que se llenan de arriba abajo antes de pasar a la siguiente.
/// El orden de lectura y de tabulación es siempre el de los bloques añadidos, se vean en la columna
/// que se vean.
///
/// Cada hijo se envuelve en su propio bloque, salvo un [`mosaic::Item`](super::Item), que es ya el
/// bloque y le da ajustes propios (ocupar varias columnas o todas, o quedar fijado a una, también a
/// partir de cada punto de corte). Los que no renderizan nada no ocupan sitio (si no queda ninguno,
/// el mosaico tampoco se renderiza):
///
/// ```html
/// <div class="mosaic">
///   <div class="mosaic-item">...</div>
///   <div class="mosaic-item">...</div>
/// </div>
/// ```
///
/// El ancho mínimo de columna es la propiedad CSS `--mosaic-column-width` (por defecto `16rem`), y
/// el espacio entre bloques, `--mosaic-gap` (por defecto `1rem`), que vale para las dos direcciones
/// salvo que se cambie sólo la vertical (`--mosaic-row-gap`) o la horizontal
/// (`--mosaic-column-gap`). Se pueden cambiar en el tema, con cualquier regla para `.mosaic` (los
/// valores por defecto no tienen especificidad), o en el propio componente; el espacio también con
/// [`with_gap()`](Self::with_gap) y [`with_gap_at()`](Self::with_gap_at), por punto de corte.
///
/// Las dos formas de decidir las columnas responden a cosas distintas: el ancho mínimo mira el
/// espacio del propio mosaico, así que se adapta también dentro de una columna lateral o de un
/// panel estrecho; los puntos de corte miran el ancho de la ventana, como el resto de la maqueta
/// del tema.
///
/// # Ejemplos
///
/// Columnas según el espacio disponible, de al menos `20rem`:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let mosaic = Mosaic::new()
///     .with_prop(PropsOp::add_style("--mosaic-column-width", "20rem"))
///     .with_child(Block::new().with_title(Lc::n("News")))
///     .with_child(Block::new().with_title(Lc::n("Events")));
/// ```
///
/// Una columna en móviles, dos desde `Md` y tres desde `Xl`, con más espacio entre bloques desde
/// `Xl` y un bloque destacado que ocupa dos columnas:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let mosaic = Mosaic::new()
///     .with_columns(1)
///     .with_columns_at(Breakpoint::Md, 2)
///     .with_columns_at(Breakpoint::Xl, 3)
///     .with_gap_at(Breakpoint::Xl, align::Gap::Both(UnitValue::RelRem(1.5)))
///     .with_child(
///         mosaic::Item::new()
///             .with_span(2)
///             .with_child(Block::new().with_title(Lc::n("Highlights"))),
///     )
///     .with_child(Block::new().with_title(Lc::n("News")))
///     .with_child(Block::new().with_title(Lc::n("Events")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Mosaic {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el número de columnas por punto de corte, si se ha fijado.
    columns: Responsive<u8>,
    /// Devuelve el espacio entre bloques por punto de corte, si se ha fijado.
    gap: Responsive<align::Gap>,
    /// Devuelve la lista de componentes hijo del mosaico.
    children: Children,
}

#[async_trait]
impl Component for Mosaic {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes("mosaic"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Un `mosaic::Item` ya pinta su propio bloque; el resto de hijos se envuelven en uno.
        let mut items = Vec::with_capacity(self.children().len());
        for child in self.children().iter() {
            let output = child.render(cx).await;
            if output.is_empty() {
                continue;
            }
            items.push(if child.downcast_ref::<mosaic::Item>().is_some() {
                output
            } else {
                html! { div class="mosaic-item" { (output) } }
            });
        }
        if items.is_empty() {
            return Ok(html! {});
        }
        cx.alter_assets(
            StyleSheet::from("/pagetop/css/mosaic.min.css").with_version(PAGETOP_VERSION),
        );
        cx.alter_assets(
            JavaScript::defer("/pagetop/js/mosaic.min.js").with_version(PAGETOP_VERSION),
        );
        let mut classes = String::new();
        self.columns_classes(cx, &mut classes);
        self.gap_classes(cx, &mut classes);
        Ok(html! {
            div (self.props().unpack_with_classes(cx, classes)) {
                @for item in items {
                    (item)
                }
            }
        })
    }
}

#[builder_impl]
impl Mosaic {
    // **< Mosaic BUILDER >*************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Fija el número de columnas, sin punto de corte: se aplica siempre, salvo donde lo cambie
    /// [`with_columns_at()`](Self::with_columns_at). Debe ser al menos 1.
    pub fn with_columns(mut self, columns: u8) -> Self {
        debug_assert!(columns > 0, "Mosaic needs at least one column");
        self.columns = self.columns.set(columns);
        self
    }

    /// Fija el número de columnas a partir del punto de corte indicado. Debe ser al menos 1.
    pub fn with_columns_at(mut self, bp: Breakpoint, columns: u8) -> Self {
        debug_assert!(columns > 0, "Mosaic needs at least one column");
        self.columns = self.columns.set_at(bp, columns);
        self
    }

    /// Establece el espacio entre bloques, sin punto de corte: se aplica siempre, salvo donde lo
    /// cambie [`with_gap_at()`](Self::with_gap_at). Con [`align::Gap::Distinct`] el espacio entre
    /// bloques de una misma columna (`row`) puede ser distinto del espacio entre columnas
    /// (`column`).
    pub fn with_gap(mut self, gap: align::Gap) -> Self {
        self.gap = self.gap.set(gap);
        self
    }

    /// Establece el espacio entre bloques a partir del punto de corte indicado.
    pub fn with_gap_at(mut self, bp: Breakpoint, gap: align::Gap) -> Self {
        self.gap = self.gap.set_at(bp, gap);
        self
    }

    /// Añade un nuevo componente al mosaico o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

impl Mosaic {
    // **< Mosaic PRIVATE >*************************************************************************

    // Clases CSS del número de columnas fijado en cada punto de corte. Cada una fija las columnas
    // de los dos modos del mosaico: `columns` sin JavaScript y `grid-template-columns` con él (cada
    // modo ignora la propiedad del otro). Van en el `<style>` de los estilos *responsive*, después
    // de `mosaic.min.css`, así que sustituyen al reparto por ancho mínimo.
    fn columns_classes(&self, cx: &mut Context, classes: &mut String) {
        use crate::html::responsive::responsive_class;

        for (bp, columns) in self.columns().by_breakpoint() {
            if columns == 0 {
                continue;
            }
            let entry = bp.resolved(cx);
            let count = columns.to_string();
            let class = responsive_class!("_mosaic-columns_", &count, entry);
            if !classes.is_empty() {
                classes.push(' ');
            }
            classes.push_str(&class);
            let tracks = util::join!("repeat(", &count, ",minmax(0,1fr))");
            cx.alter_assets(AssetsOp::add_responsive_styles(
                entry.map(|e| e.breakpoint),
                class,
                [("columns", count), ("grid-template-columns", tracks)],
            ));
        }
    }

    // Clases CSS del espacio entre bloques fijado en cada punto de corte. Cada una da valor a
    // `--mosaic-row-gap` y `--mosaic-column-gap`, las propiedades que usan los dos modos del
    // mosaico (y el script, a través de `row-gap`).
    fn gap_classes(&self, cx: &mut Context, classes: &mut String) {
        use crate::html::responsive::{responsive_class, value_to_token};

        for (bp, gap) in self.gap().by_breakpoint() {
            let (row, column) = match gap {
                align::Gap::None => continue,
                align::Gap::Both(value) => (value, value),
                align::Gap::Distinct { row, column } => (row, column),
            };
            // Sólo los valores medibles; el que no lo sea conserva el de un punto de corte menor.
            let row = row.is_measurable().then(|| CowStr::from(row));
            let column = column.is_measurable().then(|| CowStr::from(column));
            if row.is_none() && column.is_none() {
                continue;
            }
            let token = |value: &Option<CowStr>| {
                value
                    .as_deref()
                    .map_or_else(|| "none".to_owned(), value_to_token)
            };
            let tokens = util::join!(token(&row), "-", token(&column));
            let entry = bp.resolved(cx);
            let class = responsive_class!("_mosaic-gap_", &tokens, entry);
            if !classes.is_empty() {
                classes.push(' ');
            }
            classes.push_str(&class);
            let styles = [("--mosaic-row-gap", row), ("--mosaic-column-gap", column)]
                .into_iter()
                .filter_map(|(property, value)| value.map(|value| (property, value)));
            cx.alter_assets(AssetsOp::add_responsive_styles(
                entry.map(|e| e.breakpoint),
                class,
                styles,
            ));
        }
    }
}
