use pagetop::prelude::*;

// **< HelpLink >***********************************************************************************

/// Enlace de ayuda de un proveedor.
///
/// Por ejemplo, la página donde se cambia la contraseña del directorio.
///
/// Sólo admite direcciones `http://` o `https://`: el enlace se muestra en páginas de la aplicación
/// y una dirección `javascript:` sería un ataque. Ver [`HelpLink::new()`].
#[derive(Clone, Debug, Getters)]
pub struct HelpLink {
    /// Dirección del enlace.
    url: String,
    /// Texto del enlace. Sin texto, se muestra la dirección (en el enlace de recuperación del
    /// formulario de inicio de sesión, «¿Olvidaste tu contraseña?»).
    text: Lc,
}

impl HelpLink {
    /// Crea el enlace, o devuelve `None` si la dirección no empieza por `http://` o `https://`
    /// (sin distinguir mayúsculas) o contiene espacios o caracteres de control.
    pub fn new(url: impl Into<String>, text: Lc) -> Option<Self> {
        let url = url.into();
        let url = url.trim();
        let lower = url.get(..8).unwrap_or(url).to_ascii_lowercase();
        let scheme = if lower.starts_with("https://") {
            8
        } else if lower.starts_with("http://") {
            7
        } else {
            return None;
        };
        let valid = url.len() > scheme && !url.chars().any(|c| c.is_whitespace() || c.is_control());
        valid.then(|| HelpLink {
            url: url.to_owned(),
            text,
        })
    }
}

// **< Políticas >**********************************************************************************

/// Qué puede hacer el usuario con un dato de su cuenta. Usado en [`AccountPolicy`].
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldPolicy {
    /// Lo puede cambiar el propio usuario, como en cualquier cuenta local.
    #[default]
    Editable,
    /// No se puede cambiar desde la aplicación: se indica que lo gestiona el proveedor.
    ReadOnly,
}

impl FieldPolicy {
    /// Si el usuario lo puede cambiar.
    pub fn is_editable(self) -> bool {
        self == FieldPolicy::Editable
    }
}

/// Si la cuenta puede recuperar su contraseña por correo. Usado en [`AccountPolicy`].
#[derive(AutoDefault, Clone, Debug)]
pub enum ResetPolicy {
    /// Puede, como cualquier cuenta local.
    #[default]
    Allowed,
    /// No puede: la contraseña se gestiona fuera de la aplicación. En lugar del enlace, la cuenta
    /// recibe por correo un aviso, con el enlace de ayuda si lo hay.
    Forbidden {
        /// Enlace de ayuda que se incluye en el aviso, si lo hay.
        help: Option<HelpLink>,
    },
}

/// Datos de la cuenta que puede gestionar un proveedor. Usado en [`AccountPolicy`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserField {
    /// Email.
    Email,
    /// Nombre visible.
    DisplayName,
    /// Texto «Sobre mí».
    About,
    /// Idioma.
    Language,
    /// Zona horaria.
    Timezone,
    /// Tema.
    Theme,
}

impl UserField {
    /// Nombre del campo, para las trazas.
    pub const fn as_str(self) -> &'static str {
        match self {
            UserField::Email => "email",
            UserField::DisplayName => "display_name",
            UserField::About => "about",
            UserField::Language => "language",
            UserField::Timezone => "timezone",
            UserField::Theme => "theme",
        }
    }
}

/// Qué puede cambiar el usuario, o el administrador, en una cuenta gestionada.
///
/// La devuelve [`AuthProvider::account_policy()`](super::AuthProvider::account_policy).
/// `pagetop-user` la aplica en el perfil, en las páginas de cambio de contraseña y de email, en la
/// recuperación de la contraseña y en la administración de usuarios. Por defecto todo es editable,
/// como en una cuenta local; el proveedor restringe lo que gestiona él.
#[derive(Clone, Debug, Getters)]
pub struct AccountPolicy {
    /// Nombre visible del proveedor, para indicar «gestionado por ...».
    provider_label: Lc,
    /// Contraseña. Si no es editable, no se puede cambiar desde la aplicación (tampoco un
    /// administrador puede restablecerla).
    #[getters(copy)]
    password: FieldPolicy,
    /// Email.
    #[getters(copy)]
    email: FieldPolicy,
    /// Nombre de usuario.
    #[getters(copy)]
    username: FieldPolicy,
    /// Datos del perfil que actualiza el proveedor: se muestran deshabilitados y se ignoran al
    /// guardar. El email se rige por [`email`](Self::email).
    managed_fields: Vec<UserField>,
    /// Roles que asigna y retira el proveedor: no se pueden poner ni quitar a mano.
    managed_roles: Vec<i32>,
    /// Recuperación de la contraseña por correo.
    password_reset: ResetPolicy,
    /// Enlace de ayuda para el usuario.
    help: Option<HelpLink>,
}

#[builder_impl]
impl AccountPolicy {
    /// Política con todo editable, como una cuenta local, del proveedor indicado.
    pub fn new(provider_label: Lc) -> Self {
        AccountPolicy {
            provider_label,
            password: FieldPolicy::Editable,
            email: FieldPolicy::Editable,
            username: FieldPolicy::Editable,
            managed_fields: Vec::new(),
            managed_roles: Vec::new(),
            password_reset: ResetPolicy::Allowed,
            help: None,
        }
    }

    /// Política de la contraseña.
    pub fn with_password(mut self, policy: FieldPolicy) -> Self {
        self.password = policy;
        self
    }

    /// Política del email.
    pub fn with_email(mut self, policy: FieldPolicy) -> Self {
        self.email = policy;
        self
    }

    /// Política del nombre de usuario.
    pub fn with_username(mut self, policy: FieldPolicy) -> Self {
        self.username = policy;
        self
    }

    /// Datos del perfil que gestiona el proveedor.
    pub fn with_managed_fields(mut self, fields: Vec<UserField>) -> Self {
        self.managed_fields = fields;
        self
    }

    /// Roles que gestiona el proveedor.
    pub fn with_managed_roles(mut self, role_ids: Vec<i32>) -> Self {
        self.managed_roles = role_ids;
        self
    }

    /// Recuperación de la contraseña por correo.
    pub fn with_password_reset(mut self, policy: ResetPolicy) -> Self {
        self.password_reset = policy;
        self
    }

    /// Enlace de ayuda para el usuario.
    pub fn with_help(mut self, help: Option<HelpLink>) -> Self {
        self.help = help;
        self
    }

    /// Si el proveedor gestiona ese dato del perfil.
    pub fn manages(&self, field: UserField) -> bool {
        self.managed_fields.contains(&field)
    }
}

// **< LoginHints >*********************************************************************************

/// Enlace «¿Olvidaste tu contraseña?» del formulario de inicio de sesión.
///
/// Usado en [`LoginHints`].
#[derive(AutoDefault, Clone, Debug)]
pub enum ResetLink {
    /// La recuperación por correo de `pagetop-user`.
    #[default]
    Local,
    /// Una página de ayuda externa.
    External(HelpLink),
    /// Sin enlace.
    Hidden,
}

/// Lo que un proveedor quiere mostrar en el formulario de inicio de sesión.
///
/// Lo devuelve [`AuthProvider::login_hints()`](super::AuthProvider::login_hints).
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct LoginHints {
    /// Texto breve bajo el formulario (p. ej. «Usa tu usuario y contraseña de la red»).
    note: Lc,
    /// Enlace para recuperar la contraseña. Sólo se tiene en cuenta si `"local"` no está en
    /// [`sources`](crate::config::LoginSettings::sources): si lo está, se ofrece siempre la
    /// recuperación de `pagetop-user`.
    reset: ResetLink,
}

#[builder_impl]
impl LoginHints {
    /// Texto breve bajo el formulario. `Lc::none()` lo quita.
    pub fn with_note(mut self, note: Lc) -> Self {
        self.note = note;
        self
    }

    /// Enlace para recuperar la contraseña.
    pub fn with_reset(mut self, reset: ResetLink) -> Self {
        self.reset = reset;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[pagetop::test]
    async fn help_links_only_accept_http_urls() {
        assert!(HelpLink::new("https://help.example/pwd", Lc::none()).is_some());
        assert!(HelpLink::new("HTTP://help.example", Lc::none()).is_some());
        assert!(HelpLink::new("javascript:alert(1)", Lc::none()).is_none());
        assert!(HelpLink::new("https://", Lc::none()).is_none());
        assert!(HelpLink::new("http://", Lc::none()).is_none());
        assert!(HelpLink::new("http://a", Lc::none()).is_some());
        assert!(HelpLink::new("https://a b", Lc::none()).is_none());
        assert!(HelpLink::new("//help.example", Lc::none()).is_none());
    }
}
