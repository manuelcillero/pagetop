/// Es el tema básico que incluye PageTop por defecto.
use crate::prelude::*;

/// El tema básico usa las mismas regiones predefinidas por [`ThemeRegion`].
pub type BasicRegion = ThemeRegion;

/// Tema básico por defecto que extiende el funcionamiento predeterminado de [`Theme`].
pub struct Basic;

impl Extension for Basic {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

impl Theme for Basic {
    fn before_render_page_body(&self, page: &mut Page) {
        page.alter_param("include_basic_assets", true);
    }
}
