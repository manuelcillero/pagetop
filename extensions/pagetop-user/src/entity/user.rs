use pagetop_seaorm::db::*;

use pagetop::datetime::NaiveDateTime;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password_hash: String,
    pub status: i16,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub display_name: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
    pub last_access_at: Option<NaiveDateTime>,
    pub failed_login_count: i32,
    pub locked_until: Option<NaiveDateTime>,
    /// Acceso irrestricto al sistema, sin pasar por roles ni permisos.
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRoles,
    #[sea_orm(has_many = "super::session::Entity")]
    Sessions,
    #[sea_orm(has_many = "super::user_token::Entity")]
    UserTokens,
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl Related<super::user_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTokens.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
