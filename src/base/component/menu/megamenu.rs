use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Megamenu {
    id    : AttrId,
    groups: Children,
}

impl Component for Megamenu {
    fn new() -> Self {
        Megamenu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu__mega" {
                (self.groups().render(cx))
            }
        })
    }
}

impl Megamenu {
    // **< Megamenu BUILDER >***********************************************************************

    /// Establece el identificador único (`id`) del megamenú.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Añade un nuevo grupo al menú.
    pub fn add_group(mut self, group: menu::Group) -> Self {
        self.groups.alter_typed(TypedOp::Add(Typed::with(group)));
        self
    }

    /// Modifica la lista de grupos (`children`) aplicando una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_groups(mut self, op: TypedOp<menu::Group>) -> Self {
        self.groups.alter_typed(op);
        self
    }

    // **< Megamenu GETTERS >***********************************************************************

    /// Devuelve la lista de grupos (`children`) del megamenú.
    pub fn groups(&self) -> &Children {
        &self.groups
    }
}
