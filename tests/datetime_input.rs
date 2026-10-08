use pagetop::prelude::*;

async fn setup() {
    Application::new().await;
}

fn cx(language: &str, timezone: &str) -> Context {
    Context::default()
        .with_langid(&Locale::resolve(language))
        .with_timezone(timezone.parse().unwrap())
}

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn utc(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
}

// **< Contextual::parse_date() >*******************************************************************

#[pagetop::test]
async fn dates_are_read_in_the_format_of_the_language() {
    setup().await;

    let es = cx("es-ES", "UTC");
    assert_eq!(es.parse_date("06/10/2026"), Ok(Some(date(2026, 10, 6))));
    assert_eq!(es.parse_date(" 6/1/2026 "), Ok(Some(date(2026, 1, 6))));

    let en = cx("en-US", "UTC");
    assert_eq!(en.parse_date("10/06/2026"), Ok(Some(date(2026, 10, 6))));
}

// British English has no translations of its own except the day-first date formats; the rest falls
// back to en-US. Reading 06/10/2026 month first would silently store 10 June instead of 6 October.
#[pagetop::test]
async fn british_english_reads_and_shows_the_day_first() {
    setup().await;

    let gb = cx("en-GB", "UTC");
    assert_eq!(gb.parse_date("06/10/2026"), Ok(Some(date(2026, 10, 6))));
    assert_eq!(
        gb.parse_date("10/31/2026"),
        Err(DateInputError::InvalidDate {
            hint: "dd/mm/yyyy".to_string()
        })
    );
    assert_eq!(
        gb.format_date(date(2026, 10, 6), DateFormat::Medium),
        "06/10/2026"
    );
    assert_eq!(
        gb.format_date(date(2026, 10, 6), DateFormat::Long),
        "6 October 2026"
    );
    assert_eq!(
        Lc::l("relative_today").lookup(&gb).as_deref(),
        Some("today")
    );
}

#[pagetop::test]
async fn dates_also_accept_iso_and_blank_is_none() {
    setup().await;

    let es = cx("es-ES", "UTC");
    assert_eq!(es.parse_date("2026-10-06"), Ok(Some(date(2026, 10, 6))));
    assert_eq!(es.parse_date("   "), Ok(None));
}

#[pagetop::test]
async fn invalid_dates_report_the_expected_format() {
    setup().await;

    let es = cx("es-ES", "UTC");
    let expected = Err(DateInputError::InvalidDate {
        hint: "dd/mm/aaaa".to_string(),
    });
    // A two-digit year would otherwise be read as year 26.
    assert_eq!(es.parse_date("06/10/26"), expected);
    assert_eq!(es.parse_date("31/02/2026"), expected);
    assert_eq!(es.parse_date("10/31/2026"), expected);

    let message = es.parse_date("x").unwrap_err().message();
    assert_eq!(
        message.lookup(&es).as_deref(),
        Some("Fecha no válida: usa el formato dd/mm/aaaa.")
    );
}

// **< Contextual::parse_time() >*******************************************************************

#[pagetop::test]
async fn times_are_read_with_one_or_two_digit_hours_and_minutes() {
    setup().await;

    let es = cx("es-ES", "UTC");
    assert_eq!(es.parse_time("9:05"), Ok(NaiveTime::from_hms_opt(9, 5, 0)));
    assert_eq!(es.parse_time("9:5"), Ok(NaiveTime::from_hms_opt(9, 5, 0)));
    assert_eq!(
        es.parse_time("14:30"),
        Ok(NaiveTime::from_hms_opt(14, 30, 0))
    );
    assert!(matches!(
        es.parse_time("25:00"),
        Err(DateInputError::InvalidTime { .. })
    ));
}

// **< Contextual::parse_datetime() >***************************************************************

#[pagetop::test]
async fn datetimes_are_read_in_the_local_time_of_the_user() {
    setup().await;

    // Madrid is UTC+2 in October (summer time).
    let es = cx("es-ES", "Europe/Madrid");
    let expected = Ok(Some(utc(2026, 10, 6, 12, 30)));
    assert_eq!(es.parse_datetime("06/10/2026 14:30"), expected);
    assert_eq!(es.parse_datetime("2026-10-06T14:30"), expected);
    assert_eq!(es.parse_datetime("2026-10-06 14:30"), expected);

    let en = cx("en-US", "Europe/Madrid");
    assert_eq!(en.parse_datetime("10/06/2026 14:30"), expected);
}

#[pagetop::test]
async fn nonexistent_local_times_are_rejected() {
    setup().await;

    // On 2026-03-29 Madrid clocks jump from 02:00 to 03:00.
    let es = cx("es-ES", "Europe/Madrid");
    assert_eq!(
        es.parse_datetime("29/03/2026 02:30"),
        Err(DateInputError::NonexistentTime)
    );
}

#[pagetop::test]
async fn ambiguous_local_times_take_the_first_occurrence() {
    setup().await;

    // On 2026-10-25 Madrid clocks go back from 03:00 to 02:00, so 02:30 happens twice: first at
    // UTC+2 (00:30 UTC), then at UTC+1 (01:30 UTC).
    let es = cx("es-ES", "Europe/Madrid");
    assert_eq!(
        es.parse_datetime("25/10/2026 02:30"),
        Ok(Some(utc(2026, 10, 25, 0, 30)))
    );
}
