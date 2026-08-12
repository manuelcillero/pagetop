use pagetop::prelude::*;

use crate::theme::*;

pub use pagetop::base::component::Badge;

const EXTRA_TEXT_BG: &str = "bootsier.badge.text_bg";

/// Extensión de Bootsier para [`Badge`].
///
/// Proporciona el método [`with_text_bg()`](Self::with_text_bg) para fijar el color de fondo del
/// badge usando un color de texto con contraste suficiente garantizado.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let admin = bs::Badge::labeled(Lc::n("Admin")).with_text_bg(ThemeColor::Danger);
/// ```
pub trait BadgeBootsier {
    #[builder_fn]
    fn with_text_bg(self, color: ThemeColor) -> Self;
}

impl BadgeBootsier for Badge {
    /// Establece el color de fondo usando un color de texto con contraste garantizado.
    ///
    /// Igual a `with_prop(PropsOp::add_classes(class::TextColor::Bg(color)))`, pero sin el riesgo
    /// de acumular más de una clase `text-bg-{color}` si se llama varias veces.
    #[builder_fn]
    fn with_text_bg(mut self, color: ThemeColor) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_TEXT_BG, color));
        self
    }
}

// **< Badge SETUP >********************************************************************************

pub(crate) fn setup(badge: &mut Badge) {
    let color = badge.props().extra_or(EXTRA_TEXT_BG, ThemeColor::Secondary);
    badge.alter_prop(PropsOp::replace_classes(
        "badge",
        util::join!("badge ", class::TextColor::Bg(color).to_class()),
    ));
}
