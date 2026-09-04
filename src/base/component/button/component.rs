use crate::prelude::*;

/// Componente para crear un **botón**.
///
/// Renderiza un botón con soporte para las variantes:
///
/// - [`Button::submit()`]: botón de envío (por defecto).
/// - [`Button::reset()`]: botón de restablecimiento de valores.
/// - [`Button::plain()`]: botón genérico sin comportamiento predeterminado.
/// - [`Button::anchor()`]: enlace de navegación real con el aspecto de un botón.
///
/// No confundir [`Button::anchor()`] con [`button::Style::Link`]: el primero renderiza un `<a
/// href=...>` real que navega; el segundo es sólo un estilo visual (clase `button-link`) que se
/// aplica con [`with_style()`](Self::with_style) sobre cualquiera de las otras variantes, que
/// siguen siendo un `<button>`.
///
/// Un botón puede usarse dentro o fuera de un formulario.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let save   = Button::submit(Lc::n("Save"));
/// let cancel = Button::plain(Lc::n("Cancel"));
/// let clear  = Button::reset(Lc::n("Clear"));
/// let edit   = Button::anchor(Lc::n("Edit"), "/items/1/edit");
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
    /// Devuelve el texto emergente del botón (atributo `title`).
    title: Lc,
    /// Devuelve la ruta de destino cuando el botón se renderiza como enlace de navegación
    /// (`<a href=...>` en vez de `<button>`). Vacía por defecto: en ese caso `prepare()` renderiza
    /// un `<button>` normal, ignorando este campo.
    href: Route,
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

    fn setup(&mut self, cx: &Context) {
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
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
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
                    (self.label().using(cx))
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
                (self.label().using(cx))
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
    pub fn anchor(label: Lc, route: impl Into<Route>) -> Self {
        Self {
            label,
            href: route.into(),
            ..Default::default()
        }
    }

    // **< Button BUILDER >*************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
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
