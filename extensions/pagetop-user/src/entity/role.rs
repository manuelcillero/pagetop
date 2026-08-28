use pagetop_seaorm::db::*;

use pagetop::datetime::NaiveDateTime;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub machine_name: String,
    pub label: String,
    pub description: Option<String>,
    pub weight: i32,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_permission::Entity")]
    RolePermissions,
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRoles,
}

impl Related<super::role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermissions.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
