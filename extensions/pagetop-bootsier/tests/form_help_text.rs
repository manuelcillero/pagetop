// Bootsier renders its own markup for text inputs, selects and textareas: the help text must stay
// linked to the control with `aria-describedby`, as in the core components.

use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;
use pagetop_bootsier::theme::*;

async fn render(component: &mut impl Component) -> String {
    component
        .render(&mut Context::default().with_theme(&Bootsier))
        .await
        .into_string()
}

#[pagetop::test]
async fn bootsier_fields_point_to_their_help_text() {
    let help = || Lc::n("Help");
    let cases: [(String, &str); 3] = [
        (
            render(
                &mut bs::form::input::Field::text()
                    .with_name("a")
                    .with_help_text(help()),
            )
            .await,
            "edit-a-input-help",
        ),
        (
            render(
                &mut bs::form::select::Field::new()
                    .with_name("b")
                    .with_help_text(help()),
            )
            .await,
            "edit-b-select-help",
        ),
        (
            render(
                &mut bs::form::Textarea::new()
                    .with_name("c")
                    .with_help_text(help()),
            )
            .await,
            "edit-c-textarea-help",
        ),
    ];
    for (html, id) in cases {
        assert!(
            html.contains(&format!(r#"aria-describedby="{id}""#)),
            "{html}"
        );
        assert!(
            html.contains(&format!(r#"<div id="{id}" class="form-text">Help</div>"#)),
            "{html}"
        );
    }
}
