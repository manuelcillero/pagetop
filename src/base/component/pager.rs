use crate::prelude::*;

/// Define cuándo mostrar los botones de página anterior/siguiente o el formulario de salto a página
/// de [`Pager`].
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum PagerVisibility {
    /// Nunca se muestra.
    Never,
    /// Siempre se muestra (en el caso de los botones, se desactivan si no procede).
    Always,
    /// Se muestra sólo cuando el número total de páginas supera al número de páginas que se
    /// muestra en el paginador (ver la documentación de [`Pager`] para más detalle).
    #[default]
    Auto,
}

/// Define la alineación horizontal de [`Pager`] dentro de su contenedor.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum PagerAlign {
    /// Alineado al comienzo.
    Start,
    /// Centrado (comportamiento por defecto).
    #[default]
    Center,
    /// Alineado al final.
    End,
}

/// Componente para añadir un **paginador** a un listado.
///
/// `Pager` permite navegar por las páginas de un listado de ítems cuando supera el número máximo de
/// ítems admitidos por página. Resuelve el enlace para acceder a cada página del listado a partir
/// de una ruta base, un conjunto de parámetros de consulta adicionales (orden, búsqueda, etc.) y el
/// estado actual ([`current_page`], [`items_per_page`], [`total_items`]).
///
/// El componente se renderiza sólo si el listado requiere más de una página.
///
/// No pretende sustituir a [`Table`] ni imponer cómo se presenta un listado: sólo modela la
/// paginación en sí, normalmente presentada junto a `Table` dentro de un mismo contenedor.
///
/// El listado de páginas se flanquea con dos botones de navegación: página anterior y página
/// siguiente (las páginas primera y última ya están siempre disponibles como números, así que no
/// llevan un botón dedicado). Su visibilidad, junto a la del formulario de salto a página y la del
/// resumen de páginas, se controla con [`PagerVisibility`] a través de [`with_prev_next()`],
/// [`with_jump()`] y [`with_summary()`]: `Never` los oculta siempre, `Always` los muestra siempre,
/// y el valor por defecto `Auto` los muestra sólo cuando el paginador necesita truncar el listado
/// de páginas por superar el número máximo de páginas visible.
///
/// Un elemento `<nav>` envuelve todo el paginador. Lleva un `aria-label` por defecto que puede
/// sustituirse por otro más específico usando [`with_aria_label()`], por ejemplo cuando una misma
/// página tiene varios paginadores.
///
/// La alineación horizontal del paginador dentro de este contenedor se controla con [`PagerAlign`]
/// a través de [`with_align()`]. Por defecto es [`PagerAlign::Center`].
///
/// # Acotando el número de ítems del paginador
///
/// Con listados largos, mostrar un número por cada página real puede desbordar la interfaz. Con
/// [`with_window()`] se puede limitar el número de páginas que se muestran a cada lado de la página
/// actual. Por defecto vale `2`, que limita la vista a `9` celdas en total (sin contar los botones
/// de navegación anterior/siguiente). En general, el número máximo de celdas mostradas para un
/// `window` dado es `2 * window + 5`.
///
/// Incluso cuando se trunca el paginador, `Pager` siempre muestra la primera y la última página
/// como números, más la ventana indicada antes y después de la página actual, sustituyendo por una
/// elipsis (`…`) cualquier tramo oculto de dos o más páginas. Si el tramo oculto es de una sola
/// página, se muestra directamente en vez de la elipsis, porque ocultarla no ahorra espacio.
///
/// Por ejemplo, con `with_window(3)`, página actual `34` y con `200` páginas en total, el paginador
/// se mostraría así:
///
/// ```text
/// ‹ | 1 | … | 31 | 32 | 33 | [34] | 35 | 36 | 37 | … | 200 | ›
/// ```
///
/// Cuando la página actual está cerca de los extremos, se ajustan las páginas numeradas para
/// mantener el número de celdas mostradas según el valor de `window`.
///
/// Cuando corresponda según [`jump()`](Self::jump), [`Pager`] puede añadir un pequeño formulario
/// para saltar directamente a una página escribiendo su número, sin depender de JavaScript. Un
/// único campo numérico (`min`/`max` según el total de páginas) y un botón de envío.
///
/// Con [`with_summary()`] se puede añadir un texto que resume la vista de las páginas que se
/// muestran en ese momento (ver más arriba su visibilidad según [`PagerVisibility`]).
///
/// # Clases CSS
///
/// - `.pager` - clase base del componente (elemento `<nav>`).
/// - `.pager-align-start` / `.pager-align-center` / `.pager-align-end` - según [`PagerAlign`].
/// - `.pager-summary` - clase del resumen de las páginas mostradas, si está activado.
/// - `.pagination` - clase del elemento `<ul>` que contiene los enlaces de página.
/// - `.page-item` - presente en todos los `<li>` del listado.
/// - `.page-link` - presente en todos los enlaces (`<a>`) del listado, y también en el `<span>` de
///   la elipsis, para que comparta con ellos el aspecto de celda (borde, fondo, radio, margen).
/// - `.page-link-icon` - envuelve el texto de los botones de navegación anterior/siguiente.
/// - `.page-previous` / `.page-next` - añadidas a los `<li>` de página anterior/siguiente.
/// - `.page-ellipsis` - clase del `<li>` que representa un tramo de páginas ocultas.
/// - `.active` - añadida al `<li>` de la página actualmente visible.
/// - `.disabled` - añadida al `<li>` de los extremos cuando no procede navegar, y también al de la
///   elipsis, ya que tampoco es interactiva.
/// - `.pager-jump` - clase del `<form>` para saltar directamente a una página.
/// - `.pager-jump-input` - clase del campo numérico del formulario de salto.
/// - `.pager-jump-button` - clase del botón de envío del formulario de salto.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Listado de usuarios filtrado por búsqueda y ordenado por nombre: los parámetros que deben
/// // sobrevivir entre páginas se declaran con `with_extra_query()`, en el orden en que deben
/// // aparecer en la URL; `page` se añade siempre al final por `Pager`.
/// let pager = Pager::new()
///     .with_base_path("/admin/users")
///     .with_extra_query("q", "ana")
///     .with_extra_query("sort", "username")
///     .with_extra_query("dir", "asc")
///     .with_current_page(1)
///     .with_items_per_page(20)
///     .with_total_items(97);
/// ```
///
/// [`current_page`]: Self::current_page
/// [`items_per_page`]: Self::items_per_page
/// [`total_items`]: Self::total_items
/// [`with_window()`]: Self::with_window
/// [`with_align()`]: Self::with_align
/// [`with_summary()`]: Self::with_summary
/// [`with_prev_next()`]: Self::with_prev_next
/// [`with_jump()`]: Self::with_jump
/// [`with_aria_label()`]: Self::with_aria_label
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Pager {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la ruta base sobre la que se construye el enlace de cada página.
    base_path: AttrValue,
    /// Devuelve los parámetros de consulta adicionales que viajan en el enlace de cada página, en
    /// el orden en que se añadieron.
    extra_query: Vec<(String, String)>,
    /// Devuelve la página actual (siendo 1 la primera página).
    current_page: u64,
    /// Devuelve el número de elementos que se muestran por página.
    items_per_page: u64,
    /// Devuelve el número total de elementos del listado completo.
    total_items: u64,
    /// Devuelve el número de páginas mostradas a cada lado de la página actual antes de truncar con
    /// elipsis (predeterminado a `2` páginas a cada lado).
    ///
    /// El valor `0` no trunca y muestra siempre todas las páginas, recomendable sólo para listados
    /// con un número pequeño de páginas.
    #[default(2)]
    window: u64,
    /// Devuelve la alineación horizontal del paginador dentro de su contenedor.
    align: PagerAlign,
    /// Devuelve la visibilidad del resumen de las páginas que se muestran en cada vista.
    summary: PagerVisibility,
    /// Devuelve la visibilidad de los botones de página anterior/siguiente.
    prev_next: PagerVisibility,
    /// Devuelve la visibilidad del formulario para saltar directamente a una página.
    jump: PagerVisibility,
    /// Devuelve la etiqueta de accesibilidad (`aria-label`) del elemento `<nav>`.
    #[default(L10n::l("pager_aria_label"))]
    aria_label: L10n,
}

// Elemento visible del listado de páginas: un número de página o una elipsis que resume un tramo
// de páginas ocultas.
enum PageItem {
    Number(u64),
    Ellipsis,
}

#[async_trait]
impl Component for Pager {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        // Asegura un `id` propio si no está definido. El formulario de salto a página deriva sus
        // identificadores de éste para no colisionar si hay varios paginadores en la misma página.
        let id = cx.required_id::<Self>(self.id(), 1);
        self.alter_prop(PropsOp::ensure_id(id));
        self.alter_prop(PropsOp::prepend_classes("pager"));
        self.alter_prop(PropsOp::add_classes(match self.align() {
            PagerAlign::Start => "pager-align-start",
            PagerAlign::Center => "pager-align-center",
            PagerAlign::End => "pager-align-end",
        }));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let total_pages = self.total_pages();
        if total_pages <= 1 {
            return Ok(html! {});
        }
        let page = self.current_page().clamp(1, total_pages);
        let base_path = self.base_path().as_str().unwrap_or_default();

        // Ruta común a los enlaces del paginador, con los parámetros de `extra_query` añadidos a
        // `base_path`. Pasa por `cx.route()` para preservar el parámetro `lang` si corresponde.
        let mut route = cx.route(base_path.to_owned());
        for (key, value) in self.extra_query() {
            route.alter_param(key, value);
        }

        let first_disabled = page <= 1;
        let last_disabled = page >= total_pages;

        let items = self.page_items(page, total_pages);
        let truncated = items.iter().any(|item| matches!(item, PageItem::Ellipsis));

        let show_prev_next = match self.prev_next() {
            PagerVisibility::Never => false,
            PagerVisibility::Always => true,
            PagerVisibility::Auto => truncated,
        };
        let show_jump = match self.jump() {
            PagerVisibility::Never => false,
            PagerVisibility::Always => true,
            PagerVisibility::Auto => truncated,
        };
        let show_summary = match self.summary() {
            PagerVisibility::Never => false,
            PagerVisibility::Always => true,
            PagerVisibility::Auto => truncated,
        };

        Ok(html! {
            nav (self.props()) aria-label=(self.aria_label().using(cx)) {
                @if show_summary {
                    @let items_per_page = self.items_per_page().max(1);
                    @let first = (page - 1) * items_per_page + 1;
                    @let last = (page * items_per_page).min(self.total_items());
                    span.pager-summary {
                        (L10n::l("pager_summary")
                            .with_arg("first", first.to_string())
                            .with_arg("last", last.to_string())
                            .with_arg("total", self.total_items().to_string())
                            .using(cx))
                    }
                }
                ul.pagination {
                    @if show_prev_next {
                        li.page-item.page-previous.disabled[first_disabled] {
                            a.page-link
                                href=[(!first_disabled).then(|| Self::page_route(&route, page - 1))]
                                aria-disabled=[first_disabled.then_some("true")]
                                aria-label=(L10n::l("pager_previous_aria_label").using(cx)) {
                                span.page-link-icon { (L10n::l("pager_previous_label").using(cx)) }
                            }
                        }
                    }
                    @for item in items {
                        @match item {
                            PageItem::Number(n) => {
                                @let href = Self::page_route(&route, n);
                                li.page-item.active[n == page] {
                                    a.page-link
                                        href=(href)
                                        aria-current=[(n == page).then_some("page")] {
                                        (n.to_string())
                                    }
                                }
                            }
                            PageItem::Ellipsis => {
                                li.page-item.page-ellipsis.disabled aria-hidden="true" {
                                    span.page-link { "…" }
                                }
                            }
                        }
                    }
                    @if show_prev_next {
                        li.page-item.page-next.disabled[last_disabled] {
                            a.page-link
                                href=[(!last_disabled).then(|| Self::page_route(&route, page + 1))]
                                aria-disabled=[last_disabled.then_some("true")]
                                aria-label=(L10n::l("pager_next_aria_label").using(cx)) {
                                span.page-link-icon { (L10n::l("pager_next_label").using(cx)) }
                            }
                        }
                    }
                }
                @if show_jump { ({
                    // En `setup()` se garantiza que el paginador ya tiene un `id`.
                    let id = self.id().unwrap();

                    // Construye el formulario para saltar directamente a una página.
                    let mut form = Form::new()
                        .with_id(util::join!(id, "-jump"))
                        .with_prop(PropsOp::add_classes("pager-jump"))
                        .with_method(form::Method::Get)
                        .with_action(base_path.to_owned());

                    // Un formulario `GET` descarta cualquier `query` que ya tuviera `action` y la
                    // sustituye por sus propios campos al enviarse, así que `extra_query` y `lang`
                    // no bastan para formar parte de la URL: viajan como campos ocultos.
                    for (key, val) in self.extra_query() {
                        form = form.with_child(form::Hidden::field(key, val));
                    }
                    if let Some(lang) = route.param("lang") {
                        form = form.with_child(form::Hidden::field("lang", lang));
                    }

                    // Info para ajustar el ancho del campo al número de dígitos de `total_pages`.
                    let jump_width = util::join!(&total_pages.to_string().len().to_string(), "ch");

                    form.with_child(
                        form::Number::new()
                            .with_id(util::join!(id, "-jump-page"))
                            .with_prop(PropsOp::add_classes("pager-jump-input"))
                            .with_prop(PropsOp::add_style("--pager-jump-width", jump_width))
                            .with_name("page")
                            .with_min(Some(1))
                            .with_max(Some(total_pages))
                            .with_value(Some(page)),
                    )
                    .with_child(
                        Button::submit(L10n::l("pager_goto_button"))
                            .with_prop(PropsOp::add_classes("pager-jump-button"))
                            .with_title(L10n::l("pager_goto_label")),
                    )
                    .render(cx).await
                }) }
            }
        })
    }
}

impl Pager {
    // **< Pager BUILDER >*************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece la ruta base sobre la que se construye el enlace de cada página.
    #[builder_fn]
    pub fn with_base_path(mut self, base_path: impl AsRef<str>) -> Self {
        self.base_path.alter_str(base_path);
        self
    }

    /// Añade un parámetro de consulta que debe viajar en el enlace de cada página, además de
    /// `page` (que `Pager` añade siempre al final). Llamar varias veces añade varios
    /// parámetros, en el orden en que se declaren.
    #[builder_fn]
    pub fn with_extra_query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_query.push((key.into(), value.into()));
        self
    }

    /// Establece la página actual (siendo 1 la primera página).
    #[builder_fn]
    pub fn with_current_page(mut self, current_page: u64) -> Self {
        self.current_page = current_page;
        self
    }

    /// Establece el número de elementos que se muestran por página.
    #[builder_fn]
    pub fn with_items_per_page(mut self, items_per_page: u64) -> Self {
        self.items_per_page = items_per_page;
        self
    }

    /// Establece el número total de elementos del listado completo.
    #[builder_fn]
    pub fn with_total_items(mut self, total_items: u64) -> Self {
        self.total_items = total_items;
        self
    }

    /// Establece el número de páginas mostradas a cada lado de la página actual antes de truncar
    /// el listado con una elipsis. Por defecto es `2`.
    ///
    /// El valor `0` desactiva el truncado y muestra siempre todas las páginas. Usar cuando el
    /// número total de páginas sea pequeño y no haya riesgo de desbordar la interfaz.
    #[builder_fn]
    pub fn with_window(mut self, window: u64) -> Self {
        self.window = window;
        self
    }

    /// Establece la alineación horizontal del paginador dentro de su contenedor. Por defecto es
    /// [`PagerAlign::Center`].
    #[builder_fn]
    pub fn with_align(mut self, align: PagerAlign) -> Self {
        self.align = align;
        self
    }

    /// Establece la visibilidad del resumen de las páginas que se muestran en cada vista. Por
    /// defecto es `PagerVisibility::Auto`: sólo se muestra cuando el número total de páginas supera
    /// al número de páginas que se muestra en el paginador, igual que [`with_prev_next()`] y
    /// [`with_jump()`].
    ///
    /// [`with_prev_next()`]: Self::with_prev_next
    /// [`with_jump()`]: Self::with_jump
    #[builder_fn]
    pub fn with_summary(mut self, summary: PagerVisibility) -> Self {
        self.summary = summary;
        self
    }

    /// Establece la visibilidad de los botones de página anterior/siguiente. Por defecto es
    /// `PagerVisibility::Auto`: sólo se muestran cuando el número total de páginas supera al
    /// número de páginas que se muestra en el paginador (con el extremo correspondiente
    /// desactivado en vez de oculto).
    #[builder_fn]
    pub fn with_prev_next(mut self, prev_next: PagerVisibility) -> Self {
        self.prev_next = prev_next;
        self
    }

    /// Establece la visibilidad del formulario para saltar directamente a una página. Por
    /// defecto es `PagerVisibility::Auto`: sólo se muestra cuando el número total de páginas
    /// supera al número de páginas que se muestra en el paginador.
    #[builder_fn]
    pub fn with_jump(mut self, jump: PagerVisibility) -> Self {
        self.jump = jump;
        self
    }

    /// Establece la etiqueta de accesibilidad (`aria-label`) del elemento `<nav>`. Por defecto es
    /// "Page navigation" (clave `pager_aria_label`), igual que hace el paginador de Bootstrap.
    #[builder_fn]
    pub fn with_aria_label(mut self, aria_label: L10n) -> Self {
        self.aria_label = aria_label;
        self
    }

    // **< Pager HELPERS >*************************************************************************

    /// Número total de páginas según [`total_items()`](Self::total_items) y
    /// [`items_per_page()`](Self::items_per_page). Nunca es cero, aunque `total_items` sea cero:
    /// devuelve `1` igualmente (es [`prepare()`](Component::prepare) quien decide no renderizar
    /// nada cuando sólo hay una página).
    pub fn total_pages(&self) -> u64 {
        let items_per_page = self.items_per_page().max(1);
        self.total_items().div_ceil(items_per_page).max(1)
    }

    // Calcula los elementos visibles del listado de páginas. Si `window` es `0`, o si el total de
    // páginas ya cabe sin necesidad de truncar (ver más abajo), devuelve todas las páginas sin
    // elipsis. En caso contrario, siempre incluye la primera y la última página, la ventana de
    // páginas alrededor de la actual (`window` antes y después, recortada a los límites del
    // listado), y sustituye por una elipsis cualquier tramo oculto de dos o más páginas -- un
    // tramo de una sola página se muestra directamente, ya que ocultarla tras una elipsis no
    // ahorra espacio.
    fn page_items(&self, page: u64, total_pages: u64) -> Vec<PageItem> {
        // Acotado a `total_pages`: una ventana mayor no aporta nada (ya se mostrarían todas las
        // páginas) y evita operar con un valor arbitrariamente grande más abajo.
        let window = self.window().min(total_pages);

        // Con `total_pages <= 2 * window + 3` la ventana más los dos extremos ya cubren el listado
        // completo en el caso más desfavorable (página actual centrada), así que truncar no
        // ahorraría ningún número.
        if window == 0 || total_pages <= window.saturating_mul(2).saturating_add(3) {
            return (1..=total_pages).map(PageItem::Number).collect();
        }

        // Ventana centrada en `page`, protegiendo las operaciones aritméticas con signo.
        let mut low = page as i128 - window as i128;
        let mut high = page as i128 + window as i128;

        if low < 2 {
            let overflow = 2 - low;
            low += overflow;
            high += overflow;
        }
        if high > total_pages as i128 - 1 {
            let overflow = high - (total_pages as i128 - 1);
            high -= overflow;
            low -= overflow;
        }
        low = low.clamp(2, total_pages as i128 - 1);
        high = high.clamp(2, total_pages as i128 - 1);

        // Si la ventana toca la primera o la última página, ese lado no necesita elipsis ni número
        // de relleno: el hueco que se ahorra se reinvierte ampliando la ventana por el otro lado.
        if low == 2 {
            high = (high + 1).min(total_pages as i128 - 1);
        }
        if high == total_pages as i128 - 1 {
            low = (low - 1).max(2);
        }

        let low = low as u64;
        let high = high as u64;

        let mut items = vec![PageItem::Number(1)];

        match low {
            2 => {}
            3 => items.push(PageItem::Number(2)),
            _ => items.push(PageItem::Ellipsis),
        }

        items.extend((low..=high).map(PageItem::Number));

        match total_pages - high {
            1 => {}
            2 => items.push(PageItem::Number(total_pages - 1)),
            _ => items.push(PageItem::Ellipsis),
        }

        items.push(PageItem::Number(total_pages));
        items
    }

    // Construye el enlace a la página `page` añadiendo `page` sobre `route`.
    fn page_route(route: &RoutePath, page: u64) -> String {
        route
            .clone()
            .with_param("page", page.to_string())
            .to_string()
    }
}
