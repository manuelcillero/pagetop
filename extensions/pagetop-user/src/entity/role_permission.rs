use pagetop_seaorm::db::*;

use pagetop::datetime::NaiveDateTime;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "role_permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_key: String,
    pub granted_at: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_delete = "Cascade"
    )]
    Role,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
