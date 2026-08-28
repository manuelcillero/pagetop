use crate::prelude::*;

/// Elementos que puede contener una barra de navegación [`Navbar`](super::Navbar).
///
/// Cada variante determina qué se renderiza y cómo. Estos elementos se colocan **dentro del
/// contenido** de la barra (la parte colapsable), por lo que son independientes de la marca o del
/// botón que ya pueda definir el propio [`navbar::Layout`](super::Layout).
#[derive(AutoDefault, Clone, Debug)]
pub enum Item {
    /// Sin contenido, no produce salida.
    #[default]
    Void,
    /// Marca de identidad mostrada dentro del contenido de la barra de navegación.
    ///
    /// Útil cuando el [`navbar::Layout`](super::Layout) no incluye marca, y se quiere incluir
    /// dentro del área colapsable. Si el *layout* ya muestra una marca, esta variante no la
    /// sustituye, sólo añade otra dentro del bloque de contenidos.
    Brand(Embed<Brand>),
    /// Representa un menú de navegación [`Nav`].
    Nav(Embed<Nav>),
    /// Representa un *texto localizado* libre.
    Text(Lc),
}

#[async_trait]
impl Component for Item {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        match self {
            Self::Void => None,
            Self::Brand(brand) => brand.id(),
            Self::Nav(nav) => nav.id(),
            Self::Text(_) => None,
        }
    }

    fn setup(&mut self, _cx: &Context) {
        if let Self::Nav(nav) = self
            && let Some(nav) = nav.get_mut()
        {
            // Se añade aquí, antes de que `nav.render(cx)` (en `prepare()`) dispare el ciclo de
            // vida normal del `Nav` embebido y su propio `setup()` prepend las clases de `nav`/
            // `nav::Layout`.
            nav.alter_prop(PropsOp::prepend_classes("navbar-nav"));
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(match self {
            Self::Void => html! {},
            Self::Brand(brand) => html! { (brand.render(cx).await) },
            Self::Nav(nav) => html! { (nav.render(cx).await) },
            Self::Text(text) => html! {
                span class="navbar-text" {
                    (text.using(cx))
                }
            },
        })
    }
}

impl Item {
    /// Crea un elemento de tipo [`navbar::Brand`](super::super::Brand) para añadir en el contenido
    /// de [`Navbar`](super::Navbar).
    ///
    /// Pensado para barras colapsables donde se quiere que la marca aparezca en la zona
    /// desplegable.
    pub fn brand(brand: Brand) -> Self {
        Self::Brand(Embed::with(brand))
    }

    /// Crea un elemento de tipo [`Nav`] para añadir al contenido de [`Navbar`](super::Navbar).
    pub fn nav(item: Nav) -> Self {
        Self::Nav(Embed::with(item))
    }

    /// Crea un elemento con un *texto localizado*, mostrado sin interacción.
    pub fn text(item: Lc) -> Self {
        Self::Text(item)
    }
}
