use crate::prelude::*;

/// Componente para mostrar la **identidad de marca** de un sitio o aplicación.
///
/// Combina una imagen, un título y un eslogan opcional, típicamente en la cabecera de la página o
/// dentro de una barra de navegación proporcionada por un tema.
///
/// - Si hay ruta ([`with_route()`]), el bloque completo actúa como enlace. Por defecto enlaza a la
///   raíz del sitio (`/`).
/// - Si no hay imagen ([`with_image()`]) ni título ([`with_title()`]), la marca de identidad no se
///   renderiza.
/// - El eslogan ([`with_slogan()`]) es opcional; por defecto no tiene contenido.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let brand = Brand::new()
///     .with_image(Some(Image::with(image::Source::logo(PageTopSvg::Color))))
///     .with_title(Lc::n("PageTop"))
///     .with_route(Route::from("/"));
/// ```
///
/// [`with_route()`]: Self::with_route
/// [`with_image()`]: Self::with_image
/// [`with_title()`]: Self::with_title
/// [`with_slogan()`]: Self::with_slogan
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Brand {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la imagen de marca (si la hay).
    image: Embed<Image>,
    /// Devuelve el título de la identidad de marca.
    #[default(_code = "Lc::n(&global::SETTINGS.app.name)")]
    title: Lc,
    /// Devuelve el eslogan de la marca.
    slogan: Lc,
    /// Devuelve la ruta asociada a la marca (si existe).
    #[default(_code = "Some(\"/\".into())")]
    route: Option<Route>,
}

#[async_trait]
impl Component for Brand {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("brand"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let image = self.image().render(cx).await;
        let title = self.title().using(cx);
        if image.is_empty() && title.is_empty() {
            return Ok(html! {});
        }
        let slogan = self.slogan().using(cx);
        Ok(html! {
            @if let Some(route) = self.route() {
                a (self.props()) href=(route.resolve(cx)) { (image) (title) (slogan) }
            } @else {
                span (self.props()) { (image) (title) (slogan) }
            }
        })
    }
}

impl Brand {
    // **< Brand BUILDER >**************************************************************************

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

    /// Asigna o quita la imagen de marca. Si se pasa `None`, no se mostrará.
    #[builder_fn]
    pub fn with_image(mut self, image: impl Into<Option<Image>>) -> Self {
        self.image.alter_component(image);
        self
    }

    /// Establece el título de la identidad de marca.
    #[builder_fn]
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Define el eslogan de la marca.
    #[builder_fn]
    pub fn with_slogan(mut self, slogan: Lc) -> Self {
        self.slogan = slogan;
        self
    }

    /// Define la ruta de destino. Si es `None`, la marca no será un enlace.
    #[builder_fn]
    pub fn with_route(mut self, route: impl Into<Option<Route>>) -> Self {
        self.route = route.into();
        self
    }
}
