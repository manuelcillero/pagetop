use pagetop::db::entity::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rid   : u32,
    #[sea_orm(unique)]
    pub name  : String,
    #[sea_orm(default_value = 0)]
    pub weight: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RolePermission,
    UserRole,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RolePermission => Entity::has_many(super::role_permission::Entity).into(),
            Self::UserRole => Entity::has_many(super::user_role::Entity).into(),
        }
    }
}

impl Related<super::role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermission.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
