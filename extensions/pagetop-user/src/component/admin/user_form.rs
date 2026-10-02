//! Formulario de alta/edición de usuario.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use pagetop::prelude::*;

use crate::ADMIN_USERS_PATH;
use crate::LOCALES_USER;
use crate::user_path;

use crate::component::{PasswordConfirm, error_banner};

use super::{USER_ADMIN_FORM_ID, roles_fieldset};

// Regiones de la base IANA que sólo contienen alias heredados (fichero `backward`), todos con una
// zona canónica equivalente en otra región (p. ej. `US/Eastern` es `America/New_York`).
const LEGACY_REGIONS: [&str; 5] = ["Brazil", "Canada", "Chile", "Mexico", "US"];

// Zonas horarias IANA canónicas agrupadas por región (lo anterior a la primera `/`), ordenadas por
// región y nombre. Se descartan los alias heredados: los nombres sin región (`GB`, `Japan`,
// `EST5EDT`...), los de `LEGACY_REGIONS` y los de `Etc` salvo `Etc/UTC`, cuyo grupo va al final.
static TZ_BY_REGION: LazyLock<Vec<(&'static str, Vec<&'static str>)>> = LazyLock::new(|| {
    let mut regions: BTreeMap<&'static str, Vec<&'static str>> = BTreeMap::new();
    for tz in TZ_VARIANTS.iter() {
        let name = tz.name();
        let Some((region, _)) = name.split_once('/') else {
            continue;
        };
        if LEGACY_REGIONS.contains(&region) || (region == "Etc" && name != "Etc/UTC") {
            continue;
        }
        regions.entry(region).or_default().push(name);
    }
    for names in regions.values_mut() {
        names.sort_unstable();
    }
    let etc = regions.remove_entry("Etc");
    regions.into_iter().chain(etc).collect()
});

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub(crate) enum UserFormMode {
    #[default]
    New,
    Edit,
}

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct UserForm {
    mode: UserFormMode,
    error: Option<Lc>,
    user_id: Option<i32>,
    /// Listado de origen al que volver tras guardar (orden, búsqueda, página).
    waypoint: Waypoint,
    username: String,
    email: String,
    display_name: String,
    language: String,
    timezone: String,
    /// Roles asignables (excluye "anonymous" y "authenticated"); sólo se renderiza en modo `New`.
    roles: Vec<(i32, String, bool)>,
    /// Si se ofrece la casilla "administrador"; sólo cuando quien da de alta ya es administrador.
    allow_admin_field: bool,
    is_admin: bool,
}

#[async_trait]
impl Component for UserForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = match self.mode() {
            UserFormMode::New => util::join!(ADMIN_USERS_PATH, "/new"),
            UserFormMode::Edit => user_path(self.user_id().copied().unwrap_or_default(), "edit"),
        };
        let action = self.waypoint().append_to(cx.route(action));

        let mut form = Form::new()
            .with_id(USER_ADMIN_FORM_ID)
            .with_action(action)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()))
            .with_child(
                form::input::Field::text()
                    .with_name("username")
                    .with_value(self.username())
                    .with_label(Lc::t("field-username-admin", &LOCALES_USER))
                    .with_required(true)
                    .with_maxlength(Some(64)),
            )
            .with_child(
                form::input::Field::email()
                    .with_name("email")
                    .with_value(self.email())
                    .with_label(Lc::t("field-email", &LOCALES_USER))
                    .with_required(true),
            )
            .with_child(
                form::input::Field::text()
                    .with_name("display_name")
                    .with_value(self.display_name())
                    .with_label(Lc::t("field-display-name", &LOCALES_USER)),
            )
            .with_child(
                form::input::Field::text()
                    .with_name("language")
                    .with_value(self.language())
                    .with_label(Lc::t("field-language", &LOCALES_USER)),
            )
            .with_child(timezone_field(self.timezone()));

        if *self.mode() == UserFormMode::New {
            form = form.with_child(PasswordConfirm::new());
            if !self.roles().is_empty() {
                form = form.with_child(roles_fieldset(self.roles()));
            }

            if *self.allow_admin_field() {
                form = form.with_child(
                    form::Checkbox::check()
                        .with_name("is_admin")
                        .with_label(Lc::t("field-is-admin", &LOCALES_USER))
                        .with_checked(*self.is_admin()),
                );
            }
        }

        // En modo `Edit`, "Guardar" se renderiza fuera del formulario, junto al resto de acciones
        // de la pantalla (ver `USER_ADMIN_FORM_ID`); en modo `New` no hay ninguna botonera con la
        // que agruparlo, así que se queda aquí, dentro del propio `<form>`.
        if *self.mode() == UserFormMode::New {
            form = form.with_child(
                Button::submit(Lc::t("btn-save", &LOCALES_USER))
                    .with_style(button::Style::Solid(Intent::Primary)),
            );
        }

        Ok(form.render(cx).await)
    }
}

#[builder_impl]
impl UserForm {
    // **< UserForm BUILDER >***********************************************************************

    pub(crate) fn with_mode(mut self, mode: UserFormMode) -> Self {
        self.mode = mode;
        self
    }

    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    pub(crate) fn with_user_id(mut self, user_id: impl Into<Option<i32>>) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub(crate) fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }

    pub(crate) fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    pub(crate) fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub(crate) fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub(crate) fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub(crate) fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = timezone.into();
        self
    }

    pub(crate) fn with_roles(mut self, roles: Vec<(i32, String, bool)>) -> Self {
        self.roles = roles;
        self
    }

    pub(crate) fn with_allow_admin_field(mut self, allow_admin_field: bool) -> Self {
        self.allow_admin_field = allow_admin_field;
        self
    }

    pub(crate) fn with_is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }
}

// `<select>` de zona horaria: primero la opción de usar la predeterminada de la aplicación y luego
// un `<optgroup>` por región. La etiqueta de cada opción es el nombre IANA completo.
fn timezone_field(selected: &str) -> form::select::Field {
    let mut field = form::select::Field::new()
        .with_name("timezone")
        .with_label(Lc::t("field-timezone", &LOCALES_USER))
        .with_item(
            form::select::Item::new(
                "",
                Lc::t("field-timezone-site-default", &LOCALES_USER)
                    .with_arg("tz", Timezone::default_tz().name()),
            )
            .with_selected(selected.is_empty()),
        );
    for (region, names) in TZ_BY_REGION.iter() {
        let mut group = form::select::Group::new(Lc::n(*region));
        for name in names {
            group = group.with_item(
                form::select::Item::new(*name, Lc::n(*name)).with_selected(*name == selected),
            );
        }
        field = field.with_group(group);
    }
    field
}
