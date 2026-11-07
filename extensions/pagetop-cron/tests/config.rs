use pagetop::prelude::*;
use pagetop_cron::config::CronSettings;

// Job keys usually contain dots, which TOML only accepts quoted. They must reach the map as a
// single key, not as nested tables.
#[pagetop::test]
async fn quoted_job_keys_with_dots_are_kept_whole() {
    let settings = config::load_from_str::<CronSettings>(
        r#"
        [cron.jobs."shop.cleanup"]
        enabled = false
        schedule = "@hourly"
        "#,
    )
    .unwrap();
    let job = &settings.jobs["shop.cleanup"];
    assert!(!job.enabled);
    assert_eq!(job.schedule.as_deref(), Some("@hourly"));
    assert_eq!(job.timeout_secs, None);
}
