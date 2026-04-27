use pagetop::prelude::*;

/// Componente para crear un **grupo de controles relacionados** en un formulario.
///
/// Renderiza un `<fieldset>` con una leyenda opcional que sirve de encabezado y una descripción
/// también opcional que aparece justo antes de los controles. Es un elemento semántico que mejora
/// la accesibilidad porque los lectores de pantalla anuncian la leyenda antes de leer cada control
/// del contenido.
///
/// Los componentes del grupo se añaden con [`with_child()`](Fieldset::with_child). Si no hay
/// contenido para renderizar, el `fieldset` no se genera. Si está deshabilitado, todos sus
/// controles hijos quedan deshabilitados automáticamente por el navegador.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let personal_data = form::Fieldset::new()
///     .with_legend(L10n::n("Personal data"))
///     .with_description(L10n::n("Enter your full name and contact email."))
///     .with_child(form::input::Field::text().with_name("name").with_label(L10n::n("Full name")))
///     .with_child(form::input::Field::email().with_name("email").with_label(L10n::n("Email")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Fieldset {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del `fieldset`.
    classes: Classes,
    /// Devuelve la leyenda del `fieldset`.
    legend: Attr<L10n>,
    /// Devuelve la descripción del `fieldset`.
    description: Attr<L10n>,
    /// Devuelve si el `fieldset` está deshabilitado.
    disabled: bool,
    /// Devuelve la lista de componentes del `fieldset`.
    children: Children,
}

impl Component for Fieldset {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let children = self.children().render(cx);

        if children.is_empty() {
            return Ok(html! {});
        }

        Ok(html! {
            fieldset id=[self.id()] class=[self.classes().get()] disabled[*self.disabled()] {
                @if let Some(legend) = self.legend().lookup(cx) {
                    legend { (legend) }
                }
                @if let Some(description) = self.description().lookup(cx) {
                    p class="fieldset-description" { (description) }
                }
                (children)
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

    /// Establece o elimina la leyenda del `fieldset` (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_legend(mut self, legend: impl Into<Option<L10n>>) -> Self {
        self.legend.alter_opt(legend.into());
        self
    }

    /// Establece o elimina la descripción del `fieldset` (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_description(mut self, description: impl Into<Option<L10n>>) -> Self {
        self.description.alter_opt(description.into());
        self
    }

    /// Establece si el `fieldset` está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Añade un nuevo componente al `fieldset`, o aplica una operación [`ChildOp`] sobre la lista
    /// de componentes (`children`).
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
