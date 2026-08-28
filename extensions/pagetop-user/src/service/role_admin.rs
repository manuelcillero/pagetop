//! Servicio de administración de roles: listado, CRUD y permisos.

use std::collections::HashMap;

use pagetop::datetime::Utc;
use pagetop::html::SortDir;
use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, Order, Paginated, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait, dbconn, flatten_txn_err, paginate,
};

use crate::entity::{role, role_permission, user_role};
use crate::error::AuthError;
use crate::permission;

// **< listado >**************************************************************************************

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum RoleSortField {
    #[default]
    Weight,
    MachineName,
    Label,
}

impl RoleSortField {
    pub(crate) fn from_query(s: Option<&str>) -> Self {
        match s {
            Some("machine_name") => RoleSortField::MachineName,
            Some("label") => RoleSortField::Label,
            _ => RoleSortField::Weight,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            RoleSortField::Weight => "weight",
            RoleSortField::MachineName => "machine_name",
            RoleSortField::Label => "label",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RoleListItem {
    pub id: i32,
    pub machine_name: String,
    pub label: String,
    pub locked: bool,
    pub user_count: u64,
}

pub(crate) struct RoleListParams {
    pub sort: RoleSortField,
    pub dir: SortDir,
}

/// Devuelve todos los roles ordenados, sin paginar. Usado donde hace falta el catálogo completo
/// (p. ej. la lista de roles asignables en la administración de usuarios).
pub(crate) async fn list_roles(params: &RoleListParams) -> Result<Vec<RoleListItem>, AuthError> {
    let order = if params.dir == SortDir::Desc {
        Order::Desc
    } else {
        Order::Asc
    };
    let select = role::Entity::find();
    let select = match params.sort {
        RoleSortField::Weight => select.order_by(role::Column::Weight, order),
        RoleSortField::MachineName => select.order_by(role::Column::MachineName, order),
        RoleSortField::Label => select.order_by(role::Column::Label, order),
    };
    let roles = select.all(dbconn()).await?;
    role_items(roles).await
}

pub(crate) struct RolePageParams {
    pub sort: RoleSortField,
    pub dir: SortDir,
    pub page: u64,
    pub per_page: u64,
}

/// Devuelve una página de roles. Usado por el listado de administración de roles.
pub(crate) async fn list_roles_page(
    params: &RolePageParams,
) -> Result<Paginated<RoleListItem>, AuthError> {
    let order = if params.dir == SortDir::Desc {
        Order::Desc
    } else {
        Order::Asc
    };
    let select = role::Entity::find();
    let select = match params.sort {
        RoleSortField::Weight => select.order_by(role::Column::Weight, order),
        RoleSortField::MachineName => select.order_by(role::Column::MachineName, order),
        RoleSortField::Label => select.order_by(role::Column::Label, order),
    };

    paginate(select, params.page, params.per_page)
        .await?
        .map_items(role_items)
        .await
}

async fn role_items(roles: Vec<role::Model>) -> Result<Vec<RoleListItem>, AuthError> {
    let role_ids: Vec<i32> = roles.iter().map(|r| r.id).collect();
    let counts: Vec<(i32, i64)> = user_role::Entity::find()
        .filter(user_role::Column::RoleId.is_in(role_ids))
        .select_only()
        .column(user_role::Column::RoleId)
        .column_as(user_role::Column::RoleId.count(), "count")
        .group_by(user_role::Column::RoleId)
        .into_tuple()
        .all(dbconn())
        .await?;
    let counts_by_role: HashMap<i32, u64> = counts
        .into_iter()
        .map(|(role_id, count)| (role_id, count as u64))
        .collect();

    Ok(roles
        .into_iter()
        .map(|role| RoleListItem {
            user_count: counts_by_role.get(&role.id).copied().unwrap_or(0),
            id: role.id,
            machine_name: role.machine_name,
            label: role.label,
            locked: role.locked,
        })
        .collect())
}

// **< find_role / role_permission_keys >***************************************************************

pub(crate) async fn find_role(role_id: i32) -> Result<role::Model, AuthError> {
    role::Entity::find_by_id(role_id)
        .one(dbconn())
        .await?
        .ok_or(AuthError::RoleNotFound)
}

pub(crate) async fn role_permission_keys(role_id: i32) -> Result<Vec<String>, AuthError> {
    let rows = role_permission::Entity::find()
        .filter(role_permission::Column::RoleId.eq(role_id))
        .all(dbconn())
        .await?;
    Ok(rows.into_iter().map(|r| r.permission_key).collect())
}

// **< create_role >*********************************************************************************

pub(crate) struct NewRoleData<'a> {
    pub machine_name: &'a str,
    pub label: &'a str,
    pub description: Option<&'a str>,
    pub weight: i32,
}

// Sólo minúsculas ASCII, dígitos y guiones bajos (ver el texto de ayuda del formulario,
// "help-machine-name-immutable"); el machine_name es inmutable tras la creación.
fn is_valid_machine_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

pub(crate) async fn create_role(data: NewRoleData<'_>) -> Result<i32, AuthError> {
    if !is_valid_machine_name(data.machine_name) {
        return Err(AuthError::InvalidMachineName);
    }

    if role::Entity::find()
        .filter(role::Column::MachineName.eq(data.machine_name))
        .one(dbconn())
        .await?
        .is_some()
    {
        return Err(AuthError::RoleMachineNameTaken);
    }

    let now = Utc::now().naive_utc();
    let new_role = role::ActiveModel {
        id: ActiveValue::NotSet,
        machine_name: Set(data.machine_name.to_owned()),
        label: Set(data.label.to_owned()),
        description: Set(data.description.map(str::to_owned)),
        weight: Set(data.weight),
        locked: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let result = role::Entity::insert(new_role).exec(dbconn()).await?;
    Ok(result.last_insert_id)
}

// **< update_role >*********************************************************************************

pub(crate) struct RoleUpdateData<'a> {
    pub label: &'a str,
    pub description: Option<&'a str>,
    pub weight: i32,
}

pub(crate) async fn update_role(role_id: i32, data: RoleUpdateData<'_>) -> Result<(), AuthError> {
    let role = find_role(role_id).await?;
    if role.locked {
        return Err(AuthError::RoleLocked);
    }

    let now = Utc::now().naive_utc();
    role::ActiveModel {
        id: Set(role_id),
        label: Set(data.label.to_owned()),
        description: Set(data.description.map(str::to_owned)),
        weight: Set(data.weight),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;
    Ok(())
}

// **< delete_role >*********************************************************************************

pub(crate) async fn delete_role(role_id: i32) -> Result<(), AuthError> {
    let role = find_role(role_id).await?;
    if role.locked {
        return Err(AuthError::RoleLocked);
    }

    let user_count = user_role::Entity::find()
        .filter(user_role::Column::RoleId.eq(role_id))
        .count(dbconn())
        .await?;
    if user_count > 0 {
        return Err(AuthError::RoleInUse);
    }

    role::Entity::delete_by_id(role_id).exec(dbconn()).await?;
    Ok(())
}

// **< set_role_permissions >************************************************************************

/// Reemplaza por completo el conjunto de permisos concedidos a un rol. Permitido aunque el rol
/// esté bloqueado (`locked`): los roles de sistema también necesitan permisos gestionables.
pub(crate) async fn set_role_permissions(
    role_id: i32,
    permission_keys: &[String],
) -> Result<(), AuthError> {
    find_role(role_id).await?;

    let registry = permission::registry();
    for key in permission_keys {
        if !registry.has_key(key) {
            return Err(AuthError::UnknownPermission(key.clone()));
        }
    }

    let now = Utc::now().naive_utc();
    let keys = permission_keys.to_vec();
    dbconn()
        .transaction::<_, _, AuthError>(|txn| {
            Box::pin(async move {
                role_permission::Entity::delete_many()
                    .filter(role_permission::Column::RoleId.eq(role_id))
                    .exec(txn)
                    .await?;
                for key in keys {
                    role_permission::Entity::insert(role_permission::ActiveModel {
                        role_id: Set(role_id),
                        permission_key: Set(key),
                        granted_at: Set(now),
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
