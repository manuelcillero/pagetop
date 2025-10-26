use pagetop::prelude::*;

use crate::prelude::*;

/// Componente para renderizar una **imagen**.
///
/// - Ajusta su disposición según el origen definido en [`image::Source`].
/// - Permite configurar **dimensiones** ([`with_size()`](Self::with_size)), **borde**
///   ([`with_border()`](Self::with_border)) y **redondeo de esquinas**
///   ([`with_rounded()`](Self::with_rounded)).
/// - Resuelve el texto alternativo `alt` con **localización** mediante [`L10n`].
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Image {
    id     : AttrId,
    classes: AttrClasses,
    size   : image::Size,
    source : image::Source,
    alt    : AttrL10n,
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
                    image::Source::Logo(_) => "img-fluid",
                    image::Source::Responsive(_) => "img-fluid",
                    image::Source::Thumbnail(_) => "img-thumbnail",
                    image::Source::Plain(_) => "",
                }),
                self.border().to_string(),
                self.rounded().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let dimensions = match self.size() {
            image::Size::Auto => None,
            image::Size::Dimensions(w, h) => {
                let w = w.to_string();
                let h = h.to_string();
                Some(join!("width: ", w, "; height: ", h, ";"))
            }
            image::Size::Width(w) => {
                let w = w.to_string();
                Some(join!("width: ", w, ";"))
            }
            image::Size::Height(h) => {
                let h = h.to_string();
                Some(join!("height: ", h, ";"))
            }
            image::Size::Both(v) => {
                let v = v.to_string();
                Some(join!("width: ", v, "; height: ", v, ";"))
            }
        };
        let alt_text = self.alternative().lookup(cx).unwrap_or_default();
        let is_decorative = alt_text.is_empty();
        let source = match self.source() {
            image::Source::Logo(logo) => {
                return PrepareMarkup::With(html! {
                    span
                        id=[self.id()]
                        class=[self.classes().get()]
                        style=[dimensions]
                        role=[(!is_decorative).then_some("img")]
                        aria-label=[(!is_decorative).then_some(alt_text)]
                        aria-hidden=[is_decorative.then_some("true")]
                    {
                        (logo.render(cx))
                    }
                })
            }
            image::Source::Responsive(source) => Some(source),
            image::Source::Thumbnail(source) => Some(source),
            image::Source::Plain(source) => Some(source),
        };
        PrepareMarkup::With(html! {
            img
                src=[source]
                alt=(alt_text)
                id=[self.id()]
                class=[self.classes().get()]
                style=[dimensions] {}
        })
    }
}

impl Image {
    /// Crea rápidamente una imagen especificando su origen.
    pub fn with(source: image::Source) -> Self {
        Image::default().with_source(source)
    }

    // **< Image BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) de la imagen.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas a la imagen.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    /// Define las dimensiones de la imagen (auto, ancho/alto, ambos).
    #[builder_fn]
    pub fn with_size(mut self, size: image::Size) -> Self {
        self.size = size;
        self
    }

    /// Establece el origen de la imagen, influyendo en su disposición en el contenido.
    #[builder_fn]
    pub fn with_source(mut self, source: image::Source) -> Self {
        self.source = source;
        self
    }

    /// Define el texto alternativo localizado ([`L10n`]) para la imagen.
    ///
    /// Se recomienda siempre aportar un texto alternativo salvo que la imagen sea puramente
    /// decorativa.
    #[builder_fn]
    pub fn with_alternative(mut self, alt: L10n) -> Self {
        self.alt.alter_value(alt);
        self
    }

    /// Establece el borde de la imagen ([`Border`]).
    #[builder_fn]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Establece esquinas redondeadas para la imagen ([`Rounded`]).
    #[builder_fn]
    pub fn with_rounded(mut self, rounded: Rounded) -> Self {
        self.rounded = rounded;
        self
    }

    // **< Image GETTERS >**************************************************************************

    /// Devuelve las clases CSS asociadas a la imagen.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve las dimensiones de la imagen.
    pub fn size(&self) -> &image::Size {
        &self.size
    }

    /// Devuelve el origen de la imagen.
    pub fn source(&self) -> &image::Source {
        &self.source
    }

    /// Devuelve el texto alternativo localizado.
    pub fn alternative(&self) -> &AttrL10n {
        &self.alt
    }

    /// Devuelve el borde configurado de la imagen.
    pub fn border(&self) -> &Border {
        &self.border
    }

    /// Devuelve las esquinas redondeadas configuradas para la imagen.
    pub fn rounded(&self) -> &Rounded {
        &self.rounded
    }
}
