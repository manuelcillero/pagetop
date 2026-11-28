use crate::prelude::*;

/// Componente para crear un **grupo de controles relacionados** en un formulario.
///
/// Renderiza un `<fieldset>` con una leyenda opcional que sirve de encabezado y un texto de ayuda
/// también opcional que explica el grupo y aparece justo antes de los controles. Es un elemento
/// semántico que mejora la accesibilidad porque los lectores de pantalla anuncian la leyenda antes
/// de leer cada control del contenido. El texto de ayuda se pinta como el de los campos
/// ([`form::parts::Help`]) y el `fieldset` lo enlaza con `aria-describedby`; para ello, si no tiene
/// identificador, se le asigna uno único.
///
/// Los componentes del grupo se añaden con [`with_child()`](Fieldset::with_child). Si no hay
/// contenido para renderizar, el `fieldset` no se genera. Si está deshabilitado, todos sus
/// controles hijos quedan deshabilitados automáticamente por el navegador.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let personal_data = form::Fieldset::new()
///     .with_legend(Lc::n("Personal data"))
///     .with_help(Lc::n("Enter your full name and contact email."))
///     .with_child(form::input::Field::text().with_name("name").with_label(Lc::n("Full name")))
///     .with_child(form::input::Field::email().with_name("email").with_label(Lc::n("Email")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Fieldset {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la leyenda del `fieldset`.
    legend: Lc,
    /// Devuelve el texto de ayuda del `fieldset`.
    help: Lc,
    /// Devuelve si el `fieldset` está deshabilitado.
    disabled: bool,
    /// Devuelve la lista de componentes del `fieldset`.
    children: Children,
}

#[async_trait]
impl Component for Fieldset {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &mut Context) {
        // El texto de ayuda se enlaza con `aria-describedby`, que necesita el identificador.
        if !self.help().is_none() {
            let id = cx.required_id::<Self>(self.id(), 1);
            self.alter_prop(PropsOp::ensure_id(id));
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let children = self.children().render(cx).await;

        if children.is_empty() {
            return Ok(html! {});
        }

        let id = self.id();
        let help = form::parts::Help::new(self.help(), id.as_deref())
            .with_class("form-text fieldset-help");

        Ok(html! {
            fieldset
                (self.props().unpack(cx))
                disabled[*self.disabled()]
                aria-describedby=[help.id()]
            {
                @if let Some(legend) = self.legend().lookup(cx) {
                    legend { (legend) }
                }
                (help.render(cx))
                (children)
            }
        })
    }
}

#[builder_impl]
impl Fieldset {
    // **< Fieldset BUILDER >***********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece la leyenda del `fieldset` (usa [`Lc::none()`] para quitarla).
    pub fn with_legend(mut self, legend: Lc) -> Self {
        self.legend = legend;
        self
    }

    /// Establece el texto de ayuda del `fieldset`, que explica el grupo antes de sus controles (usa
    /// [`Lc::none()`] para quitarlo).
    pub fn with_help(mut self, help: Lc) -> Self {
        self.help = help;
        self
    }

    /// Establece si el `fieldset` está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Añade un nuevo componente al `fieldset`, o aplica una operación [`ChildOp`] sobre la lista
    /// de componentes (`children`).
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
