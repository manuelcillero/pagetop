//! Definiciones para mostrar iconos ([`Icon`]).

use pagetop::prelude::*;

pub use pagetop::base::component::icon::{Core, Icon, Source};

/// Extensión de Bootsier para [`Icon`].
///
/// Añade los iconos de [Bootstrap Icons](https://icons.getbootstrap.com/), que Bootsier ya carga
/// como fuente de iconos en todas las páginas. Además, Bootsier muestra los iconos del conjunto de
/// PageTop ([`Core`]) con su equivalente de Bootstrap Icons.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let home = bs::Icon::bi("house-fill").with_label(Lc::n("Home"));
/// ```
pub trait IconBsExt: Sized {
    /// Crea un icono de Bootstrap Icons a partir de su nombre, sin el prefijo `bi-` (p. ej.
    /// `"house-fill"`).
    fn bi(name: impl AsRef<str>) -> Self;
}

impl IconBsExt for Icon {
    fn bi(name: impl AsRef<str>) -> Self {
        Icon::with(Source::font(util::join!("bi bi-", name.as_ref())))
    }
}

// **< Icon SETUP >*********************************************************************************

// Sustituye cada icono del conjunto de PageTop por su equivalente de Bootstrap Icons. El `setup()`
// del icono ya se ha ejecutado, así que las clases del glifo se añaden aquí; el origen cambia para
// que se pinte como glifo.
pub(crate) fn setup(icon: &mut Icon) {
    let Some(&Source::Core(core)) = icon.source() else {
        return;
    };
    let classes = bi_classes(core);
    icon.alter_prop(PropsOp::add_classes(classes));
    icon.alter_source(Source::font(classes));
}

#[rustfmt::skip]
const fn bi_classes(icon: Core) -> &'static str {
    match icon {
        Core::BarChart       => "bi bi-bar-chart",
        Core::Check          => "bi bi-check-lg",
        Core::ChevronDown    => "bi bi-chevron-down",
        Core::ChevronLeft    => "bi bi-chevron-left",
        Core::ChevronRight   => "bi bi-chevron-right",
        Core::ChevronUp      => "bi bi-chevron-up",
        Core::Close          => "bi bi-x-lg",
        Core::Envelope       => "bi bi-envelope",
        Core::Gear           => "bi bi-gear",
        Core::Home           => "bi bi-house",
        Core::Key            => "bi bi-key",
        Core::Layers         => "bi bi-layers",
        Core::Minus          => "bi bi-dash-lg",
        Core::People         => "bi bi-people",
        Core::Person         => "bi bi-person",
        Core::Plus           => "bi bi-plus-lg",
        Core::QuestionCircle => "bi bi-question-circle",
        Core::Shield         => "bi bi-shield-lock",
        Core::SignOut        => "bi bi-box-arrow-right",
        Core::Sliders        => "bi bi-sliders",
    }
}
