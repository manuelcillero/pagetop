use pagetop::prelude::*;

use crate::ADMIN_BASE_PATH;
use crate::LOCALES_ADMIN;
use crate::action::SectionBag;
use crate::registry::AdminSection;

/// Declara las secciones incorporadas del panel de administración.
///
/// Se registra en `Admin::actions()` como `DeclareAdminSections::new(declare_default_sections)`
/// con peso negativo para ejecutarse antes que las extensiones de terceros.
pub(crate) fn declare_default_sections(bag: &mut SectionBag) {
    let base = ADMIN_BASE_PATH;

    bag.add(AdminSection {
        key: "people".to_owned(),
        path: format!("{}/people", base),
        title: Lc::t("section-people", &LOCALES_ADMIN),
        weight: 10,
    });
    bag.add(AdminSection {
        key: "structure".to_owned(),
        path: format!("{}/structure", base),
        title: Lc::t("section-structure", &LOCALES_ADMIN),
        weight: 20,
    });
    bag.add(AdminSection {
        key: "config".to_owned(),
        path: format!("{}/config", base),
        title: Lc::t("section-config", &LOCALES_ADMIN),
        weight: 30,
    });
    bag.add(AdminSection {
        key: "reports".to_owned(),
        path: format!("{}/reports", base),
        title: Lc::t("section-reports", &LOCALES_ADMIN),
        weight: 40,
    });
    bag.add(AdminSection {
        key: "help".to_owned(),
        path: format!("{}/help", base),
        title: Lc::t("section-help", &LOCALES_ADMIN),
        weight: 50,
    });
}
