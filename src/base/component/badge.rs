use crate::prelude::*;

/// Componente para mostrar una **etiqueta corta informativa** (*badge*).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let badge = Badge::labeled(Lc::n("Admin")).with_intent(Intent::Severe);
///
/// // Equivalente usando el constructor directo del tipo.
/// let badge = Badge::severe(Lc::n("Admin"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Badge {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la etiqueta del badge.
    label: Lc,
    /// Devuelve la intención semántica del badge.
    #[getters(copy)]
    #[default(Intent::Neutral)]
    intent: Intent,
}

#[async_trait]
impl Component for Badge {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes(util::join!(
            "badge badge-",
            self.intent().color(cx)
        )));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            span (self.props()) {
                (self.label().using(cx))
            }
        })
    }
}

#[builder_impl]
impl Badge {
    /// Crea un badge predeterminado (`Intent::default()`) con la etiqueta indicada.
    pub fn labeled(label: Lc) -> Self {
        Self {
            label,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *primary* con la etiqueta indicada.
    pub fn primary(label: Lc) -> Self {
        Self {
            label,
            intent: Intent::Primary,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *neutral* con la etiqueta indicada.
    pub fn neutral(label: Lc) -> Self {
        Self {
            label,
            intent: Intent::Neutral,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *success* con la etiqueta indicada.
    pub fn success(label: Lc) -> Self {
        Self {
            label,
            intent: Intent::Success,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *info* con la etiqueta indicada.
    pub fn info(label: Lc) -> Self {
        Self {
            label,
            intent: Intent::Info,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *warning* con la etiqueta indicada.
    pub fn warning(label: Lc) -> Self {
        Self {
            label,
            intent: Intent::Warning,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *severe* con la etiqueta indicada.
    pub fn severe(label: Lc) -> Self {
        Self {
            label,
            intent: Intent::Severe,
            ..Default::default()
        }
    }

    // **< Badge BUILDER >**************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece la etiqueta del badge.
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece la intención semántica del badge.
    pub fn with_intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }
}
