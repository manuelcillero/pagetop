/// Es el tema básico que incluye PageTop por defecto.
use crate::prelude::*;

/// Tema básico por defecto que extiende el funcionamiento predeterminado de [`Theme`].
pub struct Basic;

impl Extension for Basic {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

impl Theme for Basic {
    fn before_render_page_body(&self, page: &mut Page) {
        page.alter_param("include_basic_assets", true)
            .alter_child_in(
                Region::FOOTER,
                ChildOp::AddIfEmpty(Child::with(PoweredBy::new())),
            );
    }
}
