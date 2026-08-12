use pagetop::prelude::*;

use crate::theme::*;

/// Componente para renderizar una **imagen**.
///
/// A una imagen se le puede:
///
/// - Establecer su contenido a partir del origen definido en
///   [`image::Source`](crate::theme::bs::image::Source).
/// - Configurar sus **dimensiones** ([`with_size()`](Self::with_size)), **borde**
///   ([`Border`](crate::theme::class::Border)) y **redondeo de esquinas**
///   ([`Rounded`](crate::theme::class::Rounded)).
/// - Aplicar el texto alternativo `alt` con **localización** mediante [`Lc`].
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Image {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve las dimensiones de la imagen.
    size: bs::image::Size,
    /// Devuelve el origen de la imagen.
    source: bs::image::Source,
    /// Devuelve el texto alternativo localizado.
    alternative: Attr<Lc>,
}

#[async_trait]
impl Component for Image {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        // Clases CSS por defecto para la imagen, según el origen seleccionado.
        self.alter_prop(PropsOp::prepend_classes(self.source().to_class()));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let dimensions = self.size().to_style();
        let alt_text = self.alternative().lookup(cx).unwrap_or_default();
        let is_decorative = alt_text.is_empty();
        let source = match self.source() {
            bs::image::Source::Logo(logo) => {
                return Ok(html! {
                    span
                        (self.props())
                        style=[dimensions]
                        role=[(!is_decorative).then_some("img")]
                        aria-label=[(!is_decorative).then_some(alt_text)]
                        aria-hidden=[is_decorative.then_some("true")]
                    {
                        (logo.markup(cx))
                    }
                });
            }
            bs::image::Source::Responsive(source) => Some(source),
            bs::image::Source::Thumbnail(source) => Some(source),
            bs::image::Source::Plain(source) => Some(source),
        };
        Ok(html! {
            img
                src=[source]
                alt=(alt_text)
                (self.props())
                style=[dimensions] {}
        })
    }
}

impl Image {
    /// Crea rápidamente una imagen especificando su origen.
    pub fn with(source: bs::image::Source) -> Self {
        Self::default().with_source(source)
    }

    // **< Image BUILDER >**************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    ///
    /// También acepta clases predefinidas para:
    ///
    /// - Establecer bordes ([`Border`]).
    /// - Redondear las esquinas ([`Rounded`]).
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Define las dimensiones de la imagen (auto, ancho/alto, ambos).
    #[builder_fn]
    pub fn with_size(mut self, size: bs::image::Size) -> Self {
        self.size = size;
        self
    }

    /// Establece el origen de la imagen, influyendo en su disposición en el contenido.
    #[builder_fn]
    pub fn with_source(mut self, source: bs::image::Source) -> Self {
        self.source = source;
        self
    }

    /// Define un *texto localizado* ([`Lc`]) alternativo para la imagen.
    ///
    /// Se recomienda siempre aportar un texto alternativo salvo que la imagen sea puramente
    /// decorativa.
    #[builder_fn]
    pub fn with_alternative(mut self, alt: Lc) -> Self {
        self.alternative.alter_value(alt);
        self
    }
}
