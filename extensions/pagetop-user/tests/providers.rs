// Login flows with an authentication provider, against a real database. `CONFIG_DIR` points to
// `tests/providers/`, whose `common.toml` sets `[user.login] sources = ["fake", "local"]`: a fake
// directory provider checked before the local passwords.
//
// Every scenario shares the same database, so they run one after another in a single test, as in
// `tests/flows.rs`.

use pagetop::prelude::*;
use pagetop_mail::outbox::{self, SentEmail};
use pagetop_seaorm::db::sea_orm::Statement;
use pagetop_seaorm::db::{ConnectionTrait, DbBackend, dbconn};
use pagetop_user::actions::{AdminUserFormExtras, AfterSessionResolved, AfterUserStatusChanged};
use pagetop_user::error::AuthError;
use pagetop_user::provider::*;
use pagetop_user::{Account, UserStatus};

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

// **< The fake provider >**************************************************************************

// People in the fake directory: username, email and password.
const PEOPLE: &[(&str, &str, &str)] = &[
    ("dora", "dora@example.com", "dora-dir"),
    ("eva", "eva@example.com", "eva-dir"),
    ("rejected", "rejected@example.com", "rejected-dir"),
    ("admin", "admin@example.com", "admin-dir"),
];

static HANDLE: OnceLock<ProviderHandle> = OnceLock::new();
// Calls to `authenticate()`, to check what never reaches the provider.
static CALLS: AtomicUsize = AtomicUsize::new(0);
// Simulates a provider that does not answer.
static DOWN: AtomicBool = AtomicBool::new(false);
// Role managed by the provider (0 = none yet).
static STAFF: AtomicI32 = AtomicI32::new(0);
// Status changes announced by `AfterUserStatusChanged`.
static STATUS_CHANGES: Mutex<Vec<(i32, UserStatus, UserStatus)>> = Mutex::new(Vec::new());

fn person(identifier: &str) -> Option<&'static (&'static str, &'static str, &'static str)> {
    let identifier = identifier.to_lowercase();
    PEOPLE
        .iter()
        .find(|(username, email, _)| *username == identifier || *email == identifier)
}

fn handle() -> &'static ProviderHandle {
    HANDLE.get().expect("the fake provider is attached")
}

struct Fake;

#[async_trait]
impl AuthProvider for Fake {
    fn id(&self) -> &'static str {
        "fake"
    }

    fn label(&self) -> Lc {
        Lc::n("Fake directory")
    }

    fn attach(&self, handle: ProviderHandle) {
        assert!(HANDLE.set(handle).is_ok(), "attached only once");
    }

    async fn releases(&self, user: &UserRef) -> bool {
        user.username() == "released"
    }

    async fn authenticate(&self, attempt: &LoginAttempt<'_>) -> ProviderOutcome {
        CALLS.fetch_add(1, Ordering::SeqCst);
        if DOWN.load(Ordering::SeqCst) {
            return ProviderOutcome::Unavailable;
        }
        let Some((username, email, password)) = person(attempt.identifier()) else {
            return ProviderOutcome::NotFound;
        };
        let key = format!("fake:{username}");
        let Ok(linked) = handle().find_user(&key).await else {
            return ProviderOutcome::Unavailable;
        };
        if attempt.password().expose() != *password {
            return ProviderOutcome::InvalidCredentials {
                user_id: linked.map(|user| user.id()),
            };
        }
        if *username == "rejected" {
            return ProviderOutcome::Rejected(RejectReason::Disallowed);
        }
        let user_id = match linked {
            Some(user) => user.id(),
            None => {
                let pending = handle().find_pending(username).await.unwrap_or_default();
                if let [user] = pending.as_slice() {
                    if handle()
                        .link_identity(user.id(), &[username], username, &key)
                        .await
                        .is_err()
                    {
                        return ProviderOutcome::Rejected(RejectReason::Conflict);
                    }
                    user.id()
                } else if attempt.local().is_some() {
                    return ProviderOutcome::Rejected(RejectReason::Conflict);
                } else {
                    let data = ExternalUserData::new(*username, *email).with_email_verified(true);
                    match handle().create_user(username, &key, data).await {
                        Ok(user_id) => user_id,
                        Err(_) => return ProviderOutcome::Rejected(RejectReason::Conflict),
                    }
                }
            }
        };
        let staff = STAFF.load(Ordering::SeqCst);
        if staff > 0 {
            handle().grant_roles(user_id, &[staff]).await.unwrap();
        }
        ProviderOutcome::Authenticated { user_id }
    }

    async fn reauthenticate(&self, user: &UserRef, password: &util::SecretStr) -> ReauthOutcome {
        match person(user.username()) {
            Some((_, _, expected)) if password.expose() == *expected => ReauthOutcome::Valid,
            _ => ReauthOutcome::Invalid,
        }
    }

    async fn account_policy(&self, _user: &UserRef) -> Option<AccountPolicy> {
        let staff = STAFF.load(Ordering::SeqCst);
        Some(
            AccountPolicy::new(Lc::n("Fake directory"))
                .with_password(FieldPolicy::ReadOnly)
                .with_email(FieldPolicy::ReadOnly)
                .with_managed_fields(vec![UserField::DisplayName])
                .with_managed_roles(if staff > 0 { vec![staff] } else { vec![] })
                .with_password_reset(ResetPolicy::Forbidden {
                    help: HelpLink::new("https://help.example.com/password", Lc::none()),
                }),
        )
    }

    fn allows_remember(&self) -> bool {
        false
    }

    fn login_hints(&self) -> Option<LoginHints> {
        Some(LoginHints::default().with_note(Lc::n("Use your network account.")))
    }
}

static FAKE: Fake = Fake;

// **< The test application >***********************************************************************

fn declare_providers(providers: &mut AuthProviders) {
    providers.add(&FAKE);
}

fn redirect_dora(request: &HttpRequest, account: &Account) -> Option<Response> {
    (request.path() == "/user/redirect-me" && account.username() == "dora")
        .then(|| Redirect::see_other("/user"))
}

fn record_status_change(user_id: i32, before: UserStatus, after: UserStatus) {
    STATUS_CHANGES
        .lock()
        .unwrap()
        .push((user_id, before, after));
}

fn fake_panel(user_id: i32) -> Option<Child> {
    Some(Child::with(Html::with(move |_| {
        html! { p class="fake-panel" { "Fake panel of " (user_id) } }
    })))
}

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_user::User]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            DeclareAuthProviders::new(declare_providers),
            AfterSessionResolved::new(redirect_dora),
            AfterUserStatusChanged::new(record_status_change),
            AdminUserFormExtras::new(fake_panel),
        ]
    }
}

// **< Client >*************************************************************************************

type Status = web::http::StatusCode;

struct Reply {
    status: Status,
    headers: web::http::HeaderMap,
    body: String,
}

impl Reply {
    fn header(&self, name: &str) -> &str {
        self.headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
    }

    fn session_cookie(&self) -> Option<String> {
        let pair = self.header("set-cookie").split(';').next()?;
        (!pair.ends_with('=')).then(|| pair.to_owned())
    }
}

struct Client {
    app: Router,
}

impl Client {
    async fn send(&self, request: web::test::TestRequest, session: Option<&str>) -> Reply {
        let request = match session {
            Some(cookie) => request.header("cookie", cookie),
            None => request,
        };
        let response = web::test::send_request(&self.app, request.to_request()).await;
        let status = response.status();
        let headers = response.headers().clone();
        let body = web::test::read_body_text(response).await;
        Reply {
            status,
            headers,
            body,
        }
    }

    async fn get(&self, uri: &str, session: Option<&str>) -> Reply {
        self.send(web::test::TestRequest::get().uri(uri), session)
            .await
    }

    async fn post(&self, uri: &str, fields: &str, session: Option<&str>) -> Reply {
        let request = web::test::TestRequest::post().uri(uri).form(fields);
        self.send(request, session).await
    }

    async fn try_login(&self, username: &str, password: &str) -> Reply {
        let fields = form(&[
            ("username", username),
            ("password", password),
            ("remember", "true"),
        ]);
        self.post("/user/login", &fields, None).await
    }

    async fn login(&self, username: &str, password: &str) -> Option<String> {
        let reply = self.try_login(username, password).await;
        match reply.status {
            Status::SEE_OTHER => Some(reply.session_cookie().expect("login sets the cookie")),
            Status::OK => None,
            status => panic!("unexpected login status {status}"),
        }
    }
}

fn form(fields: &[(&str, &str)]) -> String {
    let mut body = String::new();
    for (name, value) in fields {
        if !body.is_empty() {
            body.push('&');
        }
        body.push_str(name);
        body.push('=');
        for byte in value.bytes() {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                body.push(char::from(byte));
            } else {
                body.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    body
}

async fn mail_to(address: &str) -> SentEmail {
    for _ in 0..200 {
        let mut sent = outbox::take_to(address);
        if !sent.is_empty() {
            assert_eq!(sent.len(), 1, "expected exactly one email for {address}");
            return sent.remove(0);
        }
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    }
    panic!("no email sent to {address}");
}

async fn query_i64(sql: &str, column: &str) -> Option<i64> {
    let row = dbconn()
        .query_one(Statement::from_string(DbBackend::Sqlite, sql.to_owned()))
        .await
        .unwrap()?;
    Some(row.try_get::<i64>("", column).unwrap())
}

async fn user_id(username: &str) -> i32 {
    let sql = format!("SELECT id FROM users WHERE username_key = '{username}'");
    i32::try_from(query_i64(&sql, "id").await.expect("user exists")).unwrap()
}

async fn failed_logins(username: &str) -> i64 {
    let sql = format!("SELECT failed_login_count FROM users WHERE username_key = '{username}'");
    query_i64(&sql, "failed_login_count").await.unwrap()
}

// **< Scenarios >**********************************************************************************

#[pagetop::test]
async fn provider_flows() {
    // SAFETY: the only test of this binary sets it before reading any setting.
    unsafe {
        std::env::set_var(
            "CONFIG_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/providers"),
        );
    }
    let client = Client {
        app: Application::prepare(&TestApp).await.test(),
    };
    let admin = client.login("admin", "admin-password").await.unwrap();

    directory_people_get_a_linked_account(&client).await;
    rejections_are_only_revealed_with_the_right_password(&client).await;
    existing_local_accounts_are_never_taken_over(&client, &admin).await;
    an_unavailable_provider_answers_the_same_with_or_without_account(&client).await;
    failed_identifiers_stop_reaching_the_provider(&client).await;
    oversized_or_invalid_input_never_reaches_the_provider(&client).await;
    pages_follow_the_account_policy(&client).await;
    password_resets_send_a_notice_instead_of_a_link(&client).await;
    the_handle_only_acts_on_its_own_accounts().await;
    managed_roles_survive_the_administration_form(&client, &admin).await;
    actions_reach_other_extensions(&client, &admin).await;
    unlinking_closes_the_sessions(&client).await;
}

async fn directory_people_get_a_linked_account(client: &Client) {
    let reply = client.try_login("dora", "dora-dir").await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    // "Remember me" was asked for, but the provider does not allow it: a browser session cookie.
    assert!(!reply.header("set-cookie").contains("Max-Age"));
    let session = reply.session_cookie().unwrap();
    assert_eq!(client.get("/user", Some(&session)).await.status, Status::OK);

    // A wrong password counts once on the account, not once per source.
    assert_eq!(client.login("dora", "wrong").await, None);
    assert_eq!(failed_logins("dora").await, 1);
    // The right one resets the count.
    assert!(client.login("dora@example.com", "dora-dir").await.is_some());
    assert_eq!(failed_logins("dora").await, 0);

    // The login form shows the provider's note.
    let page = client.get("/user/login", None).await;
    assert!(page.body.contains("Use your network account."));
    // "local" allows "remember me", so the box is still offered.
    assert!(page.body.contains("name=\"remember\""));
}

async fn rejections_are_only_revealed_with_the_right_password(client: &Client) {
    let wrong = client.try_login("rejected", "wrong").await;
    assert!(wrong.body.contains("Invalid username or password"));
    let right = client.try_login("rejected", "rejected-dir").await;
    assert!(
        right
            .body
            .contains("Your account has no access to this application.")
    );
}

async fn existing_local_accounts_are_never_taken_over(client: &Client, admin: &str) {
    // The local administrator is also in the directory: the directory password does not get in.
    let reply = client.try_login("admin", "admin-dir").await;
    assert!(reply.body.contains("You cannot sign in with this account."));
    assert!(client.login("admin", "admin-password").await.is_some());

    // A local account with a name in the directory.
    let fields = form(&[
        ("username", "eva"),
        ("email", "eva@example.com"),
        ("password", "eva-local"),
        ("confirm_password", "eva-local"),
    ]);
    let reply = client
        .post("/admin/user/users/new", &fields, Some(admin))
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    let eva = user_id("eva").await;
    let local = client.login("eva", "eva-local").await.unwrap();
    let reply = client.try_login("eva", "eva-dir").await;
    assert!(reply.body.contains("You cannot sign in with this account."));

    // Once an administrator allows it, the directory links it: the local password and every open
    // session stop working.
    let link = format!("/admin/user/users/{eva}/identity");
    let reply = client
        .post(&link, "provider=fake&op=allow", Some(admin))
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert!(client.login("eva", "eva-dir").await.is_some());
    assert_eq!(client.login("eva", "eva-local").await, None);
    assert_eq!(
        client.get("/user", Some(&local)).await.status,
        Status::SEE_OTHER
    );

    // Never for an administrator.
    let admin_id = user_id("admin").await;
    let link = format!("/admin/user/users/{admin_id}/identity");
    let reply = client
        .post(&link, "provider=fake&op=allow", Some(admin))
        .await;
    assert!(
        reply
            .body
            .contains("An administrator account cannot be linked")
    );
}

async fn an_unavailable_provider_answers_the_same_with_or_without_account(client: &Client) {
    DOWN.store(true, Ordering::SeqCst);
    let unavailable = "Your password cannot be checked right now.";
    assert!(
        client
            .try_login("nobody", "x")
            .await
            .body
            .contains(unavailable)
    );
    assert!(
        client
            .try_login("admin", "wrong")
            .await
            .body
            .contains(unavailable)
    );
    // Local accounts still get in with "local" in the sources.
    assert!(client.login("admin", "admin-password").await.is_some());

    // Attempts that could not be checked lock nobody out: neither the account nor its identifier.
    for _ in 0..4 {
        assert!(
            client
                .try_login("dora", "dora-dir")
                .await
                .body
                .contains(unavailable)
        );
    }
    assert_eq!(failed_logins("dora").await, 0);
    DOWN.store(false, Ordering::SeqCst);
    assert!(client.login("dora", "dora-dir").await.is_some());
}

async fn failed_identifiers_stop_reaching_the_provider(client: &Client) {
    let before = CALLS.load(Ordering::SeqCst);
    for _ in 0..3 {
        assert_eq!(client.login("mallory", "guess").await, None);
    }
    let reply = client.try_login("mallory", "guess").await;
    assert!(reply.body.contains("Too many requests"));
    assert_eq!(CALLS.load(Ordering::SeqCst), before + 3);
}

async fn oversized_or_invalid_input_never_reaches_the_provider(client: &Client) {
    let before = CALLS.load(Ordering::SeqCst);
    assert_eq!(client.login("dora\u{7}", "dora-dir").await, None);
    assert_eq!(client.login(&"d".repeat(256), "dora-dir").await, None);
    assert_eq!(client.login("dora", &"p".repeat(1025)).await, None);
    assert_eq!(CALLS.load(Ordering::SeqCst), before);
}

async fn pages_follow_the_account_policy(client: &Client) {
    let dora = client.login("dora", "dora-dir").await.unwrap();
    let managed = "Your account is managed by Fake directory";

    let profile = client.get("/user", Some(&dora)).await;
    assert!(profile.body.contains(managed));
    assert!(!profile.body.contains("href=\"/user/password\""));
    assert!(!profile.body.contains("href=\"/user/email\""));

    for path in ["/user/password", "/user/email"] {
        let page = client.get(path, Some(&dora)).await;
        assert!(page.body.contains(managed), "GET {path}");
        assert!(!page.body.contains("current_password"), "GET {path}");
    }
    let fields = form(&[
        ("current_password", "dora-dir"),
        ("password", "local-password"),
        ("confirm_password", "local-password"),
    ]);
    let page = client.post("/user/password", &fields, Some(&dora)).await;
    assert!(page.body.contains(managed));
    assert_eq!(client.login("dora", "local-password").await, None);
}

async fn password_resets_send_a_notice_instead_of_a_link(client: &Client) {
    let reply = client
        .post("/user/password/reset", "email=dora%40example.com", None)
        .await;
    assert_eq!(reply.status, Status::OK);
    let notice = mail_to("dora@example.com").await;
    let links = notice.links();
    assert!(
        links
            .iter()
            .any(|link| link == "https://help.example.com/password")
    );
    assert!(
        !links
            .iter()
            .any(|link| link.contains("/user/password/reset/"))
    );
}

async fn the_handle_only_acts_on_its_own_accounts() {
    let admin = user_id("admin").await;
    let update = ExternalUserUpdate::new().with_display_name(Some("Mallory".into()));
    assert!(matches!(
        handle().update_user(admin, update).await,
        Err(AuthError::NotOwnedByProvider)
    ));
    assert!(matches!(
        handle().set_status(admin, UserStatus::Blocked).await,
        Err(AuthError::NotOwnedByProvider)
    ));
    assert!(matches!(
        handle().grant_roles(admin, &[3]).await,
        Err(AuthError::NotOwnedByProvider)
    ));
    assert!(matches!(
        handle().destroy_sessions(admin).await,
        Err(AuthError::NotOwnedByProvider)
    ));
    assert!(matches!(
        handle()
            .link_identity(admin, &["admin"], "admin", "fake:admin")
            .await,
        Err(AuthError::NotPendingIdentity)
    ));
    // Even with a pending identity planted in the database, an administrator is never linked.
    dbconn()
        .execute_unprepared(&format!(
            "INSERT INTO user_identities (user_id, provider, pending, expected_key, created_at) \
             VALUES ({admin}, 'fake', true, 'admin', CURRENT_TIMESTAMP)"
        ))
        .await
        .unwrap();
    assert!(matches!(
        handle()
            .link_identity(admin, &["admin"], "admin", "fake:admin")
            .await,
        Err(AuthError::ProtectedAccount)
    ));
    dbconn()
        .execute_unprepared("DELETE FROM user_identities WHERE expected_key = 'admin'")
        .await
        .unwrap();
}

async fn managed_roles_survive_the_administration_form(client: &Client, admin: &str) {
    let fields = form(&[("machine_name", "staff"), ("label", "Staff")]);
    let reply = client
        .post("/admin/user/roles/new", &fields, Some(admin))
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    let staff = query_i64("SELECT id FROM roles WHERE machine_name = 'staff'", "id")
        .await
        .unwrap();
    STAFF.store(i32::try_from(staff).unwrap(), Ordering::SeqCst);

    assert!(client.login("dora", "dora-dir").await.is_some());
    let dora = user_id("dora").await;
    let has_staff = format!(
        "SELECT COUNT(*) AS n FROM user_roles WHERE user_id = {dora} AND role_id = {staff}"
    );
    assert_eq!(query_i64(&has_staff, "n").await, Some(1));

    // The form shows it as managed, and saving it without the role keeps it.
    let edit = format!("/admin/user/users/{dora}/edit");
    let page = client.get(&edit, Some(admin)).await;
    assert!(page.body.contains("Staff (managed by Fake directory)"));
    let fields = form(&[
        ("username", "dora"),
        ("email", "dora@example.com"),
        ("display_name", "Changed by hand"),
    ]);
    let reply = client.post(&edit, &fields, Some(admin)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(query_i64(&has_staff, "n").await, Some(1));
    // The display name is managed too: the submitted one is ignored.
    let changed = format!(
        "SELECT COUNT(*) AS n FROM users WHERE id = {dora} AND display_name = 'Changed by hand'"
    );
    assert_eq!(query_i64(&changed, "n").await, Some(0));
    // And the password of an external account cannot be reset by an administrator.
    let page = client
        .get(&format!("/admin/user/users/{dora}/password"), Some(admin))
        .await;
    assert!(
        page.body
            .contains("is managed by Fake directory and cannot be changed here")
    );

    // Nor can it be given unrestricted access: whoever controls the person in the provider would
    // have it. The button is not offered, and a forged request is refused.
    assert!(
        !client
            .get(&edit, Some(admin))
            .await
            .body
            .contains("Grant administrator")
    );
    let reply = client
        .post(
            &format!("/admin/user/users/{dora}/admin"),
            "is_admin=true",
            Some(admin),
        )
        .await;
    assert!(
        reply
            .body
            .contains("An administrator account cannot be linked")
    );
    let is_admin = format!("SELECT is_admin AS n FROM users WHERE id = {dora}");
    assert_eq!(query_i64(&is_admin, "n").await, Some(0));

    // An account created for the provider gets its pending identity in the same step.
    let fields = form(&[
        ("username", "gina"),
        ("email", "gina@example.com"),
        ("provider", "fake"),
    ]);
    let reply = client
        .post("/admin/user/users/new", &fields, Some(admin))
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    let gina = user_id("gina").await;
    let pending = format!(
        "SELECT COUNT(*) AS n FROM user_identities \
         WHERE user_id = {gina} AND provider = 'fake' AND pending AND expected_key = 'gina'"
    );
    assert_eq!(query_i64(&pending, "n").await, Some(1));
    // And only the person it waits for can take it, whatever the provider asks for.
    assert!(matches!(
        handle()
            .link_identity(
                gina,
                &["mallory", "mallory@example.com"],
                "mallory",
                "fake:mallory"
            )
            .await,
        Err(AuthError::NotPendingIdentity)
    ));
    assert_eq!(query_i64(&pending, "n").await, Some(1));
}

async fn actions_reach_other_extensions(client: &Client, admin: &str) {
    let dora = user_id("dora").await;
    let session = client.login("dora", "dora-dir").await.unwrap();
    let reply = client.get("/user/redirect-me", Some(&session)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(reply.header("location"), "/user");
    // Without a session the action does not run: the request reaches the router (`/user/{id}`).
    assert_ne!(
        client.get("/user/redirect-me", None).await.status,
        Status::SEE_OTHER
    );

    let page = client
        .get(&format!("/admin/user/users/{dora}/view"), Some(admin))
        .await;
    assert!(page.body.contains(&format!("Fake panel of {dora}")));

    let reply = client
        .post(
            &format!("/admin/user/users/{dora}/status"),
            "status=blocked",
            Some(admin),
        )
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert!(STATUS_CHANGES.lock().unwrap().contains(&(
        dora,
        UserStatus::Active,
        UserStatus::Blocked
    )));
    let reply = client.try_login("dora", "dora-dir").await;
    assert!(reply.body.contains("Your account is blocked"));
}

async fn unlinking_closes_the_sessions(client: &Client) {
    let session = client.login("eva", "eva-dir").await.unwrap();
    assert_eq!(client.get("/user", Some(&session)).await.status, Status::OK);
    handle()
        .unlink_identity(user_id("eva").await)
        .await
        .unwrap();
    assert_ne!(client.get("/user", Some(&session)).await.status, Status::OK);
}
