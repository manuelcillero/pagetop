use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::*;

const TOGGLE_COLLAPSE: &str = "collapse";
const TOGGLE_OFFCANVAS: &str = "offcanvas";

/// Componente para crear una **barra de navegación**.
///
/// Permite mostrar enlaces, menús y una marca de identidad en distintas disposiciones (simples, con
/// botón de despliegue o dentro de un [`Offcanvas`](crate::theme::bs::Offcanvas)), controladas por
/// [`navbar::Layout`](crate::theme::bs::navbar::Layout). También puede fijarse en la parte superior
/// o inferior del documento mediante [`navbar::Position`](crate::theme::bs::navbar::Position).
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplos
///
/// Barra **simple**, sólo con un menú horizontal:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let navbar = bs::Navbar::simple()
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new()
///             .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(bs::nav::Item::link(Lc::n("About"), "/about"))
///             .with_item(bs::nav::Item::link(Lc::n("Contact"), "/contact"))
///     ));
/// ```
///
/// Barra **colapsable**, con botón de despliegue y contenido en el desplegable cuando colapsa:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let navbar = bs::Navbar::simple_toggle()
///     .with_expand(BreakPoint::MD)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new()
///             .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(bs::nav::Item::link_blank(Lc::n("Doc"), "https://docs.rs"))
///             .with_item(bs::nav::Item::link(Lc::n("Support"), "/support"))
///     ));
/// ```
///
/// Barra con **marca de identidad a la izquierda** y menú a la derecha, típica de una cabecera:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let brand = bs::navbar::Brand::new()
///     .with_title(Lc::n("PageTop"))
///     .with_route(Some("/".into()));
///
/// let navbar = bs::Navbar::brand_left(brand)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new()
///             .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(bs::nav::Item::dropdown(
///                 bs::Dropdown::new()
///                     .with_title(Lc::n("Tools"))
///                     .with_item(bs::dropdown::Item::link(
///                         Lc::n("Generator"), "/tools/gen")
///                     )
///                     .with_item(bs::dropdown::Item::link(
///                         Lc::n("Reports"), "/tools/reports")
///                     )
///             ))
///             .with_item(bs::nav::Item::link_disabled(Lc::n("Disabled"), "#"))
///     ));
/// ```
///
/// Barra con **botón de despliegue a la izquierda** y **marca de identidad a la derecha**:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let brand = bs::navbar::Brand::new()
///     .with_title(Lc::n("Intranet"))
///     .with_route(Some("/".into()));
///
/// let navbar = bs::Navbar::brand_right(brand)
///     .with_expand(BreakPoint::LG)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::pills()
///             .with_item(bs::nav::Item::link(Lc::n("Dashboard"), "/dashboard"))
///             .with_item(bs::nav::Item::link(Lc::n("Users"), "/users"))
///     ));
/// ```
///
/// Barra con el **contenido en un *offcanvas***, ideal para dispositivos móviles o menús largos:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let oc = bs::Offcanvas::new()
///     .with_id("main_offcanvas")
///     .with_title(Lc::n("Main menu"))
///     .with_placement(bs::offcanvas::Placement::Start)
///     .with_backdrop(bs::offcanvas::Backdrop::Enabled);
///
/// let navbar = bs::Navbar::offcanvas(oc)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new()
///             .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
///             .with_item(bs::nav::Item::link(Lc::n("Profile"), "/profile"))
///             .with_item(bs::nav::Item::dropdown(
///                 bs::Dropdown::new()
///                     .with_title(Lc::n("More"))
///                     .with_item(bs::dropdown::Item::link(Lc::n("Settings"), "/settings"))
///                     .with_item(bs::dropdown::Item::link(Lc::n("Help"), "/help"))
///             ))
///     ));
/// ```
///
/// Barra **fija arriba**:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let brand = bs::navbar::Brand::new()
///     .with_title(Lc::n("Main App"))
///     .with_route(Some("/".into()));
///
/// let navbar = bs::Navbar::brand_left(brand)
///     .with_position(bs::navbar::Position::FixedTop)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new()
///             .with_item(bs::nav::Item::link(Lc::n("Dashboard"), "/"))
///             .with_item(bs::nav::Item::link(Lc::n("Donors"), "/donors"))
///             .with_item(bs::nav::Item::link(Lc::n("Stock"), "/stock"))
///     ));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Navbar {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el punto de ruptura configurado.
    expand: BreakPoint,
    /// Devuelve la disposición configurada para la barra de navegación.
    layout: bs::navbar::Layout,
    /// Devuelve la posición configurada para la barra de navegación.
    position: bs::navbar::Position,
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
        // Asegura que la barra de navegación tiene un identificador único.
        self.alter_prop(PropsOp::ensure_id(cx.build_id::<Self>(1)));

        // Clases CSS por defecto para la barra de navegación.
        self.alter_prop(PropsOp::prepend_classes({
            let mut classes = "navbar".to_string();
            self.expand().push_to(&mut classes, "navbar-expand", "");
            self.position().push_to(&mut classes);
            classes
        }));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Botón de despliegue (colapso u offcanvas) para la barra.
        fn button(cx: &mut Context, data_bs_toggle: &str, id_content: &str) -> Markup {
            let id_content_target = util::join!("#", id_content);
            let aria_expanded = if data_bs_toggle == TOGGLE_COLLAPSE {
                Some("false")
            } else {
                None
            };
            html! {
                button
                    type="button"
                    class="navbar-toggler"
                    data-bs-toggle=(data_bs_toggle)
                    data-bs-target=(id_content_target)
                    aria-controls=(id_content)
                    aria-expanded=[aria_expanded]
                    aria-label=[Lc::t("toggle", &LOCALES_BOOTSIER).lookup(cx)]
                {
                    span class="navbar-toggler-icon" {}
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

        Ok(html! {
            nav (self.props()) {
                div class="container-fluid" {
                    @match self.layout() {
                        // Barra más sencilla: sólo contenido.
                        bs::navbar::Layout::Simple => {
                            (items)
                        },

                        // Barra sencilla que se puede contraer/expandir.
                        bs::navbar::Layout::SimpleToggle => {
                            @let id_content = util::join!(id, "-content");

                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            div id=(&id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra con marca a la izquierda, siempre visible.
                        bs::navbar::Layout::SimpleBrandLeft(brand) => {
                            (brand.render(cx).await)
                            (items)
                        },

                        // Barra con marca a la izquierda y botón a la derecha.
                        bs::navbar::Layout::BrandLeft(brand) => {
                            @let id_content = util::join!(id, "-content");

                            (brand.render(cx).await)
                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            div id=(&id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra con botón a la izquierda y marca a la derecha.
                        bs::navbar::Layout::BrandRight(brand) => {
                            @let id_content = util::join!(id, "-content");

                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            (brand.render(cx).await)
                            div id=(&id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra cuyo contenido se muestra en un offcanvas, sin marca.
                        bs::navbar::Layout::Offcanvas(offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            @if let Some(oc) = offcanvas.get() {
                                (oc.render_offcanvas(cx, Some(self.items())).await)
                            }
                        },

                        // Barra con marca a la izquierda y contenido en offcanvas.
                        bs::navbar::Layout::OffcanvasBrandLeft(brand, offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (brand.render(cx).await)
                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            @if let Some(oc) = offcanvas.get() {
                                (oc.render_offcanvas(cx, Some(self.items())).await)
                            }
                        },

                        // Barra con contenido en offcanvas y marca a la derecha.
                        bs::navbar::Layout::OffcanvasBrandRight(brand, offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            (brand.render(cx).await)
                            @if let Some(oc) = offcanvas.get() {
                                (oc.render_offcanvas(cx, Some(self.items())).await)
                            }
                        },
                    }
                }
            }
        })
    }
}

impl Navbar {
    /// Crea una barra de navegación **simple**, sin marca y sin botón.
    pub fn simple() -> Self {
        Self::default().with_layout(bs::navbar::Layout::Simple)
    }

    /// Crea una barra de navegación **simple pero colapsable**, con botón a la izquierda.
    pub fn simple_toggle() -> Self {
        Self::default().with_layout(bs::navbar::Layout::SimpleToggle)
    }

    /// Crea una barra de navegación **con marca a la izquierda**, siempre visible.
    pub fn simple_brand_left(brand: bs::navbar::Brand) -> Self {
        Self::default().with_layout(bs::navbar::Layout::SimpleBrandLeft(Embed::with(brand)))
    }

    /// Crea una barra de navegación con **marca a la izquierda** y **botón a la derecha**.
    pub fn brand_left(brand: bs::navbar::Brand) -> Self {
        Self::default().with_layout(bs::navbar::Layout::BrandLeft(Embed::with(brand)))
    }

    /// Crea una barra de navegación con **botón a la izquierda** y **marca a la derecha**.
    pub fn brand_right(brand: bs::navbar::Brand) -> Self {
        Self::default().with_layout(bs::navbar::Layout::BrandRight(Embed::with(brand)))
    }

    /// Crea una barra de navegación cuyo contenido se muestra en un **offcanvas**.
    pub fn offcanvas(oc: bs::Offcanvas) -> Self {
        Self::default().with_layout(bs::navbar::Layout::Offcanvas(Embed::with(oc)))
    }

    /// Crea una barra de navegación con **marca a la izquierda** y contenido en **offcanvas**.
    pub fn offcanvas_brand_left(brand: bs::navbar::Brand, oc: bs::Offcanvas) -> Self {
        Self::default().with_layout(bs::navbar::Layout::OffcanvasBrandLeft(
            Embed::with(brand),
            Embed::with(oc),
        ))
    }

    /// Crea una barra de navegación con **marca a la derecha** y contenido en **offcanvas**.
    pub fn offcanvas_brand_right(brand: bs::navbar::Brand, oc: bs::Offcanvas) -> Self {
        Self::default().with_layout(bs::navbar::Layout::OffcanvasBrandRight(
            Embed::with(brand),
            Embed::with(oc),
        ))
    }

    // **< Navbar BUILDER >*************************************************************************

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
    /// - Modificar el color de fondo ([`Bg`]).
    /// - Definir la apariencia del texto ([`Text`]).
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Define a partir de qué punto de ruptura la barra de navegación deja de colapsar.
    #[builder_fn]
    pub fn with_expand(mut self, bp: BreakPoint) -> Self {
        self.expand = bp;
        self
    }

    /// Define el tipo de disposición que tendrá la barra de navegación.
    #[builder_fn]
    pub fn with_layout(mut self, layout: bs::navbar::Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Define dónde se mostrará la barra de navegación dentro del documento.
    #[builder_fn]
    pub fn with_position(mut self, position: bs::navbar::Position) -> Self {
        self.position = position;
        self
    }

    /// Añade un nuevo contenido a la barra de navegación o modifica la lista de contenidos de la
    /// barra con una operación [`ChildOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// navbar.with_item(navbar::Item::nav(...));
    /// navbar.with_item(ChildOp::AddMany(vec![
    ///     navbar::Item::nav(...).into(),
    ///     navbar::Item::text(...).into(),
    /// ]));
    /// ```
    #[builder_fn]
    pub fn with_item(mut self, op: impl Into<ChildOp>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
