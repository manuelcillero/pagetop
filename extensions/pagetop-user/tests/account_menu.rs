// Verifies the account menu offered to an anonymous user. The authenticated menu needs a logged-in
// user and an initialized admin registry, so it is not covered here.

use pagetop_user::prelude::*;

use pagetop::prelude::*;

#[pagetop::test]
async fn anonymous_user_gets_the_login_and_register_links() {
    let mut cx = Context::default();
    let html = account_menu(&cx).render(&mut cx).await.into_string();

    assert!(html.contains(r#"href="/user/login""#));
    assert!(html.contains(r#"href="/user/register""#));
    assert!(!html.contains("dropdown"));
}

#[pagetop::test]
async fn component_renders_the_same_menu_as_the_function() {
    let mut cx = Context::default();
    let from_fn = account_menu(&cx).render(&mut cx).await.into_string();
    let from_component = AccountMenu::new().render(&mut cx).await.into_string();

    assert_eq!(from_fn, from_component);
}
