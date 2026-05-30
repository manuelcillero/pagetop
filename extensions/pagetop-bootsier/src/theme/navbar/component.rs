use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::*;

const TOGGLE_COLLAPSE: &str = "collapse";
const TOGGLE_OFFCANVAS: &str = "offcanvas";

/// Componente para crear una **barra de navegación** ([`navbar`]).
///
/// Permite mostrar enlaces, menús y una marca de identidad en distintas disposiciones (simples, con
/// botón de despliegue o dentro de un [`offcanvas`]), controladas por [`navbar::Layout`]. También
/// puede fijarse en la parte superior o inferior del documento mediante [`navbar::Position`].
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplos
///
/// Barra **simple**, sólo con un menú horizontal:
///
/// ```rust
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let navbar = Navbar::simple()
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
///             .with_item(nav::Item::link(L10n::n("About"), |_| "/about".into()))
///             .with_item(nav::Item::link(L10n::n("Contact"), |_| "/contact".into()))
///     ));
/// ```
///
/// Barra **colapsable**, con botón de despliegue y contenido en el desplegable cuando colapsa:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let navbar = Navbar::simple_toggle()
///     .with_expand(BreakPoint::MD)
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
///             .with_item(nav::Item::link_blank(L10n::n("Docs"), |_| "https://docs.rs".into()))
///             .with_item(nav::Item::link(L10n::n("Support"), |_| "/support".into()))
///     ));
/// ```
///
/// Barra con **marca de identidad a la izquierda** y menú a la derecha, típica de una cabecera:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let brand = navbar::Brand::new()
///     .with_title(L10n::n("PageTop"))
///     .with_route(Some(|cx| cx.route("/")));
///
/// let navbar = Navbar::brand_left(brand)
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
///             .with_item(nav::Item::dropdown(
///                 Dropdown::new()
///                     .with_title(L10n::n("Tools"))
///                     .with_item(dropdown::Item::link(
///                         L10n::n("Generator"), |_| "/tools/gen".into())
///                     )
///                     .with_item(dropdown::Item::link(
///                         L10n::n("Reports"), |_| "/tools/reports".into())
///                     )
///             ))
///             .with_item(nav::Item::link_disabled(L10n::n("Disabled"), |_| "#".into()))
///     ));
/// ```
///
/// Barra con **botón de despliegue a la izquierda** y **marca de identidad a la derecha**:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let brand = navbar::Brand::new()
///     .with_title(L10n::n("Intranet"))
///     .with_route(Some(|cx| cx.route("/")));
///
/// let navbar = Navbar::brand_right(brand)
///     .with_expand(BreakPoint::LG)
///     .with_item(navbar::Item::nav(
///         Nav::pills()
///             .with_item(nav::Item::link(L10n::n("Dashboard"), |_| "/dashboard".into()))
///             .with_item(nav::Item::link(L10n::n("Users"), |_| "/users".into()))
///     ));
/// ```
///
/// Barra con el **contenido en un *offcanvas***, ideal para dispositivos móviles o menús largos:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let oc = Offcanvas::new()
///     .with_id("main_offcanvas")
///     .with_title(L10n::n("Main menu"))
///     .with_placement(offcanvas::Placement::Start)
///     .with_backdrop(offcanvas::Backdrop::Enabled);
///
/// let navbar = Navbar::offcanvas(oc)
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
///             .with_item(nav::Item::link(L10n::n("Profile"), |_| "/profile".into()))
///             .with_item(nav::Item::dropdown(
///                 Dropdown::new()
///                     .with_title(L10n::n("More"))
///                     .with_item(dropdown::Item::link(L10n::n("Settings"), |_| "/settings".into()))
///                     .with_item(dropdown::Item::link(L10n::n("Help"), |_| "/help".into()))
///             ))
///     ));
/// ```
///
/// Barra **fija arriba**:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let brand = navbar::Brand::new()
///     .with_title(L10n::n("Main App"))
///     .with_route(Some(|cx| cx.route("/")));
///
/// let navbar = Navbar::brand_left(brand)
///     .with_position(navbar::Position::FixedTop)
///     .with_item(navbar::Item::nav(
///         Nav::new()
///             .with_item(nav::Item::link(L10n::n("Dashboard"), |_| "/".into()))
///             .with_item(nav::Item::link(L10n::n("Donors"), |_| "/donors".into()))
///             .with_item(nav::Item::link(L10n::n("Stock"), |_| "/stock".into()))
///     ));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Navbar {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS asociadas a la barra de navegación.
    classes: Classes,
    /// Devuelve el punto de ruptura configurado.
    expand: BreakPoint,
    /// Devuelve la disposición configurada para la barra de navegación.
    layout: navbar::Layout,
    /// Devuelve la posición configurada para la barra de navegación.
    position: navbar::Position,
    /// Devuelve la lista de contenidos.
    items: Children,
}

impl Component for Navbar {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_classes(ClassesOp::Prepend, {
            let mut classes = "navbar".to_string();
            self.expand().push_class(&mut classes, "navbar-expand", "");
            self.position().push_class(&mut classes);
            classes
        });
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
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
                    aria-label=[L10n::t("toggle", &LOCALES_BOOTSIER).lookup(cx)]
                {
                    span class="navbar-toggler-icon" {}
                }
            }
        }

        // Si no hay contenidos, no tiene sentido mostrar una barra vacía.
        let items = self.items().render(cx);
        if items.is_empty() {
            return Ok(html! {});
        }

        // Asegura que la barra tiene un `id` para poder asociarlo al colapso/offcanvas.
        let id = cx.required_id::<Self>(self.id(), 1);

        Ok(html! {
            nav id=(&id) class=[self.classes().get()] {
                div class="container-fluid" {
                    @match self.layout() {
                        // Barra más sencilla: sólo contenido.
                        navbar::Layout::Simple => {
                            (items)
                        },

                        // Barra sencilla que se puede contraer/expandir.
                        navbar::Layout::SimpleToggle => {
                            @let id_content = util::join!(id, "-content");

                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            div id=(&id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra con marca a la izquierda, siempre visible.
                        navbar::Layout::SimpleBrandLeft(brand) => {
                            (brand.render(cx))
                            (items)
                        },

                        // Barra con marca a la izquierda y botón a la derecha.
                        navbar::Layout::BrandLeft(brand) => {
                            @let id_content = util::join!(id, "-content");

                            (brand.render(cx))
                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            div id=(&id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra con botón a la izquierda y marca a la derecha.
                        navbar::Layout::BrandRight(brand) => {
                            @let id_content = util::join!(id, "-content");

                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            (brand.render(cx))
                            div id=(&id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra cuyo contenido se muestra en un offcanvas, sin marca.
                        navbar::Layout::Offcanvas(offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            @if let Some(oc) = offcanvas.get() {
                                (oc.render_offcanvas(cx, Some(self.items())))
                            }
                        },

                        // Barra con marca a la izquierda y contenido en offcanvas.
                        navbar::Layout::OffcanvasBrandLeft(brand, offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (brand.render(cx))
                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            @if let Some(oc) = offcanvas.get() {
                                (oc.render_offcanvas(cx, Some(self.items())))
                            }
                        },

                        // Barra con contenido en offcanvas y marca a la derecha.
                        navbar::Layout::OffcanvasBrandRight(brand, offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            (brand.render(cx))
                            @if let Some(oc) = offcanvas.get() {
                                (oc.render_offcanvas(cx, Some(self.items())))
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
        Self::default().with_layout(navbar::Layout::Simple)
    }

    /// Crea una barra de navegación **simple pero colapsable**, con botón a la izquierda.
    pub fn simple_toggle() -> Self {
        Self::default().with_layout(navbar::Layout::SimpleToggle)
    }

    /// Crea una barra de navegación **con marca a la izquierda**, siempre visible.
    pub fn simple_brand_left(brand: navbar::Brand) -> Self {
        Self::default().with_layout(navbar::Layout::SimpleBrandLeft(Embed::with(brand)))
    }

    /// Crea una barra de navegación con **marca a la izquierda** y **botón a la derecha**.
    pub fn brand_left(brand: navbar::Brand) -> Self {
        Self::default().with_layout(navbar::Layout::BrandLeft(Embed::with(brand)))
    }

    /// Crea una barra de navegación con **botón a la izquierda** y **marca a la derecha**.
    pub fn brand_right(brand: navbar::Brand) -> Self {
        Self::default().with_layout(navbar::Layout::BrandRight(Embed::with(brand)))
    }

    /// Crea una barra de navegación cuyo contenido se muestra en un **offcanvas**.
    pub fn offcanvas(oc: Offcanvas) -> Self {
        Self::default().with_layout(navbar::Layout::Offcanvas(Embed::with(oc)))
    }

    /// Crea una barra de navegación con **marca a la izquierda** y contenido en **offcanvas**.
    pub fn offcanvas_brand_left(brand: navbar::Brand, oc: Offcanvas) -> Self {
        Self::default().with_layout(navbar::Layout::OffcanvasBrandLeft(
            Embed::with(brand),
            Embed::with(oc),
        ))
    }

    /// Crea una barra de navegación con **marca a la derecha** y contenido en **offcanvas**.
    pub fn offcanvas_brand_right(brand: navbar::Brand, oc: Offcanvas) -> Self {
        Self::default().with_layout(navbar::Layout::OffcanvasBrandRight(
            Embed::with(brand),
            Embed::with(oc),
        ))
    }

    // **< Navbar BUILDER >*************************************************************************

    /// Establece el identificador único (`id`) de la barra de navegación.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas a la barra de navegación.
    ///
    /// También acepta clases predefinidas para:
    ///
    /// - Modificar el color de fondo ([`classes::Background`]).
    /// - Definir la apariencia del texto ([`classes::Text`]).
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
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
    pub fn with_layout(mut self, layout: navbar::Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Define dónde se mostrará la barra de navegación dentro del documento.
    #[builder_fn]
    pub fn with_position(mut self, position: navbar::Position) -> Self {
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
