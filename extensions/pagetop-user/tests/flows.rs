// End-to-end account flows against a real database. `config/common.toml` gives these tests an
// in-memory SQLite database with a single connection, the memory mail transport, email
// verification on sign-up, `max_failed_logins = 3` and `max_ip_requests = 3`.
//
// Every scenario shares the same database, so they run one after another in a single test (and a
// single runtime, which also keeps the database connection alive). Each one uses its own accounts
// and addresses, and emails are sent in the background, so `mail_to()` waits for them.

use pagetop::prelude::*;
use pagetop_mail::outbox::{self, SentEmail};

use std::net::SocketAddr;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_user::User]
    }
}

type Status = web::http::StatusCode;

// Response status, headers and body text.
struct Reply {
    status: Status,
    headers: web::http::HeaderMap,
    body: String,
}

impl Reply {
    fn location(&self) -> &str {
        self.header("location")
    }

    fn header(&self, name: &str) -> &str {
        self.headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
    }

    // `name=value` of the session cookie, to send it back.
    fn session_cookie(&self) -> Option<String> {
        let cookie = self.header("set-cookie");
        let pair = cookie.split(';').next()?;
        (!pair.ends_with('=')).then(|| pair.to_owned())
    }
}

struct Client {
    app: Router,
}

impl Client {
    async fn send(&self, request: web::test::TestRequest) -> Reply {
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
        self.send(with_session(
            web::test::TestRequest::get().uri(uri),
            session,
        ))
        .await
    }

    async fn post(&self, uri: &str, fields: &str, session: Option<&str>) -> Reply {
        let request = web::test::TestRequest::post().uri(uri).form(fields);
        self.send(with_session(request, session)).await
    }

    // Signs in and returns the session cookie, or `None` if the credentials are rejected.
    async fn login(&self, username: &str, password: &str) -> Option<String> {
        let fields = form(&[("username", username), ("password", password)]);
        let reply = self.post("/user/login", &fields, None).await;
        match reply.status {
            Status::SEE_OTHER => Some(reply.session_cookie().expect("login sets the cookie")),
            Status::OK => None,
            status => panic!("unexpected login status {status}"),
        }
    }
}

fn with_session(request: web::test::TestRequest, session: Option<&str>) -> web::test::TestRequest {
    match session {
        Some(cookie) => request.header("cookie", cookie),
        None => request,
    }
}

// Form body with every value encoded.
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

// Waits for the only email sent to `address` in the background.
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

// Waits a while to check that nothing has been sent to `address`.
async fn no_mail_to(address: &str) {
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert!(
        outbox::take_to(address).is_empty(),
        "no email expected for {address}"
    );
}

// Path of the first link in the email that starts with `prefix`, without the base URL.
fn link(sent: &SentEmail, prefix: &str) -> String {
    sent.links()
        .into_iter()
        .filter_map(|link| link.strip_prefix("https://example.com").map(str::to_owned))
        .find(|path| path.starts_with(prefix))
        .unwrap_or_else(|| panic!("no {prefix} link in {:?}", sent.links()))
}

// User id in a `/{base}/{uid}/{token}` link.
fn uid(path: &str) -> &str {
    path.rsplit('/').nth(1).unwrap()
}

// Path without its query string.
fn path(link: &str) -> &str {
    link.split('?').next().unwrap()
}

// Registers and verifies an account, and returns its id.
async fn verified_account(client: &Client, username: &str, email: &str, password: &str) -> String {
    let fields = form(&[
        ("username", username),
        ("email", email),
        ("password", password),
        ("confirm_password", password),
    ]);
    let reply = client.post("/user/register", &fields, None).await;
    assert_eq!(reply.status, Status::OK, "sign-up of {username} failed");
    let verify = link(&mail_to(email).await, "/user/verify/");
    let reply = client.post(path(&verify), "", None).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    uid(path(&verify)).to_owned()
}

#[pagetop::test]
async fn account_flows() {
    let client = Client {
        app: Application::prepare(&TestApp).await.test(),
    };
    registration_requires_confirming_the_address(&client).await;
    password_reset_links_are_single_use(&client).await;
    failed_logins_lock_the_account(&client).await;
    password_changes_are_notified(&client).await;
    email_changes_are_confirmed_from_the_new_address(&client).await;
    password_reset_cancels_a_pending_email_change(&client).await;
    administrators_activate_accounts_and_reset_passwords(&client).await;
    password_changes_revoke_older_reset_links(&client).await;
    email_changes_revoke_links_sent_to_the_old_address(&client).await;
    email_change_links_are_bound_to_their_address(&client).await;
    wrong_current_passwords_lock_the_account(&client).await;
    requests_are_limited_by_address(&client).await;
}

async fn registration_requires_confirming_the_address(client: &Client) {
    let fields = form(&[
        ("username", "ana"),
        ("email", "ana@example.com"),
        ("password", "ana-password"),
        ("confirm_password", "ana-password"),
    ]);
    let reply = client.post("/user/register", &fields, None).await;
    assert_eq!(reply.status, Status::OK);
    let sent = mail_to("ana@example.com").await;
    // The address may belong to someone else: the email does not include the chosen username.
    assert!(!sent.email().html().unwrap().as_str().contains("ana,"));
    let verify = link(&sent, "/user/verify/");

    // A pending account cannot sign in.
    assert_eq!(client.login("ana", "ana-password").await, None);

    // Opening the link only asks for confirmation, without leaking the token to other sites.
    let reply = client.get(&verify, None).await;
    assert_eq!(reply.status, Status::OK);
    assert_eq!(reply.header("referrer-policy"), "no-referrer");
    assert_eq!(client.login("ana", "ana-password").await, None);

    let reply = client.post(path(&verify), "", None).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(path(reply.location()), "/user/login");
    assert!(client.login("ana", "ana-password").await.is_some());
    assert!(
        client
            .login("ANA@example.com", "ana-password")
            .await
            .is_some()
    );

    // The link is single use.
    assert_eq!(client.get(&verify, None).await.status, Status::NOT_FOUND);
}

async fn password_reset_links_are_single_use(client: &Client) {
    verified_account(client, "bea", "bea@example.com", "bea-password").await;

    // Unknown addresses get the same answer, and no email.
    let unknown = client
        .post("/user/password/reset", "email=nobody%40example.com", None)
        .await;
    let reply = client
        .post("/user/password/reset", "email=bea%40example.com", None)
        .await;
    assert_eq!(unknown.status, Status::OK);
    assert_eq!(unknown.body, reply.body);
    no_mail_to("nobody@example.com").await;
    let reset = link(&mail_to("bea@example.com").await, "/user/password/reset/");

    let reply = client.get(&reset, None).await;
    assert_eq!(reply.status, Status::OK);
    assert_eq!(reply.header("referrer-policy"), "no-referrer");

    // An invalid password does not spend the link.
    let mismatch = form(&[
        ("password", "bea-new-password"),
        ("confirm_password", "something-else"),
    ]);
    let reply = client.post(path(&reset), &mismatch, None).await;
    assert_eq!(reply.status, Status::OK);
    assert_eq!(reply.header("referrer-policy"), "no-referrer");
    assert_eq!(client.get(&reset, None).await.status, Status::OK);

    let fields = form(&[
        ("password", "bea-new-password"),
        ("confirm_password", "bea-new-password"),
    ]);
    let reply = client.post(path(&reset), &fields, None).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(client.get(&reset, None).await.status, Status::NOT_FOUND);
    assert_eq!(client.login("bea", "bea-password").await, None);
    assert!(client.login("bea", "bea-new-password").await.is_some());
}

async fn failed_logins_lock_the_account(client: &Client) {
    verified_account(client, "carla", "carla@example.com", "carla-password").await;
    for _ in 0..3 {
        assert_eq!(client.login("carla", "wrong-password").await, None);
    }
    // Locked: even the right password is rejected until the lock expires.
    assert_eq!(client.login("carla", "carla-password").await, None);

    // Resetting the password lifts the lock.
    client
        .post("/user/password/reset", "email=carla%40example.com", None)
        .await;
    let reset = link(&mail_to("carla@example.com").await, "/user/password/reset/");
    let fields = form(&[
        ("password", "carla-new-password"),
        ("confirm_password", "carla-new-password"),
    ]);
    client.post(path(&reset), &fields, None).await;
    assert!(client.login("carla", "carla-new-password").await.is_some());
}

async fn password_changes_are_notified(client: &Client) {
    verified_account(client, "dani", "dani@example.com", "dani-password").await;
    let session = client.login("dani", "dani-password").await.unwrap();

    let wrong = form(&[
        ("current_password", "wrong-password"),
        ("password", "dani-new-password"),
        ("confirm_password", "dani-new-password"),
    ]);
    let reply = client.post("/user/password", &wrong, Some(&session)).await;
    assert_eq!(reply.status, Status::OK);
    no_mail_to("dani@example.com").await;

    let fields = form(&[
        ("current_password", "dani-password"),
        ("password", "dani-new-password"),
        ("confirm_password", "dani-new-password"),
    ]);
    let reply = client.post("/user/password", &fields, Some(&session)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    let notice = mail_to("dani@example.com").await;
    // A verified address is greeted by name and offered the way to recover the account.
    assert!(notice.email().html().unwrap().as_str().contains("dani"));
    link(&notice, "/user/password/reset");
}

async fn email_changes_are_confirmed_from_the_new_address(client: &Client) {
    verified_account(client, "eva", "eva@example.com", "eva-password").await;
    let session = client.login("eva", "eva-password").await.unwrap();
    assert_eq!(
        client.get("/user/email", Some(&session)).await.status,
        Status::OK
    );

    // The current password is required.
    let wrong = form(&[
        ("email", "eva.new@example.com"),
        ("current_password", "wrong-password"),
    ]);
    let reply = client.post("/user/email", &wrong, Some(&session)).await;
    assert_eq!(reply.status, Status::OK);
    no_mail_to("eva.new@example.com").await;

    // The profile form cannot change the address.
    let profile = form(&[("email", "eva.other@example.com"), ("display_name", "Eva")]);
    let reply = client.post("/user/edit", &profile, Some(&session)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(
        client.login("eva.other@example.com", "eva-password").await,
        None
    );

    let fields = form(&[
        ("email", "eva.new@example.com"),
        ("current_password", "eva-password"),
    ]);
    let reply = client.post("/user/email", &fields, Some(&session)).await;
    assert_eq!(reply.status, Status::OK);
    let confirm = mail_to("eva.new@example.com").await;
    // The new address may belong to someone else: no name in the greeting.
    assert!(!confirm.email().html().unwrap().as_str().contains("Eva"));
    let confirm = link(&confirm, "/user/email/");
    link(&mail_to("eva@example.com").await, "/user/password/reset");

    // Nothing changes until the new address is confirmed.
    assert_eq!(
        client.login("eva.new@example.com", "eva-password").await,
        None
    );
    assert!(
        client
            .login("eva@example.com", "eva-password")
            .await
            .is_some()
    );
    let reply = client.get(&confirm, None).await;
    assert_eq!(reply.status, Status::OK);
    assert_eq!(reply.header("referrer-policy"), "no-referrer");

    let reply = client.post(path(&confirm), "", None).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(path(reply.location()), "/user");
    assert!(
        client
            .login("eva.new@example.com", "eva-password")
            .await
            .is_some()
    );
    assert_eq!(client.login("eva@example.com", "eva-password").await, None);
    assert_eq!(client.get(&confirm, None).await.status, Status::NOT_FOUND);

    // The address of another account cannot be requested.
    let taken = form(&[
        ("email", "ana@example.com"),
        ("current_password", "eva-password"),
    ]);
    let reply = client.post("/user/email", &taken, Some(&session)).await;
    assert_eq!(reply.status, Status::OK);
    no_mail_to("ana@example.com").await;
}

async fn password_reset_cancels_a_pending_email_change(client: &Client) {
    verified_account(client, "fede", "fede@example.com", "fede-password").await;
    let session = client.login("fede", "fede-password").await.unwrap();
    let fields = form(&[
        ("email", "intruder@example.com"),
        ("current_password", "fede-password"),
    ]);
    client.post("/user/email", &fields, Some(&session)).await;
    let confirm = link(&mail_to("intruder@example.com").await, "/user/email/");
    mail_to("fede@example.com").await;

    // The owner follows the notice and resets the password.
    client
        .post("/user/password/reset", "email=fede%40example.com", None)
        .await;
    let reset = link(&mail_to("fede@example.com").await, "/user/password/reset/");
    let new_password = form(&[
        ("password", "fede-new-password"),
        ("confirm_password", "fede-new-password"),
    ]);
    client.post(path(&reset), &new_password, None).await;

    assert_eq!(client.get(&confirm, None).await.status, Status::NOT_FOUND);
    assert_eq!(
        client.post(path(&confirm), "", None).await.status,
        Status::NOT_FOUND
    );
    assert!(
        client
            .login("fede@example.com", "fede-new-password")
            .await
            .is_some()
    );
    // The reset also closed the session that requested the change.
    let reply = client.get("/user/email", Some(&session)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
}

async fn administrators_activate_accounts_and_reset_passwords(client: &Client) {
    let fields = form(&[
        ("username", "gil"),
        ("email", "gil@example.com"),
        ("password", "gil-password"),
        ("confirm_password", "gil-password"),
    ]);
    client.post("/user/register", &fields, None).await;
    let id = uid(path(&link(
        &mail_to("gil@example.com").await,
        "/user/verify/",
    )))
    .to_owned();
    assert_eq!(client.login("gil", "gil-password").await, None);

    // Only users with the permission reach the administration.
    let ana = client.login("ana", "ana-password").await.unwrap();
    let status = format!("/admin/user/users/{id}/status");
    let reply = client.post(&status, "status=active", Some(&ana)).await;
    assert_eq!(reply.status, Status::FORBIDDEN);
    assert_eq!(client.login("gil", "gil-password").await, None);

    let admin = client.login("admin", "admin-password").await.unwrap();
    let edit = client
        .get(&format!("/admin/user/users/{id}/edit"), Some(&admin))
        .await;
    assert_eq!(edit.status, Status::OK);
    assert!(edit.body.contains(&format!("action=\"{status}\"")));
    let reply = client.post(&status, "status=active", Some(&admin)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    let gil = client.login("gil", "gil-password").await.unwrap();

    // An address set by an administrator replaces the one pending confirmation.
    let request = form(&[
        ("email", "gil.pending@example.com"),
        ("current_password", "gil-password"),
    ]);
    client.post("/user/email", &request, Some(&gil)).await;
    let confirm = link(&mail_to("gil.pending@example.com").await, "/user/email/");
    mail_to("gil@example.com").await;
    let edit = form(&[("username", "gil"), ("email", "gil.admin@example.com")]);
    let reply = client
        .post(&format!("/admin/user/users/{id}/edit"), &edit, Some(&admin))
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(
        client.post(path(&confirm), "", None).await.status,
        Status::NOT_FOUND
    );
    assert!(
        client
            .login("gil.admin@example.com", "gil-password")
            .await
            .is_some()
    );

    let fields = form(&[
        ("password", "gil-new-password"),
        ("confirm_password", "gil-new-password"),
    ]);
    let password = format!("/admin/user/users/{id}/password");
    let reply = client.post(&password, &fields, Some(&admin)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    link(
        &mail_to("gil.admin@example.com").await,
        "/user/password/reset",
    );
    assert!(client.login("gil", "gil-new-password").await.is_some());
    // Every session of the user is closed.
    assert_eq!(
        client.get("/user/email", Some(&gil)).await.status,
        Status::SEE_OTHER
    );
}

async fn password_changes_revoke_older_reset_links(client: &Client) {
    verified_account(client, "irene", "irene@example.com", "irene-password").await;
    client
        .post("/user/password/reset", "email=irene%40example.com", None)
        .await;
    let reset = link(&mail_to("irene@example.com").await, "/user/password/reset/");

    let session = client.login("irene", "irene-password").await.unwrap();
    let fields = form(&[
        ("current_password", "irene-password"),
        ("password", "irene-new-password"),
        ("confirm_password", "irene-new-password"),
    ]);
    let reply = client.post("/user/password", &fields, Some(&session)).await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    mail_to("irene@example.com").await;
    // A link requested before must not undo the change.
    assert_eq!(client.get(&reset, None).await.status, Status::NOT_FOUND);
}

async fn email_changes_revoke_links_sent_to_the_old_address(client: &Client) {
    verified_account(client, "juan", "juan@example.com", "juan-password").await;
    // Whoever still reads the old address (e.g. a former employer) may hold this link.
    client
        .post("/user/password/reset", "email=juan%40example.com", None)
        .await;
    let reset = link(&mail_to("juan@example.com").await, "/user/password/reset/");

    let session = client.login("juan", "juan-password").await.unwrap();
    let request = |email| form(&[("email", email), ("current_password", "juan-password")]);
    client
        .post(
            "/user/email",
            &request("juan.first@example.com"),
            Some(&session),
        )
        .await;
    let first = link(&mail_to("juan.first@example.com").await, "/user/email/");
    mail_to("juan@example.com").await;
    client
        .post(
            "/user/email",
            &request("juan.second@example.com"),
            Some(&session),
        )
        .await;
    let second = link(&mail_to("juan.second@example.com").await, "/user/email/");
    mail_to("juan@example.com").await;

    // Only the last requested change can be confirmed.
    assert_eq!(client.get(&first, None).await.status, Status::NOT_FOUND);
    assert_eq!(
        client.post(path(&first), "", None).await.status,
        Status::NOT_FOUND
    );
    assert_eq!(
        client.post(path(&second), "", None).await.status,
        Status::SEE_OTHER
    );
    assert!(
        client
            .login("juan.second@example.com", "juan-password")
            .await
            .is_some()
    );
    assert_eq!(client.get(&reset, None).await.status, Status::NOT_FOUND);
}

async fn email_change_links_are_bound_to_their_address(client: &Client) {
    use pagetop_seaorm::db::execute;

    verified_account(client, "lia", "lia@example.com", "lia-password").await;
    let session = client.login("lia", "lia-password").await.unwrap();
    let fields = form(&[
        ("email", "lia.own@example.com"),
        ("current_password", "lia-password"),
    ]);
    client.post("/user/email", &fields, Some(&session)).await;
    let confirm = link(&mail_to("lia.own@example.com").await, "/user/email/");
    mail_to("lia@example.com").await;

    // A newer request may replace the pending address before the older link is revoked: that link
    // must not apply an address nobody has confirmed.
    let pending =
        |email: &str| format!("UPDATE users SET pending_email = '{email}' WHERE username = 'lia'");
    execute(pending("someone.else@example.com")).await.unwrap();
    assert_eq!(client.get(&confirm, None).await.status, Status::NOT_FOUND);
    assert_eq!(
        client.post(path(&confirm), "", None).await.status,
        Status::NOT_FOUND
    );
    assert_eq!(
        client
            .login("someone.else@example.com", "lia-password")
            .await,
        None
    );

    // The link itself is still valid for its own address.
    execute(pending("lia.own@example.com")).await.unwrap();
    assert_eq!(
        client.post(path(&confirm), "", None).await.status,
        Status::SEE_OTHER
    );
    assert!(
        client
            .login("lia.own@example.com", "lia-password")
            .await
            .is_some()
    );
}

async fn wrong_current_passwords_lock_the_account(client: &Client) {
    verified_account(client, "kim", "kim@example.com", "kim-password").await;
    let session = client.login("kim", "kim-password").await.unwrap();
    let change = |current| {
        form(&[
            ("current_password", current),
            ("password", "kim-new-password"),
            ("confirm_password", "kim-new-password"),
        ])
    };
    for _ in 0..3 {
        let reply = client
            .post("/user/password", &change("wrong-password"), Some(&session))
            .await;
        assert_eq!(reply.status, Status::OK);
    }
    // With a stolen session, guessing the password here is as limited as signing in.
    let reply = client
        .post("/user/password", &change("kim-password"), Some(&session))
        .await;
    assert_eq!(reply.status, Status::OK);
    let email = form(&[
        ("email", "kim.new@example.com"),
        ("current_password", "kim-password"),
    ]);
    let reply = client.post("/user/email", &email, Some(&session)).await;
    assert_eq!(reply.status, Status::OK);
    no_mail_to("kim.new@example.com").await;
    no_mail_to("kim@example.com").await;
    assert_eq!(client.login("kim", "kim-password").await, None);
}

async fn requests_are_limited_by_address(client: &Client) {
    let from = |ip: [u8; 4]| {
        web::test::TestRequest::post()
            .with_extension(web::ConnectInfo(SocketAddr::from((ip, 4711))))
    };
    let reset = |ip, email: &str| {
        from(ip)
            .uri("/user/password/reset")
            .form(form(&[("email", email)]))
    };
    for _ in 0..2 {
        client.send(reset([192, 0, 2, 9], "bea@example.com")).await;
        mail_to("bea@example.com").await;
    }
    let register = |ip| {
        from(ip).uri("/user/register").form(form(&[
            ("username", "hugo"),
            ("email", "hugo@example.com"),
            ("password", "short"),
            ("confirm_password", "short"),
        ]))
    };
    let reply = client.send(register([192, 0, 2, 9])).await;
    assert!(!reply.body.contains("Too many requests"));

    // The fourth request from the same address is refused, with the same answer for resets.
    let reply = client.send(reset([192, 0, 2, 9], "ana@example.com")).await;
    assert_eq!(reply.status, Status::OK);
    no_mail_to("ana@example.com").await;
    let reply = client.send(register([192, 0, 2, 9])).await;
    assert!(reply.body.contains("Too many requests"));

    // Other addresses keep their own count.
    let reply = client.send(register([192, 0, 2, 10])).await;
    assert!(!reply.body.contains("Too many requests"));
}
