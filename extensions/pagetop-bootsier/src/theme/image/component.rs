use pagetop::prelude::*;

use crate::prelude::*;

/// Componente para renderizar una **imagen**.
///
/// - Ajusta su disposición según el origen definido en [`image::Source`].
/// - Permite configurar **dimensiones** ([`with_size()`](Self::with_size)), **borde**
///   ([`classes::Border`](crate::theme::classes::Border)) y **redondeo de esquinas**
///   ([`classes::Rounded`](crate::theme::classes::Rounded)).
/// - Resuelve el texto alternativo `alt` con **localización** mediante [`L10n`].
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Image {
    id     : AttrId,
    classes: AttrClasses,
    size   : image::Size,
    source : image::Source,
    alt    : AttrL10n,
}

impl Component for Image {
    fn new() -> Self {
        Image::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, self.source().to_class());
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let dimensions = self.size().to_style();
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
    ///
    /// También acepta clases predefinidas para:
    ///
    /// - Establecer bordes ([`classes::Border`]).
    /// - Redondear las esquinas ([`classes::Rounded`]).
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
}
