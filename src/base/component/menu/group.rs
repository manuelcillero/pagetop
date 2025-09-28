use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Group {
    id      : AttrId,
    elements: Children,
}

impl Component for Group {
    fn new() -> Self {
        Group::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu-group" {
                (self.elements().render(cx))
            }
        })
    }
}

impl Group {
    // **< Group BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) del grupo.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Añade un nuevo elemento al menú.
    pub fn add_element(mut self, element: menu::Element) -> Self {
        self.elements
            .alter_typed(TypedOp::Add(Typed::with(element)));
        self
    }

    /// Modifica la lista de elementos (`children`) aplicando una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_elements(mut self, op: TypedOp<menu::Element>) -> Self {
        self.elements.alter_typed(op);
        self
    }

    // **< Group GETTERS >**************************************************************************

    /// Devuelve la lista de elementos (`children`) del grupo.
    pub fn elements(&self) -> &Children {
        &self.elements
    }
}
