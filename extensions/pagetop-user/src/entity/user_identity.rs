use pagetop_seaorm::db::*;

/// Vínculo entre una cuenta y la identidad de una persona en un proveedor de autenticación externo,
/// o marca de que la cuenta puede vincularse con él (`pending`).
#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "user_identities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub provider: String,
    pub external_id: Option<String>,
    pub external_key: Option<String>,
    pub pending: bool,
    pub expected_key: Option<String>,
    pub data: Option<String>,
    pub created_at: DateTimeUtc,
    pub linked_at: Option<DateTimeUtc>,
    pub last_login_at: Option<DateTimeUtc>,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
