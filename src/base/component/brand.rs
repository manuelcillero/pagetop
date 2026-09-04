use crate::prelude::*;

/// Componente para mostrar la **identidad de marca** de un sitio o aplicación.
///
/// Combina una imagen y un título, típicamente en la cabecera de la página o dentro de una barra de
/// navegación.
///
/// - Si hay ruta ([`with_route()`]), el bloque completo actúa como enlace. Por defecto enlaza a la
///   raíz del sitio (`/`).
/// - Si no tiene ningún contenido (ni imagen ni título), la marca de identidad no se renderiza.
/// - El título predefinido es el nombre de la aplicación ([`global::SETTINGS.app.name`]). La imagen
///   es opcional, por defecto no tiene contenido.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let brand = Brand::new()
///     .with_image(image::Source::logo(PageTopSvg::Color))
///     .with_title(Lc::n("PageTop"))
///     .with_route(Route::from("/"));
/// ```
///
/// [`with_route()`]: Self::with_route
/// [`global::SETTINGS.app.name`]: crate::global::App::name
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Brand {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la imagen de marca (si la hay).
    image: Embed<Image>,
    /// Devuelve el título de la identidad de marca.
    #[default(_code = "Lc::n(&global::SETTINGS.app.name)")]
    title: Lc,
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
        let inner_brand = html! {
            (image)
            @if !title.is_empty() {
                @if !image.is_empty() {
                    (" ")
                }
                span class="brand-title" { (title) }
            }
        };
        if inner_brand.is_empty() {
            return Ok(html! {});
        }
        Ok(html! {
            @if let Some(route) = self.route() {
                a (self.props().unpack(cx)) href=(route.resolve(cx)) { (inner_brand) }
            } @else {
                span (self.props().unpack(cx)) { (inner_brand) }
            }
        })
    }
}

#[builder_impl]
impl Brand {
    // **< Brand BUILDER >**************************************************************************

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

    /// Asigna o quita la imagen de marca. Si se pasa `None`, no se mostrará.
    pub fn with_image(mut self, image: impl Into<Option<Image>>) -> Self {
        self.image.alter_component(image);
        self
    }

    /// Establece el título de la identidad de marca.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Define la ruta de destino. Si es `None`, la marca no será un enlace.
    pub fn with_route(mut self, route: impl Into<Option<Route>>) -> Self {
        self.route = route.into();
        self
    }
}
