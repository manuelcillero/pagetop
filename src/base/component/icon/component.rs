use crate::prelude::*;

/// Componente para mostrar un **icono**.
///
/// El icono se define con un [`icon::Source`]: un SVG en línea, un icono del conjunto de PageTop
/// ([`icon::Core`]) o un glifo de una fuente de iconos. Acepta también cualquier tipo convertible
/// en él. Sin origen no se renderiza nada.
///
/// El icono mide `1em`, así que toma el tamaño de la fuente del texto que lo rodea; se puede
/// cambiar con [`with_size()`](Self::with_size). Toma el color del texto, salvo que se le asigne
/// una intención ([`with_intent()`](Self::with_intent)). Siempre se le añade la clase `icon`.
///
/// Sin etiqueta el icono es **decorativo** (`aria-hidden="true"`); con una etiqueta que resuelva a
/// texto ([`with_label()`](Self::with_label)) se anuncia como imagen (`role="img"` y `aria-label`).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let close = Icon::with(icon::Core::Close).with_label(Lc::n("Close"));
///
/// let big = Icon::with(icon::Core::Check)
///     .with_size(UnitValue::RelRem(2.0))
///     .with_intent(Intent::Success);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Icon {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el origen del icono, si lo tiene.
    source: Option<icon::Source>,
    /// Devuelve el tamaño del icono.
    #[getters(copy)]
    size: UnitValue,
    /// Devuelve la intención semántica que da color al icono (si la tiene).
    #[getters(copy)]
    intent: Option<Intent>,
    /// Devuelve la etiqueta de accesibilidad del icono.
    label: Lc,
}

#[async_trait]
impl Component for Icon {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &mut Context) {
        match self.source() {
            None => return,
            Some(icon::Source::Svg { .. } | icon::Source::Core(_)) => {
                self.alter_prop(PropsOp::prepend_classes("icon"))
            }
            Some(icon::Source::Font(classes)) => {
                let classes = util::join!("icon ", classes.as_ref());
                self.alter_prop(PropsOp::prepend_classes(classes))
            }
        };
        if let Some(intent) = self.intent() {
            self.alter_prop(PropsOp::add_classes(util::join!("icon-", intent.color(cx))));
        }
        if self.size() != UnitValue::None {
            let size = self.size().to_string();
            self.alter_prop(PropsOp::add_style("font-size", size));
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let Some(source) = self.source() else {
            return Ok(html! {});
        };
        // Una etiqueta en blanco no da nombre al icono: se trata como decorativo.
        let label = self
            .label()
            .lookup(cx)
            .filter(|label| !label.trim().is_empty());
        let has_label = label.is_some();
        let (viewbox, shapes) = match source {
            icon::Source::Svg { viewbox, shapes } => (viewbox.as_ref(), shapes.0.as_str()),
            icon::Source::Core(icon) => ("0 0 16 16", icon.svg()),
            icon::Source::Font(_) => {
                return Ok(html! {
                    i
                        role=[has_label.then_some("img")]
                        aria-label=[label]
                        aria-hidden=[(!has_label).then_some("true")]
                        (self.props().unpack(cx)) {}
                });
            }
        };
        Ok(html! {
            svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox=(viewbox)
                width="1em"
                height="1em"
                fill="currentColor"
                focusable="false"
                role=[has_label.then_some("img")]
                aria-label=[label]
                aria-hidden=[(!has_label).then_some("true")]
                (self.props().unpack(cx))
            {
                (PreEscaped(shapes))
            }
        })
    }
}

#[builder_impl]
impl Icon {
    /// Crea un icono a partir de su origen, o de cualquier tipo convertible en él.
    pub fn with(source: impl Into<icon::Source>) -> Self {
        Self::default().with_source(source.into())
    }

    // **< Icon BUILDER >***************************************************************************

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

    /// Asigna o quita el origen del icono. Con `None` no se renderiza.
    pub fn with_source(mut self, source: impl Into<Option<icon::Source>>) -> Self {
        self.source = source.into();
        self
    }

    /// Establece el tamaño del icono como tamaño de fuente (`font-size`). Con
    /// [`UnitValue::None`] toma el del texto que lo rodea.
    pub fn with_size(mut self, size: UnitValue) -> Self {
        self.size = size;
        self
    }

    /// Asigna o quita la intención semántica que da color al icono. Añade la clase `icon-*` con el
    /// color que el tema activo asocia a la intención; sin intención, toma el color del texto.
    pub fn with_intent(mut self, intent: impl Into<Option<Intent>>) -> Self {
        self.intent = intent.into();
        self
    }

    /// Establece la etiqueta de accesibilidad del icono. Con [`Lc::none()`] el icono es decorativo.
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }
}
