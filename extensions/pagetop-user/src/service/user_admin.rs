//! Servicio de administración de usuarios: listado, CRUD, roles y estado.

use std::collections::HashMap;

use pagetop::prelude::*;
use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, Order, Paginated,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait, dbconn, flatten_txn_err,
    paginate,
};

use crate::account::UserStatus;
use crate::config::{user_language_applies, user_timezone_applies};
use crate::entity::{role, user, user_role};
use crate::error::AuthError;
use crate::password;
use crate::session;

// **< listado >************************************************************************************

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum UserSortField {
    #[default]
    Username,
    Email,
    CreatedAt,
}

impl UserSortField {
    pub(crate) fn from_query(s: Option<&str>) -> Self {
        match s {
            Some("email") => UserSortField::Email,
            Some("created_at") => UserSortField::CreatedAt,
            _ => UserSortField::Username,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            UserSortField::Username => "username",
            UserSortField::Email => "email",
            UserSortField::CreatedAt => "created_at",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UserListItem {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub status: UserStatus,
    pub roles: Vec<String>,
    pub is_admin: bool,
}

pub(crate) struct UserListParams {
    pub query: Option<String>,
    pub sort: UserSortField,
    pub dir: SortDir,
    pub page: u64,
    pub per_page: u64,
}

/// Devuelve una página de usuarios. Usado por el listado de administración de usuarios.
pub(crate) async fn list_users(
    params: &UserListParams,
) -> Result<Paginated<UserListItem>, AuthError> {
    let mut select = user::Entity::find();

    if let Some(q) = params.query.as_deref().and_then(util::non_blank) {
        select = select.filter(
            Condition::any()
                .add(user::Column::Username.contains(q))
                .add(user::Column::Email.contains(q))
                .add(user::Column::DisplayName.contains(q)),
        );
    }

    let order = if params.dir == SortDir::Desc {
        Order::Desc
    } else {
        Order::Asc
    };
    select = match params.sort {
        UserSortField::Username => select.order_by(user::Column::Username, order),
        UserSortField::Email => select.order_by(user::Column::Email, order),
        UserSortField::CreatedAt => select.order_by(user::Column::CreatedAt, order),
    };

    paginate(select, params.page, params.per_page)
        .await?
        .map_items(user_items)
        .await
}

async fn user_items(users: Vec<user::Model>) -> Result<Vec<UserListItem>, AuthError> {
    let user_ids: Vec<i32> = users.iter().map(|u| u.id).collect();
    let role_rows = user_role::Entity::find()
        .filter(user_role::Column::UserId.is_in(user_ids))
        .find_also_related(role::Entity)
        .all(dbconn())
        .await?;

    let mut roles_by_user: HashMap<i32, Vec<String>> = HashMap::new();
    for (ur, role) in role_rows {
        if let Some(role) = role {
            roles_by_user
                .entry(ur.user_id)
                .or_default()
                .push(role.machine_name);
        }
    }

    Ok(users
        .into_iter()
        .map(|u| UserListItem {
            id: u.id,
            username: u.username,
            email: u.email,
            display_name: u.display_name,
            status: UserStatus::from_i16(u.status),
            roles: roles_by_user.remove(&u.id).unwrap_or_default(),
            is_admin: u.is_admin,
        })
        .collect())
}

// **< find_user / user_role_ids >******************************************************************

pub(crate) async fn find_user(user_id: i32) -> Result<user::Model, AuthError> {
    user::Entity::find_by_id(user_id)
        .one(dbconn())
        .await?
        .ok_or(AuthError::UserNotFound)
}

pub(crate) async fn user_role_ids(user_id: i32) -> Result<Vec<i32>, AuthError> {
    let rows = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .all(dbconn())
        .await?;
    Ok(rows.into_iter().map(|r| r.role_id).collect())
}

pub(crate) async fn user_roles(user_id: i32) -> Result<Vec<role::Model>, AuthError> {
    let role_ids = user_role_ids(user_id).await?;
    Ok(role::Entity::find()
        .filter(role::Column::Id.is_in(role_ids))
        .order_by(role::Column::Weight, Order::Asc)
        .all(dbconn())
        .await?)
}

// **< create_user >********************************************************************************

pub(crate) struct NewUserData<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub confirm_password: &'a str,
    pub display_name: Option<&'a str>,
    pub language: Option<&'a str>,
    pub timezone: Option<&'a str>,
    pub theme: Option<&'a str>,
    pub initial_role_ids: &'a [i32],
    /// El *caller* es responsable de comprobar que sólo un administrador puede pasar `true`.
    pub is_admin: bool,
}

/// Da de alta un usuario administrativamente. A diferencia de `auth::register`, el usuario queda
/// activo y con el email verificado de inmediato (lo crea un administrador de confianza), y admite
/// asignar roles iniciales.
pub(crate) async fn create_user(data: NewUserData<'_>) -> Result<i32, AuthError> {
    password::validate_strength(data.password)?;
    password::passwords_match(data.password, data.confirm_password)?;
    // El idioma y la zona horaria que no se aplican no se ofrecen en el formulario: se ignora lo
    // que pudiera llegar y el usuario se crea sin ellos.
    let language = if user_language_applies() {
        validate_language(data.language)?
    } else {
        None
    };
    let timezone = if user_timezone_applies() {
        validate_timezone(data.timezone)?
    } else {
        None
    };
    let theme = validate_theme(data.theme)?;
    ensure_username_available(data.username, None).await?;
    ensure_email_available(data.email, None).await?;

    let hash = password::hash_password(data.password)?;
    let now = Utc::now();

    let new_user = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: Set(data.username.to_owned()),
        email: Set(data.email.to_owned()),
        email_verified_at: Set(Some(now)),
        password_hash: Set(hash),
        status: Set(UserStatus::Active.as_i16()),
        language: Set(language.map(str::to_owned)),
        timezone: Set(timezone.map(str::to_owned)),
        theme: Set(theme.map(str::to_owned)),
        display_name: Set(data.display_name.map(str::to_owned)),
        about: Set(None),
        last_login_at: Set(None),
        last_access_at: Set(None),
        failed_login_count: Set(0),
        locked_until: Set(None),
        is_admin: Set(data.is_admin),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let result = user::Entity::insert(new_user).exec(dbconn()).await?;
    let user_id = result.last_insert_id;

    for role_id in data.initial_role_ids {
        crate::auth::assign_role(user_id, *role_id).await?;
    }

    Ok(user_id)
}

// **< update_user >********************************************************************************

/// Longitud máxima, en caracteres, del texto "Sobre mí". `u16` porque también limita el campo del
/// formulario (`maxlength`).
pub(crate) const ABOUT_MAX_CHARS: u16 = 2000;

pub(crate) struct UserUpdateData<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub display_name: Option<&'a str>,
    pub about: Option<&'a str>,
    pub language: Option<&'a str>,
    pub timezone: Option<&'a str>,
    pub theme: Option<&'a str>,
}

pub(crate) async fn update_user(user_id: i32, data: UserUpdateData<'_>) -> Result<(), AuthError> {
    // El idioma y la zona horaria que no se aplican tampoco se ofrecen en el formulario, así que no
    // llegan: se conserva lo guardado por si se vuelven a aplicar.
    let language = if user_language_applies() {
        Set(validate_language(data.language)?.map(str::to_owned))
    } else {
        ActiveValue::NotSet
    };
    let timezone = if user_timezone_applies() {
        let timezone = match validate_timezone(data.timezone) {
            Err(AuthError::InvalidTimezone) => {
                keep_current_timezone(user_id, data.timezone).await?
            }
            result => result?,
        };
        Set(timezone.map(str::to_owned))
    } else {
        ActiveValue::NotSet
    };
    let theme = validate_theme(data.theme)?;
    // El navegador envía los saltos de línea de un `<textarea>` como `\r\n`, pero `maxlength` puede
    // contarlos como un único carácter: se normalizan antes de medir para no rechazar un texto que
    // el formulario sí admitió.
    let about = data.about.map(|about| about.replace("\r\n", "\n"));
    let max_about = usize::from(ABOUT_MAX_CHARS);
    if about
        .as_deref()
        .is_some_and(|about| about.chars().count() > max_about)
    {
        return Err(AuthError::AboutTooLong(max_about));
    }
    ensure_username_available(data.username, Some(user_id)).await?;
    ensure_email_available(data.email, Some(user_id)).await?;

    let now = Utc::now();
    user::ActiveModel {
        id: Set(user_id),
        username: Set(data.username.to_owned()),
        email: Set(data.email.to_owned()),
        display_name: Set(data.display_name.map(str::to_owned)),
        about: Set(about),
        language,
        timezone,
        theme: Set(theme.map(str::to_owned)),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;
    Ok(())
}

// **< set_user_roles >*****************************************************************************

/// Reemplaza por completo el conjunto de roles asignados a un usuario.
///
/// "authenticated" ([`crate::AUTHENTICATED_ROLE_ID`]) nunca se almacena: es implícito para toda
/// cuenta autenticada, por lo que se descarta si llega en `role_ids`.
pub(crate) async fn set_user_roles(user_id: i32, role_ids: &[i32]) -> Result<(), AuthError> {
    find_user(user_id).await?;

    let mut role_ids: Vec<i32> = role_ids
        .iter()
        .copied()
        .filter(|id| *id != crate::AUTHENTICATED_ROLE_ID)
        .collect();
    role_ids.sort_unstable();
    role_ids.dedup();

    dbconn()
        .transaction::<_, _, AuthError>(|txn| {
            Box::pin(async move {
                user_role::Entity::delete_many()
                    .filter(user_role::Column::UserId.eq(user_id))
                    .exec(txn)
                    .await?;
                for role_id in role_ids {
                    user_role::Entity::insert(user_role::ActiveModel {
                        user_id: Set(user_id),
                        role_id: Set(role_id),
                    })
                    .exec(txn)
                    .await?;
                }
                Ok(())
            })
        })
        .await
        .map_err(flatten_txn_err)
}

// **< set_user_status >****************************************************************************

/// Cambia el estado de la cuenta. Rechaza que un usuario se bloquee a sí mismo o bloquee al último
/// administrador. Al bloquear, invalida todas las sesiones activas del usuario.
pub(crate) async fn set_user_status(
    user_id: i32,
    new_status: UserStatus,
    acting_user_id: i32,
) -> Result<(), AuthError> {
    find_user(user_id).await?;

    if new_status == UserStatus::Blocked {
        if user_id == acting_user_id {
            return Err(AuthError::CannotBlockSelf);
        }
        if is_last_administrator(user_id).await? {
            return Err(AuthError::LastAdministrator);
        }
    }

    let now = Utc::now();
    user::ActiveModel {
        id: Set(user_id),
        status: Set(new_status.as_i16()),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;

    if new_status == UserStatus::Blocked {
        session::destroy_user_sessions(user_id)
            .await
            .map_err(AuthError::Database)?;
    }

    Ok(())
}

// **< set_user_admin >*****************************************************************************

/// Concede o revoca el acceso irrestricto (`is_admin`). No es un permiso del catálogo: sólo un
/// administrador puede concederlo o revocarlo (el handler comprueba `account.is_admin()`
/// directamente, sin pasar por `require_permission`).
///
/// Rechaza que un administrador se automodifique el flag. No hace falta proteger aparte al
/// "último administrador": para llegar aquí quien actúa ya tiene que ser administrador, así que si
/// sólo queda uno, sólo él podría revocarse a sí mismo, y eso ya lo bloquea la comprobación
/// anterior.
pub(crate) async fn set_user_admin(
    user_id: i32,
    is_admin: bool,
    acting_user_id: i32,
) -> Result<(), AuthError> {
    find_user(user_id).await?;

    if user_id == acting_user_id {
        return Err(AuthError::CannotModifyOwnAdminFlag);
    }

    let now = Utc::now();
    user::ActiveModel {
        id: Set(user_id),
        is_admin: Set(is_admin),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;
    Ok(())
}

// **< admin_reset_password >***********************************************************************

/// Restablece la contraseña de un usuario como acción administrativa e invalida sus sesiones
/// activas.
pub(crate) async fn admin_reset_password(
    user_id: i32,
    new_password: &str,
) -> Result<(), AuthError> {
    find_user(user_id).await?;
    password::validate_strength(new_password)?;
    let hash = password::hash_password(new_password)?;

    let now = Utc::now();
    user::ActiveModel {
        id: Set(user_id),
        password_hash: Set(hash),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;

    session::destroy_user_sessions(user_id)
        .await
        .map_err(AuthError::Database)?;
    Ok(())
}

// **< HELPERS >************************************************************************************

// Devuelve el idioma sin espacios, tal como debe guardarse. Ha de ser uno de los identificadores
// que ofrece el selector (`Locale::supported_languages()`); uno ausente o en blanco es válido y
// devuelve `None`: equivale a usar el predeterminado de la aplicación.
fn validate_language(language: Option<&str>) -> Result<Option<&str>, AuthError> {
    let language = language.and_then(util::non_blank);
    if let Some(code) = language
        && !Locale::supported_languages()
            .iter()
            .any(|(langid, _)| langid.to_string() == code)
    {
        return Err(AuthError::InvalidLanguage);
    }
    Ok(language)
}

// Devuelve el nombre corto del tema tal como debe guardarse, el que declara el propio tema aunque
// llegue con otras mayúsculas. Ha de ser uno de los temas habilitados; uno ausente o en blanco es
// válido y devuelve `None`: equivale a usar el predeterminado de la aplicación.
fn validate_theme(theme: Option<&str>) -> Result<Option<&'static str>, AuthError> {
    match theme.and_then(util::non_blank) {
        Some(name) => theme_by_short_name(name)
            .map(|theme| Some(theme.short_name()))
            .ok_or(AuthError::InvalidTheme),
        None => Ok(None),
    }
}

// Devuelve la zona sin espacios, tal como debe guardarse. Ha de ser una de las que ofrece el
// selector (`Timezone::supported_by_region()`); una ausente o en blanco es válida y devuelve
// `None`: equivale a usar la predeterminada de la aplicación.
fn validate_timezone(timezone: Option<&str>) -> Result<Option<&str>, AuthError> {
    let timezone = timezone.and_then(util::non_blank);
    if let Some(tz) = timezone
        && !Timezone::supported_by_region()
            .iter()
            .any(|(_, names)| names.contains(&tz))
    {
        return Err(AuthError::InvalidTimezone);
    }
    Ok(timezone)
}

// Acepta una zona que ya no se ofrece si es la que el usuario tenía guardada: el selector la sigue
// mostrando para que volver a guardar el formulario sin tocarla no la descarte. Sólo se consulta la
// base de datos cuando la zona recibida no se ofrece.
async fn keep_current_timezone(
    user_id: i32,
    timezone: Option<&str>,
) -> Result<Option<&str>, AuthError> {
    let timezone = timezone.and_then(util::non_blank);
    let current = find_user(user_id).await?.timezone;
    if timezone.is_some() && timezone == current.as_deref() {
        Ok(timezone)
    } else {
        Err(AuthError::InvalidTimezone)
    }
}

async fn ensure_username_available(
    username: &str,
    exclude_id: Option<i32>,
) -> Result<(), AuthError> {
    let mut query = user::Entity::find().filter(user::Column::Username.eq(username));
    if let Some(id) = exclude_id {
        query = query.filter(user::Column::Id.ne(id));
    }
    if query.one(dbconn()).await?.is_some() {
        return Err(AuthError::UsernameTaken);
    }
    Ok(())
}

async fn ensure_email_available(email: &str, exclude_id: Option<i32>) -> Result<(), AuthError> {
    let mut query = user::Entity::find().filter(user::Column::Email.eq(email));
    if let Some(id) = exclude_id {
        query = query.filter(user::Column::Id.ne(id));
    }
    if query.one(dbconn()).await?.is_some() {
        return Err(AuthError::EmailTaken);
    }
    Ok(())
}

// Comprueba si `user_id` es actualmente el único usuario con `is_admin = true`.
async fn is_last_administrator(user_id: i32) -> Result<bool, AuthError> {
    let user = find_user(user_id).await?;
    if !user.is_admin {
        return Ok(false);
    }
    let admin_count = user::Entity::find()
        .filter(user::Column::IsAdmin.eq(true))
        .count(dbconn())
        .await?;
    Ok(admin_count <= 1)
}
