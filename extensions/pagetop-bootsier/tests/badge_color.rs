// Verifies `BadgeBootsier::with_color()`: it overrides the Bootstrap color that `Badge` would
// otherwise derive from its `Intent`, without disturbing the class that `Badge::setup()` (core)
// already generated from that `Intent`.

use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;
use pagetop_bootsier::theme::*;

#[pagetop::test]
async fn without_an_override_the_color_comes_from_the_intent() {
    let mut badge = Badge::labeled(Lc::n("Admin")).with_intent(Intent::Severe);
    let html = badge
        .render(&mut Context::default().with_theme(&Bootsier))
        .await
        .into_string();

    assert!(html.contains("text-bg-danger"));
}

#[pagetop::test]
async fn with_color_overrides_the_intent_derived_color() {
    let mut badge = Badge::labeled(Lc::n("Beta"))
        .with_intent(Intent::Severe)
        .with_color(BootsierColors::Dark);
    let html = badge
        .render(&mut Context::default().with_theme(&Bootsier))
        .await
        .into_string();

    assert!(html.contains("text-bg-dark"));
    assert!(!html.contains("text-bg-danger"));
}

#[pagetop::test]
async fn with_color_none_restores_the_intent_derived_color() {
    let mut badge = Badge::labeled(Lc::n("Beta"))
        .with_intent(Intent::Severe)
        .with_color(BootsierColors::Dark)
        .with_color(None);
    let html = badge
        .render(&mut Context::default().with_theme(&Bootsier))
        .await
        .into_string();

    assert!(html.contains("text-bg-danger"));
}
