use pagetop::prelude::*;

use std::fs;

// `web::Upload` saves the files of a `multipart/form-data` body under `upload.dir` (set to
// `target/test-uploads` in `config/test.toml`). Each test uses its own subdirectory, so they can
// run in parallel.

const BOUNDARY: &str = "pagetop-test-boundary";

async fn save_all(upload: web::Upload, mut multipart: web::Multipart) -> String {
    let mut out = String::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match upload.save(field).await {
            Ok(Some(file)) => out.push_str(&format!(
                "saved {} {} {} {};",
                file.path(),
                file.original_name(),
                file.size(),
                file.content_type().map(String::as_str).unwrap_or("-"),
            )),
            Ok(None) => out.push_str("none;"),
            Err(web::UploadError::NotAFile(name)) => out.push_str(&format!("not-a-file {name};")),
            Err(web::UploadError::TooLarge { file, limit }) => {
                out.push_str(&format!("too-large {file} {limit};"))
            }
            Err(web::UploadError::ExtensionNotAllowed { file, allowed }) => {
                out.push_str(&format!("extension {file} {};", allowed.join(",")))
            }
            Err(err) => out.push_str(&format!("error {err};")),
        }
    }
    out
}

#[derive(Debug, serde::Deserialize)]
struct Note {
    title: String,
    #[serde(default)]
    tags: Vec<String>,
}

// Receives a note into `subdir`; files are kept unless the title is "discard".
async fn receive_note(subdir: String, multipart: web::Multipart) -> String {
    let upload = web::Upload::new()
        .with_subdir(subdir)
        .with_extensions(["txt"])
        .with_file_fields(["doc", "extra"])
        .with_max_files(2);
    match upload.receive::<Note>(multipart).await {
        Ok(sent) => {
            let mut out = format!(
                "title={};tags={};",
                sent.form().title,
                sent.form().tags.join(",")
            );
            for file in sent.files() {
                out.push_str(&format!("{}={};", file.field(), file.path()));
            }
            if sent.form().title != "discard" {
                sent.keep();
            }
            out
        }
        Err(err) => format!("error {err:?}"),
    }
}

async fn serve(request: HttpRequest, download: web::Download) -> Response {
    match download.serve(&request).await {
        Ok(response) => response,
        Err(web::UploadError::NotFound(_)) => web::http::StatusCode::NOT_FOUND.into_response(),
        Err(_) => web::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

struct Uploads;

#[async_trait]
impl Extension for Uploads {
    fn configure_router(&self, router: Router) -> Router {
        router
            .route(
                "/plain",
                web::post(|multipart: web::Multipart| {
                    save_all(web::Upload::new().with_subdir("plain"), multipart)
                }),
            )
            .route(
                "/images",
                web::post(|multipart: web::Multipart| {
                    let upload = web::Upload::new()
                        .with_subdir("images")
                        .with_extensions([".JPG", "png"]);
                    save_all(upload, multipart)
                }),
            )
            .route(
                "/small",
                web::post(|multipart: web::Multipart| {
                    let upload = web::Upload::new().with_subdir("small").with_max_size(4);
                    save_all(upload, multipart)
                }),
            )
            .route(
                "/escape",
                web::post(|multipart: web::Multipart| {
                    save_all(web::Upload::new().with_subdir("../escape"), multipart)
                }),
            )
            .route(
                "/receive/{subdir}",
                web::post(
                    |web::Path(subdir): web::Path<String>, multipart: web::Multipart| {
                        receive_note(subdir, multipart)
                    },
                ),
            )
            .route(
                "/form",
                web::post(
                    |web::Form(form): web::Form<Vec<(String, String)>>| async move {
                        form.len().to_string()
                    },
                ),
            )
            .route(
                "/files/{*path}",
                web::get(|request: HttpRequest, web::Path(path): web::Path<String>| {
                    serve(request, web::Download::new(path))
                }),
            )
            .route(
                "/named/{*path}",
                web::get(|request: HttpRequest, web::Path(path): web::Path<String>| {
                    serve(
                        request,
                        web::Download::new(path).with_file_name("Plan ñ.txt"),
                    )
                }),
            )
            .route(
                "/attach/{*path}",
                web::get(|request: HttpRequest, web::Path(path): web::Path<String>| {
                    serve(request, web::Download::new(path).with_attachment(true))
                }),
            )
    }
}

// Builds a multipart body; a part with a file name is a file, one without is a text field.
fn multipart_body(parts: &[(&str, Option<&str>, &str)]) -> String {
    let mut body = String::new();
    for (name, file_name, content) in parts {
        body.push_str(&format!("--{BOUNDARY}\r\n"));
        match file_name {
            Some(file_name) => body.push_str(&format!(
                "Content-Disposition: form-data; name=\"{name}\"; filename=\"{file_name}\"\r\n\
                 Content-Type: text/plain\r\n\r\n"
            )),
            None => body.push_str(&format!(
                "Content-Disposition: form-data; name=\"{name}\"\r\n\r\n"
            )),
        }
        body.push_str(content);
        body.push_str("\r\n");
    }
    body.push_str(&format!("--{BOUNDARY}--\r\n"));
    body
}

async fn router() -> Router {
    web::test::init_router(Application::prepare(&Uploads).await.test())
}

async fn post(route: &str, content_type: &str, body: String) -> Response {
    let request = web::test::TestRequest::post()
        .uri(route)
        .header("content-type", content_type)
        .body(body)
        .to_request();
    web::test::send_request(&router().await, request).await
}

async fn send(route: &str, parts: &[(&str, Option<&str>, &str)]) -> String {
    let content_type = format!("multipart/form-data; boundary={BOUNDARY}");
    let response = post(route, &content_type, multipart_body(parts)).await;
    assert_eq!(response.status(), web::http::StatusCode::OK);
    web::test::read_body_text(response).await
}

async fn get(route: &str, headers: &[(&str, &str)]) -> Response {
    let mut request = web::test::TestRequest::get().uri(route);
    for (name, value) in headers {
        request = request.header(name, value);
    }
    web::test::send_request(&router().await, request.to_request()).await
}

fn header<'a>(response: &'a Response, name: &str) -> &'a str {
    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
}

// Writes a file straight into the upload dir, as if it had been uploaded before.
fn stored(path: &str, content: &str) {
    let local = web::Upload::local_path(path).unwrap();
    fs::create_dir_all(local.parent().unwrap()).unwrap();
    fs::write(local, content).unwrap();
}

// A missing subdir counts as empty: rejected uploads may never create it.
fn is_empty_dir(subdir: &str) -> bool {
    fs::read_dir(web::Upload::local_path(subdir).unwrap()).map_or(true, |dir| dir.count() == 0)
}

fn stored_path(result: &str) -> &str {
    result
        .strip_prefix("saved ")
        .and_then(|rest| rest.split(' ').next())
        .unwrap_or_else(|| panic!("file not saved: {result}"))
}

#[pagetop::test]
async fn file_is_saved_with_a_random_name_and_its_extension() {
    let result = send("/plain", &[("doc", Some("../My Notes.TXT"), "hello")]).await;

    let path = stored_path(&result);
    assert!(path.starts_with("plain/"), "{result}");
    assert!(path.ends_with(".txt"), "{result}");
    assert!(!path.contains("Notes"), "{result}");
    assert!(result.ends_with(" My Notes.TXT 5 text/plain;"), "{result}");

    let local = web::Upload::local_path(path).unwrap();
    assert_eq!(fs::read_to_string(local).unwrap(), "hello");
}

#[pagetop::test]
async fn same_file_name_never_overwrites() {
    let first = send("/plain", &[("doc", Some("same.txt"), "one")]).await;
    let second = send("/plain", &[("doc", Some("same.txt"), "two")]).await;

    assert_ne!(stored_path(&first), stored_path(&second));
}

#[pagetop::test]
async fn empty_file_field_is_none_and_text_field_is_not_a_file() {
    let result = send(
        "/plain",
        &[("doc", Some(""), ""), ("title", None, "Lettuce")],
    )
    .await;

    assert_eq!(result, "none;not-a-file title;");
}

#[pagetop::test]
async fn only_allowed_extensions_are_saved() {
    let result = send(
        "/images",
        &[
            ("photo", Some("garden.jpg"), "jpg"),
            ("photo", Some("page.html"), "<b>x</b>"),
            ("photo", Some("noext"), "x"),
        ],
    )
    .await;

    let mut parts = result.split(';');
    assert!(
        parts.next().unwrap().starts_with("saved images/"),
        "{result}"
    );
    assert_eq!(
        parts.next(),
        Some("extension page.html jpg,png"),
        "{result}"
    );
    assert_eq!(parts.next(), Some("extension noext jpg,png"), "{result}");
}

#[pagetop::test]
async fn too_large_file_is_rejected_and_removed() {
    let result = send("/small", &[("doc", Some("big.txt"), "too large")]).await;

    assert_eq!(result, "too-large big.txt 4;");
    assert!(is_empty_dir("small"));
}

#[pagetop::test]
async fn subdir_cannot_escape_the_upload_dir() {
    let result = send("/escape", &[("doc", Some("a.txt"), "x")]).await;

    assert!(result.starts_with("error invalid upload path"), "{result}");
}

#[pagetop::test]
async fn unwritable_upload_dir_is_a_server_error() {
    // A regular file where the subdirectory should be makes `create_dir_all` fail.
    let blocked = web::Upload::local_path("blocked").unwrap();
    fs::create_dir_all(blocked.parent().unwrap()).unwrap();
    fs::write(&blocked, "not a directory").unwrap();

    let result = send("/receive/blocked", &[("doc", Some("a.txt"), "x")]).await;

    assert!(result.starts_with("error Io("), "{result}");
}

#[pagetop::test]
async fn remove_deletes_a_saved_file_and_ignores_missing_ones() {
    let result = send("/plain", &[("doc", Some("temp.txt"), "bye")]).await;
    let path = stored_path(&result);

    web::Upload::remove(path).await.unwrap();
    assert!(!web::Upload::local_path(path).unwrap().exists());
    web::Upload::remove(path).await.unwrap();
    assert!(matches!(
        web::Upload::remove("../config/test.toml").await,
        Err(web::UploadError::InvalidPath(_))
    ));
}

#[pagetop::test]
async fn local_path_rejects_paths_outside_the_upload_dir() {
    assert!(web::Upload::local_path("photos/a.jpg").is_ok());
    assert!(matches!(
        web::Upload::local_path("../config/test.toml"),
        Err(web::UploadError::InvalidPath(_))
    ));
    assert!(web::Upload::local_path("/etc/passwd").is_err());
    assert!(web::Upload::local_path("").is_err());
}

#[pagetop::test]
async fn receive_deserializes_text_fields_and_keeps_the_files() {
    let result = send(
        "/receive/receive-keep",
        &[
            ("title", None, "Lettuce"),
            ("tags", None, "green"),
            ("doc", Some("notes.txt"), "water daily"),
            ("tags", None, "leafy"),
            ("extra", Some(""), ""),
        ],
    )
    .await;

    assert!(
        result.starts_with("title=Lettuce;tags=green,leafy;doc=receive-keep/"),
        "{result}"
    );
    assert_eq!(result.matches(';').count(), 3, "{result}");
    let path = result.split("doc=").nth(1).unwrap().trim_end_matches(';');
    assert!(web::Upload::local_path(path).unwrap().exists(), "{result}");
}

#[pagetop::test]
async fn receive_removes_the_files_when_not_kept() {
    let result = send(
        "/receive/receive-discard",
        &[
            ("title", None, "discard"),
            ("doc", Some("notes.txt"), "draft"),
        ],
    )
    .await;

    assert!(result.contains("doc=receive-discard/"), "{result}");
    assert!(is_empty_dir("receive-discard"));
}

#[pagetop::test]
async fn receive_removes_the_saved_files_when_a_later_part_fails() {
    let result = send(
        "/receive/receive-error",
        &[
            ("title", None, "Lettuce"),
            ("doc", Some("notes.txt"), "ok"),
            ("extra", Some("page.html"), "<b>x</b>"),
        ],
    )
    .await;

    assert!(result.starts_with("error ExtensionNotAllowed"), "{result}");
    assert!(is_empty_dir("receive-error"));
}

#[pagetop::test]
async fn receive_limits_the_number_of_files() {
    let result = send(
        "/receive/receive-many",
        &[
            ("title", None, "Lettuce"),
            ("doc", Some("a.txt"), "a"),
            ("doc", Some("b.txt"), "b"),
            ("doc", Some("c.txt"), "c"),
        ],
    )
    .await;

    assert_eq!(result, "error TooManyFiles { limit: 2 }");
    assert!(is_empty_dir("receive-many"));
}

#[pagetop::test]
async fn receive_rejects_files_in_unexpected_fields() {
    let result = send(
        "/receive/receive-unexpected",
        &[("title", None, "Lettuce"), ("other", Some("a.txt"), "a")],
    )
    .await;

    assert_eq!(result, "error UnexpectedField(\"other\")");
    assert!(is_empty_dir("receive-unexpected"));
}

#[pagetop::test]
async fn receive_rejects_missing_text_fields() {
    let result = send(
        "/receive/receive-invalid",
        &[("doc", Some("notes.txt"), "no title")],
    )
    .await;

    assert!(result.starts_with("error InvalidForm"), "{result}");
    assert!(is_empty_dir("receive-invalid"));
}

#[pagetop::test]
async fn download_shows_plain_text_inline_with_safe_headers() {
    stored("download/notes.txt", "hello world");
    let response = get("/named/download/notes.txt", &[]).await;

    assert_eq!(response.status(), web::http::StatusCode::OK);
    assert!(header(&response, "content-type").starts_with("text/plain"));
    assert_eq!(
        header(&response, "content-disposition"),
        "inline; filename=\"Plan _.txt\"; filename*=UTF-8''Plan%20%C3%B1.txt"
    );
    assert_eq!(header(&response, "x-content-type-options"), "nosniff");
    assert_eq!(header(&response, "cache-control"), "private, no-cache");
    assert_eq!(web::test::read_body_text(response).await, "hello world");
}

#[pagetop::test]
async fn download_never_shows_active_content_inline() {
    stored("download/page.html", "<script>alert(1)</script>");
    let response = get("/files/download/page.html", &[]).await;

    assert_eq!(response.status(), web::http::StatusCode::OK);
    assert!(
        header(&response, "content-disposition").starts_with("attachment; filename=\"page.html\""),
        "{}",
        header(&response, "content-disposition")
    );
}

#[pagetop::test]
async fn multipart_requests_have_a_larger_body_limit() {
    // 3 MiB: over `server.max_body_size` (2 MiB), under `upload.max_request_size` (32 MiB).
    let big = "a".repeat(3 * 1024 * 1024);

    let form = post(
        "/form",
        "application/x-www-form-urlencoded",
        format!("big={big}"),
    )
    .await;
    assert_eq!(form.status(), web::http::StatusCode::PAYLOAD_TOO_LARGE);

    let result = send("/plain", &[("doc", Some("big.txt"), &big)]).await;
    assert!(stored_path(&result).starts_with("plain/"), "{result}");
}

#[pagetop::test]
async fn receive_limits_the_size_of_the_text_fields() {
    let big = "a".repeat(3 * 1024 * 1024);
    let result = send(
        "/receive/fields-limit",
        &[("doc", Some("a.txt"), "a"), ("title", None, &big)],
    )
    .await;

    assert!(result.starts_with("error FieldsTooLarge"), "{result}");
    assert!(is_empty_dir("fields-limit"));
}

#[pagetop::test]
async fn download_answers_conditional_requests() {
    stored("download/etag.txt", "hello");
    let response = get("/files/download/etag.txt", &[]).await;
    let etag = header(&response, "etag").to_owned();
    assert!(!etag.is_empty());

    let again = get("/files/download/etag.txt", &[("if-none-match", &etag)]).await;
    assert_eq!(again.status(), web::http::StatusCode::NOT_MODIFIED);
}

#[pagetop::test]
async fn download_can_force_an_attachment() {
    stored("download/forced.txt", "hello");
    let response = get("/attach/download/forced.txt", &[]).await;

    assert!(header(&response, "content-disposition").starts_with("attachment;"));
}

#[pagetop::test]
async fn download_supports_range_requests() {
    stored("download/range.txt", "hello world");
    let response = get("/files/download/range.txt", &[("range", "bytes=0-4")]).await;

    assert_eq!(response.status(), web::http::StatusCode::PARTIAL_CONTENT);
    assert_eq!(web::test::read_body_text(response).await, "hello");
}

#[pagetop::test]
async fn download_of_a_missing_or_escaping_path_fails() {
    let missing = get("/files/download/missing.txt", &[]).await;
    assert_eq!(missing.status(), web::http::StatusCode::NOT_FOUND);

    let escaping = get("/files/..%2Fconfig%2Ftest.toml", &[]).await;
    assert_eq!(
        escaping.status(),
        web::http::StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[pagetop::test]
async fn upload_errors_have_translated_messages() {
    let too_large = web::UploadError::TooLarge {
        file: "plan.txt".to_owned(),
        limit: 8 * 1024 * 1024,
    };
    let text = too_large
        .message()
        .lookup(&Locale::resolve("en-US"))
        .unwrap();
    assert!(
        text.contains("plan.txt") && text.contains("8 MiB"),
        "{text}"
    );

    let one_file = web::UploadError::TooManyFiles { limit: 1 };
    let text = one_file
        .message()
        .lookup(&Locale::resolve("es-ES"))
        .unwrap();
    assert!(text.contains("un archivo"), "{text}");

    let io = web::UploadError::Io(std::io::ErrorKind::PermissionDenied.into());
    assert!(!io.is_client_error());
    let text = io.message().lookup(&Locale::resolve("en-US")).unwrap();
    assert!(!text.contains("denied"), "{text}");
}
