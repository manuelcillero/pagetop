// The tests run with `config/common.toml` of this crate: memory transport, a default sender and
// `https://example.com/` as base URL. Tests in a binary run in parallel and share the outbox, so
// each test uses its own recipient and collects its emails with `outbox::take_to()`.

use pagetop::prelude::*;
use pagetop_mail::outbox::{self, SentEmail};
use pagetop_mail::prelude::*;

async fn send_one(email: Email, recipient: &str) -> SentEmail {
    email.send().await.unwrap();
    let mut sent = outbox::take_to(recipient);
    assert_eq!(sent.len(), 1, "expected exactly one email for {recipient}");
    sent.remove(0)
}

// **< Mailbox >************************************************************************************

#[pagetop::test]
async fn mailbox_parses_address_with_and_without_name() {
    let plain = Mailbox::from("  ana@example.com ");
    assert_eq!(plain.name(), None);
    assert_eq!(plain.address(), "ana@example.com");

    let named = Mailbox::from("Ana Pérez <ana@example.com>");
    assert_eq!(named.name().map(String::as_str), Some("Ana Pérez"));
    assert_eq!(named.address(), "ana@example.com");

    let quoted = Mailbox::from("\"Pérez, Ana\" <ana@example.com>");
    assert_eq!(quoted.name().map(String::as_str), Some("Pérez, Ana"));

    let tuple = Mailbox::from(("  ", "ana@example.com"));
    assert_eq!(tuple.name(), None, "a blank name must be dropped");

    assert_eq!(named.to_string(), "Ana Pérez <ana@example.com>");
    assert_eq!(plain.to_string(), "ana@example.com");
}

#[pagetop::test]
async fn mailbox_validates_its_address() {
    assert!(
        Mailbox::from("Ana Pérez <ana@example.com>")
            .validate()
            .is_ok()
    );
    for invalid in [
        "",
        "ana",
        "ana@",
        "@example.com",
        "ana @example.com",
        "a@b@example.com",
    ] {
        assert!(
            matches!(
                Mailbox::new(invalid).validate(),
                Err(MailError::InvalidAddress { .. })
            ),
            "{invalid:?} must be rejected"
        );
    }
}

#[pagetop::test]
async fn control_characters_in_names_cannot_inject_headers() {
    // `lettre` panics on a header with control characters; the name may come from user data.
    let mailbox = Mailbox::with_name("Ana\r\nBcc: evil@example.com", "control@example.com");
    assert_eq!(
        mailbox.name().map(String::as_str),
        Some("Ana  Bcc: evil@example.com")
    );

    let sent = send_one(
        Email::new()
            .with_to(mailbox)
            .with_subject("Hi\r\nBcc: evil@example.com")
            .with_text("Hi"),
        "control@example.com",
    )
    .await;
    assert_eq!(sent.envelope_to(), &["control@example.com".to_string()]);
    assert!(!sent.raw().contains("\r\nBcc:"));
    // A subject is a single line: control characters become spaces.
    assert_eq!(sent.email().subject(), "Hi  Bcc: evil@example.com");
}

// **< Composition >********************************************************************************

#[pagetop::test]
async fn text_email_uses_default_sender_and_reaches_every_recipient() {
    let sent = send_one(
        Email::new()
            .with_to("text-to@example.com")
            .with_cc("text-cc@example.com")
            .with_bcc("text-bcc@example.com")
            .with_subject("Plain text")
            .with_text("Hello"),
        "text-to@example.com",
    )
    .await;

    let from = sent.email().from().unwrap();
    assert_eq!(from.address(), "tests@example.com");
    assert_eq!(sent.envelope_to().len(), 3);
    assert!(sent.is_delivered_to("TEXT-BCC@example.com"));
    assert!(sent.raw().contains("Subject: Plain text"));
    assert!(
        sent.raw()
            .lines()
            .any(|line| line.starts_with("Message-ID: <") && line.ends_with("@example.com>")),
        "Message-ID must use the sender domain"
    );
    assert!(sent.raw().contains("Content-Type: text/plain"));
    // Bcc recipients get the email but must not appear in its headers.
    assert!(!sent.raw().contains("text-bcc@example.com"));
}

#[pagetop::test]
async fn explicit_sender_overrides_default_sender() {
    let sent = send_one(
        Email::new()
            .with_from(("Support", "support@example.com"))
            .with_reply_to("help@example.com")
            .with_to("override@example.com")
            .with_text("Hi"),
        "override@example.com",
    )
    .await;

    assert_eq!(
        sent.email().from().unwrap().address(),
        "support@example.com"
    );
    assert!(sent.raw().contains("From: Support <support@example.com>"));
    assert!(sent.raw().contains("Reply-To: help@example.com"));
}

#[pagetop::test]
async fn text_and_html_are_sent_as_alternatives() {
    let sent = send_one(
        Email::new()
            .with_to("alternative@example.com")
            .with_subject("Both")
            .with_text("Text version")
            .with_html(html! { p { "HTML version" } }),
        "alternative@example.com",
    )
    .await;

    assert!(sent.raw().contains("multipart/alternative"));
    assert!(sent.raw().contains("text/plain"));
    assert!(sent.raw().contains("text/html"));
}

#[pagetop::test]
async fn html_content_is_wrapped_in_the_default_layout() {
    let sent = send_one(
        Email::new()
            .with_to("layout@example.com")
            .with_subject("Layout <test>")
            .with_langid(&Locale::resolve("es-ES"))
            .with_html(html! { p { "Body content" } }),
        "layout@example.com",
    )
    .await;

    let html = sent.email().html().unwrap().as_str();
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains(r#"<html lang="es-ES">"#));
    assert!(html.contains("<title>Layout &lt;test&gt;</title>"));
    assert!(html.contains("<p>Body content</p>"));
    assert!(
        html.contains("Mensaje enviado por"),
        "footer must follow the email language"
    );
}

#[pagetop::test]
async fn html_document_is_sent_without_layout() {
    let sent = send_one(
        Email::new()
            .with_to("document@example.com")
            .with_html_document(html! { html { body { "Own document" } } }),
        "document@example.com",
    )
    .await;

    assert_eq!(
        sent.email().html().unwrap().as_str(),
        "<html><body>Own document</body></html>"
    );
}

// **< Attachments >********************************************************************************

#[pagetop::test]
async fn attachments_are_sent_as_mixed_parts_with_guessed_content_type() {
    let sent = send_one(
        Email::new()
            .with_to("attachment@example.com")
            .with_text("See attached")
            .with_attachment(Attachment::new("report.pdf", b"%PDF-1.4".to_vec()))
            .with_attachment(
                Attachment::new("data.bin", vec![0u8, 1, 2]).with_content_type("text/csv"),
            ),
        "attachment@example.com",
    )
    .await;

    let raw = sent.raw();
    assert!(raw.contains("multipart/mixed"));
    assert!(raw.contains("Content-Type: application/pdf"));
    assert!(raw.contains(r#"filename="report.pdf""#));
    assert!(raw.contains("Content-Type: text/csv"));
}

#[pagetop::test]
async fn inline_images_are_related_to_the_html() {
    let sent = send_one(
        Email::new()
            .with_to("inline@example.com")
            .with_html(html! { img src="cid:logo"; })
            .with_attachment(
                Attachment::new("logo.png", vec![137u8, 80, 78, 71]).with_content_id("logo"),
            ),
        "inline@example.com",
    )
    .await;

    let raw = sent.raw();
    assert!(raw.contains("multipart/related"));
    assert!(raw.contains("Content-ID: <logo>"));
    assert!(!raw.contains("multipart/mixed"));
}

#[pagetop::test]
async fn inline_images_without_html_are_sent_as_attachments() {
    let sent = send_one(
        Email::new()
            .with_to("inline-text@example.com")
            .with_text("No HTML")
            .with_attachment(Attachment::new("logo.png", vec![1u8]).with_content_id("logo")),
        "inline-text@example.com",
    )
    .await;

    assert!(sent.raw().contains("multipart/mixed"));
    assert!(!sent.raw().contains("multipart/related"));
}

#[pagetop::test]
async fn attachment_reads_file_from_disk() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    let attachment = Attachment::from_path(path).await.unwrap();
    assert_eq!(attachment.filename(), "Cargo.toml");
    assert!(!attachment.body().is_empty());
    assert!(!attachment.is_inline());

    let missing = Attachment::from_path("does/not/exist.pdf").await;
    assert!(matches!(missing, Err(MailError::ReadAttachment { .. })));
}

#[pagetop::test]
async fn invalid_content_type_fails_on_send() {
    let result = Email::new()
        .with_to("bad-type@example.com")
        .with_attachment(Attachment::new("x", vec![]).with_content_type("not a type"))
        .send()
        .await;
    assert!(matches!(result, Err(MailError::InvalidContentType(_))));
}

// **< Errors >*************************************************************************************

#[pagetop::test]
async fn email_without_recipients_fails() {
    let result = Email::new().with_text("Nobody").send().await;
    assert!(matches!(result, Err(MailError::NoRecipients)));
}

#[pagetop::test]
async fn invalid_address_fails() {
    let result = Email::new().with_to("not-an-address").send().await;
    let Err(MailError::InvalidAddress { address, .. }) = result else {
        panic!("expected an invalid address error, got {result:?}");
    };
    assert_eq!(address, "not-an-address");
}

// **< Templates >**********************************************************************************

struct Greeting {
    name: String,
}

impl MailTemplate for Greeting {
    fn subject(&self, cx: &Context) -> String {
        util::join!("Hello ", &self.name, " [", cx.langid().to_string(), "]")
    }

    fn html(&self, _cx: &Context) -> Option<Markup> {
        Some(html! { p { "Hello " (self.name) } })
    }

    fn text(&self, _cx: &Context) -> Option<String> {
        Some(util::join!("Hello ", &self.name))
    }
}

#[pagetop::test]
async fn template_builds_subject_html_and_text_in_the_context_language() {
    let cx = Context::default().with_langid(&Locale::resolve("es-ES"));
    let email = Email::from_template(
        &Greeting {
            name: "Ana".to_string(),
        },
        &cx,
    )
    .with_to("template@example.com");

    assert_eq!(email.subject(), "Hello Ana [es-ES]");
    assert_eq!(email.text().map(String::as_str), Some("Hello Ana"));
    assert_eq!(email.langid().to_string(), "es-ES");

    let sent = send_one(email, "template@example.com").await;
    let html = sent.email().html().unwrap().as_str();
    assert!(html.contains(r#"<html lang="es-ES">"#));
    assert!(html.contains("<p>Hello Ana</p>"));
}

// **< URLs >***************************************************************************************

#[pagetop::test]
async fn absolute_url_prepends_the_configured_base_url() {
    assert_eq!(
        absolute_url("/user/verify"),
        "https://example.com/user/verify"
    );
    assert_eq!(absolute_url("user"), "https://example.com/user");
    assert_eq!(
        absolute_url(RoutePath::new("/search").with_param("q", "mail")),
        "https://example.com/search?q=mail"
    );
    assert_eq!(absolute_url("https://other.org/x"), "https://other.org/x");
}

// **< Plain text from HTML >***********************************************************************

#[pagetop::test]
async fn html_only_email_gets_plain_text_from_its_content() {
    let sent = send_one(
        Email::new()
            .with_to("auto-text@example.com")
            .with_html(html! {
                p { "Hello " strong { "Ana" } }
                p { a href="https://example.com/x" { "Open" } }
            }),
        "auto-text@example.com",
    )
    .await;

    // Generated from the content, before the layout: the footer is not part of it.
    assert_eq!(
        sent.email().text().map(String::as_str),
        Some("Hello Ana\n\nOpen (https://example.com/x)")
    );
    assert!(sent.raw().contains("multipart/alternative"));
}

#[pagetop::test]
async fn explicit_text_is_not_replaced() {
    let sent = send_one(
        Email::new()
            .with_to("own-text@example.com")
            .with_text("Own text")
            .with_html(html! { p { "HTML" } }),
        "own-text@example.com",
    )
    .await;

    assert_eq!(sent.email().text().map(String::as_str), Some("Own text"));
}
