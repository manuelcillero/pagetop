use pagetop::prelude::*;

use crate::theme::form;

/// Componente para crear un **formulario**.
///
/// Este componente renderiza un `<form>` estándar con soporte para los atributos más habituales:
///
/// - `id`: identificador opcional del formulario.
/// - `classes`: clases CSS adicionales (p. ej. utilidades CSS).
/// - `action`: URL/ruta de destino para el envío.
/// - `method`: método usado por el formulario para el envío de los datos (ver explicaciones en
///   [`form::Method`](crate::theme::form::Method)).
/// - `accept-charset`: juego de caracteres aceptado (por defecto es `"UTF-8"`).
/// - `children`: contenido del formulario.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let search = Form::new()
///     .with_id("search")
///     .with_action("/search")
///     .with_method(form::Method::Get)
///     .with_child(form::input::Field::search().with_name("q"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Form {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del formulario.
    classes: Classes,
    /// Devuelve la URL/ruta de destino del formulario.
    action: AttrValue,
    /// Devuelve el método para enviar el formulario.
    method: form::Method,
    /// Devuelve el juego de caracteres aceptado por el formulario.
    #[default(_code = "AttrValue::new(\"UTF-8\")")]
    charset: AttrValue,
    /// Devuelve la lista de componentes del formulario.
    children: Children,
}

impl Component for Form {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_classes(ClassesOp::Prepend, "form");
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let method = match self.method() {
            form::Method::Post => Some("post"),
            form::Method::Get => None,
        };
        Ok(html! {
            form
                id=[self.id()]
                class=[self.classes().get()]
                action=[self.action().get()]
                method=[method]
                accept-charset=[self.charset().get()]
            {
                (self.children().render(cx))
            }
        })
    }
}

impl Form {
    // **< Form BUILDER >***************************************************************************

    /// Establece el identificador único (`id`) del formulario.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al formulario.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece la URL/ruta de destino del formulario.
    #[builder_fn]
    pub fn with_action(mut self, action: impl AsRef<str>) -> Self {
        self.action.alter_str(action);
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
