//! Nombre de un usuario enlazado a su perfil público.

use pagetop::prelude::*;

use crate::Account;
use crate::permission::UserPermission;
use crate::profile_path;

/// Componente para mostrar el **nombre de un usuario**, enlazado a su perfil público cuando quien
/// mira puede verlo.
///
/// Se renderiza como enlace a `/user/{id}` si quien hace la petición tiene `user:view_profiles` o
/// es el propio usuario; si no, como texto. Las propiedades del componente (identificador, clases,
/// atributos) se aplican en ambos casos.
///
/// Recibe el identificador y el nombre que el llamador ya tiene de su propia consulta, así que no
/// consulta la base de datos al renderizarse. El nombre suele ser el nombre visible del usuario o,
/// si no tiene, su nombre de usuario.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// use pagetop_user::prelude::*;
///
/// // En una celda de tabla, p. ej. el autor de un cambio en un historial.
/// let row = table::Row::new()
///     .with_cell(UserName::of(42, "Ana Pérez"))
///     .with_cell("Cambio de ubicación");
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct UserName {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el identificador del usuario.
    user_id: i32,
    /// Devuelve el nombre que se muestra.
    name: String,
}

#[async_trait]
impl Component for UserName {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes("user-name"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let user_id = self.user_id();
        let can_view = cx.request().is_some_and(|request| {
            request
                .extension::<Account>()
                .is_some_and(|a| a.id() == user_id)
                || has_permission(request, &UserPermission::ViewProfiles)
        });

        Ok(if can_view {
            let href = cx.route(profile_path(user_id));
            html! { a (self.props().unpack(cx)) href=(href) { (self.name()) } }
        } else {
            html! { span (self.props().unpack(cx)) { (self.name()) } }
        })
    }
}

#[builder_impl]
impl UserName {
    /// Crea el nombre del usuario `user_id` con el texto indicado.
    pub fn of(user_id: i32, name: impl Into<String>) -> Self {
        Self {
            user_id,
            name: name.into(),
            ..Default::default()
        }
    }

    // **< UserName BUILDER >***********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el identificador del usuario.
    pub fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }

    /// Establece el nombre que se muestra.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}
