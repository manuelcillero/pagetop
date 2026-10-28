// `config/common.toml` uses the external mode, the Madrid timezone and job settings for some of the
// jobs declared here. The scheduler never starts in tests (`Application::run()` is not called), so
// jobs only run through `run_now()` and the external trigger route.

use pagetop::prelude::*;
use pagetop_cron::prelude::*;

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

// Never due while the tests run, so the external trigger only starts the jobs a test expects.
const LATER: &str = "0 0 0 1 1 * 2099";

static OK_RUNS: AtomicUsize = AtomicUsize::new(0);
static REPORTS: Mutex<Vec<(String, bool)>> = Mutex::new(Vec::new());

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_cron::Cron]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            DeclareCronJobs::new(declare_jobs),
            AfterJobRun::new(|run| {
                REPORTS
                    .lock()
                    .unwrap()
                    .push((run.key().to_string(), run.outcome().is_success()));
            }),
        ]
    }
}

fn declare_jobs(jobs: &mut CronJobs) {
    jobs.add(
        Job::new("test.ok", Schedule::cron(LATER), |_| async {
            OK_RUNS.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .with_title(Lc::n("Works")),
    );
    jobs.add(Job::new("test.fails", Schedule::cron(LATER), |_| async {
        Err(std::io::Error::other("boom").into())
    }));
    jobs.add(Job::new("test.panics", Schedule::cron(LATER), |_| async {
        panic!("kaboom");
    }));
    jobs.add(
        Job::new("test.slow", Schedule::cron(LATER), |_| async {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            Ok(())
        })
        .with_timeout(Duration::milliseconds(100)),
    );
    jobs.add(Job::new("test.busy", Schedule::cron(LATER), |_| async {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        Ok(())
    }));
    jobs.add(
        Job::new("test.parallel", Schedule::cron(LATER), |_| async {
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            Ok(())
        })
        .with_overlap(Overlap::Allow),
    );
    jobs.add(Job::new(
        "test.context",
        Schedule::cron(LATER),
        |cx| async move {
            if cx.key() != "test.context"
                || cx.trigger() != RunTrigger::Manual
                || cx.scheduled_at().is_some()
                || cx.is_stopping()
            {
                return Err("unexpected job context".into());
            }
            Ok(())
        },
    ));
    jobs.add(Job::new(
        "test.disabled",
        Schedule::cron("* * * * *"),
        |_| async { Ok(()) },
    ));
    jobs.add(
        Job::new("test.overridden", Schedule::cron("0 * * * *"), |_| async {
            Ok(())
        })
        .with_timeout(Duration::minutes(1)),
    );
    jobs.add(Job::new("test.due", Schedule::cron(LATER), |_| async { Ok(()) }).with_run_on_start());
}

async fn prepare() {
    Application::prepare(&TestApp).await;
}

fn reported(key: &str) -> Vec<bool> {
    REPORTS
        .lock()
        .unwrap()
        .iter()
        .filter(|(k, _)| k == key)
        .map(|(_, ok)| *ok)
        .collect()
}

#[pagetop::test]
async fn a_job_runs_on_demand_and_reports() {
    prepare().await;
    let before = OK_RUNS.load(Ordering::SeqCst);

    let run = pagetop_cron::run_now("test.ok").await.unwrap();
    assert_eq!(run.key(), "test.ok");
    assert_eq!(run.trigger(), RunTrigger::Manual);
    assert_eq!(run.outcome(), &JobOutcome::Succeeded);
    assert!(run.duration() >= Duration::zero());
    assert_eq!(OK_RUNS.load(Ordering::SeqCst), before + 1);

    let status = pagetop_cron::job("test.ok").unwrap();
    assert!(status.runs() >= 1);
    assert_eq!(status.failures(), 0);
    assert_eq!(status.last_run().unwrap().outcome(), &JobOutcome::Succeeded);
    assert!(reported("test.ok").contains(&true));
}

#[pagetop::test]
async fn the_job_receives_its_context() {
    prepare().await;
    let run = pagetop_cron::run_now("test.context").await.unwrap();
    assert!(run.outcome().is_success(), "{}", run.outcome());
}

#[pagetop::test]
async fn errors_and_panics_are_reported_as_failures() {
    prepare().await;

    let run = pagetop_cron::run_now("test.fails").await.unwrap();
    assert_eq!(run.outcome(), &JobOutcome::Failed("boom".to_owned()));
    assert_eq!(run.outcome().to_string(), "failed: boom");

    let run = pagetop_cron::run_now("test.panics").await.unwrap();
    assert_eq!(run.outcome(), &JobOutcome::Panicked("kaboom".to_owned()));
    // A panic does not prevent the next run.
    let run = pagetop_cron::run_now("test.panics").await.unwrap();
    assert!(matches!(run.outcome(), JobOutcome::Panicked(_)));

    let status = pagetop_cron::job("test.panics").unwrap();
    assert_eq!(status.running(), 0);
    assert!(status.failures() >= 2);
    assert!(reported("test.fails").contains(&false));
}

#[pagetop::test]
async fn a_job_over_its_timeout_is_cancelled() {
    prepare().await;
    let clock = std::time::Instant::now();
    let run = pagetop_cron::run_now("test.slow").await.unwrap();
    assert_eq!(run.outcome(), &JobOutcome::TimedOut);
    assert!(clock.elapsed() < std::time::Duration::from_secs(5));
    // The run is released once the cancelled task has actually stopped.
    let mut running = 1;
    for _ in 0..50 {
        running = pagetop_cron::job("test.slow").unwrap().running();
        if running == 0 {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    assert_eq!(running, 0);
}

#[pagetop::test]
async fn overlapping_runs_are_refused_unless_allowed() {
    prepare().await;

    let (first, second) = tokio::join!(pagetop_cron::run_now("test.busy"), async {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        pagetop_cron::run_now("test.busy").await
    });
    assert!(first.unwrap().outcome().is_success());
    assert!(matches!(second, Err(CronError::AlreadyRunning(key)) if key == "test.busy"));

    let (first, second) = tokio::join!(pagetop_cron::run_now("test.parallel"), async {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        pagetop_cron::run_now("test.parallel").await
    });
    assert!(first.unwrap().outcome().is_success());
    assert!(second.unwrap().outcome().is_success());
}

#[pagetop::test]
async fn unknown_jobs_cannot_run() {
    prepare().await;
    let err = pagetop_cron::run_now("test.missing").await.unwrap_err();
    assert!(matches!(err, CronError::UnknownJob(key) if key == "test.missing"));
    assert!(pagetop_cron::job("test.missing").is_none());
}

#[pagetop::test]
async fn statuses_show_the_effective_settings() {
    prepare().await;
    let jobs = pagetop_cron::jobs();
    let keys: Vec<_> = jobs.iter().map(|job| job.key().to_string()).collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
    assert_eq!(keys.len(), 10);

    let ok = pagetop_cron::job("test.ok").unwrap();
    assert!(!ok.title().is_none());
    assert!(pagetop_cron::job("test.fails").unwrap().title().is_none());
    assert_eq!(ok.schedule().to_string(), LATER);
    assert_eq!(ok.timezone(), Tz::Europe__Madrid);
    assert!(ok.enabled());
    // External mode: the next run is known from startup.
    assert!(ok.next_run().is_some_and(|next| next > Utc::now()));

    let disabled = pagetop_cron::job("test.disabled").unwrap();
    assert!(!disabled.enabled());
    assert!(disabled.next_run().is_none());

    let overridden = pagetop_cron::job("test.overridden").unwrap();
    assert_eq!(overridden.schedule().to_string(), "@delay 1m30s");

    // A disabled job still runs on demand.
    let run = pagetop_cron::run_now("test.disabled").await.unwrap();
    assert!(run.outcome().is_success());
}

async fn post(app: &Router, auth: &str) -> (web::http::StatusCode, String) {
    let req = web::test::TestRequest::post()
        .uri("/cron")
        .header("authorization", auth)
        .to_request();
    let resp = web::test::send_request(app, req).await;
    let status = resp.status();
    (status, web::test::read_body_text(resp).await)
}

#[pagetop::test]
async fn the_external_trigger_runs_the_due_jobs() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    for auth in [
        "Bearer wrong-token-0123456789",
        "Bearer test-token-012345678",
        "test-token-0123456789",
        "Basic test-token-0123456789",
    ] {
        let (status, _) = post(&app, auth).await;
        assert_eq!(status, web::http::StatusCode::NOT_FOUND, "{auth}");
    }
    // Without the header at all, with another method, or with the token in the path.
    let req = web::test::TestRequest::post().uri("/cron").to_request();
    let resp = web::test::send_request(&app, req).await;
    assert_eq!(resp.status(), web::http::StatusCode::NOT_FOUND);
    let req = web::test::TestRequest::get().uri("/cron").to_request();
    let resp = web::test::send_request(&app, req).await;
    assert_eq!(resp.status(), web::http::StatusCode::NOT_FOUND);
    let req = web::test::TestRequest::post()
        .uri("/cron/test-token-0123456789")
        .to_request();
    let resp = web::test::send_request(&app, req).await;
    assert_eq!(resp.status(), web::http::StatusCode::NOT_FOUND);
    assert_eq!(pagetop_cron::job("test.due").unwrap().runs(), 0);

    // "test.due" runs on the first call because it runs on start. The scheme is case-insensitive.
    // The response arrives once the jobs have finished, with one line per job.
    let (status, body) = post(&app, "bearer test-token-0123456789").await;
    assert_eq!(status, web::http::StatusCode::OK);
    assert!(body.starts_with("Ran "), "{body}");
    assert!(body.contains("\ntest.due: succeeded\n"), "{body}");
    assert_eq!(pagetop_cron::job("test.due").unwrap().runs(), 1);
    let due = pagetop_cron::job("test.due").unwrap();
    assert_eq!(due.last_run().unwrap().trigger(), RunTrigger::Startup);
    assert!(due.next_run().is_some_and(|next| next > Utc::now()));

    // Not again on later calls.
    let (status, body) = post(&app, "Bearer test-token-0123456789").await;
    assert_eq!(status, web::http::StatusCode::OK);
    assert!(!body.contains("test.due"), "{body}");
    assert_eq!(pagetop_cron::job("test.due").unwrap().runs(), 1);
}
