use pagetop::prelude::*;
use pagetop_admin::prelude::*;

// The form saves every value as text, so the defaults are kept the same way.
#[pagetop::test]
async fn defaults_are_kept_as_the_form_saves_values() {
    let default = |field: SettingField| field.default_value().cloned();
    let text = SettingField::text("notice", Lc::n("Notice")).with_default(&"Closed today");
    let number = SettingField::number("plots", Lc::n("Plots")).with_default(&42);
    let boolean = SettingField::boolean("open", Lc::n("Open")).with_default(&true);

    assert_eq!(default(text).as_deref(), Some("Closed today"));
    assert_eq!(default(number).as_deref(), Some("42"));
    assert_eq!(default(boolean).as_deref(), Some("true"));
}

#[pagetop::test]
async fn a_field_without_help_has_none() {
    let field = SettingField::text("notice", Lc::n("Notice"));
    assert!(field.help_text().is_none());
    let field = field.with_help(Lc::n("Shown on the home page."));
    assert!(!field.help_text().is_none());
}
