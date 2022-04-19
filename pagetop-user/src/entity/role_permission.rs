use pagetop::db::entity::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rid       : u32,
    #[sea_orm(primary_key)]
    pub permission: String,
    pub module    : String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::Rid)
                .to(super::role::Column::Rid)
                .into(),
        }
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
