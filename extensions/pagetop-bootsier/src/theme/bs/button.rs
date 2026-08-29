use pagetop::prelude::*;

use crate::theme::BootsierColors;

pub use pagetop::base::component::button::{Button, Kind, Size, Style};

const EXTRA_ACTIVE: &str = "bootsier.button.active";
const EXTRA_FULL_WIDTH: &str = "bootsier.button.full_width";
const EXTRA_COLOR: &str = "bootsier.button.color";

// **< ButtonBootsier >*****************************************************************************

/// Extensión de Bootsier para [`Button`].
///
/// Añade funcionalidad de Bootstrap que no cubre el componente base: estado activo (`.active`,
/// `aria-pressed`), ancho completo (`w-100`, el reemplazo de `.btn-block` desde Bootstrap 5), y un
/// color de la paleta de Bootsier que fuerza el que le correspondería a la [`Intent`] del botón --
/// por ejemplo, para usar `Light`/`Dark`, que `Intent` no tiene.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let toggle = bs::Button::plain(Lc::n("Bold"))
///     .with_style(button::Style::Outline(Intent::Neutral))
///     .with_active(true);
///
/// let submit = bs::Button::submit(Lc::n("Save"))
///     .with_style(button::Style::Solid(Intent::Primary))
///     .with_full_width(true);
///
/// let subtle = bs::Button::plain(Lc::n("Cancel"))
///     .with_style(button::Style::Solid(Intent::Neutral))
///     .with_color(BootsierColors::Light);
/// ```
#[builder_impl]
pub trait ButtonBootsier {
    /// Marca el botón como activo (`.active`, `aria-pressed="true"`).
    fn with_active(self, active: bool) -> Self;

    /// Expande el botón al ancho completo de su contenedor (`w-100`).
    fn with_full_width(self, full_width: bool) -> Self;

    /// Fuerza un color de la paleta de Bootsier, ignorando el que le correspondería a la `Intent`
    /// del botón. `None` restablece el comportamiento por defecto (color derivado de la `Intent`).
    /// Sin efecto si el estilo del botón es [`Style::Link`] o [`Style::None`].
    fn with_color(self, color: impl Into<Option<BootsierColors>>) -> Self;
}

#[builder_impl]
impl ButtonBootsier for Button {
    fn with_active(mut self, active: bool) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_ACTIVE, active));
        self
    }

    fn with_full_width(mut self, full_width: bool) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_FULL_WIDTH, full_width));
        self
    }

    fn with_color(mut self, color: impl Into<Option<BootsierColors>>) -> Self {
        match color.into() {
            Some(color) => self.alter_prop(PropsOp::set_extra(EXTRA_COLOR, color)),
            None => self.alter_prop(PropsOp::remove_extra(EXTRA_COLOR)),
        };
        self
    }
}

// **< Button SETUP >*******************************************************************************

pub(crate) fn setup(button: &mut Button) {
    button.alter_prop(PropsOp::replace_classes("button", "btn"));

    // `Button::setup()` (core) ya ha traducido la intención con `Theme::intent_color()` -- aquí
    // sólo queda cambiar el prefijo `button-`/`button-outline-` por el equivalente
    // `btn-`/`btn-outline-` de Bootstrap, conservando el mismo nombre de color salvo que
    // `with_color()` lo sobrescriba.
    let override_color = button
        .props()
        .extra::<BootsierColors>(EXTRA_COLOR)
        .ok()
        .copied();
    let (core_class, btn_class) = match button.style() {
        Style::None => (String::new(), String::new()),
        Style::Solid(intent) => {
            let intent_color = BootsierColors::from(intent).as_str();
            let color = override_color.map_or(intent_color, |color| color.as_str());
            (
                util::join!("button-", intent_color),
                util::join!("btn-", color),
            )
        }
        Style::Outline(intent) => {
            let intent_color = BootsierColors::from(intent).as_str();
            let color = override_color.map_or(intent_color, |color| color.as_str());
            (
                util::join!("button-outline-", intent_color),
                util::join!("btn-outline-", color),
            )
        }
        Style::Link => ("button-link".to_string(), "btn-link".to_string()),
    };
    if !core_class.is_empty() {
        button.alter_prop(PropsOp::replace_classes(core_class, btn_class));
    }

    let (size_core, size_btn) = match button.size() {
        Size::None => (String::new(), String::new()),
        Size::Small => ("button-sm".to_string(), "btn-sm".to_string()),
        Size::Large => ("button-lg".to_string(), "btn-lg".to_string()),
    };
    if !size_core.is_empty() {
        button.alter_prop(PropsOp::replace_classes(size_core, size_btn));
    }

    // Renombra el vocabulario neutro para abrir/cerrar un `Dialog` (común a todos los temas, ver
    // `base::component::dialog`) al que reconoce el JS de Bootstrap.
    button.alter_prop(PropsOp::rename("data-dialog-toggle", "data-bs-toggle"));
    button.alter_prop(PropsOp::rename("data-dialog-target", "data-bs-target"));
    button.alter_prop(PropsOp::rename("data-dialog-dismiss", "data-bs-dismiss"));

    if button.props().extra_or(EXTRA_ACTIVE, false) {
        button.alter_prop(PropsOp::add_classes("active"));
        button.alter_prop(PropsOp::set("aria-pressed", "true"));
    }

    if button.props().extra_or(EXTRA_FULL_WIDTH, false) {
        button.alter_prop(PropsOp::add_classes("w-100"));
    }
}
