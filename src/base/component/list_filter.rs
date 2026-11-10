use crate::prelude::*;

use crate::base::component::form;

/// Componente para crear los **filtros de un listado** (ver [`ListView`]).
///
/// Es un formulario GET con los campos en fila y el botón para aplicarlos que se coloca en la
/// página por encima del listado ([`ListView`]), no dentro: si se reemplazara con él, un campo de
/// texto perdería el foco en cada pulsación.
///
/// Envía los campos a la ruta de [`with_filter_action()`], que vuelve a pintar el listado ya
/// filtrado:
///
/// ```html
/// <form class="list-filter" action="...">
///   ...                                <!-- campos -->
///   <button type="submit">...</button> <!-- "Filtrar", si no se indica otro texto -->
///   <a href="..." target="_top">...</a> <!-- "Limpiar": el listado sin filtros -->
/// </form>
/// ```
///
/// Por defecto, los filtros **no** se envían solos: se aplican al pulsar el botón (o Enter en un
/// campo de texto), de modo que se pueden ajustar varios campos y pedir el listado una sola vez.
/// Con [`with_auto_submit(true)`](Self::with_auto_submit) se aplican en cuanto cambia cualquier
/// campo, sin botón.
///
/// Junto al botón aparece por defecto el enlace "Limpiar", que vuelve a la ruta del filtro sin
/// ningún parámetro: el listado tal como se ve al entrar, también sin orden ni página. Recarga la
/// página entera, para que los campos del filtro se vacíen también. Se quita con
/// [`with_reset(false)`](Self::with_reset).
///
/// Con el botón, y si hay JavaScript, cada campo cuyo valor ya no coincide con el del filtro
/// aplicado se marca (clase `list-filter-changed` en su contenedor, `list-filter-pending` en el
/// formulario) hasta que se pulsa el botón, y un aviso oculto lo anuncia a los lectores de
/// pantalla. Así se sabe que el listado aún no corresponde a lo que muestra el filtro. El aspecto
/// de las marcas lo da cada tema.
///
/// Al enviar un formulario GET, el navegador descarta los parámetros que ya traiga la ruta de
/// destino, así que el idioma (`lang`) de la ruta se conserva en un campo oculto.
///
/// Como [`ListView`], no sabe nada de HTMX. Para aplicar los filtros sin recargar la página, se le
/// añaden los atributos necesarios con [`with_prop()`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let filter = ListFilter::new()
///     .with_filter_action("/items")
///     .with_field(
///         form::input::Field::search()
///             .with_name("q")
///             .with_placeholder(Lc::n("Search...")),
///     );
/// ```
///
/// [`with_filter_action()`]: Self::with_filter_action
/// [`with_prop()`]: Self::with_prop
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct ListFilter {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la ruta del formulario del filtro (su atributo `action`).
    filter_action: Route,
    /// Devuelve los campos de los filtros.
    fields: Children,
    /// Devuelve el texto del botón, o [`Lc::none()`] para el predeterminado ("Filtrar").
    submit: Lc,
    /// Devuelve si los filtros se aplican en cuanto cambia un campo, sin botón (por defecto, no).
    #[getters(copy)]
    auto_submit: bool,
    /// Devuelve si se muestra el enlace "Limpiar" (por defecto, sí).
    #[default(true)]
    #[getters(copy)]
    reset: bool,
}

#[async_trait]
impl Component for ListFilter {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes("list-filter"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = self.filter_action().try_resolve(cx);
        let lang = action
            .as_ref()
            .and_then(|route| route.param("lang"))
            .map(|lang| form::Hidden::field("lang", lang));
        let label = if self.submit().is_none() {
            Lc::l("list_filter_submit")
        } else {
            self.submit().clone()
        };
        let submit = Button::submit(label)
            .with_style(button::Style::Solid(Intent::Neutral))
            .render(cx)
            .await;
        // `target="_top"` navega igual que un enlace normal, pero las librerías que interceptan los
        // enlaces para cargar sólo una parte de la página (como HTMX con `hx-boost`) dejan pasar
        // los que tienen un `target` propio. Así se recarga la página entera y se vacían los
        // campos, que no se volverían a pintar si sólo se actualizara el listado.
        let reset = match (&action, self.reset()) {
            (Some(route), true) => Some(
                Button::anchor(Lc::l("list_filter_reset"), route.clone())
                    .with_style(button::Style::Link)
                    .with_prop(PropsOp::set("target", "_top"))
                    .render(cx)
                    .await,
            ),
            _ => None,
        };
        let auto_submit = self.auto_submit();
        // Con el botón, un script del componente marca los campos cambiados y no aplicados todavía,
        // y el aviso oculto los anuncia a los lectores de pantalla (`role="status"`).
        let pending = (!auto_submit).then(|| Lc::l("list_filter_pending").lookup(cx));
        if pending.is_some() {
            cx.alter_assets(
                JavaScript::defer("/pagetop/js/list-filter.min.js").with_version(PAGETOP_VERSION),
            );
        }
        Ok(html! {
            form
                (self.props().unpack(cx))
                action=[action]
                onchange=[auto_submit.then_some("this.requestSubmit()")]
            {
                @if let Some(mut lang) = lang { (lang.render(cx).await) }
                (self.fields().render(cx).await)
                @if auto_submit { noscript { (submit) } } @else { (submit) }
                @if let Some(reset) = reset { (reset) }
                @if let Some(pending) = pending {
                    span
                        class="list-filter-status visually-hidden"
                        role="status"
                        data-pending=[pending]
                    {}
                }
            }
        })
    }
}

#[builder_impl]
impl ListFilter {
    // **< ListFilter BUILDER >*********************************************************************

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

    /// Establece la ruta del formulario del filtro: el atributo `action` del `<form>` que genera el
    /// componente, es decir, la ruta a la que se envían los filtros.
    ///
    /// Normalmente es la del propio listado. Su *handler* recibe los campos en la consulta
    /// (`?q=...`) y vuelve a pintar el listado con ellos aplicados: la página entera en una
    /// petición normal o, con filtrado en vivo, sólo el listado. No tiene que ver con las acciones
    /// de PageTop ([`core::action`](crate::core::action)).
    ///
    /// Acepta un literal, un `String` o una [`Route`]; como en cualquier enlace, se le añade el
    /// idioma (`lang`) cuando corresponde, y el componente lo conserva en un campo oculto para que
    /// el formulario no lo pierda al enviarse.
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// // GET /items?q=... vuelve a pintar el listado con la búsqueda aplicada.
    /// let filter = ListFilter::new()
    ///     .with_filter_action("/items")
    ///     .with_field(form::input::Field::search().with_name("q"));
    /// ```
    pub fn with_filter_action(mut self, action: impl Into<Route>) -> Self {
        self.filter_action = action.into();
        self
    }

    /// Añade un campo de filtro (texto, desplegable, casilla...).
    pub fn with_field(mut self, op: impl Into<ChildOp>) -> Self {
        self.fields.alter_child(op);
        self
    }

    /// Aplica los filtros en cuanto cambia cualquier campo, sin pulsar el botón.
    ///
    /// **Por defecto está desactivado** (`false`): los filtros se aplican sólo con el botón.
    /// Con JavaScript activo, el filtro se muestra **sin botón**: un desplegable o una casilla se
    /// aplican al cambiar, y un campo de texto al pulsar Enter o al salir de él (no con cada
    /// tecla). Sin JavaScript, el botón sigue apareciendo y los filtros se aplican al pulsarlo.
    ///
    /// Cada cambio pide de nuevo el listado. Conviene en filtros de uno o dos campos; con muchos,
    /// el botón evita consultas intermedias que el usuario no quería.
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let filter = ListFilter::new()
    ///     .with_filter_action("/items")
    ///     .with_field(form::Checkbox::check().with_name("active").with_label(Lc::n("Active")))
    ///     .with_auto_submit(true);
    /// ```
    pub fn with_auto_submit(mut self, auto_submit: bool) -> Self {
        self.auto_submit = auto_submit;
        self
    }

    /// Muestra u oculta el enlace "Limpiar", que vuelve al listado sin filtros.
    ///
    /// **Por defecto se muestra** (`true`), al contrario que
    /// [`with_auto_submit()`](Self::with_auto_submit); con `false` se quita, por ejemplo en un
    /// filtro de un único campo de texto, que ya se limpia vaciándolo.
    pub fn with_reset(mut self, reset: bool) -> Self {
        self.reset = reset;
        self
    }

    /// Cambia el texto del botón; con [`Lc::none()`] vuelve al predeterminado ("Filtrar").
    pub fn with_submit(mut self, label: Lc) -> Self {
        self.submit = label;
        self
    }
}
