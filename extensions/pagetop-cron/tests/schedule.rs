use pagetop::prelude::*;
use pagetop_cron::prelude::*;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().unwrap()
}

fn next(schedule: &str, after: &str, tz: Tz) -> String {
    Schedule::parse(schedule)
        .unwrap()
        .next_after(at(after), tz)
        .unwrap()
        .unwrap()
        .to_rfc3339()
}

#[pagetop::test]
async fn cron_expressions_and_nicknames_are_accepted() {
    for text in [
        "*/10 * * * *",
        "*/30 * * * * *",
        "30 4 * * *",
        "0 8-18 * * MON-FRI",
        "0 0 L * *",
        "0 9 15W * *",
        "0 9 * * MON#1",
        "0 18 * * 5L",
        "@yearly",
        "@annually",
        "@monthly",
        "@weekly",
        "@daily",
        "@hourly",
    ] {
        let schedule = Schedule::parse(text).unwrap_or_else(|err| panic!("{text}: {err}"));
        // The text is kept as written.
        assert_eq!(schedule.to_string(), text);
    }
}

#[pagetop::test]
async fn intervals_are_parsed_and_written_back() {
    let cases = [
        (
            "@every 30s",
            "@every 30s",
            Schedule::fixed_rate(Duration::seconds(30)),
        ),
        (
            "@every 1h30m",
            "@every 1h30m",
            Schedule::fixed_rate(Duration::minutes(90)),
        ),
        (
            "@every 1h 30m",
            "@every 1h30m",
            Schedule::fixed_rate(Duration::minutes(90)),
        ),
        (
            "@EVERY 2d",
            "@every 2d",
            Schedule::fixed_rate(Duration::days(2)),
        ),
        (
            "  @delay   500ms ",
            "@delay 500ms",
            Schedule::fixed_delay(Duration::milliseconds(500)),
        ),
        (
            "@delay 90s",
            "@delay 1m30s",
            Schedule::fixed_delay(Duration::seconds(90)),
        ),
    ];
    for (text, written, expected) in cases {
        let schedule: Schedule = text.parse().unwrap();
        assert_eq!(schedule, expected, "{text}");
        assert_eq!(schedule.to_string(), written, "{text}");
    }
}

#[pagetop::test]
async fn invalid_schedules_are_rejected() {
    for text in [
        "",
        "61 * * * *",
        "* * *",
        "0 0 32 * *",
        "every day",
        "@every",
        "@every 0s",
        "@every 5",
        "@every 5x",
        "@every -1s",
        "@delay m",
        "@every 36501d",
    ] {
        let err = Schedule::parse(text).unwrap_err();
        assert!(
            matches!(err, CronError::InvalidSchedule { .. }),
            "{text}: {err}"
        );
    }
    // Constructors do not validate; the first use does.
    let zero = Schedule::fixed_delay(Duration::zero());
    assert!(zero.next_after(Utc::now(), Tz::UTC).is_err());
}

#[pagetop::test]
async fn cron_expressions_follow_the_timezone() {
    // 9:00 in Madrid: 8:00 UTC in winter, 7:00 UTC in summer.
    let madrid = Tz::Europe__Madrid;
    assert_eq!(
        next("0 9 * * *", "2026-01-15T00:00:00Z", madrid),
        "2026-01-15T08:00:00+00:00"
    );
    assert_eq!(
        next("0 9 * * *", "2026-07-15T00:00:00Z", madrid),
        "2026-07-15T07:00:00+00:00"
    );
    assert_eq!(
        next("0 9 * * *", "2026-07-15T00:00:00Z", Tz::UTC),
        "2026-07-15T09:00:00+00:00"
    );
}

#[pagetop::test]
async fn special_characters_select_the_right_days() {
    let madrid = Tz::Europe__Madrid;
    // First Monday of November 2026 (CET, UTC+1).
    assert_eq!(
        next("0 9 * * MON#1", "2026-10-19T00:00:00Z", madrid),
        "2026-11-02T08:00:00+00:00"
    );
    // Last day of February 2026.
    assert_eq!(
        next("0 0 L * *", "2026-02-10T00:00:00Z", madrid),
        "2026-02-27T23:00:00+00:00"
    );
    // Last Friday of October 2026 (still CEST, UTC+2).
    assert_eq!(
        next("0 18 * * 5L", "2026-10-01T00:00:00Z", madrid),
        "2026-10-30T17:00:00+00:00"
    );
    // Nearest weekday to Saturday 15 August 2026: Friday 14.
    assert_eq!(
        next("0 9 15W * *", "2026-08-01T00:00:00Z", madrid),
        "2026-08-14T07:00:00+00:00"
    );
}

#[pagetop::test]
async fn daylight_saving_changes_run_once() {
    let madrid = Tz::Europe__Madrid;
    // Spring forward on 29 March 2026: 2:30 does not exist; it runs when the next hour starts
    // (3:00 CEST = 1:00 UTC), and the day after at 2:30 again.
    assert_eq!(
        next("30 2 * * *", "2026-03-28T12:00:00Z", madrid),
        "2026-03-29T01:00:00+00:00"
    );
    assert_eq!(
        next("30 2 * * *", "2026-03-29T01:00:00Z", madrid),
        "2026-03-30T00:30:00+00:00"
    );
    // Fall back on 25 October 2026: 2:30 happens twice; it runs only once.
    let first = next("30 2 * * *", "2026-10-24T12:00:00Z", madrid);
    assert_eq!(first, "2026-10-25T00:30:00+00:00");
    assert_eq!(
        next("30 2 * * *", &first, madrid),
        "2026-10-26T01:30:00+00:00"
    );
}

#[pagetop::test]
async fn intervals_ignore_the_timezone() {
    let after = "2026-03-29T00:30:00Z";
    let every = Schedule::fixed_rate(Duration::hours(1));
    let next = every.next_after(at(after), Tz::Europe__Madrid).unwrap();
    assert_eq!(next, Some(at("2026-03-29T01:30:00Z")));
}
