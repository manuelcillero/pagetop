use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::*;

const TOGGLE_COLLAPSE: &str = "collapse";
const TOGGLE_OFFCANVAS: &str = "offcanvas";

const EXTRA_LAYOUT: &str = "bootsier.navbar.layout";

/// Extensión de Bootsier para [`Navbar`].
///
/// Permite mostrar enlaces, menús y una marca de identidad en distintas disposiciones (simples, con
/// botón de despliegue o dentro de un [`Offcanvas`]), controladas por [`navbar::Layout`]. También
/// puede fijarse en la parte superior o inferior del documento mediante [`navbar::Position`]. El
/// punto de corte a partir del cual deja de colapsar se define con [`Navbar::with_expand()`].
///
/// [`Navbar`]: crate::theme::bs::Navbar
/// [`Offcanvas`]: crate::theme::bs::Offcanvas
/// [`navbar::Layout`]: crate::theme::bs::navbar::Layout
/// [`navbar::Position`]: crate::theme::bs::navbar::Position
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
/// let brand = Brand::new()
///     .with_title(Lc::n("PageTop"))
///     .with_route(Route::from("/"));
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
/// let brand = Brand::new()
///     .with_title(Lc::n("Intranet"))
///     .with_route(Route::from("/"));
///
/// let navbar = bs::Navbar::brand_right(brand)
///     .with_expand(Breakpoint::Lg)
///     .with_item(bs::navbar::Item::nav(
///         bs::Nav::new()
///             .with_kind(bs::nav::Kind::Pills)
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
/// let brand = Brand::new()
///     .with_title(Lc::n("Main App"))
///     .with_route(Route::from("/"));
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
#[builder_impl]
pub trait NavbarBootsier {
    /// Crea una barra de navegación cuyo contenido se muestra en un **offcanvas**.
    fn offcanvas(oc: bs::Offcanvas) -> Self;

    /// Crea una barra de navegación con **marca de identidad** y contenido en **offcanvas**.
    fn offcanvas_brand_left(brand: Brand, oc: bs::Offcanvas) -> Self;

    /// Crea una barra de navegación con **marca de identidad** y contenido en **offcanvas**.
    fn offcanvas_brand_right(brand: Brand, oc: bs::Offcanvas) -> Self;
}

#[builder_impl]
impl NavbarBootsier for Navbar {
    fn offcanvas(oc: bs::Offcanvas) -> Self {
        let mut navbar = Self::new();
        navbar.alter_prop(PropsOp::set_extra(
            EXTRA_LAYOUT,
            bs::navbar::Layout::Offcanvas(Embed::with(oc)),
        ));
        navbar
    }

    fn offcanvas_brand_left(brand: Brand, oc: bs::Offcanvas) -> Self {
        let mut navbar = Self::new();
        navbar.alter_prop(PropsOp::set_extra(
            EXTRA_LAYOUT,
            bs::navbar::Layout::OffcanvasBrandLeft(Embed::with(brand), Embed::with(oc)),
        ));
        navbar
    }

    fn offcanvas_brand_right(brand: Brand, oc: bs::Offcanvas) -> Self {
        let mut navbar = Self::new();
        navbar.alter_prop(PropsOp::set_extra(
            EXTRA_LAYOUT,
            bs::navbar::Layout::OffcanvasBrandRight(Embed::with(brand), Embed::with(oc)),
        ));
        navbar
    }
}

// **< Navbar SETUP >*******************************************************************************

pub(crate) fn setup(navbar: &mut Navbar) {
    // Sin botón de despliegue no hay nada que colapsar, así que el punto de corte no debe afectar a
    // la barra: Bootstrap apilaría igualmente el menú por debajo de él. `navbar-expand` a secas es
    // su «expandida siempre». Las disposiciones de Bootsier (`EXTRA_LAYOUT`, con `Offcanvas`)
    // siempre llevan botón; de las de base sólo `Simple` y `SimpleBrandLeft` no lo llevan.
    let has_toggle = navbar
        .props()
        .extra::<bs::navbar::Layout>(EXTRA_LAYOUT)
        .is_ok()
        || !matches!(
            navbar.layout(),
            navbar::Layout::Simple | navbar::Layout::SimpleBrandLeft(_)
        );
    if !has_toggle {
        let expand = navbar.props().get_classes().and_then(|classes| {
            classes
                .split_whitespace()
                .find(|class| class.starts_with("navbar-expand-"))
                .map(String::from)
        });
        if let Some(class) = expand {
            navbar.alter_prop(PropsOp::replace_classes(class, "navbar-expand"));
        }
    }
}

// **< Navbar RENDER >******************************************************************************

pub(crate) async fn render(navbar: &Navbar, cx: &mut Context) -> Result<Markup, ComponentError> {
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
    let items = navbar.items().render(cx).await;
    if items.is_empty() {
        return Ok(html! {});
    }

    // `Navbar::setup()` (base) garantiza que habrá un `id` antes de renderizar.
    let id = navbar.id().unwrap();

    // `with_layout()` (extra propio de Bootsier) tiene prioridad; si no se ha usado, se traduce el
    // `navbar::Layout` de base que hayan podido fijar los constructores heredados de `Navbar`
    // (`simple()`, `brand_left()`...), que Bootsier no puede sobrescribir por nombre -las funciones
    // inherentes de base siempre ganan sobre las de un trait con el mismo nombre-.
    let layout = navbar
        .props()
        .extra::<bs::navbar::Layout>(EXTRA_LAYOUT)
        .cloned()
        .unwrap_or_else(|_| translate_layout(navbar.layout()));

    Ok(html! {
        nav (navbar.props().unpack(cx)) {
            div class="container-fluid" {
                @match layout {
                    // Barra más sencilla: sólo contenido.
                    bs::navbar::Layout::Simple => {
                        (items)
                    },

                    // Barra sencilla que se puede contraer/expandir.
                    bs::navbar::Layout::SimpleToggle => {
                        @let id_content = util::join!(&id, "-content");

                        (button(cx, TOGGLE_COLLAPSE, &id_content))
                        div id=(&id_content) class="collapse navbar-collapse" {
                            (items)
                        }
                    },

                    // Barra con marca a la izquierda, siempre visible. El contenedor deja los
                    // contenidos junto a la marca: sin él, el `space-between` de `container-fluid`
                    // los llevaría al extremo opuesto. Sin `collapse`, `setup()` garantiza
                    // `navbar-expand` y Bootstrap lo muestra siempre.
                    bs::navbar::Layout::SimpleBrandLeft(brand) => {
                        (brand.render(cx).await)
                        div class="navbar-collapse" { (items) }
                    },

                    // Barra con marca a la izquierda y botón a la derecha.
                    bs::navbar::Layout::BrandLeft(brand) => {
                        @let id_content = util::join!(&id, "-content");

                        (brand.render(cx).await)
                        (button(cx, TOGGLE_COLLAPSE, &id_content))
                        div id=(&id_content) class="collapse navbar-collapse" {
                            (items)
                        }
                    },

                    // Barra con botón a la izquierda y marca a la derecha.
                    bs::navbar::Layout::BrandRight(brand) => {
                        @let id_content = util::join!(&id, "-content");

                        (button(cx, TOGGLE_COLLAPSE, &id_content))
                        (brand.render(cx).await)
                        div id=(&id_content) class="collapse navbar-collapse" {
                            (items)
                        }
                    },

                    // Barra cuyo contenido se muestra en un offcanvas, sin marca.
                    bs::navbar::Layout::Offcanvas(offcanvas) => {
                        @let offcanvas = setup_offcanvas(&offcanvas, cx);
                        @let id_content = offcanvas
                            .as_ref()
                            .and_then(|oc| oc.id()).unwrap_or_default();

                        (button(cx, TOGGLE_OFFCANVAS, &id_content))
                        @if let Some(oc) = &offcanvas {
                            (oc.render_offcanvas(cx, Some(navbar.items())).await)
                        }
                    },

                    // Barra con marca a la izquierda y contenido en offcanvas.
                    bs::navbar::Layout::OffcanvasBrandLeft(brand, offcanvas) => {
                        @let offcanvas = setup_offcanvas(&offcanvas, cx);
                        @let id_content = offcanvas
                            .as_ref()
                            .and_then(|oc| oc.id()).unwrap_or_default();

                        (brand.render(cx).await)
                        (button(cx, TOGGLE_OFFCANVAS, &id_content))
                        @if let Some(oc) = &offcanvas {
                            (oc.render_offcanvas(cx, Some(navbar.items())).await)
                        }
                    },

                    // Barra con contenido en offcanvas y marca a la derecha.
                    bs::navbar::Layout::OffcanvasBrandRight(brand, offcanvas) => {
                        @let offcanvas = setup_offcanvas(&offcanvas, cx);
                        @let id_content = offcanvas
                            .as_ref()
                            .and_then(|oc| oc.id()).unwrap_or_default();

                        (button(cx, TOGGLE_OFFCANVAS, &id_content))
                        (brand.render(cx).await)
                        @if let Some(oc) = &offcanvas {
                            (oc.render_offcanvas(cx, Some(navbar.items())).await)
                        }
                    },
                }
            }
        }
    })
}

// El panel se renderiza con `render_offcanvas()` para inyectarle los contenidos de la barra, así
// que no pasa por el ciclo de vida normal. Se clona y se le aplica aquí su `setup()`; sin él no
// tendría `id` (el botón lo necesita para referenciarlo) ni las clases `offcanvas*` que lo ocultan
// y lo colocan.
fn setup_offcanvas(offcanvas: &Embed<bs::Offcanvas>, cx: &mut Context) -> Option<bs::Offcanvas> {
    let mut oc = offcanvas.get()?.clone();
    oc.setup(cx);
    Some(oc)
}

// Traduce el `navbar::Layout` semántico de base (sin `Offcanvas`, sin `Position`/`expand`) a la
// variante equivalente de `bs::navbar::Layout`, para las barras construidas con los constructores
// heredados de `Navbar` (`simple()`, `simple_toggle()`, `simple_brand_left()`, `brand_left()`,
// `brand_right()`) en vez de con `with_layout()`.
fn translate_layout(layout: &navbar::Layout) -> bs::navbar::Layout {
    match layout {
        navbar::Layout::Simple => bs::navbar::Layout::Simple,
        navbar::Layout::SimpleToggle => bs::navbar::Layout::SimpleToggle,
        navbar::Layout::SimpleBrandLeft(brand) => {
            bs::navbar::Layout::SimpleBrandLeft(brand.clone())
        }
        navbar::Layout::BrandLeft(brand) => bs::navbar::Layout::BrandLeft(brand.clone()),
        navbar::Layout::BrandRight(brand) => bs::navbar::Layout::BrandRight(brand.clone()),
    }
}
