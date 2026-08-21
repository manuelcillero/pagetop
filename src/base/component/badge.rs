use crate::prelude::*;

// **< BadgeKind >**********************************************************************************

/// Tipo de [`Badge`].
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BadgeKind {
    Primary,
    #[default]
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
}

// **< Badge >**************************************************************************************

/// Componente para mostrar una **etiqueta corta informativa** (*badge*).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let badge = Badge::labeled(Lc::n("Admin")).with_kind(BadgeKind::Danger);
///
/// // Equivalente usando el constructor directo del tipo.
/// let badge = Badge::danger(Lc::n("Admin"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Badge {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la etiqueta del badge.
    label: Lc,
    /// Devuelve el tipo del badge.
    #[getters(copy)]
    kind: BadgeKind,
}

#[async_trait]
impl Component for Badge {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    #[rustfmt::skip]
    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes(match self.kind() {
            BadgeKind::Primary   => "badge badge-primary",
            BadgeKind::Secondary => "badge badge-secondary",
            BadgeKind::Success   => "badge badge-success",
            BadgeKind::Info      => "badge badge-info",
            BadgeKind::Warning   => "badge badge-warning",
            BadgeKind::Danger    => "badge badge-danger",
        }));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            span (self.props()) {
                (self.label().using(cx))
            }
        })
    }
}

impl Badge {
    /// Crea un badge predeterminado (`BadgeKind::default()`) con la etiqueta indicada.
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
            kind: BadgeKind::Primary,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *secondary* con la etiqueta indicada.
    pub fn secondary(label: Lc) -> Self {
        Self {
            label,
            kind: BadgeKind::Secondary,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *success* con la etiqueta indicada.
    pub fn success(label: Lc) -> Self {
        Self {
            label,
            kind: BadgeKind::Success,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *info* con la etiqueta indicada.
    pub fn info(label: Lc) -> Self {
        Self {
            label,
            kind: BadgeKind::Info,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *warning* con la etiqueta indicada.
    pub fn warning(label: Lc) -> Self {
        Self {
            label,
            kind: BadgeKind::Warning,
            ..Default::default()
        }
    }

    /// Crea un badge de tipo *danger* con la etiqueta indicada.
    pub fn danger(label: Lc) -> Self {
        Self {
            label,
            kind: BadgeKind::Danger,
            ..Default::default()
        }
    }

    // **< Badge BUILDER >**************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece la etiqueta del badge.
    #[builder_fn]
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece el tipo del badge.
    #[builder_fn]
    pub fn with_kind(mut self, kind: BadgeKind) -> Self {
        self.kind = kind;
        self
    }
}
