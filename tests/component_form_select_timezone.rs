use pagetop::prelude::*;

#[pagetop::test]
async fn groups_zones_by_region_and_marks_the_selected_one() {
    let mut field = form::SelectTimezone::new()
        .with_name("timezone")
        .with_selected("Europe/Madrid");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<optgroup label="Europe">"#));
    assert!(html.contains(r#"<option value="Europe/Madrid" selected>"#));
    assert!(html.contains(r#"value="Etc/UTC""#));
}

#[pagetop::test]
async fn region_labels_are_translated() {
    let mut field = form::SelectTimezone::new();
    let mut cx = Context::default().with_langid(&Locale::resolve("es-ES"));
    let html = field.render(&mut cx).await.into_string();

    assert!(html.contains(r#"<optgroup label="Europa">"#));
    assert!(html.contains(r#"<optgroup label="Otras">"#));
}

#[pagetop::test]
async fn legacy_and_fixed_offset_zones_are_not_offered() {
    let mut field = form::SelectTimezone::new();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"value="US/Eastern""#));
    assert!(!html.contains(r#"value="Japan""#));
    assert!(!html.contains(r#"value="Etc/GMT+1""#));
}

#[pagetop::test]
async fn optional_field_offers_the_site_time_zone_first() {
    let mut field = form::SelectTimezone::new();
    let html = field.render(&mut Context::default()).await.into_string();

    let site = html.find(r#"<option value="" selected>Use the site time zone: "#);
    let first_group = html.find("<optgroup");
    assert!(site.is_some());
    assert!(site < first_group);
}

#[pagetop::test]
async fn required_field_asks_to_choose_only_when_nothing_valid_is_selected() {
    let mut empty = form::SelectTimezone::new().with_required(true);
    let html = empty.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Choose a time zone...</option>"#));

    let mut unknown = form::SelectTimezone::new()
        .with_required(true)
        .with_selected("Mars/Olympus");
    let html = unknown.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Choose a time zone...</option>"#));

    let mut chosen = form::SelectTimezone::new()
        .with_required(true)
        .with_selected("Europe/Madrid");
    let html = chosen.render(&mut Context::default()).await.into_string();
    assert!(!html.contains(r#"value="""#));
}

// Zones without daylight saving time keep the expected labels stable all year round.
#[pagetop::test]
async fn utc_offset_is_shown_only_when_enabled() {
    let mut plain = form::SelectTimezone::new();
    let html = plain.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="Asia/Kolkata">Asia/Kolkata</option>"#));
    assert!(!html.contains("(UTC"));

    let mut with_offset = form::SelectTimezone::new().with_utc_offset(true);
    let html = with_offset
        .render(&mut Context::default())
        .await
        .into_string();
    assert!(html.contains(r#"<option value="Asia/Kolkata">Asia/Kolkata (UTC+05:30)</option>"#));
    assert!(html.contains(r#"<option value="America/Bogota">America/Bogota (UTC-05:00)</option>"#));
    assert!(html.contains(r#"<option value="Etc/UTC">Etc/UTC (UTC+00:00)</option>"#));
}

// Zones of a region are sorted by name, or by offset and then by name when offsets are shown:
// Kathmandu (+05:45) comes after Kolkata (+05:30), and Seoul precedes Tokyo (both +09:00).
#[pagetop::test]
async fn zones_are_sorted_by_offset_and_then_by_name_when_offsets_are_shown() {
    let positions = |html: &str| -> Vec<usize> {
        [
            "Asia/Dubai",
            "Asia/Kathmandu",
            "Asia/Kolkata",
            "Asia/Seoul",
            "Asia/Tokyo",
        ]
        .iter()
        .map(|zone| html.find(&format!(r#"value="{zone}""#)).unwrap())
        .collect()
    };

    let mut plain = form::SelectTimezone::new();
    let html = plain.render(&mut Context::default()).await.into_string();
    let [dubai, kathmandu, kolkata, seoul, tokyo] = positions(&html)[..] else {
        unreachable!()
    };
    assert!(dubai < kathmandu && kathmandu < kolkata && kolkata < seoul && seoul < tokyo);

    let mut with_offset = form::SelectTimezone::new().with_utc_offset(true);
    let html = with_offset
        .render(&mut Context::default())
        .await
        .into_string();
    let [dubai, kathmandu, kolkata, seoul, tokyo] = positions(&html)[..] else {
        unreachable!()
    };
    assert!(dubai < kolkata && kolkata < kathmandu && kathmandu < seoul && seoul < tokyo);
}

fn group_labels(html: &str) -> Vec<&str> {
    html.split(r#"<optgroup label=""#)
        .skip(1)
        .map(|group| group.split('"').next().unwrap())
        .collect()
}

#[pagetop::test]
async fn defaults_come_from_the_global_settings() {
    let field = form::SelectTimezone::new();
    assert_eq!(*field.order(), global::SETTINGS.app.timezone_order);
    let regions: Vec<TzRegion> = global::SETTINGS
        .app
        .timezone_regions
        .split(',')
        .filter_map(TzRegion::from_name)
        .collect();
    assert_eq!(field.regions(), &regions);
}

#[pagetop::test]
async fn with_regions_limits_the_offered_regions_and_keeps_etc() {
    let mut field = form::SelectTimezone::new().with_regions([TzRegion::Europe]);
    let html = field.render(&mut Context::default()).await.into_string();

    assert_eq!(group_labels(&html), ["Europe", "Other"]);
    assert!(html.contains(r#"value="Europe/Madrid""#));
    assert!(!html.contains(r#"value="America/Bogota""#));
}

#[pagetop::test]
async fn with_order_listed_follows_the_given_regions() {
    let mut field = form::SelectTimezone::new()
        .with_order(global::TimezoneOrder::Listed)
        .with_regions([TzRegion::Pacific, TzRegion::Europe, TzRegion::Asia]);
    let html = field.render(&mut Context::default()).await.into_string();

    assert_eq!(group_labels(&html), ["Pacific", "Europe", "Asia", "Other"]);
}

#[pagetop::test]
async fn with_order_alphabetical_sorts_by_translated_name() {
    let mut field = form::SelectTimezone::new()
        .with_order(global::TimezoneOrder::Alphabetical)
        .with_regions([
            TzRegion::Pacific,
            TzRegion::Arctic,
            TzRegion::Indian,
            TzRegion::Europe,
            TzRegion::Asia,
        ]);
    let mut cx = Context::default().with_langid(&Locale::resolve("es-ES"));
    let html = field.render(&mut cx).await.into_string();

    assert_eq!(
        group_labels(&html),
        ["Ártico", "Asia", "Europa", "Índico", "Pacífico", "Otras"]
    );
}

// A valid zone that is no longer offered stays selectable, so saving the form keeps it.
#[pagetop::test]
async fn a_valid_but_unoffered_selected_zone_is_kept_in_its_own_group() {
    let mut field = form::SelectTimezone::new()
        .with_regions([TzRegion::Europe])
        .with_selected("America/Bogota");
    let html = field.render(&mut Context::default()).await.into_string();

    assert_eq!(
        group_labels(&html),
        ["Current time zone", "Europe", "Other"]
    );
    assert!(html.contains(r#"<option value="America/Bogota" selected>America/Bogota</option>"#));
    assert!(!html.contains(r#"<option value="" selected>"#));

    let mut legacy = form::SelectTimezone::new().with_selected("US/Eastern");
    let html = legacy.render(&mut Context::default()).await.into_string();
    assert_eq!(group_labels(&html).first(), Some(&"Current time zone"));
    assert!(html.contains(r#"<option value="US/Eastern" selected>US/Eastern</option>"#));

    let mut required = form::SelectTimezone::new()
        .with_required(true)
        .with_regions([TzRegion::Europe])
        .with_selected("America/Bogota");
    let html = required.render(&mut Context::default()).await.into_string();
    assert!(!html.contains(r#"value="""#));
}

#[pagetop::test]
async fn an_invalid_selected_zone_is_not_offered() {
    let mut field = form::SelectTimezone::new().with_selected("Mars/Olympus");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("Current time zone"));
    assert!(!html.contains("Mars/Olympus"));
    assert!(html.contains(r#"<option value="" selected>"#));
}

// Labels replace underscores with spaces; values keep the IANA name.
#[pagetop::test]
async fn labels_are_readable_while_values_keep_the_iana_name() {
    let mut field = form::SelectTimezone::new();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<option value="America/New_York">America/New York</option>"#));
    assert!(html.contains(
        r#"<option value="America/Argentina/Buenos_Aires">America/Argentina/Buenos Aires</option>"#
    ));
    assert!(html.contains(r#"<option value="Etc/UTC">Etc/UTC</option>"#));

    let mut current = form::SelectTimezone::new()
        .with_regions([TzRegion::Europe])
        .with_selected("America/New_York");
    let html = current.render(&mut Context::default()).await.into_string();
    assert!(
        html.contains(r#"<option value="America/New_York" selected>America/New York</option>"#)
    );
}
