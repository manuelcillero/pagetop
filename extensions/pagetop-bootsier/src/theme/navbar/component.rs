use pagetop::prelude::*;

use crate::prelude::*;
use crate::LOCALES_BOOTSIER;

const TOGGLE_COLLAPSE: &str = "collapse";
const TOGGLE_OFFCANVAS: &str = "offcanvas";

/// Componente para crear una **barra de navegación**.
///
/// Permite mostrar enlaces, menús y una marca de identidad en distintas disposiciones (simples, con
/// botón de despliegue o dentro de un [`offcanvas`]), controladas por [`navbar::Layout`]. También
/// puede fijarse en la parte superior o inferior del documento mediante [`navbar::Position`].
///
/// Ver ejemplos en el módulo [`navbar`].
/// Si no contiene elementos, el componente **no se renderiza**.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Navbar {
    id          : AttrId,
    classes     : AttrClasses,
    expand      : BreakPoint,
    layout      : navbar::Layout,
    position    : navbar::Position,
    items       : Children,
}

impl Component for Navbar {
    fn new() -> Self {
        Navbar::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                "navbar".to_string(),
                self.expand().try_class("navbar-expand").unwrap_or_default(),
                match self.position() {
                    navbar::Position::Static => "",
                    navbar::Position::FixedTop => "fixed-top",
                    navbar::Position::FixedBottom => "fixed-bottom",
                    navbar::Position::StickyTop => "sticky-top",
                    navbar::Position::StickyBottom => "sticky-bottom",
                }
                .to_string(),
            ]
            .join_classes(),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        // Botón de despliegue (colapso u offcanvas) para la barra.
        fn button(cx: &mut Context, data_bs_toggle: &str, id_content: &str) -> Markup {
            let id_content_target = join!("#", id_content);
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
            return PrepareMarkup::None;
        }

        // Asegura que la barra tiene un id estable para poder asociarlo al colapso/offcanvas.
        let id = cx.required_id::<Self>(self.id());

        PrepareMarkup::With(html! {
            nav id=(id) class=[self.classes().get()] {
                div class="container-fluid" {
                    @match self.layout() {
                        // Barra más sencilla: sólo contenido.
                        navbar::Layout::Simple => {
                            (items)
                        },

                        // Barra sencilla que se puede contraer/expandir.
                        navbar::Layout::SimpleToggle => {
                            @let id_content = join!(id, "-content");

                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            div id=(id_content) class="collapse navbar-collapse" {
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
                            @let id_content = join!(id, "-content");

                            (brand.render(cx))
                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            div id=(id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra con botón a la izquierda y marca a la derecha.
                        navbar::Layout::BrandRight(brand) => {
                            @let id_content = join!(id, "-content");

                            (button(cx, TOGGLE_COLLAPSE, &id_content))
                            (brand.render(cx))
                            div id=(id_content) class="collapse navbar-collapse" {
                                (items)
                            }
                        },

                        // Barra cuyo contenido se muestra en un offcanvas, sin marca.
                        navbar::Layout::Offcanvas(offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            @if let Some(oc) = offcanvas.borrow() {
                                (oc.render_offcanvas(cx, Some(self.items())))
                            }
                        },

                        // Barra con marca a la izquierda y contenido en offcanvas.
                        navbar::Layout::OffcanvasBrandLeft(brand, offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (brand.render(cx))
                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            @if let Some(oc) = offcanvas.borrow() {
                                (oc.render_offcanvas(cx, Some(self.items())))
                            }
                        },

                        // Barra con contenido en offcanvas y marca a la derecha.
                        navbar::Layout::OffcanvasBrandRight(brand, offcanvas) => {
                            @let id_content = offcanvas.id().unwrap_or_default();

                            (button(cx, TOGGLE_OFFCANVAS, &id_content))
                            (brand.render(cx))
                            @if let Some(oc) = offcanvas.borrow() {
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
        Navbar::default().with_layout(navbar::Layout::Simple)
    }

    /// Crea una barra de navegación **simple pero colapsable**, con botón a la izquierda.
    pub fn simple_toggle() -> Self {
        Navbar::default().with_layout(navbar::Layout::SimpleToggle)
    }

    /// Crea una barra de navegación **con marca a la izquierda**, siempre visible.
    pub fn simple_brand_left(brand: navbar::Brand) -> Self {
        Navbar::default().with_layout(navbar::Layout::SimpleBrandLeft(Typed::with(brand)))
    }

    /// Crea una barra de navegación con **marca a la izquierda** y **botón a la derecha**.
    pub fn brand_left(brand: navbar::Brand) -> Self {
        Navbar::default().with_layout(navbar::Layout::BrandLeft(Typed::with(brand)))
    }

    /// Crea una barra de navegación con **botón a la izquierda** y **marca a la derecha**.
    pub fn brand_right(brand: navbar::Brand) -> Self {
        Navbar::default().with_layout(navbar::Layout::BrandRight(Typed::with(brand)))
    }

    /// Crea una barra de navegación cuyo contenido se muestra en un **offcanvas**.
    pub fn offcanvas(oc: Offcanvas) -> Self {
        Navbar::default().with_layout(navbar::Layout::Offcanvas(Typed::with(oc)))
    }

    /// Crea una barra de navegación con **marca a la izquierda** y contenido en **offcanvas**.
    pub fn offcanvas_brand_left(brand: navbar::Brand, oc: Offcanvas) -> Self {
        Navbar::default().with_layout(navbar::Layout::OffcanvasBrandLeft(
            Typed::with(brand),
            Typed::with(oc),
        ))
    }

    /// Crea una barra de navegación con **marca a la derecha** y contenido en **offcanvas**.
    pub fn offcanvas_brand_right(brand: navbar::Brand, oc: Offcanvas) -> Self {
        Navbar::default().with_layout(navbar::Layout::OffcanvasBrandRight(
            Typed::with(brand),
            Typed::with(oc),
        ))
    }

    // **< Navbar BUILDER >*************************************************************************

    /// Establece el identificador único (`id`) de la barra de navegación.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
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
        self.classes.alter_value(op, classes);
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

    /// Añade un nuevo contenido hijo.
    #[inline]
    pub fn add_item(mut self, item: navbar::Item) -> Self {
        self.items.add(Child::with(item));
        self
    }

    /// Modifica la lista de contenidos (`children`) aplicando una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_items(mut self, op: TypedOp<navbar::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // **< Navbar GETTERS >*************************************************************************

    /// Devuelve las clases CSS asociadas a la barra de navegación.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el punto de ruptura configurado.
    pub fn expand(&self) -> &BreakPoint {
        &self.expand
    }

    /// Devuelve la disposición configurada para la barra de navegación.
    pub fn layout(&self) -> &navbar::Layout {
        &self.layout
    }

    /// Devuelve la posición configurada para la barra de navegación.
    pub fn position(&self) -> &navbar::Position {
        &self.position
    }

    /// Devuelve la lista de contenidos (`children`).
    pub fn items(&self) -> &Children {
        &self.items
    }
}
