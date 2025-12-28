use pagetop::prelude::*;

use crate::prelude::*;

/// Marca de identidad para mostrar en una barra de navegación [`Navbar`].
///
/// Representa la identidad del sitio con una imagen, título y eslogan:
///
/// - Si hay URL ([`with_route()`](Self::with_route)), el bloque completo actúa como enlace. Por
///   defecto enlaza a la raíz del sitio (`/`).
/// - Si no hay imagen ([`with_image()`](Self::with_image)) ni título
///   ([`with_title()`](Self::with_title)), la marca de identidad no se renderiza.
/// - El eslogan ([`with_slogan()`](Self::with_slogan)) es opcional; por defecto no tiene contenido.
#[derive(AutoDefault, Getters)]
pub struct Brand {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve la imagen de marca (si la hay).
    image: Typed<Image>,
    /// Devuelve el título de la identidad de marca.
    #[default(_code = "L10n::n(&global::SETTINGS.app.name)")]
    title: L10n,
    /// Devuelve el eslogan de la marca.
    slogan: L10n,
    /// Devuelve la función que resuelve la URL asociada a la marca (si existe).
    #[default(_code = "Some(|cx| cx.route(\"/\"))")]
    route: Option<FnPathByContext>,
}

impl Component for Brand {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let image = self.image().render(cx);
        let title = self.title().using(cx);
        if title.is_empty() && image.is_empty() {
            return PrepareMarkup::None;
        }
        let slogan = self.slogan().using(cx);
        PrepareMarkup::With(html! {
            @if let Some(route) = self.route() {
                a class="navbar-brand" href=(route(cx)) { (image) (title) (slogan) }
            } @else {
                span class="navbar-brand" { (image) (title) (slogan) }
            }
        })
    }
}

impl Brand {
    // **< Brand BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) de la marca.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Asigna o quita la imagen de marca. Si se pasa `None`, no se mostrará.
    #[builder_fn]
    pub fn with_image(mut self, image: Option<Image>) -> Self {
        self.image.alter_component(image);
        self
    }

    /// Establece el título de la identidad de marca.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Define el eslogan de la marca.
    #[builder_fn]
    pub fn with_slogan(mut self, slogan: L10n) -> Self {
        self.slogan = slogan;
        self
    }

    /// Define la URL de destino. Si es `None`, la marca no será un enlace.
    #[builder_fn]
    pub fn with_route(mut self, route: Option<FnPathByContext>) -> Self {
        self.route = route;
        self
    }
}
