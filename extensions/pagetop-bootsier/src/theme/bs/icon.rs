use crate::theme::*;

const DEFAULT_VIEWBOX: &str = "0 0 16 16";

#[derive(AutoDefault, Clone)]
pub enum IconKind {
    #[default]
    None,
    Font(FontSize),
    Svg {
        shapes: Markup,
        viewbox: AttrValue,
    },
}

#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Icon {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    icon_kind: IconKind,
    aria_label: AttrLc,
}

#[async_trait]
impl Component for Icon {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        if !matches!(self.icon_kind(), IconKind::None) {
            self.alter_prop(PropsOp::prepend_classes("icon"));
        }
        if let IconKind::Font(font_size) = self.icon_kind() {
            self.alter_prop(PropsOp::add_classes(font_size.as_str()));
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(match self.icon_kind() {
            IconKind::None => html! {},
            IconKind::Font(_) => {
                let aria_label = self.aria_label().lookup(cx);
                let has_label = aria_label.is_some();
                html! {
                    i
                        (self.props())
                        role=[has_label.then_some("img")]
                        aria-label=[aria_label]
                        aria-hidden=[(!has_label).then_some("true")]
                    {}
                }
            }
            IconKind::Svg { shapes, viewbox } => {
                let aria_label = self.aria_label().lookup(cx);
                let has_label = aria_label.is_some();
                let viewbox = viewbox.get().unwrap_or_else(|| DEFAULT_VIEWBOX.to_string());
                html! {
                    svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox=(viewbox)
                        fill="currentColor"
                        focusable="false"
                        (self.props())
                        role=[has_label.then_some("img")]
                        aria-label=[aria_label]
                        aria-hidden=[(!has_label).then_some("true")]
                    {
                        (shapes)
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl Icon {
    pub fn font() -> Self {
        Self::default().with_icon_kind(IconKind::Font(FontSize::default()))
    }

    pub fn font_sized(font_size: FontSize) -> Self {
        Self::default().with_icon_kind(IconKind::Font(font_size))
    }

    pub fn svg(shapes: Markup) -> Self {
        Self::default().with_icon_kind(IconKind::Svg {
            shapes,
            viewbox: AttrValue::default(),
        })
    }

    pub fn svg_with_viewbox(shapes: Markup, viewbox: impl AsRef<str>) -> Self {
        Self::default().with_icon_kind(IconKind::Svg {
            shapes,
            viewbox: AttrValue::new(viewbox),
        })
    }

    // **< Icon BUILDER >***************************************************************************

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

    pub fn with_icon_kind(mut self, icon_kind: IconKind) -> Self {
        self.icon_kind = icon_kind;
        self
    }

    pub fn with_aria_label(mut self, label: Lc) -> Self {
        self.aria_label.alter_value(label);
        self
    }
}
