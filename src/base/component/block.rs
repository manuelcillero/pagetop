use crate::prelude::*;

/// Componente genérico que representa un bloque de contenido.
///
/// Los bloques se utilizan como contenedores de otros componentes o contenidos, con un título
/// opcional y un cuerpo que sólo se renderiza si existen componentes hijos (*children*).
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Block {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS asociadas al bloque.
    classes: Classes,
    /// Devuelve el título del bloque.
    title: L10n,
    /// Devuelve la lista de componentes hijo del bloque.
    children: Children,
}

impl Component for Block {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_classes(ClassesOp::Prepend, "block");
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let block_body = self.children().render(cx);

        if block_body.is_empty() {
            return Ok(html! {});
        }

        let id = cx.required_id::<Block>(self.id());

        Ok(html! {
            div id=(id) class=[self.classes().get()] {
                @if let Some(title) = self.title().lookup(cx) {
                    h2 class="block__title" { span { (title) } }
                }
                div class="block__body" { (block_body) }
            }
        })
    }
}

impl Block {
    // **< Block BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) del bloque.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al bloque.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece el título del bloque.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Añade un nuevo componente al bloque o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
