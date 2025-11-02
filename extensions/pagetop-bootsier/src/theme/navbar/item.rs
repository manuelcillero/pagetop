use pagetop::prelude::*;

use crate::prelude::*;

/// Elementos que puede contener una barra de navegación [`Navbar`](crate::theme::Navbar).
///
/// Cada variante determina qué se renderiza y cómo. Estos elementos se colocan **dentro del
/// contenido** de la barra (la parte colapsable, el *offcanvas* o el bloque simple), por lo que son
/// independientes de la marca o del botón que ya pueda definir el propio [`navbar::Layout`].
#[derive(AutoDefault)]
pub enum Item {
    /// Sin contenido, no produce salida.
    #[default]
    Void,
    /// Marca de identidad mostrada dentro del contenido de la barra de navegación.
    ///
    /// Útil cuando el [`navbar::Layout`] no incluye marca, y se quiere incluir dentro del área
    /// colapsable/*offcanvas*. Si el *layout* ya muestra una marca, esta variante no la sustituye,
    /// sólo añade otra dentro del bloque de contenidos.
    Brand(Typed<navbar::Brand>),
    /// Representa un menú de navegación [`Nav`](crate::theme::Nav).
    Nav(Typed<Nav>),
    /// Representa un texto libre localizado.
    Text(L10n),
}

impl Component for Item {
    fn new() -> Self {
        Item::default()
    }

    fn id(&self) -> Option<String> {
        match self {
            Self::Void => None,
            Self::Brand(brand) => brand.id(),
            Self::Nav(nav) => nav.id(),
            Self::Text(_) => None,
        }
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self {
            Self::Void => PrepareMarkup::None,
            Self::Brand(brand) => PrepareMarkup::With(html! { (brand.render(cx)) }),
            Self::Nav(nav) => {
                if let Some(nav) = nav.borrow() {
                    let items = nav.items().render(cx);
                    if items.is_empty() {
                        return PrepareMarkup::None;
                    }
                    let classes = AttrClasses::new(
                        [
                            "navbar-nav",
                            match nav.nav_kind() {
                                nav::Kind::Default => "",
                                nav::Kind::Tabs => "nav-tabs",
                                nav::Kind::Pills => "nav-pills",
                                nav::Kind::Underline => "nav-underline",
                            },
                            match nav.nav_layout() {
                                nav::Layout::Default => "",
                                nav::Layout::Start => "justify-content-start",
                                nav::Layout::Center => "justify-content-center",
                                nav::Layout::End => "justify-content-end",
                                nav::Layout::Vertical => "flex-column",
                                nav::Layout::Fill => "nav-fill",
                                nav::Layout::Justified => "nav-justified",
                            },
                        ]
                        .join(" "),
                    );
                    PrepareMarkup::With(html! {
                        ul id=[nav.id()] class=[classes.get()] {
                            (items)
                        }
                    })
                } else {
                    PrepareMarkup::None
                }
            }
            Self::Text(text) => PrepareMarkup::With(html! {
                span class="navbar-text" {
                    (text.using(cx))
                }
            }),
        }
    }
}

impl Item {
    /// Crea un elemento de tipo [`navbar::Brand`] para añadir en el contenido de [`Navbar`].
    ///
    /// Pensado para barras colapsables u offcanvas donde se quiere que la marca aparezca en la zona
    /// desplegable.
    pub fn brand(brand: navbar::Brand) -> Self {
        Self::Brand(Typed::with(brand))
    }

    /// Crea un elemento de tipo [`Nav`] para añadir al contenido de [`Navbar`].
    pub fn nav(item: Nav) -> Self {
        Self::Nav(Typed::with(item))
    }

    /// Crea un elemento de texto localizado, mostrado sin interacción.
    pub fn text(item: L10n) -> Self {
        Self::Text(item)
    }
}
