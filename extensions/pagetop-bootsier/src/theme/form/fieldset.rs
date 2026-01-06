use pagetop::prelude::*;

/// Agrupa controles relacionados de un formulario (`<fieldset>`).
///
/// Se usa para mejorar la accesibilidad cuando se acompaña de una leyenda que encabeza el grupo.
#[derive(AutoDefault, Getters)]
pub struct Fieldset {
    #[getters(skip)]
    id: AttrId,
    classes: Classes,
    legend: Attr<L10n>,
    disabled: bool,
    children: Children,
}

impl Component for Fieldset {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            fieldset id=[self.id()] class=[self.classes().get()] disabled[*self.disabled()] {
                @if let Some(legend) = self.legend().lookup(cx) {
                    legend { (legend) }
                }
                (self.children().render(cx))
            }
        })
    }
}

impl Fieldset {
    // **< Fieldset BUILDER >***********************************************************************

    /// Establece el identificador único (`id`) del `fieldset` (grupo de controles).
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al `fieldset`.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece la leyenda del `fieldset`.
    #[builder_fn]
    pub fn with_legend(mut self, legend: L10n) -> Self {
        self.legend.alter_value(legend);
        self
    }

    /// Establece si el `fieldset` está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Añade un nuevo componente hijo al `fieldset`.
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
