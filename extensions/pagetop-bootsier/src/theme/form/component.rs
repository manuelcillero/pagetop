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
/// ```ignore
/// use pagetop::prelude::*;
/// use crate::prelude::*;
///
/// let form = Form::new()
///     .with_id("search")
///     .with_action("/search")
///     .with_method(form::Method::Get)
///     .with_classes(ClassesOp::Add, "mb-3")
///     .add_child(Input::new().with_name("q"));
/// ```
#[derive(AutoDefault, Getters)]
pub struct Form {
    #[getters(skip)]
    id: AttrId,
    classes: Classes,
    action: AttrValue,
    method: form::Method,
    #[default(_code = "AttrValue::new(\"UTF-8\")")]
    charset: AttrValue,
    children: Children,
}

impl Component for Form {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "form");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let method = match self.method() {
            form::Method::Post => Some("post"),
            form::Method::Get => None,
        };
        PrepareMarkup::With(html! {
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
    /// Por defecto se usa `"UTF-8"`.
    #[builder_fn]
    pub fn with_charset(mut self, charset: impl AsRef<str>) -> Self {
        self.charset.alter_str(charset);
        self
    }

    /// Añade un nuevo componente hijo al formulario.
    #[inline]
    pub fn add_child(mut self, component: impl Component) -> Self {
        self.children.add(Child::with(component));
        self
    }

    /// Modifica la lista de componentes (`children`) aplicando una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }
}
