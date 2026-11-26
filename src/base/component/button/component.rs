use crate::prelude::*;

/// Componente para crear un **botón**.
///
/// Un botón puede usarse dentro o fuera de un formulario. Se crea con uno de los constructores
/// **básicos**, que reciben el texto, o con uno de los botones **habituales**, que ya traen el
/// texto traducido y el estilo de su función.
///
/// # Constructores básicos
///
/// - [`Button::submit()`]: botón de envío (`type="submit"`).
/// - [`Button::reset()`]: botón de restablecimiento de valores (`type="reset"`).
/// - [`Button::plain()`]: botón genérico sin comportamiento predeterminado (`type="button"`).
/// - [`Button::anchor()`]: enlace de navegación real (`<a href=...>`) con el aspecto de un botón.
/// - [`Button::anchor_blank()`]: igual, pero se abre en una nueva ventana o pestaña.
///
/// No confundir [`Button::anchor()`] con [`button::Style::Link`]: el primero renderiza un `<a
/// href=...>` real que navega; el segundo es sólo un estilo visual (clase `button-link`) que se
/// aplica con [`with_style()`](Self::with_style) sobre cualquiera de las otras variantes, que
/// siguen siendo un `<button>`.
///
/// # Botones habituales
///
/// Las acciones que se repiten en casi cualquier aplicación tienen su propio constructor, para que
/// todas las pantallas usen el mismo texto (traducido por PageTop) y el mismo estilo:
///
/// | Constructor                   | Elemento                 | Texto             | Estilo             | Para qué                                    |
/// |-------------------------------|--------------------------|-------------------|--------------------|---------------------------------------------|
/// | [`Button::save()`]            | `<button type="submit">` | «Guardar»         | `Solid(Primary)`   | Enviar un formulario.                       |
/// | [`Button::save_and_new()`]    | `<button type="submit">` | «Guardar y nuevo» | `Outline(Primary)` | Guardar y crear otro registro.              |
/// | [`Button::edit()`]            | `<a href>`               | «Editar»          | `Solid(Primary)`   | Ir al formulario de edición de un registro. |
/// | [`Button::delete()`]          | `<button type="button">` | «Eliminar»        | `Solid(Severe)`    | Borrar un registro, tras confirmarlo.       |
/// | [`Button::dismiss()`]         | `<button type="button">` | «Cancelar»        | `Outline(Neutral)` | Cerrar un [`Dialog`] sin hacer nada.        |
/// | [`Button::cancel()`]          | `<a href>`               | «Cancelar»        | `Link`             | Salir de un formulario sin guardar.         |
/// | [`Button::back()`]            | `<a href>`               | «Volver»          | `Link`             | Salir de una ficha de sólo lectura.         |
///
/// Siguen siendo un `Button` como cualquier otro, así que se pueden ajustar después (tamaño,
/// icono, atributos...). Para un texto distinto se usa el constructor básico correspondiente.
///
/// # Ejemplos
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let send  = Button::submit(Lc::n("Send"));
/// let clear = Button::reset(Lc::n("Clear"));
/// let docs  = Button::anchor_blank(Lc::n("Documentation"), "https://docs.rs/pagetop");
///
/// // Con un icono al principio o al final de la etiqueta, o sólo con el icono.
/// let add   = Button::plain(Lc::n("Add")).with_icon_start(Icon::with(icon::Core::Plus));
/// let next  = Button::plain(Lc::n("Next")).with_icon_end(Icon::with(icon::Core::ChevronRight));
/// let close = Button::plain(Lc::none())
///     .with_icon_start(Icon::with(icon::Core::Close).with_label(Lc::n("Close")));
///
/// // Botones habituales: la fila de acciones de un formulario de alta...
/// let actions = Flex::buttons()
///     .with_child(Button::save())
///     .with_child(Button::save_and_new())
///     .with_child(Button::cancel("/items"));
///
/// // ... y el botón de editar de una fila de una tabla.
/// let edit = Button::edit("/items/1/edit").with_size(button::Size::Small);
/// ```
///
/// Cuando el botón activa el envío, el navegador incluye el par `name=value` en los datos del
/// formulario **sólo si** tiene el atributo `name` definido. Es la forma habitual de identificar
/// cuál de los botones de envío fue pulsado. En el servidor se deserializa como `Option<String>`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     #[serde(default)]
///     action: Option<String>, // p. ej., "save" o "delete"; `None` si el botón no tenía `name`.
/// }
/// ```
///
/// [`Button::save_and_new()`] ya lo resuelve así para el caso de «Guardar y nuevo», con
/// [`button::SaveAndNew`] en el servidor.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Button {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el comportamiento del botón al activarse.
    kind: button::Kind,
    /// Devuelve el tamaño visual del botón.
    #[getters(copy)]
    size: button::Size,
    /// Devuelve el estilo visual del botón.
    #[getters(copy)]
    style: button::Style,
    /// Devuelve el nombre del botón.
    name: AttrName,
    /// Devuelve el valor del botón.
    value: AttrValue,
    /// Devuelve la etiqueta del botón.
    label: Lc,
    /// Devuelve el texto que completa la etiqueta sólo para los lectores de pantalla.
    hidden_label: Lc,
    /// Devuelve el icono al principio del botón, antes de la etiqueta (si lo hay).
    icon_start: Embed<Icon>,
    /// Devuelve el icono al final del botón, después de la etiqueta (si lo hay).
    icon_end: Embed<Icon>,
    /// Devuelve el texto emergente del botón (atributo `title`).
    title: Lc,
    /// Devuelve la ruta de destino cuando el botón se renderiza como enlace de navegación
    /// (`<a href=...>` en vez de `<button>`). Vacía por defecto: en ese caso `prepare()` renderiza
    /// un `<button>` normal, ignorando este campo.
    href: Route,
    /// Devuelve si el enlace se abre en una nueva ventana o pestaña (sólo con una ruta de destino).
    blank: bool,
    /// Devuelve si el botón recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el botón está deshabilitado.
    disabled: bool,
}

#[async_trait]
impl Component for Button {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        self.href().is_accessible(cx)
    }

    fn setup(&mut self, cx: &mut Context) {
        use button::{Size, Style};

        self.alter_prop(PropsOp::prepend_classes(match self.size() {
            Size::None => "",
            Size::Small => "button-sm",
            Size::Large => "button-lg",
        }));
        self.alter_prop(PropsOp::prepend_classes(match self.style() {
            Style::None => "button".to_string(),
            Style::Solid(intent) => util::join!("button button-", intent.color(cx)),
            Style::Outline(intent) => util::join!("button button-outline-", intent.color(cx)),
            Style::Link => "button button-link".to_string(),
        }));

        // Un `<button disabled>` es deshabilitado por el navegador (`:disabled`), pero un `<a>` sin
        // `href` (ver `prepare()`) no tiene un estado nativo similar. Por eso la clase `disabled`.
        if *self.disabled() {
            self.alter_prop(PropsOp::add_classes("disabled"));
        }

        // Por `Props` y no en el `<a>` de `prepare()`: un atributo escrito en el elemento omite el
        // mismo atributo de `Props`, y se perdería un `target` añadido con `with_prop()`.
        if *self.blank() && !*self.disabled() && self.href().try_resolve(cx).is_some() {
            self.alter_prop(PropsOp::set("target", "_blank"));
            self.alter_prop(PropsOp::set("rel", "noopener noreferrer"));
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Sin iconos, el contenido es la etiqueta tal cual. Con alguno, un espacio separa cada
        // pieza de la siguiente, sólo si las dos tienen contenido.
        let label = self.label().using(cx);
        // El texto oculto va justo detrás de la etiqueta, antes del icono final: así el nombre
        // accesible contiene la etiqueta tal como se ve, y el espacio que los separa no se ve.
        let label = match self
            .hidden_label()
            .lookup(cx)
            .filter(|hidden| !hidden.is_empty())
        {
            Some(hidden) => html! {
                (label) span class="visually-hidden" { " " (hidden) }
            },
            None => label,
        };
        let content = if self.icon_start().get().is_none() && self.icon_end().get().is_none() {
            label
        } else {
            let start = self.icon_start().render(cx).await;
            let end = self.icon_end().render(cx).await;
            html! {
                (start)
                @if !start.is_empty() && (!label.is_empty() || !end.is_empty()) {
                    (" ")
                }
                (label)
                @if !label.is_empty() && !end.is_empty() {
                    (" ")
                }
                (end)
            }
        };

        if let Some(route) = self.href().try_resolve(cx) {
            let disabled = *self.disabled();
            let href = (!disabled).then_some(route);
            let aria_disabled = disabled.then_some("true");
            let tabindex = disabled.then_some("-1");

            return Ok(html! {
                a
                    (self.props().unpack(cx))
                    href=[href]
                    title=[self.title().lookup(cx)]
                    autofocus[*self.autofocus()]
                    aria-disabled=[aria_disabled]
                    tabindex=[tabindex]
                {
                    (content)
                }
            });
        }

        Ok(html! {
            button
                type=(self.kind())
                (self.props().unpack(cx))
                name=[self.name().as_deref()]
                value=[self.value().as_deref()]
                title=[self.title().lookup(cx)]
                autofocus[*self.autofocus()]
                disabled[*self.disabled()]
            {
                (content)
            }
        })
    }
}

#[builder_impl]
impl Button {
    /// Crea un botón de **envío** (`type="submit"`).
    ///
    /// Es la acción predeterminada al pulsar un botón en la mayoría de los formularios: envía los
    /// datos al servidor.
    pub fn submit(label: Lc) -> Self {
        Self {
            kind: button::Kind::Submit,
            label,
            ..Default::default()
        }
    }

    /// Crea un botón de **restablecimiento** (`type="reset"`).
    ///
    /// Al pulsarlo, devuelve todos los campos del formulario a sus valores iniciales.
    pub fn reset(label: Lc) -> Self {
        Self {
            kind: button::Kind::Reset,
            label,
            ..Default::default()
        }
    }

    /// Crea un **botón genérico** (`type="button"`).
    ///
    /// No tiene un comportamiento predeterminado sobre el formulario. Su comportamiento puede
    /// definirse mediante JavaScript.
    pub fn plain(label: Lc) -> Self {
        Self {
            kind: button::Kind::Plain,
            label,
            ..Default::default()
        }
    }

    /// Crea un **enlace de navegación** con el aspecto de un botón (`<a href=...>`).
    ///
    /// A diferencia de [`Button::submit()`], [`Button::reset()`] y [`Button::plain()`], que
    /// siempre renderizan un `<button>`, este constructor produce un enlace real. Navega a `route`
    /// en vez de interactuar con un formulario. Se aplican las mismas clases de estilo (ver
    /// [`with_style()`](Self::with_style)), por lo que el tema activo puede mostrarlo igual que
    /// cualquier otra variante de `Button`. No confundir con [`button::Style::Link`], que es sólo
    /// un estilo visual sobre un `<button>`.
    ///
    /// No se renderiza si el usuario actual no puede acceder a la ruta (ver [`Endpoint`]).
    pub fn anchor(label: Lc, route: impl Into<Route>) -> Self {
        Self {
            label,
            href: route.into(),
            ..Default::default()
        }
    }

    /// Crea un **enlace de navegación** con el aspecto de un botón que se abre en una nueva ventana
    /// o pestaña. Ver [`Button::anchor()`].
    pub fn anchor_blank(label: Lc, route: impl Into<Route>) -> Self {
        Self {
            blank: true,
            ..Self::anchor(label, route)
        }
    }

    /// Crea el botón **«Guardar»** que envía un formulario.
    ///
    /// Es un [`Button::submit()`] con el texto traducido de PageTop y el estilo
    /// [`button::Style::Solid`] con [`Intent::Primary`]: la acción principal del formulario. Para
    /// otro texto («Enviar», «Aplicar cambios»...) se usa [`Button::submit()`] con el estilo que
    /// convenga.
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let form = Form::new()
    ///     .with_action("/items/new")
    ///     .with_child(form::input::Field::text().with_name("name"))
    ///     .with_child(Button::save());
    /// ```
    pub fn save() -> Self {
        Self::submit(Lc::l("button_save")).with_style(button::Style::Solid(Intent::Primary))
    }

    /// Crea el botón **«Guardar y nuevo»** de un formulario de alta.
    ///
    /// Se coloca junto a [`Button::save()`] para guardar el registro y volver a un formulario vacío
    /// en vez de al listado, cuando se van a crear varios seguidos. Es un [`Button::submit()`] con
    /// el texto traducido de PageTop y el estilo [`button::Style::Outline`] con
    /// [`Intent::Primary`], para que destaque menos que «Guardar».
    ///
    /// Al pulsarlo envía un campo que reconoce [`button::SaveAndNew`]: con él, el servidor sabe qué
    /// botón se pulsó y puede mostrar el aviso de [`SaveAndNew::notice()`]. La documentación de
    /// [`button::SaveAndNew`] tiene un ejemplo completo. No tiene sentido en un formulario de
    /// edición.
    ///
    /// [`SaveAndNew::notice()`]: button::SaveAndNew::notice
    pub fn save_and_new() -> Self {
        Self::submit(Lc::l("button_save_and_new"))
            .with_name("save_and_new")
            .with_value("1")
            .with_style(button::Style::Outline(Intent::Primary))
    }

    /// Crea el enlace **«Editar»** que lleva al formulario de edición de un registro.
    ///
    /// Es un [`Button::anchor()`] a `route` con el texto traducido de PageTop y el estilo
    /// [`button::Style::Solid`] con [`Intent::Primary`]. Como cualquier enlace, no se renderiza si
    /// el usuario no puede acceder a la ruta (ver [`Endpoint`]). En una fila de una tabla suele
    /// usarse en tamaño pequeño y con el nombre del registro para los lectores de pantalla (ver
    /// [`with_hidden_label()`](Self::with_hidden_label)):
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let edit = Button::edit("/items/1/edit")
    ///     .with_size(button::Size::Small)
    ///     .with_hidden_label(Lc::n("Item 1"));
    /// ```
    pub fn edit(route: impl Into<Route>) -> Self {
        Self::anchor(Lc::l("button_edit"), route).with_style(button::Style::Solid(Intent::Primary))
    }

    /// Crea el botón **«Eliminar»** de un registro.
    ///
    /// Es un [`Button::plain()`] (`type="button"`) con el texto traducido de PageTop y el estilo
    /// [`button::Style::Solid`] con [`Intent::Severe`]. No lleva ruta ni hace nada por sí mismo:
    /// quien lo usa le añade la petición que borra y la confirmación previa, por ejemplo con HTMX
    /// (la extensión `pagetop-htmx` confirma en un [`Dialog`] compartido por todo el listado con su
    /// módulo `hx_confirm`). Es un botón y no un enlace porque borrar nunca debe hacerse con una
    /// petición `GET`. En una fila de una tabla, como [`Button::edit()`], conviene darle el nombre
    /// del registro con [`with_hidden_label()`](Self::with_hidden_label):
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let delete = Button::delete()
    ///     .with_hidden_label(Lc::n("Item 1"))
    ///     .with_prop(PropsOp::set("hx-post", "/items/1/delete"))
    ///     .with_prop(PropsOp::set("hx-confirm", "Delete «Item 1»?"));
    /// ```
    pub fn delete() -> Self {
        Self::plain(Lc::l("button_delete")).with_style(button::Style::Solid(Intent::Severe))
    }

    /// Crea el botón **«Cancelar»** que cierra un [`Dialog`] sin hacer nada.
    ///
    /// Es un [`Button::plain()`] (`type="button"`) con el texto traducido de PageTop, el estilo
    /// [`button::Style::Outline`] con [`Intent::Neutral`] y el atributo
    /// `data-dialog-dismiss="modal"`, que es el que cierra el diálogo (y que cada tema adapta a su
    /// propio vocabulario). Se coloca en el pie del diálogo, junto al botón que confirma:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let dialog = Dialog::new()
    ///     .with_id("confirm-delete")
    ///     .with_child(Html::with(|_| html! { p { "Delete the record?" } }))
    ///     .with_footer(Button::dismiss())
    ///     .with_footer(
    ///         Button::delete()
    ///             .with_prop(PropsOp::set("hx-post", "/items/1/delete"))
    ///             .with_prop(PropsOp::set("data-dialog-dismiss", "modal")),
    ///     );
    /// ```
    ///
    /// Para salir de un formulario hacia otra página se usa [`Button::cancel()`], que es un enlace.
    pub fn dismiss() -> Self {
        Self::plain(Lc::l("button_cancel"))
            .with_prop(PropsOp::set("data-dialog-dismiss", "modal"))
            .with_style(button::Style::Outline(Intent::Neutral))
    }

    /// Crea el enlace **«Cancelar»** para salir de un formulario sin guardar.
    ///
    /// Es un [`Button::anchor()`] a `route` con el texto traducido de PageTop y el estilo
    /// [`button::Style::Link`]. Se coloca junto al botón de envío, y `route` suele ser el listado o
    /// la ficha de la que se vino:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let actions = Flex::buttons()
    ///     .with_child(Button::save())
    ///     .with_child(Button::cancel("/items"));
    /// ```
    ///
    /// Para cerrar un [`Dialog`] se usa [`Button::dismiss()`], que tiene el mismo texto pero no
    /// navega.
    pub fn cancel(route: impl Into<Route>) -> Self {
        Self::anchor(Lc::l("button_cancel"), route).with_style(button::Style::Link)
    }

    /// Crea el enlace **«Volver»** para salir de una ficha o pantalla de sólo lectura.
    ///
    /// Es un [`Button::anchor()`] a `route` con el texto traducido de PageTop y el estilo
    /// [`button::Style::Link`]. Un formulario usa [`Button::cancel()`] en su lugar, que deja claro
    /// que no se guarda nada.
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let back = Button::back("/items");
    /// ```
    pub fn back(route: impl Into<Route>) -> Self {
        Self::anchor(Lc::l("button_back"), route).with_style(button::Style::Link)
    }

    // **< Button BUILDER >*************************************************************************

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

    /// Establece el comportamiento del botón al activarse.
    pub fn with_kind(mut self, kind: button::Kind) -> Self {
        self.kind = kind;
        self
    }

    /// Establece el tamaño visual del botón (usa [`button::Size::None`] para quitarlo).
    pub fn with_size(mut self, size: button::Size) -> Self {
        self.size = size;
        self
    }

    /// Establece el estilo visual del botón (usa [`button::Style::None`] para quitarlo).
    pub fn with_style(mut self, style: button::Style) -> Self {
        self.style = style;
        self
    }

    /// Establece el nombre del botón (atributo `name`).
    ///
    /// Cuando el formulario tiene varios botones de envío, el navegador incluye en el envío el par
    /// `name=value` sólo del botón que activó el formulario. Permite identificar cuál fue pulsado.
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor del botón (atributo `value`).
    ///
    /// Es el dato que el navegador transmite al servidor junto con el `name` cuando este botón
    /// activa el envío. Útil para distinguir entre varios botones de envío en un mismo formulario.
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }

    /// Establece la etiqueta visible del botón (usa [`Lc::none()`] para quitarla).
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Completa la etiqueta con un texto que sólo leen los lectores de pantalla (usa [`Lc::none()`]
    /// para quitarlo).
    ///
    /// Sirve para los botones que se repiten con el mismo texto, como los de cada fila de una
    /// tabla: un lector de pantalla no distingue un «Editar» de otro, pero sí «Editar
    /// Administradores» de «Editar Editores». El texto se añade tras la etiqueta, separado por un
    /// espacio, en un `<span class="visually-hidden">`, así que quien ve la pantalla sigue leyendo
    /// sólo «Editar» y el nombre accesible contiene la etiqueta tal como se ve, como pide WCAG
    /// (2.5.3, *Label in Name*).
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let edit = Button::edit("/roles/2/edit")
    ///     .with_size(button::Size::Small)
    ///     .with_hidden_label(Lc::n("Administradores"));
    /// ```
    pub fn with_hidden_label(mut self, label: Lc) -> Self {
        self.hidden_label = label;
        self
    }

    /// Asigna o quita el icono al principio del botón, antes de la etiqueta. Si se pasa `None`, no
    /// se mostrará.
    ///
    /// *Principio* y *final* siguen el sentido de la escritura: en un idioma que se escribe de
    /// derecha a izquierda, el principio queda a la derecha.
    ///
    /// Un botón sólo con icono (etiqueta [`Lc::none()`]) necesita un nombre accesible: se lo da la
    /// etiqueta del propio icono ([`Icon::with_label()`]).
    pub fn with_icon_start(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.icon_start.alter_component(icon);
        self
    }

    /// Asigna o quita el icono al final del botón, después de la etiqueta. Si se pasa `None`, no se
    /// mostrará. Ver [`with_icon_start()`](Self::with_icon_start).
    pub fn with_icon_end(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.icon_end.alter_component(icon);
        self
    }

    /// Establece el texto emergente del botón (usa [`Lc::none()`] para quitarlo).
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece la ruta de destino y convierte el botón en enlace de navegación (`<a href=...>`).
    /// Puedes usar un [`Route`] vacío (por defecto) para que vuelva a renderizarse como `<button>`.
    /// Ver [`Button::anchor()`] para el constructor equivalente.
    pub fn with_href(mut self, route: impl Into<Route>) -> Self {
        self.href = route.into();
        self
    }

    /// Establece si el botón recibe el foco automáticamente al cargar la página.
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el botón está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
