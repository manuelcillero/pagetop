// Entidad de la tabla `mail_queue`: un correo pendiente de entregar o fallido.

use pagetop_seaorm::db::*;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "mail_queue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub envelope_from: Option<String>,
    // Destinatarios del sobre, uno por línea.
    pub envelope_to: String,
    pub subject: String,
    // Mensaje compuesto en formato RFC 5322.
    pub message: Vec<u8>,
    pub attempts: i32,
    pub next_attempt_at: DateTimeUtc,
    // Reserva de quien lo está enviando; pasada la fecha, otro proceso puede volver a enviarlo.
    pub locked_until: Option<DateTimeUtc>,
    pub last_error: Option<String>,
    pub failed_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
