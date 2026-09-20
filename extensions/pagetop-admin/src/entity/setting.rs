use pagetop::prelude::*;
use pagetop_seaorm::db::*;

/// Entidad SeaORM para la tabla `settings`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    pub scope: String,
    pub value: String,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i32>,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
