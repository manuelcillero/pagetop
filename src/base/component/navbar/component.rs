use crate::prelude::*;

/// Componente para crear una **barra de navegación**.
///
/// Permite mostrar enlaces, menús desplegables ([`nav::Item::dropdown()`]) y una marca de
/// identidad, en distintas disposiciones controladas por [`navbar::Layout`].
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
/// Barra **colapsable**, con botón de despliegue:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let navbar = Navbar::simple_toggle()
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
/// Barra con **botón de despliegue** y **marca de identidad**, en ese orden:
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

    fn setup(&mut self, cx: &Context) {
        // Asegura que la barra de navegación tiene un identificador único: lo necesita el botón de
        // despliegue para referenciar el contenido colapsable con `aria-controls`.
        self.alter_prop(PropsOp::ensure_id(cx.build_id::<Self>(1)));
        self.alter_prop(PropsOp::prepend_classes("navbar"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Botón de despliegue para el contenido colapsable de la barra.
        fn button(cx: &mut Context, id_content: &str) -> Markup {
            html! {
                button
                    type="button"
                    class="navbar-toggle"
                    aria-expanded="false"
                    aria-controls=(id_content)
                    aria-label=[Lc::l("navbar_toggle").lookup(cx)]
                {
                    span class="navbar-toggle-icon" {}
                }
            }
        }

        // Si no hay contenidos, no tiene sentido mostrar una barra vacía.
        let items = self.items().render(cx).await;
        if items.is_empty() {
            return Ok(html! {});
        }

        // `setup()` garantiza que habrá un `id` antes de renderizar.
        let id = self.id().unwrap();
        let id_content = util::join!(id, "-content");

        Ok(html! {
            nav (self.props()) {
                @match self.layout() {
                    // Barra más sencilla: sólo contenido, siempre visible.
                    navbar::Layout::Simple => {
                        div class="navbar-content" { (items) }
                    },

                    // Barra sencilla que se puede contraer/expandir.
                    navbar::Layout::SimpleToggle => {
                        (button(cx, &id_content))
                        div id=(&id_content) class="navbar-content" { (items) }
                    },

                    // Barra con marca, siempre visible, sin botón.
                    navbar::Layout::SimpleBrandLeft(brand) => {
                        (brand.render(cx).await)
                        div class="navbar-content" { (items) }
                    },

                    // Barra con marca y botón, en ese orden.
                    navbar::Layout::BrandLeft(brand) => {
                        (brand.render(cx).await)
                        (button(cx, &id_content))
                        div id=(&id_content) class="navbar-content" { (items) }
                    },

                    // Barra con botón y marca, en ese orden.
                    navbar::Layout::BrandRight(brand) => {
                        (button(cx, &id_content))
                        div id=(&id_content) class="navbar-content" { (items) }
                        (brand.render(cx).await)
                    },
                }
            }
        })
    }
}

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

    /// Crea una barra de navegación con **botón de despliegue** y **marca de identidad**, en ese
    /// orden.
    pub fn brand_right(brand: Brand) -> Self {
        Self::default().with_layout(navbar::Layout::BrandRight(Embed::with(brand)))
    }

    // **< Navbar BUILDER >*************************************************************************

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

    /// Define el tipo de disposición que tendrá la barra de navegación.
    #[builder_fn]
    pub fn with_layout(mut self, layout: navbar::Layout) -> Self {
        self.layout = layout;
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
    #[builder_fn]
    pub fn with_item(mut self, op: impl Into<TypedOp<navbar::Item>>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
