use crate::prelude::*;

/// Componente para renderizar una **imagen**.
///
/// A una imagen se le puede:
///
/// - Establecer su contenido a partir del origen definido en [`image::Source`].
/// - Configurar sus **dimensiones** ([`with_size()`](Self::with_size)).
/// - Aplicar el texto alternativo `alt` con **localización** mediante [`Lc`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let logo = Image::with(image::Source::logo(PageTopSvg::Color))
///     .with_alternative(Lc::n("PageTop"));
///
/// let photo = Image::with(image::Source::responsive("/files/photo.jpg"))
///     .with_size(image::Size::Width(UnitValue::Px(320)))
///     .with_alternative(Lc::n("Team photo"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Image {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve las dimensiones de la imagen.
    size: image::Size,
    /// Devuelve el origen de la imagen.
    source: image::Source,
    /// Devuelve el texto alternativo localizado.
    alternative: Lc,
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
        self.alter_prop(PropsOp::prepend_classes(match self.source() {
            image::Source::Logo(_) => "image image-fluid",
            image::Source::Responsive(_) => "image image-fluid",
            image::Source::Thumbnail(_) => "image image-thumbnail",
            image::Source::Plain(_) => "image",
        }));

        // Se asigna un tamaño predefinido para el logotipo que `with_size()` puede sobrescribir.
        if matches!(self.source(), image::Source::Logo(_)) {
            self.alter_prop(PropsOp::add_style("width", "1.25em"));
            self.alter_prop(PropsOp::add_style("height", "1.25em"));
        }

        // El tamaño se aplica como declaraciones `style` individuales sobre `Props`.
        match *self.size() {
            image::Size::Auto => {}
            image::Size::Dimensions(w, h) => {
                self.alter_prop(PropsOp::add_style("width", w.to_string()));
                self.alter_prop(PropsOp::add_style("height", h.to_string()));
            }
            image::Size::Width(w) => {
                self.alter_prop(PropsOp::add_style("width", w.to_string()));
            }
            image::Size::Height(h) => {
                self.alter_prop(PropsOp::add_style("height", h.to_string()));
            }
            image::Size::Both(v) => {
                self.alter_prop(PropsOp::add_style("width", v.to_string()));
                self.alter_prop(PropsOp::add_style("height", v.to_string()));
            }
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let alt_text = self.alternative().lookup(cx).unwrap_or_default();
        let source = match self.source() {
            image::Source::Logo(svg) => {
                let label = (!alt_text.is_empty()).then_some(alt_text.as_str());
                return Ok(svg.markup_with(self.props(), label));
            }
            image::Source::Responsive(source) => Some(source),
            image::Source::Thumbnail(source) => Some(source),
            image::Source::Plain(source) => Some(source),
        };
        Ok(html! {
            img
                src=[source]
                alt=(alt_text)
                (self.props()) {}
        })
    }
}

#[builder_impl]
impl Image {
    /// Crea rápidamente una imagen especificando su origen.
    pub fn with(source: image::Source) -> Self {
        Self::default().with_source(source)
    }

    // **< Image BUILDER >**************************************************************************

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

    /// Define las dimensiones de la imagen (auto, ancho/alto, ambos).
    pub fn with_size(mut self, size: image::Size) -> Self {
        self.size = size;
        self
    }

    /// Establece el origen de la imagen, influyendo en su disposición en el contenido.
    pub fn with_source(mut self, source: image::Source) -> Self {
        self.source = source;
        self
    }

    /// Define un *texto localizado* ([`Lc`]) alternativo para la imagen.
    ///
    /// Se recomienda siempre aportar un texto alternativo salvo que la imagen sea puramente
    /// decorativa.
    pub fn with_alternative(mut self, alt: Lc) -> Self {
        self.alternative = alt;
        self
    }
}

impl From<image::Source> for Image {
    /// Igual que [`Image::with()`].
    fn from(source: image::Source) -> Self {
        Self::with(source)
    }
}

impl From<image::Source> for Option<Image> {
    /// Permite pasar un [`image::Source`] directamente donde se espera `impl Into<Option<Image>>`
    /// (p. ej. [`Brand::with_image()`](super::super::Brand::with_image)), sin construir la
    /// [`Image`] a mano.
    fn from(source: image::Source) -> Self {
        Some(Image::with(source))
    }
}
