use pagetop::prelude::*;

async fn setup() {
    Application::new().await;
}

// **< Context::timezone() >************************************************************************

#[pagetop::test]
async fn resolve_uses_anonymous_fallback_to_default() {
    setup().await;

    let cx = Context::default();
    assert_eq!(cx.timezone(), Timezone::default_tz());
}

// `CurrentUser::timezone()` resolves the user's own timezone when it's `Some`; this only checks
// that the value, when present, reaches `Context` intact. The `None` case (no personal timezone,
// falls back to the application's) is already covered by `tests/auth.rs`.
#[pagetop::test]
async fn resolve_uses_the_timezone_already_resolved_by_the_authenticated_user() {
    setup().await;

    let madrid: Tz = "Europe/Madrid".parse().unwrap();
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::Authenticated {
            id: 1,
            display_name: "Alice".to_owned(),
            timezone: Some(madrid),
        })
        .to_http_request();
    let cx = Context::new(req);
    assert_eq!(cx.timezone(), madrid);
}

// **< Context::with_timezone() >*******************************************************************

#[pagetop::test]
async fn with_timezone_forces_the_effective_timezone() {
    setup().await;

    let new_york: Tz = "America/New_York".parse().unwrap();
    let cx = Context::default().with_timezone(new_york);
    assert_eq!(cx.timezone(), new_york);
}

// **< Context::format_datetime() >*****************************************************************

// Without a request, the effective language is the fallback ("en-US"): month/day order
// (American convention), defined in `src/locale/en-US/datetime.ftl`. `format_datetime()`
// combines an independent `DateFormat` and `TimeFormat` with the `datetime_join` separator.
#[pagetop::test]
async fn format_datetime_uses_en_us_month_first_order_by_default() {
    setup().await;

    // 2026-06-15T10:30:00Z -> 12:30 in Europe/Madrid (CEST, UTC+2).
    let dt = Utc.with_ymd_and_hms(2026, 6, 15, 10, 30, 0).unwrap();
    let cx = Context::default().with_timezone("Europe/Madrid".parse().unwrap());

    assert_eq!(
        cx.format_datetime(dt, DateFormat::Short, TimeFormat::Short),
        "06/15/26, 12:30"
    );
    assert_eq!(
        cx.format_datetime(dt, DateFormat::Medium, TimeFormat::Short),
        "06/15/2026, 12:30"
    );
    assert_eq!(
        cx.format_datetime(dt, DateFormat::Long, TimeFormat::Long),
        "June 15, 2026, 12:30:00"
    );
    // Different formats for date and time: long date with short time.
    assert_eq!(
        cx.format_datetime(dt, DateFormat::Long, TimeFormat::Short),
        "June 15, 2026, 12:30"
    );
}

// `format_iso_datetime()` doesn't depend on the language (it doesn't go through Fluent) but does
// convert to the active timezone, with its offset: same result under any language.
#[pagetop::test]
async fn format_iso_datetime_is_the_same_in_every_language() {
    setup().await;

    let dt = Utc.with_ymd_and_hms(2026, 6, 15, 10, 30, 0).unwrap();
    let cx = Context::default().with_timezone("Europe/Madrid".parse().unwrap());

    assert_eq!(cx.format_iso_datetime(dt), "2026-06-15T12:30:00+02:00");
    assert_eq!(
        cx.with_langid(&Locale::resolve("es-ES"))
            .format_iso_datetime(dt),
        "2026-06-15T12:30:00+02:00"
    );
}

// The day/month/year order depends on the effective language, not a global constant: in es-ES
// the pattern is day/month (Spanish convention), defined in `src/locale/es-ES/datetime.ftl`.
#[pagetop::test]
async fn format_datetime_uses_es_es_day_first_order() {
    setup().await;

    let dt = Utc.with_ymd_and_hms(2026, 6, 15, 10, 30, 0).unwrap();
    let cx = Context::default()
        .with_timezone("Europe/Madrid".parse().unwrap())
        .with_langid(&Locale::resolve("es-ES"));

    assert_eq!(
        cx.format_datetime(dt, DateFormat::Short, TimeFormat::Short),
        "15/06/26, 12:30"
    );
    assert_eq!(
        cx.format_datetime(dt, DateFormat::Medium, TimeFormat::Short),
        "15/06/2026, 12:30"
    );
    assert_eq!(
        cx.format_datetime(dt, DateFormat::Long, TimeFormat::Long),
        "15 de junio de 2026, 12:30:00"
    );
}

// **< Context::format_time() >*********************************************************************

// `format_time()` only shows the time, already converted to the active timezone.
#[pagetop::test]
async fn format_time_converts_to_the_effective_timezone() {
    setup().await;

    let dt = Utc.with_ymd_and_hms(2026, 6, 15, 10, 30, 45).unwrap();
    let cx = Context::default().with_timezone("Europe/Madrid".parse().unwrap());

    assert_eq!(cx.format_time(dt, TimeFormat::Short), "12:30");
    assert_eq!(cx.format_time(dt, TimeFormat::Long), "12:30:45");
    assert_eq!(
        cx.format_time(dt, TimeFormat::Custom("%H:%M:%S")),
        "12:30:45"
    );
}

// **< Context::format_date() >*********************************************************************

// `format_date()` doesn't convert timezone (it doesn't apply to a date without a time); the
// result is the same regardless of the context's effective timezone. Under the fallback
// language ("en-US"), month/day order.
#[pagetop::test]
async fn format_date_does_not_depend_on_timezone() {
    setup().await;

    let date = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
    let cx = Context::default().with_timezone("Pacific/Auckland".parse().unwrap());

    assert_eq!(cx.format_date(date, DateFormat::Short), "06/15/26");
    assert_eq!(cx.format_date(date, DateFormat::Medium), "06/15/2026");
    assert_eq!(cx.format_date(date, DateFormat::Long), "June 15, 2026");
    assert_eq!(
        cx.format_date(date, DateFormat::Custom("%d-%m-%Y")),
        "15-06-2026"
    );
}

// Same day/month/year order as `format_datetime`, dependent on the effective language.
#[pagetop::test]
async fn format_date_uses_es_es_day_first_order() {
    setup().await;

    let date = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
    let cx = Context::default().with_langid(&Locale::resolve("es-ES"));

    assert_eq!(cx.format_date(date, DateFormat::Short), "15/06/26");
    assert_eq!(cx.format_date(date, DateFormat::Medium), "15/06/2026");
    assert_eq!(
        cx.format_date(date, DateFormat::Long),
        "15 de junio de 2026"
    );
}

// `Iso` doesn't depend on the language (it doesn't go through Fluent): same result under any
// language.
#[pagetop::test]
async fn format_date_iso_is_the_same_in_every_language() {
    setup().await;

    let date = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
    let cx = Context::default();

    assert_eq!(cx.format_date(date, DateFormat::Iso), "2026-06-15");
    assert_eq!(
        cx.with_langid(&Locale::resolve("es-ES"))
            .format_date(date, DateFormat::Iso),
        "2026-06-15"
    );
}

// **< Context::format_relative() >*****************************************************************
//
// The exact calendar breakdown (leap years, zero components...) is tested with fixed dates in
// `src/datetime/relative.rs` (`RelativeFormat::apply()` is `pub(crate)`, not visible from here).
// These tests verify the connection with the public API: timezone conversion, resolution via a
// real `Utc::now()`, and day offsets -- always < 28, so the result doesn't depend on which real
// month they run in (a month offset could cross a different month-end depending on the real day
// of execution).

// A value from a few minutes ago or a few minutes from now falls on the same civil date as "now":
// "today", regardless of the language.
#[pagetop::test]
async fn format_relative_collapses_to_today_within_the_same_civil_date() {
    setup().await;

    let cx = Context::default();
    let just_before = Utc::now() - Duration::minutes(3);
    let just_after = Utc::now() + Duration::minutes(3);

    assert_eq!(
        cx.format_relative(just_before, RelativeFormat::Short),
        "today"
    );
    assert_eq!(
        cx.with_langid(&Locale::resolve("es-ES"))
            .format_relative(just_after, RelativeFormat::Short),
        "hoy"
    );
}

// A small day offset (below any month) always gives a single days component, both in the past
// and in the future.
#[pagetop::test]
async fn format_relative_uses_day_only_offsets() {
    setup().await;

    let cx = Context::default();
    let three_days_ago = Utc::now() - Duration::days(3);
    let in_five_days = Utc::now() + Duration::days(5);

    assert_eq!(
        cx.format_relative(three_days_ago, RelativeFormat::Short),
        "3 days ago"
    );
    assert_eq!(
        cx.with_langid(&Locale::resolve("es-ES"))
            .format_relative(in_five_days, RelativeFormat::Short),
        "dentro de 5 días"
    );
}

// Fluent's plural selector distinguishes singular (1) from plural (everything else), via
// `Lc::with_number()`.
#[pagetop::test]
async fn format_relative_pluralizes_the_unit() {
    setup().await;

    let cx = Context::default();
    let yesterday = Utc::now() - Duration::days(1);
    let three_days_ago = Utc::now() - Duration::days(3);

    assert_eq!(
        cx.format_relative(yesterday, RelativeFormat::Short),
        "1 day ago"
    );
    assert_eq!(
        cx.format_relative(three_days_ago, RelativeFormat::Short),
        "3 days ago"
    );
}

// **< Context::format_since() / format_until() >***************************************************

// The translated month and, in `Long`, the article "el" before the day (Spanish only) are part
// of each level's own template, not a shared generic wrapper.
#[pagetop::test]
async fn format_since_reveals_increasing_precision() {
    setup().await;

    let date = NaiveDate::from_ymd_opt(2026, 6, 3).unwrap();
    let en = Context::default();
    let es = Context::default().with_langid(&Locale::resolve("es-ES"));

    assert_eq!(en.format_since(date, DatePrecision::Short), "since June");
    assert_eq!(
        en.format_since(date, DatePrecision::Medium),
        "since June 2026"
    );
    assert_eq!(
        en.format_since(date, DatePrecision::Long),
        "since June 3, 2026"
    );

    assert_eq!(es.format_since(date, DatePrecision::Short), "desde junio");
    assert_eq!(
        es.format_since(date, DatePrecision::Medium),
        "desde junio de 2026"
    );
    assert_eq!(
        es.format_since(date, DatePrecision::Long),
        "desde el 3 de junio de 2026"
    );
}

// Same mechanism as `format_since()`, symmetric for an end date.
#[pagetop::test]
async fn format_until_is_symmetric_to_format_since() {
    setup().await;

    let date = NaiveDate::from_ymd_opt(2026, 6, 3).unwrap();
    let en = Context::default();
    let es = Context::default().with_langid(&Locale::resolve("es-ES"));

    assert_eq!(en.format_until(date, DatePrecision::Short), "until June");
    assert_eq!(
        en.format_until(date, DatePrecision::Medium),
        "until June 2026"
    );
    assert_eq!(
        en.format_until(date, DatePrecision::Long),
        "until June 3, 2026"
    );

    assert_eq!(es.format_until(date, DatePrecision::Short), "hasta junio");
    assert_eq!(
        es.format_until(date, DatePrecision::Medium),
        "hasta junio de 2026"
    );
    assert_eq!(
        es.format_until(date, DatePrecision::Long),
        "hasta el 3 de junio de 2026"
    );
}
