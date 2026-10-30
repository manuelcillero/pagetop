use pagetop_seaorm::migration::*;

use sea_orm::DbBackend;

pub struct Migration;

#[pagetop::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // En MySQL, `BLOB` admite sólo 64 KB, menos que muchos correos con adjuntos.
        let message = if manager.get_database_backend() == DbBackend::MySql {
            ColumnDef::new(MailQueue::Message)
                .custom(Alias::new("longblob"))
                .not_null()
                .take()
        } else {
            blob(MailQueue::Message)
        };
        manager
            .create_table(
                table_auto(MailQueue::Table)
                    .col(pk_auto(MailQueue::Id))
                    .col(string_null(MailQueue::EnvelopeFrom))
                    .col(text(MailQueue::EnvelopeTo))
                    .col(text(MailQueue::Subject))
                    .col(message)
                    .col(integer(MailQueue::Attempts).default(0))
                    .col(timestamp(MailQueue::NextAttemptAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(MailQueue::LockedUntil))
                    .col(text_null(MailQueue::LastError))
                    .col(timestamp_null(MailQueue::FailedAt))
                    .to_owned(),
            )
            .await?;
        // Cada lote busca los correos cuyo próximo intento ya ha llegado.
        manager
            .create_index(
                Index::create()
                    .name("idx_mail_queue_next_attempt_at")
                    .table(MailQueue::Table)
                    .col(MailQueue::NextAttemptAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MailQueue::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MailQueue {
    Table,
    Id,
    EnvelopeFrom,
    EnvelopeTo,
    Subject,
    Message,
    Attempts,
    NextAttemptAt,
    LockedUntil,
    LastError,
    FailedAt,
}
