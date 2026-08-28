// Verifies that `Bootsier` overrides `Theme::intent_color()` to translate `Intent` to its own
// Bootstrap color names, instead of PageTop's own semantic vocabulary.

use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;

#[pagetop::test]
async fn bootsier_translates_button_intent_to_its_bootstrap_color() {
    let mut button = Button::submit(Lc::n("Save")).with_style(button::Style::Solid(Intent::Severe));
    let html = button
        .render(&mut Context::default().with_theme(&Bootsier))
        .await
        .into_string();

    assert!(html.contains("btn-danger"));
    assert!(!html.contains("severe"));
}

#[pagetop::test]
async fn bootsier_translates_badge_intent_to_its_bootstrap_color() {
    let mut badge = Badge::labeled(Lc::n("Admin")).with_intent(Intent::Severe);
    let html = badge
        .render(&mut Context::default().with_theme(&Bootsier))
        .await
        .into_string();

    assert!(html.contains("text-bg-danger"));
    assert!(!html.contains("severe"));
}
