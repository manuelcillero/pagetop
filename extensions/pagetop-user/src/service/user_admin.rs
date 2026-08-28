//! Servicio de administración de usuarios: listado, CRUD, roles y estado.

use std::collections::HashMap;

use pagetop::datetime::Utc;
use pagetop::html::SortDir;
use pagetop::util;
use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, Order, Paginated,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait, dbconn, flatten_txn_err,
    paginate,
};

use crate::account::UserStatus;
use crate::entity::{role, user, user_role};
use crate::error::AuthError;
use crate::password;
use crate::session;

// **< listado >**************************************************************************************

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

// **< find_user / user_role_ids >**********************************************************************

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

// **< create_user >*********************************************************************************

pub(crate) struct NewUserData<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub confirm_password: &'a str,
    pub display_name: Option<&'a str>,
    pub language: Option<&'a str>,
    pub timezone: Option<&'a str>,
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
    ensure_username_available(data.username, None).await?;
    ensure_email_available(data.email, None).await?;

    let hash = password::hash_password(data.password)?;
    let now = Utc::now().naive_utc();

    let new_user = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: Set(data.username.to_owned()),
        email: Set(data.email.to_owned()),
        email_verified_at: Set(Some(now)),
        password_hash: Set(hash),
        status: Set(UserStatus::Active.as_i16()),
        language: Set(data.language.map(str::to_owned)),
        timezone: Set(data.timezone.map(str::to_owned)),
        display_name: Set(data.display_name.map(str::to_owned)),
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

    crate::auth::assign_role(user_id, crate::AUTHENTICATED_ROLE_ID).await?;
    for role_id in data.initial_role_ids {
        crate::auth::assign_role(user_id, *role_id).await?;
    }

    Ok(user_id)
}

// **< update_user >*********************************************************************************

pub(crate) struct UserUpdateData<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub display_name: Option<&'a str>,
    pub language: Option<&'a str>,
    pub timezone: Option<&'a str>,
}

pub(crate) async fn update_user(user_id: i32, data: UserUpdateData<'_>) -> Result<(), AuthError> {
    ensure_username_available(data.username, Some(user_id)).await?;
    ensure_email_available(data.email, Some(user_id)).await?;

    let now = Utc::now().naive_utc();
    user::ActiveModel {
        id: Set(user_id),
        username: Set(data.username.to_owned()),
        email: Set(data.email.to_owned()),
        display_name: Set(data.display_name.map(str::to_owned)),
        language: Set(data.language.map(str::to_owned)),
        timezone: Set(data.timezone.map(str::to_owned)),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;
    Ok(())
}

// **< set_user_roles >******************************************************************************

/// Reemplaza por completo el conjunto de roles asignados a un usuario.
///
/// "authenticated" ([`crate::AUTHENTICATED_ROLE_ID`]) se reintroduce siempre, esté o no en
/// `role_ids`: la UI no lo ofrece como casilla (ver `available_roles()`), pero toda cuenta
/// activa lo tiene concedido por definición y debe seguir apareciendo en `Account.roles`.
pub(crate) async fn set_user_roles(user_id: i32, role_ids: &[i32]) -> Result<(), AuthError> {
    find_user(user_id).await?;

    let mut role_ids: Vec<i32> = role_ids.to_vec();
    role_ids.push(crate::AUTHENTICATED_ROLE_ID);
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

// **< set_user_status >*****************************************************************************

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

    let now = Utc::now().naive_utc();
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

// **< set_user_admin >******************************************************************************

/// Concede o revoca el acceso irrestricto (`is_admin`). No es un permiso del catálogo: sólo un
/// administrador puede concederlo o revocarlo (el handler comprueba `account.is_admin`
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

    let now = Utc::now().naive_utc();
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

// **< admin_reset_password >************************************************************************

/// Restablece la contraseña de un usuario como acción administrativa e invalida sus sesiones
/// activas.
pub(crate) async fn admin_reset_password(
    user_id: i32,
    new_password: &str,
) -> Result<(), AuthError> {
    find_user(user_id).await?;
    password::validate_strength(new_password)?;
    let hash = password::hash_password(new_password)?;

    let now = Utc::now().naive_utc();
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

// **< helpers privados >****************************************************************************

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
