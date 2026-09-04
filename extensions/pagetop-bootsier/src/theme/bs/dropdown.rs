//! Definiciones para crear menús desplegables ([`Dropdown`]).
//!
//! Cada [`dropdown::Item`] representa un elemento individual del
//! desplegable [`Dropdown`], con distintos comportamientos según su finalidad, como enlaces de
//! navegación, botones de acción, encabezados o divisores visuales.
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`Lc`].

use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;

mod props;
pub use props::{AutoClose, Direction, MenuAlign, MenuPosition};

pub use pagetop::base::component::Dropdown;
pub use pagetop::base::component::dropdown::{Item, ItemKind};

const EXTRA_BUTTON_GROUPED: &str = "bootsier.dropdown.button_grouped";
const EXTRA_AUTO_CLOSE: &str = "bootsier.dropdown.auto_close";
const EXTRA_DIRECTION: &str = "bootsier.dropdown.direction";
const EXTRA_MENU_ALIGN: &str = "bootsier.dropdown.menu_align";
const EXTRA_MENU_POSITION: &str = "bootsier.dropdown.menu_position";

/// Extensión de Bootsier para [`Dropdown`].
///
/// Admite variaciones para el tamaño y el color del botón, y también para la dirección de
/// apertura, la alineación o la política de cierre del menú.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let dd = bs::Dropdown::new()
///     .with_title(Lc::n("Menu"))
///     .with_button_size(button::Size::Small)
///     .with_button_style(button::Style::Solid(Intent::Neutral))
///     .with_auto_close(bs::dropdown::AutoClose::ClickableInside)
///     .with_direction(bs::dropdown::Direction::Dropend)
///     .with_item(bs::dropdown::Item::link(Lc::n("Home"), "/"))
///     .with_item(bs::dropdown::Item::link_blank(Lc::n("Doc"), "https://docs.rs"))
///     .with_item(bs::dropdown::Item::divider())
///     .with_item(bs::dropdown::Item::header(Lc::n("User session")))
///     .with_item(bs::dropdown::Item::button(Lc::n("Sign out")));
/// ```
#[builder_impl]
pub trait DropdownBootsier {
    /// Indica si el botón del menú está integrado en un grupo de botones.
    fn with_button_grouped(self, grouped: bool) -> Self;

    /// Establece la política de cierre automático del menú desplegable.
    fn with_auto_close(self, auto_close: AutoClose) -> Self;

    /// Establece la dirección de despliegue del menú.
    fn with_direction(self, direction: Direction) -> Self;

    /// Configura la alineación horizontal (con posible comportamiento *responsive* adicional).
    fn with_menu_align(self, align: MenuAlign) -> Self;

    /// Configura la posición del menú.
    fn with_menu_position(self, position: MenuPosition) -> Self;
}

#[builder_impl]
impl DropdownBootsier for Dropdown {
    fn with_button_grouped(mut self, grouped: bool) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_BUTTON_GROUPED, grouped));
        self
    }

    fn with_auto_close(mut self, auto_close: AutoClose) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_AUTO_CLOSE, auto_close));
        self
    }

    fn with_direction(mut self, direction: Direction) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_DIRECTION, direction));
        self
    }

    fn with_menu_align(mut self, align: MenuAlign) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_MENU_ALIGN, align));
        self
    }

    fn with_menu_position(mut self, position: MenuPosition) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_MENU_POSITION, position));
        self
    }
}

// **< Dropdown SETUP >*****************************************************************************

pub(crate) fn setup(dropdown: &mut Dropdown) {
    let direction = dropdown
        .props()
        .extra_or(EXTRA_DIRECTION, Direction::default());
    let grouped = dropdown.props().extra_or(EXTRA_BUTTON_GROUPED, false);
    dropdown.alter_prop(PropsOp::replace_classes(
        "dropdown",
        direction.to_class(grouped),
    ));
}

// **< Dropdown RENDER >****************************************************************************

pub(crate) async fn render(
    dropdown: &Dropdown,
    cx: &mut Context,
) -> Result<Markup, ComponentError> {
    // Si no hay elementos en el menú, no se prepara.
    let items = dropdown.items().render(cx).await;
    if items.is_empty() {
        return Ok(html! {});
    }

    // Título opcional para el menú desplegable.
    let title = dropdown.title().using(cx);

    if title.is_empty() {
        // Sin título: menú contextual estático, sin botón ni comportamiento de apertura/cierre.
        return Ok(html! {
            div (dropdown.props().unpack(cx)) {
                ul class="dropdown-menu" { (items) }
            }
        });
    }

    let button_size = dropdown.button_size();
    let style = dropdown.button_style();
    let auto_close = dropdown
        .props()
        .extra_or(EXTRA_AUTO_CLOSE, AutoClose::default());
    let direction = dropdown
        .props()
        .extra_or(EXTRA_DIRECTION, Direction::default());
    let menu_align = dropdown
        .props()
        .extra_or(EXTRA_MENU_ALIGN, MenuAlign::default());
    let menu_position = dropdown
        .props()
        .extra_or(EXTRA_MENU_POSITION, MenuPosition::default());

    let btn_base = {
        let mut classes = String::from("btn");
        match button_size {
            button::Size::None => {}
            button::Size::Small => classes.push_str(" btn-sm"),
            button::Size::Large => classes.push_str(" btn-lg"),
        }
        match style {
            button::Style::None => {}
            button::Style::Solid(intent) => {
                classes.push_str(" btn-");
                classes.push_str(intent.color(cx));
            }
            button::Style::Outline(intent) => {
                classes.push_str(" btn-outline-");
                classes.push_str(intent.color(cx));
            }
            button::Style::Link => classes.push_str(" btn-link"),
        }
        classes
    };
    let offset = menu_position.data_offset();
    let reference = menu_position.data_reference();
    let auto_close = auto_close.opt_str();
    let menu_classes = {
        let mut classes = "dropdown-menu".to_string();
        menu_align.push_to(&mut classes);
        classes
    };

    Ok(html! {
        div (dropdown.props().unpack(cx)) {
            // Renderizado en modo split (dos botones) o simple (un botón).
            @if *dropdown.button_split() {
                // Botón principal (acción/etiqueta).
                @let btn = html! {
                    button
                        type="button"
                        class=(&btn_base)
                    {
                        (&title)
                    }
                };
                // Botón *toggle* que abre/cierra el menú asociado.
                @let btn_toggle_classes =
                    util::join!(&btn_base, " dropdown-toggle dropdown-toggle-split");
                @let btn_toggle = html! {
                    button
                        type="button"
                        class=(&btn_toggle_classes)
                        data-bs-toggle="dropdown"
                        data-bs-offset=[offset]
                        data-bs-reference=[reference]
                        data-bs-auto-close=[auto_close]
                        aria-expanded="false"
                    {
                        span class="visually-hidden" {
                            (Lc::t("dropdown_toggle", &LOCALES_BOOTSIER).using(cx))
                        }
                    }
                };
                // Orden según dirección (en `dropstart` el *toggle* se sitúa antes).
                @match direction {
                    Direction::Dropstart => {
                        (btn_toggle)
                        ul class=(&menu_classes) { (items) }
                        (btn)
                    }
                    _ => {
                        (btn)
                        (btn_toggle)
                        ul class=(&menu_classes) { (items) }
                    }
                }
            } @else {
                // Botón único con funcionalidad de *toggle*.
                @let btn_toggle_classes = util::join!(&btn_base, " dropdown-toggle");
                button
                    type="button"
                    class=(&btn_toggle_classes)
                    data-bs-toggle="dropdown"
                    data-bs-offset=[offset]
                    data-bs-reference=[reference]
                    data-bs-auto-close=[auto_close]
                    aria-expanded="false"
                {
                    (&title)
                }
                ul class=(&menu_classes) { (items) }
            }
        }
    })
}
