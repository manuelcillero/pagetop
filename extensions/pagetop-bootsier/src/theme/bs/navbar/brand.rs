use pagetop::prelude::*;

use crate::theme::*;

/// Marca de identidad para mostrar en una barra de navegación [`Navbar`](crate::theme::bs::Navbar).
///
/// Representa la identidad del sitio con una imagen, título y eslogan:
///
/// - Si hay URL ([`with_route()`](Self::with_route)), el bloque completo actúa como enlace. Por
///   defecto enlaza a la raíz del sitio (`/`).
/// - Si no hay imagen ([`with_image()`](Self::with_image)) ni título
///   ([`with_title()`](Self::with_title)), la marca de identidad no se renderiza.
/// - El eslogan ([`with_slogan()`](Self::with_slogan)) es opcional; por defecto no tiene contenido.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Brand {
    /// Devuelve la imagen de marca (si la hay).
    image: Embed<bs::Image>,
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

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let image = self.image().render(cx).await;
        let title = self.title().using(cx);
        if title.is_empty() && image.is_empty() {
            return Ok(html! {});
        }
        let slogan = self.slogan().using(cx);
        Ok(html! {
            @if let Some(route) = self.route() {
                a class="navbar-brand" href=(route.resolve(cx)) { (image) (title) (slogan) }
            } @else {
                span class="navbar-brand" { (image) (title) (slogan) }
            }
        })
    }
}

impl Brand {
    // **< Brand BUILDER >**************************************************************************

    /// Asigna o quita la imagen de marca. Si se pasa `None`, no se mostrará.
    #[builder_fn]
    pub fn with_image(mut self, image: Option<bs::Image>) -> Self {
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
    pub fn with_route(mut self, route: Option<Route>) -> Self {
        self.route = route;
        self
    }
}
