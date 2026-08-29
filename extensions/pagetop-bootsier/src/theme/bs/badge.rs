use pagetop::prelude::*;

use crate::theme::BootsierColors;

pub use pagetop::base::component::Badge;

const EXTRA_COLOR: &str = "bootsier.badge.color";

/// Extensión de Bootsier para [`Badge`].
///
/// Permite forzar un color de la paleta de Bootsier ([`BootsierColors`]) en vez del que le
/// correspondería por defecto a la [`Intent`] del badge -- por ejemplo, para usar `Light`/`Dark`,
/// que `Intent` no tiene.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let badge = bs::Badge::labeled(Lc::n("Beta")).with_color(BootsierColors::Dark);
/// ```
#[builder_impl]
pub trait BadgeBootsier {
    /// Fuerza un color de la paleta de Bootsier, ignorando el que le correspondería a la `Intent`
    /// del badge. `None` restablece el comportamiento por defecto (color derivado de la `Intent`).
    fn with_color(self, color: impl Into<Option<BootsierColors>>) -> Self;
}

#[builder_impl]
impl BadgeBootsier for Badge {
    fn with_color(mut self, color: impl Into<Option<BootsierColors>>) -> Self {
        match color.into() {
            Some(color) => self.alter_prop(PropsOp::set_extra(EXTRA_COLOR, color)),
            None => self.alter_prop(PropsOp::remove_extra(EXTRA_COLOR)),
        };
        self
    }
}

// **< Badge SETUP >********************************************************************************

pub(crate) fn setup(badge: &mut Badge) {
    // `Badge::setup()` (core) ya ha traducido la intención con `Theme::intent_color()` -- la clase
    // `badge-*` que hay que localizar es siempre la derivada de la `Intent`, con independencia de
    // que `BadgeBootsier::with_color()` fuerce un color distinto para el destino `text-bg-*`.
    let intent_color = BootsierColors::from(badge.intent()).as_str();
    let color = badge
        .props()
        .extra::<BootsierColors>(EXTRA_COLOR)
        .ok()
        .copied()
        .map_or(intent_color, |color| color.as_str());

    badge.alter_prop(PropsOp::replace_classes(
        util::join!("badge-", intent_color),
        util::join!("text-bg-", color),
    ));
}
