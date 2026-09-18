use crate::prelude::*;

/// Componente para crear una **barra de navegación**.
///
/// Permite mostrar enlaces, menús desplegables ([`nav::Item::dropdown()`]) y una marca de
/// identidad, en distintas disposiciones controladas por [`navbar::Layout`]. El punto de corte a
/// partir del cual deja de colapsar se define con [`with_expand()`](Self::with_expand).
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplos
///
/// Barra **simple**, sólo con un menú horizontal:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let navbar = Navbar::simple()
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(nav::Item::link(Lc::n("About"), "/about"))
///             .with_item(nav::Item::link(Lc::n("Contact"), "/contact")),
///     ));
/// ```
///
/// Barra **colapsable**, con botón de despliegue, que muestra su contenido en línea a partir de un
/// punto de corte (por defecto, [`Breakpoint::Md`]):
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let navbar = Navbar::simple_toggle()
///     .with_expand(Breakpoint::Lg)
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(nav::Item::link_blank(Lc::n("Doc"), "https://docs.rs"))
///             .with_item(nav::Item::link(Lc::n("Support"), "/support")),
///     ));
/// ```
///
/// Barra con **marca de identidad** y menú, con menús desplegables:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let brand = Brand::new()
///     .with_title(Lc::n("PageTop"))
///     .with_route(Route::from("/"));
///
/// let navbar = Navbar::brand_left(brand)
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(nav::Item::dropdown(
///                 Dropdown::new()
///                     .with_title(Lc::n("Tools"))
///                     .with_item(dropdown::Item::link(Lc::n("Generator"), "/tools/gen"))
///                     .with_item(dropdown::Item::link(Lc::n("Reports"), "/tools/reports")),
///             ))
///             .with_item(nav::Item::link_disabled(Lc::n("Disabled"), "#")),
///     ));
/// ```
///
/// Barra con **botón de despliegue**, contenido y **marca de identidad**:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let brand = Brand::new()
///     .with_title(Lc::n("Intranet"))
///     .with_route(Route::from("/"));
///
/// let navbar = Navbar::brand_right(brand).with_item(navbar::Item::nav(
///     Nav::new()
///         .with_item(nav::Item::link(Lc::n("Dashboard"), "/dashboard"))
///         .with_item(nav::Item::link(Lc::n("Users"), "/users")),
/// ));
/// ```
///
/// [`nav::Item::dropdown()`]: super::super::nav::Item::dropdown
/// [`navbar::Layout`]: super::Layout
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Navbar {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la disposición configurada para la barra de navegación.
    layout: navbar::Layout,
    /// Devuelve el punto de corte a partir del cual la barra deja de colapsar.
    #[default(Breakpoint::Md)]
    #[getters(copy)]
    expand: Breakpoint,
    /// Devuelve la lista de contenidos.
    items: Children,
}

#[async_trait]
impl Component for Navbar {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &mut Context) {
        // Asegura que la barra de navegación tiene un identificador único: siempre se usa para
        // derivar el `id` del área de contenido (`id_content`, ver `prepare()`); además, el botón
        // de despliegue lo necesita para referenciar ese contenido colapsable con `aria-controls`
        // si el *layout* lo incluye.
        self.alter_prop(PropsOp::ensure_id(cx.build_id::<Self>(1)));

        let class: CowStr = match self.expand().resolved(cx) {
            Some(entry) => util::join!("navbar-expand-", entry.name).into(),
            None => "navbar-expand".into(),
        };
        self.alter_prop(PropsOp::prepend_classes(class));
        self.alter_prop(PropsOp::prepend_classes("navbar"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Si no hay contenidos, no tiene sentido mostrar una barra vacía.
        let items = self.items().render(cx).await;
        if items.is_empty() {
            return Ok(html! {});
        }

        // `setup()` garantiza que habrá un `id` antes de renderizar.
        let id = self.id().unwrap();
        let id_content = util::join!(id, "-content");

        // Botón de despliegue para el contenido colapsable de la barra.
        let button = html! {
            button
                type="button"
                class="navbar-toggle"
                aria-expanded="false"
                aria-controls=(&id_content)
                aria-label=[Lc::l("navbar_toggle").lookup(cx)]
            {}
        };

        let content_props = Props::classes("navbar-content").with_id(id_content);

        Ok(html! {
            nav (self.props().unpack(cx)) {
                @match self.layout() {
                    // Barra más sencilla: sólo contenido, siempre visible.
                    navbar::Layout::Simple => {
                        div (content_props.unpack(cx)) { (items) }
                    },

                    // Barra sencilla que se puede contraer/expandir.
                    navbar::Layout::SimpleToggle => {
                        (button)
                        div (content_props.unpack(cx)) { (items) }
                    },

                    // Barra con marca, siempre visible, sin botón.
                    navbar::Layout::SimpleBrandLeft(brand) => {
                        (brand.render(cx).await)
                        div (content_props.unpack(cx)) { (items) }
                    },

                    // Barra con marca y botón, en ese orden.
                    navbar::Layout::BrandLeft(brand) => {
                        (brand.render(cx).await)
                        (button)
                        div (content_props.unpack(cx)) { (items) }
                    },

                    // Barra con botón, contenido y marca (en ese orden).
                    navbar::Layout::BrandRight(brand) => {
                        (button)
                        div (content_props.unpack(cx)) { (items) }
                        (brand.render(cx).await)
                    },
                }
            }
        })
    }
}

#[builder_impl]
impl Navbar {
    /// Crea una barra de navegación **simple**, sin marca y sin botón.
    pub fn simple() -> Self {
        Self::default().with_layout(navbar::Layout::Simple)
    }

    /// Crea una barra de navegación **simple pero colapsable**, con botón de despliegue.
    pub fn simple_toggle() -> Self {
        Self::default().with_layout(navbar::Layout::SimpleToggle)
    }

    /// Crea una barra de navegación **con marca de identidad**, siempre visible, sin botón.
    pub fn simple_brand_left(brand: Brand) -> Self {
        Self::default().with_layout(navbar::Layout::SimpleBrandLeft(Embed::with(brand)))
    }

    /// Crea una barra de navegación con **marca de identidad** y **botón de despliegue**, en ese
    /// orden.
    pub fn brand_left(brand: Brand) -> Self {
        Self::default().with_layout(navbar::Layout::BrandLeft(Embed::with(brand)))
    }

    /// Crea una barra de navegación con **botón de despliegue**, el contenido y, en último lugar,
    /// la **marca de identidad**. A diferencia de [`brand_left()`](Self::brand_left), aquí no queda
    /// junto al botón, sino que se renderiza tras el contenido, para acabar en el extremo derecho
    /// de la barra (el botón sólo es visible en viewports estrechos).
    pub fn brand_right(brand: Brand) -> Self {
        Self::default().with_layout(navbar::Layout::BrandRight(Embed::with(brand)))
    }

    // **< Navbar BUILDER >*************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Define el tipo de disposición que tendrá la barra de navegación.
    pub fn with_layout(mut self, layout: navbar::Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Define a partir de qué punto de corte ([`Breakpoint`]) la barra de navegación deja de
    /// colapsar y muestra su contenido en línea.
    ///
    /// Por debajo de ese punto de corte, en las disposiciones con botón de despliegue el contenido
    /// queda oculto tras el botón. Con [`Breakpoint::Xs`] la barra nunca colapsa. Por defecto es
    /// [`Breakpoint::Md`].
    pub fn with_expand(mut self, bp: Breakpoint) -> Self {
        self.expand = bp;
        self
    }

    /// Añade un nuevo contenido a la barra de navegación o modifica la lista de contenidos de la
    /// barra con una operación [`TypedOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// navbar.with_item(navbar::Item::nav(...));
    /// navbar.with_item(TypedOp::AddMany(vec![
    ///     navbar::Item::nav(...),
    ///     navbar::Item::text(...),
    /// ]));
    /// ```
    pub fn with_item(mut self, op: impl Into<TypedOp<navbar::Item>>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
