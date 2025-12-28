use crate::prelude::*;

const DEFAULT_VIEWBOX: &str = "0 0 16 16";

#[derive(AutoDefault)]
pub enum IconKind {
    #[default]
    None,
    Font(FontSize),
    Svg {
        shapes: Markup,
        viewbox: AttrValue,
    },
}

#[derive(AutoDefault, Getters)]
pub struct Icon {
    /// Devuelve las clases CSS asociadas al icono.
    classes: Classes,
    icon_kind: IconKind,
    aria_label: AttrL10n,
}

impl Component for Icon {
    fn new() -> Self {
        Self::default()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        if !matches!(self.icon_kind(), IconKind::None) {
            self.alter_classes(ClassesOp::Prepend, "icon");
        }
        if let IconKind::Font(font_size) = self.icon_kind() {
            self.alter_classes(ClassesOp::Add, font_size.as_str());
        }
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.icon_kind() {
            IconKind::None => PrepareMarkup::None,
            IconKind::Font(_) => {
                let aria_label = self.aria_label().lookup(cx);
                let has_label = aria_label.is_some();
                PrepareMarkup::With(html! {
                    i
                        class=[self.classes().get()]
                        role=[has_label.then_some("img")]
                        aria-label=[aria_label]
                        aria-hidden=[(!has_label).then_some("true")]
                    {}
                })
            }
            IconKind::Svg { shapes, viewbox } => {
                let aria_label = self.aria_label().lookup(cx);
                let has_label = aria_label.is_some();
                let viewbox = viewbox.get().unwrap_or_else(|| DEFAULT_VIEWBOX.to_string());
                PrepareMarkup::With(html! {
                    svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox=(viewbox)
                        fill="currentColor"
                        focusable="false"
                        class=[self.classes().get()]
                        role=[has_label.then_some("img")]
                        aria-label=[aria_label]
                        aria-hidden=[(!has_label).then_some("true")]
                    {
                        (shapes)
                    }
                })
            }
        }
    }
}

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

    /// Modifica la lista de clases CSS aplicadas al icono.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[builder_fn]
    pub fn with_icon_kind(mut self, icon_kind: IconKind) -> Self {
        self.icon_kind = icon_kind;
        self
    }

    #[builder_fn]
    pub fn with_aria_label(mut self, label: L10n) -> Self {
        self.aria_label.alter_value(label);
        self
    }
}
