//! Handlers HTTP para el perfil del propio usuario autenticado (consulta, edición de sus datos y
//! cambio de contraseña) y para el perfil público de cualquier usuario.

use serde::Deserialize;

use pagetop::prelude::*;

use crate::account::{Account, UserStatus};
use crate::auth;
use crate::component::admin::{UserForm, UserFormMode, status_key};
use crate::component::{ChangePasswordForm, language_name, multiline_text, theme_name};
use crate::config::{user_language_applies, user_timezone_applies};
use crate::entity::{role, user};
use crate::error::AuthError;
use crate::handlers::admin::map_auth_error;
use crate::password;
use crate::permission::UserPermission;
use crate::service::user_admin;
use crate::session;
use crate::{LOCALES_USER, LOGIN_PATH, PROFILE_EDIT_PATH, PROFILE_PASSWORD_PATH, PROFILE_PATH};

// **< current_user_id / login_redirect >***********************************************************

// Identificador del usuario autenticado; el middleware de sesión sólo inserta `Account` si hay una
// sesión activa.
fn current_user_id(request: &HttpRequest) -> Option<i32> {
    request.extension::<Account>().map(|a| a.id())
}

// Redirección al formulario de inicio de sesión conservando `next` como URL de retorno.
fn login_redirect(request: HttpRequest, next: &'static str) -> Response {
    let cx = Context::new(request);
    Redirect::see_other(cx.route(LOGIN_PATH).with_param("next", next)).into_response()
}

// **< profile_get >********************************************************************************

/// GET /user - Perfil del usuario autenticado. Redirige al formulario de inicio de sesión si no hay
/// sesión activa, conservando la URL de retorno.
pub(crate) async fn profile_get(request: HttpRequest) -> Response {
    let Some(id) = current_user_id(&request) else {
        return login_redirect(request, PROFILE_PATH);
    };

    let user = match user_admin::find_user(id).await {
        Ok(user) => user,
        Err(_) => return ErrorPage::NotFound(Some(request)).into_response(),
    };
    let roles = match user_admin::user_roles(id).await {
        Ok(roles) => roles,
        Err(_) => return ErrorPage::InternalError(Some(request)).into_response(),
    };
    let status = UserStatus::from_i16(user.status);
    let can_edit = has_permission(&request, &UserPermission::EditOwnProfile);
    let can_change_password = has_permission(&request, &UserPermission::ChangeOwnPassword);

    let mut page = Page::new(request);
    let details_block = profile_details(&user, status, page.context()).await;
    let roles_block = profile_roles(&roles, page.context()).await;
    let actions = profile_actions(can_edit, can_change_password, page.context());

    page.with_title(Lc::t("title-profile", &LOCALES_USER))
        .with_child(details_block)
        .with_child(actions)
        .with_child(roles_block)
        .render()
        .await
        .into_response()
}

// Botones para editar el perfil y cambiar la contraseña, cada uno sólo si se tiene el permiso.
// Sin ninguno de los dos, el contenedor queda vacío y no se renderiza.
fn profile_actions(can_edit: bool, can_change_password: bool, cx: &Context) -> Flex {
    let mut actions = Flex::new()
        .with_wrap(flex::Behavior::Wrap)
        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)));
    if can_edit {
        actions = actions.with_child(
            Button::anchor(
                Lc::t("btn-edit-profile", &LOCALES_USER),
                cx.route(PROFILE_EDIT_PATH),
            )
            .with_style(button::Style::Solid(Intent::Primary)),
        );
    }
    if can_change_password {
        actions = actions.with_child(
            Button::anchor(
                Lc::t("btn-set-password", &LOCALES_USER),
                cx.route(PROFILE_PASSWORD_PATH),
            )
            .with_style(button::Style::Solid(Intent::Neutral)),
        );
    }
    actions
}

// Bloque de sólo lectura con los datos de perfil del usuario autenticado.
async fn profile_details(user: &user::Model, status: UserStatus, cx: &mut Context) -> Block {
    let mut table = Table::new()
        .with_prop(PropsOp::add_classes("user-admin-table"))
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-username-admin", &LOCALES_USER))
                .with_cell(user.username.as_str()),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-email", &LOCALES_USER))
                .with_cell(user.email.as_str()),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-display-name", &LOCALES_USER))
                .with_cell(user.display_name.as_deref().unwrap_or("-")),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-about", &LOCALES_USER))
                .with_cell(multiline_text(
                    user.about.clone().unwrap_or_else(|| "-".into()),
                )),
        );
    // Sólo se muestran si se aplican, igual que en el formulario de edición.
    if user_language_applies() {
        table = table.with_row(
            table::Row::new()
                .with_cell(Lc::t("field-language", &LOCALES_USER))
                .with_cell(language_name(user.language.as_deref())),
        );
    }
    if user_timezone_applies() {
        table = table.with_row(
            table::Row::new()
                .with_cell(Lc::t("field-timezone", &LOCALES_USER))
                .with_cell(user.timezone.as_deref().unwrap_or("-")),
        );
    }
    table = table
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-theme", &LOCALES_USER))
                .with_cell(theme_name(user.theme.as_deref())),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("col-status", &LOCALES_USER))
                .with_cell(Lc::t(status_key(status), &LOCALES_USER)),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-member-since", &LOCALES_USER))
                .with_cell(cx.format_date(
                    user.created_at.with_timezone(&cx.timezone()).date_naive(),
                    DateFormat::Long,
                )),
        );

    if user.is_admin {
        let badge = Badge::severe(Lc::t("badge-admin", &LOCALES_USER))
            .render(cx)
            .await;
        table = table.with_row(
            table::Row::new()
                .with_cell("")
                .with_cell(Html::with(move |_| badge.clone())),
        );
    }

    Block::new()
        .with_title(Lc::t("title-user-details", &LOCALES_USER))
        .with_child(table)
}

// Bloque de sólo lectura con los roles del usuario autenticado. A diferencia de la vista de
// administración, no enlaza cada rol a su pantalla de detalle: un usuario sin permisos de
// administración no puede acceder a ella.
async fn profile_roles(roles: &[role::Model], cx: &mut Context) -> Block {
    // Un bloque sin hijos no se renderiza.
    if roles.is_empty() {
        return Block::new();
    }

    let mut items: Vec<(String, Option<Markup>)> = Vec::with_capacity(roles.len());
    for r in roles {
        let system_badge = if r.locked {
            Some(
                Badge::warning(Lc::t("badge-system-role", &LOCALES_USER))
                    .render(cx)
                    .await,
            )
        } else {
            None
        };
        items.push((r.label.clone(), system_badge));
    }

    Block::new()
        .with_title(Lc::t("field-roles", &LOCALES_USER))
        .with_child(Html::with(move |_cx| {
            html! {
                ul class="user-profile-roles" {
                    @for (label, system_badge) in &items {
                        li {
                            (label.as_str())
                            @if let Some(badge) = system_badge {
                                " "
                                (badge)
                            }
                        }
                    }
                }
            }
        }))
}

// **< public_profile_get >*************************************************************************

/// GET /user/{id} - Perfil público de un usuario: nombre de usuario, nombre visible, "Sobre mí" y
/// fecha de alta. Requiere `user:view_profiles`, salvo para ver el propio. El perfil de una cuenta
/// que no está activa sólo lo ve quien administra usuarios.
pub(crate) async fn public_profile_get(
    request: HttpRequest,
    web::Path(id): web::Path<i32>,
) -> Result<Response, ErrorPage> {
    // El permiso se comprueba antes de buscar al usuario para no revelar qué ids existen.
    if current_user_id(&request) != Some(id) {
        require_permission(&request, &UserPermission::ViewProfiles)?;
    }
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(Some(request)));
    };
    if UserStatus::from_i16(user.status) != UserStatus::Active
        && !has_permission(&request, &UserPermission::AdminUsers)
    {
        return Err(ErrorPage::NotFound(Some(request)));
    }

    let title = match util::non_blank(user.display_name.as_deref().unwrap_or_default()) {
        Some(name) => name.to_owned(),
        None => user.username.clone(),
    };
    let mut page = Page::new(request);
    let details = public_profile_details(&user, page.context());

    Ok(page
        .with_title(Lc::n(title))
        .with_child(details)
        .render()
        .await
        .into_response())
}

// Bloque con los únicos datos que se muestran a otros usuarios; nunca el email, los roles, el
// estado ni la zona horaria.
fn public_profile_details(user: &user::Model, cx: &Context) -> Block {
    let member_since = cx.format_date(
        user.created_at.with_timezone(&cx.timezone()).date_naive(),
        DateFormat::Long,
    );
    let table = Table::new()
        .with_prop(PropsOp::add_classes("user-admin-table"))
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-username-admin", &LOCALES_USER))
                .with_cell(user.username.as_str()),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-display-name", &LOCALES_USER))
                .with_cell(user.display_name.as_deref().unwrap_or("-")),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-about", &LOCALES_USER))
                .with_cell(multiline_text(
                    user.about.clone().unwrap_or_else(|| "-".into()),
                )),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-member-since", &LOCALES_USER))
                .with_cell(member_since),
        );
    Block::new().with_child(table)
}

// **< profile_edit_get / profile_edit_post >*******************************************************

/// GET /user/edit - Formulario de edición del perfil propio.
pub(crate) async fn profile_edit_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_EDIT_PATH));
    };
    require_permission(&request, &UserPermission::EditOwnProfile)?;
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(Some(request)));
    };

    let form = UserForm::new()
        .with_username(user.username)
        .with_email(user.email)
        .with_display_name(user.display_name.unwrap_or_default())
        .with_about(user.about.unwrap_or_default())
        .with_language(user.language.unwrap_or_default())
        .with_timezone(user.timezone.unwrap_or_default())
        .with_theme(user.theme.unwrap_or_default());
    Ok(render_profile_edit(request, form).await)
}

#[derive(Deserialize)]
pub(crate) struct ProfileFormData {
    #[serde(default)]
    username: String,
    email: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    about: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    timezone: String,
    #[serde(default)]
    theme: String,
}

/// POST /user/edit - Guarda los datos del perfil propio. El nombre de usuario sólo cambia con
/// `user:change_own_username`; roles, estado y acceso irrestricto no se tocan nunca desde aquí.
pub(crate) async fn profile_edit_post(
    request: HttpRequest,
    web::Form(form): web::Form<ProfileFormData>,
) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_EDIT_PATH));
    };
    require_permission(&request, &UserPermission::EditOwnProfile)?;
    let Ok(user) = user_admin::find_user(id).await else {
        return Err(ErrorPage::NotFound(Some(request)));
    };

    // Sin permiso se conserva el nombre actual, aunque la petición se haya manipulado a mano.
    let username = if has_permission(&request, &UserPermission::ChangeOwnUsername) {
        form.username
    } else {
        user.username
    };

    let result = user_admin::update_user(
        id,
        user_admin::UserUpdateData {
            username: &username,
            email: &form.email,
            display_name: util::non_blank(&form.display_name),
            about: util::non_blank(&form.about),
            language: util::non_blank(&form.language),
            timezone: util::non_blank(&form.timezone),
            theme: util::non_blank(&form.theme),
        },
    )
    .await;

    match result {
        Ok(()) => {
            let cx = Context::new(request);
            Ok(Redirect::see_other(cx.route(PROFILE_PATH)).into_response())
        }
        Err(err) => {
            let form = UserForm::new()
                .with_username(username)
                .with_email(form.email)
                .with_display_name(form.display_name)
                .with_about(form.about)
                .with_language(form.language)
                .with_timezone(form.timezone)
                .with_theme(form.theme)
                .with_error(map_auth_error(&err));
            Ok(render_profile_edit(request, form).await)
        }
    }
}

// Página de edición del perfil propio con el formulario ya relleno.
async fn render_profile_edit(request: HttpRequest, form: UserForm) -> Response {
    let can_change_username = has_permission(&request, &UserPermission::ChangeOwnUsername);
    let form = form
        .with_mode(UserFormMode::Profile)
        .with_allow_username_field(can_change_username);
    render_profile_page(request, Lc::t("title-profile-edit", &LOCALES_USER), form).await
}

// **< password_get / password_post >***************************************************************

/// GET /user/password - Formulario de cambio de la contraseña propia.
pub(crate) async fn password_get(request: HttpRequest) -> Result<Response, ErrorPage> {
    if current_user_id(&request).is_none() {
        return Ok(login_redirect(request, PROFILE_PASSWORD_PATH));
    }
    require_permission(&request, &UserPermission::ChangeOwnPassword)?;
    let title = Lc::t("title-profile-password", &LOCALES_USER);
    Ok(render_profile_page(request, title, ChangePasswordForm::new()).await)
}

#[derive(Deserialize)]
pub(crate) struct ChangePasswordFormData {
    current_password: String,
    password: String,
    confirm_password: String,
}

/// POST /user/password - Cambia la contraseña propia tras comprobar la actual y cierra el resto de
/// sesiones abiertas del usuario, conservando la actual.
pub(crate) async fn password_post(
    request: HttpRequest,
    web::Form(form): web::Form<ChangePasswordFormData>,
) -> Result<Response, ErrorPage> {
    let Some(id) = current_user_id(&request) else {
        return Ok(login_redirect(request, PROFILE_PASSWORD_PATH));
    };
    require_permission(&request, &UserPermission::ChangeOwnPassword)?;

    let sid = session::extract_sid(Some(request.headers()));
    let result = match password::passwords_match(&form.password, &form.confirm_password) {
        Ok(()) => {
            auth::change_own_password(id, &form.current_password, &form.password, sid.as_deref())
                .await
        }
        Err(err) => Err(err),
    };

    match result {
        Ok(()) => {
            let cx = Context::new(request);
            Ok(Redirect::see_other(cx.route(PROFILE_PATH)).into_response())
        }
        Err(err) => {
            let error = match err {
                AuthError::InvalidCredentials => Lc::t("error-current-password", &LOCALES_USER),
                err => map_auth_error(&err),
            };
            let title = Lc::t("title-profile-password", &LOCALES_USER);
            let form = ChangePasswordForm::new().with_error(error);
            Ok(render_profile_page(request, title, form).await)
        }
    }
}

// **< render_profile_page >************************************************************************

// Página del perfil propio con un formulario y un botón para volver al perfil sin guardar.
async fn render_profile_page(
    request: HttpRequest,
    title: Lc,
    form: impl Component + 'static,
) -> Response {
    let mut page = Page::new(request);
    let cancel = Button::anchor(
        Lc::t("btn-cancel", &LOCALES_USER),
        page.context().route(PROFILE_PATH),
    );
    page.with_title(title.clone())
        .with_child(
            Block::new()
                .with_title(title)
                .with_child(form)
                .with_child(cancel),
        )
        .render()
        .await
        .into_response()
}
