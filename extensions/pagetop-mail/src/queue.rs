// Cola de envío persistente (*feature* `queue`).
//
// Los correos se guardan ya compuestos en la tabla `mail_queue`, y el trabajo `mail.queue` de
// `pagetop-cron` los envía por lotes. Una fila enviada se borra; una fallida se conserva con su
// último error. Para que varias instancias de la aplicación no envíen el mismo correo, cada una
// reserva la fila (`locked_until`) con una sentencia condicionada a que nadie la tenga reservada.

use pagetop::prelude::*;
use pagetop_cron::prelude::*;
use pagetop_seaorm::db::query::Expr;
use pagetop_seaorm::db::{
    ColumnTrait, Condition, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, dbconn,
};
use pagetop_seaorm::install_migrations;

use crate::transport::{self, Composed};
use crate::{Email, LOCALES_MAIL, MailError, config, outbox};

use lettre::Address;
use lettre::address::Envelope;

mod entity;
mod migration;

use entity::{ActiveModel, Column, Entity, Model};

const JOB_KEY: &str = "mail.queue";

// Espera máxima admitida en `mail.queue.max_retry_secs`: un año.
const MAX_RETRY_SECS: u64 = 31_536_000;

// Comprueba los ajustes de `[mail.queue]` que tienen un rango limitado.
pub(crate) fn check_settings() -> Result<(), MailError> {
    let queue = &config::SETTINGS.queue;
    transport::check_range(
        "mail.queue.batch_size",
        queue.batch_size.into(),
        1,
        u32::MAX.into(),
    )?;
    transport::check_range(
        "mail.queue.max_retry_secs",
        queue.max_retry_secs,
        1,
        MAX_RETRY_SECS,
    )
}

pub(crate) async fn install() -> Result<(), BoxError> {
    install_migrations!(m20261028_000001_create_mail_queue)?;
    Ok(())
}

pub(crate) fn declare_jobs(jobs: &mut CronJobs) {
    jobs.add(
        Job::new(
            JOB_KEY,
            Schedule::fixed_delay(Duration::minutes(1)),
            process,
        )
        .with_title(Lc::t("queue_job_title", &LOCALES_MAIL)),
    );
}

// `Email::queue()`: guarda el correo para el siguiente lote.
pub(crate) async fn push(email: Email) -> Result<(), MailError> {
    let mailer = transport::mailer()?;
    if mailer.is_log() {
        return transport::log(mailer, email);
    }
    store(mailer, transport::compose(mailer, email)?).await
}

// `Email::send_later()`: guarda el correo ya reservado y lo intenta enseguida. Los errores de la
// entrega se anotan en la cola y en las trazas, y sólo devuelve los anteriores, salvo si no puede
// guardarlo: entonces lo entrega sin la cola y devuelve también los de la entrega.
pub(crate) async fn send_later(email: Email) -> Result<(), MailError> {
    let mailer = transport::mailer()?;
    if mailer.is_log() {
        return transport::log(mailer, email);
    }
    let composed = transport::compose(mailer, email)?;
    // Sin plaza para entregarlo (demasiados correos esperando turno), queda para el siguiente lote
    // en lugar de esperar en memoria.
    let Ok(ticket) = mailer.ticket() else {
        trace::warn!(
            "Too many emails waiting to be delivered, email \"{}\" left in the mail queue",
            composed.email.subject()
        );
        return store(mailer, composed).await;
    };
    let Composed {
        email,
        envelope,
        raw,
    } = composed;
    let subject = email.subject().to_owned();
    let id = match insert(&envelope, &subject, raw.clone(), true).await {
        Ok(id) => id,
        Err(err) => {
            trace::warn!(
                "Cannot store email \"{subject}\" in the mail queue, sending it without the queue: \
                 {err}"
            );
            return transport::deliver_raw(mailer, ticket, &envelope, raw, Some(email)).await;
        }
    };
    let result = transport::deliver_raw(mailer, ticket, &envelope, raw, Some(email)).await;
    finish(id, &subject, 1, result).await;
    Ok(())
}

// Guarda el correo compuesto para el siguiente lote.
async fn store(mailer: &transport::Mailer, composed: Composed) -> Result<(), MailError> {
    let Composed {
        email,
        envelope,
        raw,
    } = composed;
    let id = insert(&envelope, email.subject(), raw, false)
        .await
        .map_err(MailError::Queue)?;
    if mailer.is_memory() {
        outbox::hold(id, email);
    }
    Ok(())
}

// Ejecución del trabajo `mail.queue`: envía, uno tras otro, hasta `batch_size` correos pendientes.
// Un fallo del transporte (servidor que no responde...) interrumpe el lote: los demás correos
// fallarían igual tras esperar cada uno su tiempo máximo, así que esperan a la siguiente ejecución
// sin gastar intentos. También lo interrumpe que no haya plaza para entregar (demasiados correos de
// `send_later()` esperando turno), antes de reservar la fila.
async fn process(cx: JobContext) -> JobResult {
    let mailer = transport::mailer()?;

    give_up_exhausted().await?;

    // Sólo los identificadores: cada mensaje se lee al reservarlo, para no tener en memoria a la
    // vez los de todo el lote, que pueden llevar adjuntos.
    let now = Utc::now();
    let pending: Vec<(i32, i32)> = Entity::find()
        .select_only()
        .column(Column::Id)
        .column(Column::Attempts)
        .filter(Column::FailedAt.is_null())
        .filter(Column::Attempts.lt(max_attempts()))
        .filter(Column::NextAttemptAt.lte(now))
        .filter(unlocked(now))
        .order_by_asc(Column::NextAttemptAt)
        .order_by_asc(Column::Id)
        .limit(u64::from(config::SETTINGS.queue.batch_size))
        .into_tuple()
        .all(dbconn())
        .await?;

    for (id, attempts) in pending {
        if cx.is_stopping() {
            break;
        }
        let Ok(ticket) = mailer.ticket() else {
            trace::warn!("Mail queue batch stopped: too many emails waiting to be delivered");
            break;
        };
        if !claim(id, attempts).await? {
            continue;
        }
        let Some(row) = Entity::find_by_id(id).one(dbconn()).await? else {
            continue;
        };
        let email = outbox::release(id);
        let result = match envelope(&row) {
            Ok(envelope) => {
                transport::deliver_raw(mailer, ticket, &envelope, row.message, email).await
            }
            Err(err) => Err(err),
        };
        let halt = matches!(&result, Err(err) if err.is_transport_failure());
        finish(id, &row.subject, row.attempts, result).await;
        if halt {
            trace::warn!("Mail queue batch stopped: the transport is failing");
            break;
        }
    }

    purge_failed().await?;
    Ok(())
}

// Guarda el correo compuesto y devuelve su identificador. Con `locked`, ya reservado y con el
// primer intento contado, para enviarlo enseguida sin que lo tome un lote.
async fn insert(
    envelope: &Envelope,
    subject: &str,
    message: Vec<u8>,
    locked: bool,
) -> Result<i32, DbErr> {
    let now = Utc::now();
    let mut envelope_to = String::new();
    for address in envelope.to() {
        if !envelope_to.is_empty() {
            envelope_to.push('\n');
        }
        envelope_to.push_str(address.as_ref());
    }
    let row = ActiveModel {
        envelope_from: Set(envelope.from().map(ToString::to_string)),
        envelope_to: Set(envelope_to),
        subject: Set(subject.to_owned()),
        message: Set(message),
        attempts: Set(i32::from(locked)),
        next_attempt_at: Set(now),
        locked_until: Set(locked.then(|| lock_until(now))),
        last_error: Set(None),
        failed_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    // Con `Entity::insert()` y no `ActiveModel::insert()`, que leería de vuelta la fila entera,
    // mensaje incluido.
    Ok(Entity::insert(row).exec(dbconn()).await?.last_insert_id)
}

// Reserva la fila y cuenta el intento, sólo si sigue igual que al leerla: si otro proceso se
// adelanta, no afecta a ninguna fila y el correo se deja para él.
async fn claim(id: i32, attempts: i32) -> Result<bool, DbErr> {
    let now = Utc::now();
    let result = Entity::update_many()
        .col_expr(Column::LockedUntil, Expr::value(lock_until(now)))
        .col_expr(Column::Attempts, Expr::value(attempts + 1))
        .col_expr(Column::UpdatedAt, Expr::value(now))
        .filter(Column::Id.eq(id))
        .filter(Column::Attempts.eq(attempts))
        .filter(Column::FailedAt.is_null())
        .filter(unlocked(now))
        .exec(dbconn())
        .await?;
    Ok(result.rows_affected == 1)
}

// Anota el resultado del intento número `attempts`: borra el correo enviado, o lo deja para
// reintentarlo más tarde o como fallido. Si no se puede anotar, el correo enviado se volverá a
// enviar al caducar la reserva.
async fn finish(id: i32, subject: &str, attempts: i32, result: Result<(), MailError>) {
    let update = match result {
        Ok(()) => Entity::delete_by_id(id).exec(dbconn()).await.map(|_| ()),
        Err(err) => {
            let now = Utc::now();
            let message = err.describe();
            let mut update = Entity::update_many()
                .col_expr(Column::LockedUntil, Expr::value(None::<DateTime<Utc>>))
                .col_expr(Column::UpdatedAt, Expr::value(now));
            if err.is_permanent() || attempts >= max_attempts() {
                trace::error!(
                    "Cannot send email \"{subject}\" (attempt {attempts}), giving up: {message}"
                );
                update = update.col_expr(Column::FailedAt, Expr::value(now));
            } else {
                let queue = &config::SETTINGS.queue;
                let retry_at =
                    now + retry_delay(queue.retry_base_secs, queue.max_retry_secs, attempts);
                trace::warn!(
                    "Cannot send email \"{subject}\" (attempt {attempts}), retrying at \
                     {retry_at}: {message}"
                );
                update = update.col_expr(Column::NextAttemptAt, Expr::value(retry_at));
            }
            // Si el envío tardó más que la reserva, otro proceso puede haber tomado ya la fila y
            // contado otro intento: entonces el resultado es suyo y éste se descarta.
            update
                .col_expr(Column::LastError, Expr::value(message))
                .filter(Column::Id.eq(id))
                .filter(Column::Attempts.eq(attempts))
                .exec(dbconn())
                .await
                .map(|_| ())
        }
    };
    if let Err(err) = update {
        trace::error!(
            "Cannot update email \"{subject}\" in the mail queue, it may be sent again: {err}"
        );
    }
}

// Da por fallidos los correos sin intentos pendientes que no están reservados: el último intento se
// interrumpió (el proceso cayó a mitad de envío) o se redujo `mail.queue.max_attempts`.
async fn give_up_exhausted() -> Result<(), DbErr> {
    let now = Utc::now();
    Entity::update_many()
        .col_expr(Column::FailedAt, Expr::value(now))
        .col_expr(Column::LockedUntil, Expr::value(None::<DateTime<Utc>>))
        .col_expr(Column::LastError, Expr::value("no attempts left"))
        .col_expr(Column::UpdatedAt, Expr::value(now))
        .filter(Column::FailedAt.is_null())
        .filter(Column::Attempts.gte(max_attempts()))
        .filter(unlocked(now))
        .exec(dbconn())
        .await?;
    Ok(())
}

// Borra los correos fallidos más antiguos que `mail.queue.keep_failed_days`. Con 0, o con tantos
// días que la fecha límite no se puede representar, no borra ninguno.
async fn purge_failed() -> Result<(), DbErr> {
    let days = config::SETTINGS.queue.keep_failed_days;
    if days == 0 {
        return Ok(());
    }
    let Some(before) = Utc::now().checked_sub_signed(Duration::days(i64::from(days))) else {
        return Ok(());
    };
    Entity::delete_many()
        .filter(Column::FailedAt.lt(before))
        .exec(dbconn())
        .await?;
    Ok(())
}

// Filas sin reservar o con la reserva caducada.
fn unlocked(now: DateTime<Utc>) -> Condition {
    Condition::any()
        .add(Column::LockedUntil.is_null())
        .add(Column::LockedUntil.lt(now))
}

// Fin de la reserva de una fila mientras se envía; pasado, si el proceso cayó a mitad de envío,
// otro puede volver a enviarla. Dura el triple del tiempo máximo de una entrega (15 minutos por
// defecto), para que nadie tome la fila mientras su envío sigue en curso: en SMTP, al agotarse ese
// tiempo sólo se deja de esperar, y el envío puede seguir hasta fallar por su cuenta.
fn lock_until(now: DateTime<Utc>) -> DateTime<Utc> {
    let secs = config::SETTINGS.delivery_timeout_secs.saturating_mul(3);
    now + Duration::seconds(i64::try_from(secs).unwrap_or(i64::MAX))
}

fn max_attempts() -> i32 {
    i32::try_from(config::SETTINGS.queue.max_attempts.max(1)).unwrap_or(i32::MAX)
}

// Espera antes del siguiente intento tras `attempts` intentos fallidos: `base_secs`, el doble en
// cada uno, hasta `max_secs` y nunca más de un año.
fn retry_delay(base_secs: u64, max_secs: u64, attempts: i32) -> Duration {
    let doublings = u32::try_from(attempts.saturating_sub(1))
        .unwrap_or(0)
        .min(32);
    let secs = base_secs
        .saturating_mul(1 << doublings)
        .min(max_secs.min(MAX_RETRY_SECS));
    Duration::seconds(i64::try_from(secs).unwrap_or(i64::MAX))
}

// Reconstruye el sobre guardado.
fn envelope(row: &Model) -> Result<Envelope, MailError> {
    let from = row.envelope_from.as_deref().map(address).transpose()?;
    let to = row
        .envelope_to
        .lines()
        .map(address)
        .collect::<Result<Vec<_>, _>>()?;
    Envelope::new(from, to).map_err(MailError::Build)
}

fn address(value: &str) -> Result<Address, MailError> {
    value.parse().map_err(|source| MailError::InvalidAddress {
        address: value.to_owned(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::{Duration, MailError, address, retry_delay};

    #[test]
    fn transport_failures_stop_the_batch_but_rejected_emails_do_not() {
        assert!(MailError::Timeout(300).is_transport_failure());
        assert!(MailError::Interrupted.is_transport_failure());
        assert!(MailError::Overloaded.is_transport_failure());

        let invalid = address("not an address").unwrap_err();
        assert!(invalid.is_permanent());
        assert!(!invalid.is_transport_failure());
    }

    #[test]
    fn retry_delay_doubles_after_each_attempt_up_to_the_maximum() {
        const DAY: u64 = 86_400;
        assert_eq!(retry_delay(60, DAY, 1), Duration::seconds(60));
        assert_eq!(retry_delay(60, DAY, 2), Duration::seconds(120));
        assert_eq!(retry_delay(60, DAY, 4), Duration::seconds(480));
        assert_eq!(retry_delay(60, DAY, 20), Duration::days(1));
        assert_eq!(retry_delay(u64::MAX, DAY, 1000), Duration::days(1));
        assert_eq!(retry_delay(60, DAY, 0), Duration::seconds(60));
        assert_eq!(retry_delay(60, 300, 4), Duration::seconds(300));
        assert_eq!(retry_delay(u64::MAX, u64::MAX, 1), Duration::days(365));
    }
}
