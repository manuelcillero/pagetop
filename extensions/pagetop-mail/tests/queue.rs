// Needs the `queue` feature. `config/common.toml` gives these tests an in-memory SQLite database
// with a single connection, the memory transport and `batch_size = 2`. The scheduler never starts
// in tests, so the queue only runs through `run_now()`.
//
// Every scenario shares the same queue table and batches pick the oldest emails first, so they run
// one after another in a single test (and a single runtime, which also keeps the database
// connection alive).

use pagetop::prelude::*;
use pagetop_mail::outbox;
use pagetop_mail::prelude::*;
use pagetop_seaorm::db::sea_orm::Statement;
use pagetop_seaorm::db::{ConnectionTrait, dbconn, execute};

const JOB: &str = "mail.queue";

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_mail::Mail]
    }
}

fn email(to: &str, subject: &str) -> Email {
    Email::new()
        .with_to(to)
        .with_subject(subject)
        .with_text("Queued email")
}

async fn run_job() {
    let run = pagetop_cron::run_now(JOB).await.unwrap();
    assert!(
        run.outcome().is_success(),
        "{JOB} failed: {:?}",
        run.outcome()
    );
}

async fn count(condition: &str) -> i64 {
    let conn = dbconn();
    let sql = format!("SELECT COUNT(*) AS n FROM mail_queue WHERE {condition}");
    let row = conn
        .query_one(Statement::from_string(conn.get_database_backend(), sql))
        .await
        .unwrap()
        .unwrap();
    row.try_get("", "n").unwrap()
}

async fn update(set: &str, subject: &str) {
    execute(format!(
        "UPDATE mail_queue SET {set} WHERE subject = '{subject}'"
    ))
    .await
    .unwrap();
}

fn subjects(to: &str) -> Vec<String> {
    outbox::take_to(to)
        .iter()
        .map(|sent| sent.email().subject().to_owned())
        .collect()
}

#[pagetop::test]
async fn queue_sends_in_batches_and_gives_up() {
    Application::prepare(&TestApp).await;

    queued_emails_wait_for_the_job_and_go_in_batches().await;
    send_later_delivers_at_once_and_leaves_nothing_queued().await;
    reserved_emails_wait_until_the_reservation_expires().await;
    permanent_errors_fail_at_the_first_attempt().await;
    emails_without_attempts_left_fail_and_are_purged_later().await;
}

async fn queued_emails_wait_for_the_job_and_go_in_batches() {
    let to = "batch@example.com";
    for n in 1..=3 {
        email(to, &format!("Batch {n}")).queue().await.unwrap();
    }
    assert!(
        subjects(to).is_empty(),
        "queued emails must wait for the job"
    );
    assert_eq!(count("subject LIKE 'Batch %'").await, 3);

    run_job().await;
    assert_eq!(
        subjects(to),
        ["Batch 1", "Batch 2"],
        "one batch, oldest first"
    );
    assert_eq!(
        count("subject LIKE 'Batch %'").await,
        1,
        "sent emails are deleted"
    );

    run_job().await;
    let sent = outbox::take_to(to);
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].email().subject(), "Batch 3");
    assert!(sent[0].raw().contains("Subject: Batch 3"));
    assert_eq!(count("subject LIKE 'Batch %'").await, 0);
}

async fn send_later_delivers_at_once_and_leaves_nothing_queued() {
    let to = "later@example.com";
    email(to, "Later").send_later();

    let mut sent = Vec::new();
    for _ in 0..100 {
        sent.extend(subjects(to));
        if !sent.is_empty() && count("subject = 'Later'").await == 0 {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    assert_eq!(sent, ["Later"], "send_later() must not wait for the job");
    assert_eq!(
        count("subject = 'Later'").await,
        0,
        "the sent email must leave the queue"
    );
}

async fn reserved_emails_wait_until_the_reservation_expires() {
    let to = "reserved@example.com";
    email(to, "Reserved").queue().await.unwrap();

    update("locked_until = '2099-01-01 00:00:00'", "Reserved").await;
    run_job().await;
    assert!(subjects(to).is_empty(), "another process is sending it");

    update("locked_until = '2000-01-01 00:00:00'", "Reserved").await;
    run_job().await;
    assert_eq!(
        subjects(to),
        ["Reserved"],
        "an expired reservation is released"
    );
}

async fn permanent_errors_fail_at_the_first_attempt() {
    let to = "invalid@example.com";
    email(to, "Invalid").queue().await.unwrap();

    // A stored envelope that is no longer valid cannot be fixed by retrying.
    update("envelope_to = 'not an address'", "Invalid").await;
    run_job().await;
    assert!(subjects(to).is_empty());
    assert_eq!(
        count(
            "subject = 'Invalid' AND attempts = 1 AND failed_at IS NOT NULL \
             AND locked_until IS NULL AND last_error LIKE 'invalid email address%'"
        )
        .await,
        1,
        "a permanent error gives up without retrying"
    );
}

async fn emails_without_attempts_left_fail_and_are_purged_later() {
    let to = "exhausted@example.com";
    email(to, "Exhausted").queue().await.unwrap();

    // The default `max_attempts` is 5: the last attempt was interrupted before it finished.
    update(
        "attempts = 5, locked_until = '2000-01-01 00:00:00'",
        "Exhausted",
    )
    .await;
    run_job().await;
    assert!(subjects(to).is_empty(), "no attempts left");
    assert_eq!(
        count(
            "subject = 'Exhausted' AND failed_at IS NOT NULL AND last_error = 'no attempts left'"
        )
        .await,
        1
    );

    // Failed emails are kept for `keep_failed_days` (30 by default).
    run_job().await;
    assert_eq!(
        count("subject = 'Exhausted'").await,
        1,
        "recent failures are kept"
    );
    update("failed_at = '2000-01-01 00:00:00'", "Exhausted").await;
    run_job().await;
    assert_eq!(
        count("subject = 'Exhausted'").await,
        0,
        "old failures are purged"
    );
}
