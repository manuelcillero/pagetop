use crate::prelude::*;

use crate::base::component::form;

/// Componente para crear un **formulario**.
///
/// Renderiza un formulario estándar con soporte para los atributos más habituales:
///
/// - `id`: identificador opcional del formulario.
/// - `classes`: clases CSS adicionales (p. ej. utilidades CSS).
/// - `action`: URL/ruta de destino para el envío.
/// - `method`: método usado por el formulario para el envío de los datos (ver [`form::Method`]).
/// - `accept-charset`: juego de caracteres aceptado (por defecto es `"UTF-8"`).
/// - `children`: contenido del formulario.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let form_login = Form::new()
///     .with_id("login")
///     .with_action("/login")
///     .with_child(
///         form::input::Field::email()
///             .with_name("email")
///             .with_label(Lc::n("Email"))
///             .with_required(true),
///     )
///     .with_child(
///         form::input::Field::password()
///             .with_name("password")
///             .with_label(Lc::n("Password"))
///             .with_required(true),
///     )
///     .with_child(
///         form::Checkbox::check()
///             .with_name("remember")
///             .with_label(Lc::n("Remember me")),
///     )
///     .with_child(
///         Button::submit(Lc::n("Sign in"))
///     );
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Form {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la ruta de destino del formulario.
    action: Route,
    /// Devuelve el método para enviar el formulario.
    method: form::Method,
    /// Devuelve el juego de caracteres aceptado por el formulario.
    #[default(_code = "AttrValue::new(\"UTF-8\")")]
    charset: AttrValue,
    /// Devuelve la lista de componentes del formulario.
    children: Children,
}

#[async_trait]
impl Component for Form {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("form"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let method = match self.method() {
            form::Method::Post => Some("post"),
            form::Method::Get => None,
        };
        Ok(html! {
            form
                (self.props())
                action=[self.action().try_resolve(cx)]
                method=[method]
                accept-charset=[self.charset().as_deref()]
            {
                (self.children().render(cx).await)
            }
        })
    }
}

impl Form {
    // **< Form BUILDER >***************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece la ruta de destino del formulario.
    ///
    /// Acepta un literal, un `String`, o una [`Route`] explícita construida con [`Route::with()`]
    /// para rutas que dependan del contexto de renderizado.
    #[builder_fn]
    pub fn with_action(mut self, action: impl Into<Route>) -> Self {
        self.action = action.into();
        self
    }

    /// Establece el método para enviar el formulario.
    ///
    /// - `GET`: el atributo `method` se omite.
    /// - `POST`: se establece `method="post"`.
    #[builder_fn]
    pub fn with_method(mut self, method: form::Method) -> Self {
        self.method = method;
        self
    }

    /// Establece el juego de caracteres aceptado por el formulario.
    ///
    /// Por defecto se utiliza `"UTF-8"`.
    #[builder_fn]
    pub fn with_charset(mut self, charset: impl AsRef<str>) -> Self {
        self.charset.alter_str(charset);
        self
    }

    /// Añade un nuevo componente al formulario o modifica la lista de componentes (`children`) con
    /// una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
