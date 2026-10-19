use pagetop_seaorm::db::*;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// Nombre de usuario con sus mayúsculas, normalizado (NFKC) y sin espacios en los extremos.
    pub username: String,
    /// Email con sus mayúsculas, sin espacios en los extremos.
    pub email: String,
    /// Nombre de usuario para buscar sin distinguir mayúsculas (ver `identity.rs`).
    #[sea_orm(unique)]
    pub username_key: String,
    /// *Hash* SHA-256 del esqueleto de confusión del nombre de usuario, único para que no haya dos
    /// que se confundan.
    #[sea_orm(unique)]
    pub username_skeleton: String,
    /// Email para buscar sin distinguir mayúsculas.
    #[sea_orm(unique)]
    pub email_key: String,
    pub email_verified_at: Option<DateTimeUtc>,
    pub password_hash: String,
    pub status: i16,
    pub language: Option<String>,
    pub timezone: Option<String>,
    /// Nombre corto del tema preferido (p. ej. `"Bootsier"`), si tiene uno.
    pub theme: Option<String>,
    pub display_name: Option<String>,
    /// Texto libre "Sobre mí" que el propio usuario escribe en su perfil.
    pub about: Option<String>,
    pub last_login_at: Option<DateTimeUtc>,
    /// Último acceso: al iniciar sesión y al prorrogar la sesión, con la precisión de
    /// `user.session.write_interval_secs`.
    pub last_access_at: Option<DateTimeUtc>,
    /// Intentos fallidos seguidos dentro de la ventana de recuento (`failed_login_window_secs`).
    pub failed_login_count: i32,
    /// Último intento fallido, para saber si el siguiente cae dentro de la misma ventana.
    pub failed_login_at: Option<DateTimeUtc>,
    pub locked_until: Option<DateTimeUtc>,
    /// Acceso irrestricto al sistema, sin pasar por roles ni permisos.
    pub is_admin: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
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
