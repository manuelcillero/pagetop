use pagetop::prelude::*;

// **< CurrentUser >********************************************************************************

#[pagetop::test]
async fn anonymous_reports_itself_correctly() {
    let user = CurrentUser::Anonymous;
    assert!(user.is_anonymous());
    assert!(!user.is_authenticated());
    assert_eq!(user.id(), None);
    assert_eq!(user.display_name(), None);
}

#[pagetop::test]
async fn authenticated_reports_itself_correctly() {
    let user = CurrentUser::Authenticated {
        id: 42,
        display_name: "Alice".to_owned(),
    };
    assert!(!user.is_anonymous());
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(42));
    assert_eq!(user.display_name(), Some("Alice"));
}

// **< Context::current_user() >********************************************************************

#[pagetop::test]
async fn current_user_defaults_to_anonymous() {
    let cx = Context::default();
    assert!(cx.current_user().is_anonymous());
}

#[pagetop::test]
async fn current_user_propagates_from_request_extensions() {
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::Authenticated {
            id: 7,
            display_name: "Bob".to_owned(),
        })
        .to_http_request();
    let cx = Context::new(Some(req));
    let user = cx.current_user();
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(7));
    assert_eq!(user.display_name(), Some("Bob"));
}

// **< HttpRequest::extension() >*******************************************************************

#[pagetop::test]
async fn request_extension_returns_none_for_unknown_type() {
    let req = web::test::TestRequest::get().to_http_request();
    assert!(req.extension::<CurrentUser>().is_none());
}

#[pagetop::test]
async fn request_extension_returns_injected_value() {
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::Authenticated {
            id: 1,
            display_name: "Carol".to_owned(),
        })
        .to_http_request();

    let user = req
        .extension::<CurrentUser>()
        .expect("extension should exist");
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(1));
    assert_eq!(user.display_name(), Some("Carol"));
}

// **< Page::new() >********************************************************************************

#[pagetop::test]
async fn page_new_propagates_current_user_from_request_extensions() {
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::Authenticated {
            id: 5,
            display_name: "Dave".to_owned(),
        })
        .to_http_request();
    let page = Page::new(req);
    let user = page.current_user();
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(5));
    assert_eq!(user.display_name(), Some("Dave"));
}

#[pagetop::test]
async fn page_new_uses_anonymous_when_no_extension() {
    let req = web::test::TestRequest::get().to_http_request();
    let page = Page::new(req);
    assert!(page.current_user().is_anonymous());
}
