use pagetop::db::entity::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid     : u32,
    #[sea_orm(unique)]
    pub name    : String,
    pub pass    : String,
    pub mail    : Option<String>,
    pub created : DateTimeUtc,
    pub changed : DateTimeUtc,
    pub access  : DateTimeUtc,
    pub login   : DateTimeUtc,
    pub status  : bool,
    pub timezone: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserRole,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserRole => Entity::has_many(super::user_role::Entity).into(),
        }
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
