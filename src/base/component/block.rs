use crate::prelude::*;

/// Componente genérico que representa un bloque de contenido.
///
/// Los bloques se utilizan como contenedores de otros componentes o contenidos, con un título
/// opcional y un cuerpo que sólo se renderiza si existen componentes hijos (*children*).
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Block {
    id      : AttrId,
    classes : AttrClasses,
    title   : L10n,
    children: Children,
}

impl Component for Block {
    fn new() -> Self {
        Block::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "block");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let block_body = self.children().render(cx);

        if block_body.is_empty() {
            return PrepareMarkup::None;
        }

        let id = cx.required_id::<Block>(self.id());

        PrepareMarkup::With(html! {
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
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al bloque.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    /// Establece el título del bloque.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Añade un nuevo componente hijo al bloque.
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

    // **< Block GETTERS >**************************************************************************

    /// Devuelve las clases CSS asociadas al bloque.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el título del bloque como [`L10n`].
    pub fn title(&self) -> &L10n {
        &self.title
    }

    /// Devuelve la lista de componentes (`children`) del bloque.
    pub fn children(&self) -> &Children {
        &self.children
    }
}
