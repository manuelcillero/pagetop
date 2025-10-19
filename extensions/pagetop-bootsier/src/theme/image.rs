use pagetop::prelude::*;

use crate::prelude::*;

#[derive(AutoDefault)]
pub enum ImageSource {
    #[default]
    //Logo(PageTopLogo),
    Responsive(String),
    Thumbnail(String),
    Static(String),
}

#[derive(AutoDefault)]
pub enum ImageSize {
    #[default]
    Auto,
    Dimensions(UnitValue, UnitValue),
    Width(UnitValue),
    Height(UnitValue),
    Both(UnitValue),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Image {
    id     : AttrId,
    classes: AttrClasses,
    source : ImageSource,
    alt    : AttrL10n,
    size   : ImageSize,
    border : Border,
    rounded: Rounded,
}

impl Component for Image {
    fn new() -> Self {
        Image::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                String::from(match self.source() {
                    //ImageSource::Logo(_) => "img-fluid",
                    ImageSource::Responsive(_) => "img-fluid",
                    ImageSource::Thumbnail(_) => "img-thumbnail",
                    _ => "",
                }),
                self.border().to_string(),
                self.rounded().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let dimensions = match self.size() {
            ImageSize::Auto => None,
            ImageSize::Dimensions(w, h) => {
                let w = w.to_string();
                let h = h.to_string();
                Some(join!("width: ", w, "; height: ", h, ";"))
            }
            ImageSize::Width(w) => {
                let w = w.to_string();
                Some(join!("width: ", w, ";"))
            }
            ImageSize::Height(h) => {
                let h = h.to_string();
                Some(join!("height: ", h, ";"))
            }
            ImageSize::Both(v) => {
                let v = v.to_string();
                Some(join!("width: ", v, "; height: ", v, ";"))
            }
        };
        let source = match self.source() {
            /*
            ImageSource::Logo(logo) => {
                return PrepareMarkup::With(html! {
                    span
                        id=[self.id()]
                        class=[self.classes().get()]
                        style=[dimensions]
                    {
                        (logo.render(cx))
                    }
                })
            }
            */
            ImageSource::Responsive(source) => Some(source),
            ImageSource::Thumbnail(source) => Some(source),
            ImageSource::Static(source) => Some(source),
        };
        PrepareMarkup::With(html! {
            img
                src=[source]
                alt=[self.alternative().lookup(cx)]
                id=[self.id()]
                class=[self.classes().get()]
                style=[dimensions] {}
        })
    }
}

impl Image {
    pub fn with(source: ImageSource) -> Self {
        Image::default().with_source(source)
    }

    // **< Image BUILDER >**************************************************************************

    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[builder_fn]
    pub fn with_source(mut self, source: ImageSource) -> Self {
        self.source = source;
        self
    }

    #[builder_fn]
    pub fn with_alternative(mut self, alt: L10n) -> Self {
        self.alt.alter_value(alt);
        self
    }

    #[builder_fn]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    #[builder_fn]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    #[builder_fn]
    pub fn with_rounded(mut self, rounded: Rounded) -> Self {
        self.rounded = rounded;
        self
    }

    // **< Image GETTERS >**************************************************************************

    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    pub fn source(&self) -> &ImageSource {
        &self.source
    }

    pub fn alternative(&self) -> &AttrL10n {
        &self.alt
    }

    pub fn size(&self) -> &ImageSize {
        &self.size
    }

    pub fn border(&self) -> &Border {
        &self.border
    }

    pub fn rounded(&self) -> &Rounded {
        &self.rounded
    }
}
