// End-to-end tests of the generated administration of a catalog, against an in-memory SQLite
// database (`config/common.toml`, with 3 records per page).
//
// Two catalogs: `families`, and `colors`, whose `family_id` references a family with a foreign key.
// Permissions come from the `x-role` header: `manager` has every permission, `viewer` only the
// `view_*` ones, and without it nothing is granted. A recorder keeps the changes in memory and
// offers the history, and a guard refuses to deactivate or delete the color with code `LOCKED`.
//
// Every scenario shares the same database, so they run one after another in a single test.

use pagetop::prelude::*;
use pagetop_catalog::{
    Catalog, CatalogBag, ChangeAction, ChangeRecorder, ChangeSet, DeclareCatalogs,
    DeclareChangeRecorders, DeclareDeactivationGuards, Field, Guard, GuardBag, GuardResult,
    HistoryEntry, RecorderBag,
};
use pagetop_seaorm::db::{DatabaseTransaction, DbErr};
use pagetop_seaorm::install_migrations;

use std::sync::Mutex;

// **< Test application >***************************************************************************

mod migration {
    pub mod m20260101_000001_create_families {
        use pagetop_catalog::schema::{CommonColumns, catalog_table};
        use pagetop_seaorm::migration::*;

        pub struct Migration;

        #[pagetop::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                let columns = CommonColumns {
                    code_max_len: Some(8),
                    translations: false,
                };
                manager
                    .create_table(catalog_table(Families::Table, columns))
                    .await
            }
        }

        #[derive(DeriveIden)]
        enum Families {
            Table,
        }
    }

    pub mod m20260101_000002_create_colors {
        use pagetop_catalog::schema::{CommonColumns, catalog_table};
        use pagetop_seaorm::migration::*;

        pub struct Migration;

        #[pagetop::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                let columns = CommonColumns {
                    code_max_len: Some(8),
                    translations: true,
                };
                manager
                    .create_table(
                        catalog_table(Colors::Table, columns)
                            .col(string_len_null(Colors::Hex, 7))
                            .col(boolean(Colors::IsDark).default(false))
                            .col(integer_null(Colors::Shade))
                            .col(string_len_null(Colors::Finish, 16))
                            .col(integer_null(Colors::FamilyId))
                            .foreign_key(
                                ForeignKey::create()
                                    .from(Colors::Table, Colors::FamilyId)
                                    .to(Alias::new("families"), Alias::new("id"))
                                    .on_delete(ForeignKeyAction::Restrict),
                            )
                            .to_owned(),
                    )
                    .await
            }
        }

        #[derive(DeriveIden)]
        enum Colors {
            Table,
            Hex,
            IsDark,
            Shade,
            Finish,
            FamilyId,
        }
    }
}

mod sizes_migration {
    pub mod m20260101_000003_create_sizes {
        use pagetop_catalog::schema::{CommonColumns, catalog_table};
        use pagetop_seaorm::migration::*;

        pub struct Migration;

        #[pagetop::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                let columns = CommonColumns {
                    code_max_len: None,
                    translations: false,
                };
                manager
                    .create_table(
                        catalog_table(Sizes::Table, columns)
                            .col(integer(Sizes::FamilyId))
                            .col(double_null(Sizes::Inches))
                            .index(
                                Index::create()
                                    .unique()
                                    .col(Sizes::FamilyId)
                                    .col(Alias::new("name")),
                            )
                            .to_owned(),
                    )
                    .await
            }
        }

        #[derive(DeriveIden)]
        enum Sizes {
            Table,
            FamilyId,
            Inches,
        }
    }
}

fn declare_sections(bag: &mut AdminSectionBag) {
    bag.add(AdminSection::new("tests", Lc::n("Tests")));
}

fn declare_catalogs(bag: &mut CatalogBag) {
    bag.add(
        Catalog::new("sizes", "tests", Lc::n("Sizes"))
            .without_code()
            .with_unique(&["family_id", "name"])
            .with_qualifier("family_id", |name, family| format!("{name} ({family})"))
            .with_field(
                Field::reference("family_id", Lc::n("Family"), "families")
                    .with_required(true)
                    .with_filter(true)
                    .with_in_list(true),
            )
            .with_field(
                Field::decimal("inches", Lc::n("Inches"))
                    .with_decimal_range(1.0, None)
                    .with_in_list(true),
            ),
    );
    bag.add(Catalog::new("families", "tests", Lc::n("Families")).with_code_max_len(8));
    bag.add(
        Catalog::new("colors", "tests", Lc::n("Colors"))
            .with_permission_prefix("tests")
            .with_code_max_len(8)
            .with_translations(true)
            .with_field(
                Field::text("hex", Lc::n("Hex"))
                    .with_max_len(7)
                    .with_in_list(true),
            )
            .with_field(Field::boolean("is_dark", Lc::n("Dark")))
            .with_field(Field::integer("shade", Lc::n("Shade")).with_range(0, 100))
            .with_field(Field::choice(
                "finish",
                Lc::n("Finish"),
                vec![("matte", Lc::n("Matte")), ("gloss", Lc::n("Gloss"))],
            ))
            .with_field(
                Field::reference("family_id", Lc::n("Family"), "families").with_in_list(true),
            )
            .with_validator(|record| {
                if record.name().eq_ignore_ascii_case("forbidden") {
                    Err(Lc::n("That name is forbidden."))
                } else {
                    Ok(())
                }
            }),
    );
}

// Changes recorded, as "<catalog> <action> <code> <columns>", and the history entries.
static CHANGES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static HISTORY: Mutex<Vec<(String, i32, HistoryEntry)>> = Mutex::new(Vec::new());

struct MemoryRecorder;

#[async_trait]
impl ChangeRecorder for MemoryRecorder {
    async fn record(&self, _txn: &DatabaseTransaction, set: &ChangeSet<'_>) -> Result<(), DbErr> {
        for change in &set.changes {
            let columns: Vec<&str> = change.fields.iter().map(|f| f.column.as_str()).collect();
            CHANGES.lock().unwrap().push(format!(
                "{} {} {} {}",
                set.catalog.name(),
                change.action.as_str(),
                change.code,
                columns.join(",")
            ));
            HISTORY.lock().unwrap().push((
                set.catalog.name().to_owned(),
                change.record_id,
                HistoryEntry {
                    at: Utc::now(),
                    user: Some("tester".to_owned()),
                    action: change.action,
                    fields: change.fields.clone(),
                    comment: None,
                },
            ));
        }
        Ok(())
    }

    async fn history(
        &self,
        catalog: &Catalog,
        record_id: i32,
    ) -> Result<Option<Vec<HistoryEntry>>, DbErr> {
        let history = HISTORY.lock().unwrap();
        let mut entries: Vec<HistoryEntry> = history
            .iter()
            .filter(|(name, id, _)| name == catalog.name() && *id == record_id)
            .map(|(_, _, entry)| entry.clone())
            .collect();
        entries.reverse();
        Ok(Some(entries))
    }
}

fn declare_recorders(bag: &mut RecorderBag) {
    bag.add(&MemoryRecorder);
}

async fn locked_color(id: i32) -> GuardResult {
    Ok((id == *LOCKED_ID.lock().unwrap()).then(|| Lc::n("The color is locked.")))
}

static LOCKED_ID: Mutex<i32> = Mutex::new(-1);

fn declare_guards(bag: &mut GuardBag) {
    bag.add(Guard::new("colors", locked_color));
}

fn check_permission(request: &HttpRequest, perm: PermissionRef, granted: &mut bool) {
    let role = request
        .headers()
        .get("x-role")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    *granted = match role {
        "manager" => true,
        "viewer" => perm.key().contains(":view_"),
        _ => false,
    };
}

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_catalog::Catalogs]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::admin::DeclareAdminSections::new(declare_sections),
            action::auth::CheckPermission::new(check_permission),
            DeclareCatalogs::new(declare_catalogs),
            DeclareChangeRecorders::new(declare_recorders),
            DeclareDeactivationGuards::new(declare_guards),
        ]
    }

    async fn initialize(&self) -> Result<(), BoxError> {
        install_migrations!(
            m20260101_000001_create_families,
            m20260101_000002_create_colors,
        )?;
        {
            use sizes_migration as migration;
            install_migrations!(m20260101_000003_create_sizes)?;
        }
        Ok(())
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
    fn location(&self) -> &str {
        self.headers
            .get("location")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
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

    async fn get(&self, uri: &str, role: &str) -> Reply {
        self.send(
            web::test::TestRequest::get()
                .uri(uri)
                .header("x-role", role),
        )
        .await
    }

    async fn post(&self, uri: &str, fields: &[(&str, &str)], role: &str) -> Reply {
        let request = web::test::TestRequest::post()
            .uri(uri)
            .header("x-role", role)
            .form(form(fields));
        self.send(request).await
    }

    async fn htmx_post(&self, uri: &str, role: &str) -> Reply {
        let request = web::test::TestRequest::post()
            .uri(uri)
            .header("x-role", role)
            .header("hx-request", "true")
            .form("");
        self.send(request).await
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

// A multipart body with `content` in the field `name`: a file (`data.csv`) or a text field.
fn multipart(name: &str, file: bool, content: &str) -> (String, String) {
    let boundary = "----catalog-test-boundary";
    let disposition = if file {
        format!("form-data; name=\"{name}\"; filename=\"data.csv\"\r\nContent-Type: text/csv")
    } else {
        format!("form-data; name=\"{name}\"")
    };
    let body = format!(
        "--{boundary}\r\nContent-Disposition: {disposition}\r\n\r\n{content}\r\n--{boundary}--\r\n"
    );
    (format!("multipart/form-data; boundary={boundary}"), body)
}

impl Client {
    async fn send_multipart(&self, uri: &str, name: &str, file: bool, content: &str) -> Reply {
        let (content_type, body) = multipart(name, file, content);
        let request = web::test::TestRequest::post()
            .uri(uri)
            .header("x-role", "manager")
            .header("content-type", content_type)
            .body(body);
        self.send(request).await
    }

    // Uploads a CSV to preview its import.
    async fn upload(&self, uri: &str, content: &str) -> Reply {
        self.send_multipart(uri, "file", true, content).await
    }

    // Confirms an import, sending the CSV again as the preview form does.
    async fn confirm(&self, uri: &str, csv: &str) -> Reply {
        self.send_multipart(uri, "csv", false, csv).await
    }
}

fn changes() -> Vec<String> {
    CHANGES.lock().unwrap().clone()
}

// Id of the record with the code `code`, taken from the edit link of its row in the list.
async fn id_of(client: &Client, list: &str, code: &str) -> i32 {
    let reply = client
        .get(&format!("{list}?status=all&q={code}"), "manager")
        .await;
    let marker = format!("{list}/");
    reply
        .body
        .match_indices(&marker)
        .find_map(|(at, _)| {
            reply.body[at + marker.len()..]
                .split('/')
                .next()
                .and_then(|id| id.parse().ok())
        })
        .unwrap_or_else(|| panic!("no record {code} in {}", reply.body))
}

// **< Scenarios >**********************************************************************************

#[pagetop::test]
async fn catalog_administration() {
    let app = Application::prepare(&TestApp).await.test();
    let client = Client { app };

    permissions_are_derived_from_the_catalog(&client).await;
    families_are_created(&client).await;
    a_record_is_created_with_its_fields(&client).await;
    invalid_input_is_reported(&client).await;
    the_list_searches_filters_and_pages(&client).await;
    an_edit_is_recorded_and_stale_edits_are_refused(&client).await;
    deactivation_hides_and_guards_refuse(&client).await;
    deletion_is_protected(&client).await;
    viewers_only_view(&client).await;
    a_csv_is_exported_and_imported(&client).await;
    a_catalog_without_code_works_by_its_unique_columns(&client).await;
    the_history_lists_the_changes(&client).await;
    an_inactive_reference_is_kept_but_not_offered(&client).await;
}

async fn permissions_are_derived_from_the_catalog(client: &Client) {
    let registry = PermissionRegistry::global();
    let keys: Vec<String> = registry.all().map(|p| p.key().into_owned()).collect();
    for key in [
        "catalog:view_families",
        "catalog:manage_families",
        "tests:view_colors",
        "tests:manage_colors",
    ] {
        assert!(keys.iter().any(|k| k == key), "{key} not declared");
    }
    let label = pagetop_catalog::catalog("colors")
        .unwrap()
        .manage_permission()
        .label()
        .get();
    assert_eq!(label.as_deref(), Some("Manage Colors"));

    // Without permissions, nothing is reachable.
    assert_eq!(
        client.get("/admin/tests/colors", "").await.status,
        Status::FORBIDDEN
    );
}

async fn families_are_created(client: &Client) {
    for (code, name) in [("WARM", "Warm"), ("COLD", "Cold")] {
        let reply = client
            .post(
                "/admin/tests/families/new",
                &[("code", code), ("name", name)],
                "manager",
            )
            .await;
        assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
        assert!(reply.location().ends_with("/admin/tests/families"));
    }
    assert!(changes().contains(&"families create WARM code,name,weight".to_owned()));
}

async fn a_record_is_created_with_its_fields(client: &Client) {
    let warm = id_of(client, "/admin/tests/families", "WARM")
        .await
        .to_string();
    let reply = client
        .post(
            "/admin/tests/colors/new",
            &[
                ("code", "RED"),
                ("name", "Red"),
                ("hex", "#ff0000"),
                ("is_dark", "true"),
                ("shade", "50"),
                ("finish", "gloss"),
                ("family_id", &warm),
                ("weight", "5"),
                ("i18n-es-ES", "Rojo"),
                ("save_and_new", "1"),
            ],
            "manager",
        )
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
    // "Save & New" opens an empty creation form.
    assert!(reply.location().ends_with("/admin/tests/colors/new"));
    assert!(
        changes().contains(
            &"colors create RED code,name,weight,name_i18n,hex,is_dark,shade,finish,family_id"
                .to_owned()
        )
    );

    let list = client.get("/admin/tests/colors", "manager").await;
    assert!(list.body.contains("RED"));
    assert!(list.body.contains("#ff0000"));
    assert!(list.body.contains("WARM - Warm"));

    // The translation is shown in its language.
    let list = client
        .get("/admin/tests/colors?lang=es-ES", "manager")
        .await;
    assert!(list.body.contains("Rojo"), "{}", list.body);
}

async fn invalid_input_is_reported(client: &Client) {
    let before = changes().len();
    let cases: [(&[(&str, &str)], &str); 8] = [
        (
            &[("code", "RED"), ("name", "Again")],
            "Another record already has the same Code.",
        ),
        (&[("code", ""), ("name", "No code")], "Code is required."),
        (
            &[("code", "A B"), ("name", "Spaces")],
            "The code cannot contain spaces.",
        ),
        (
            &[("code", "TOOLONGCODE"), ("name", "Long")],
            "Code can have at most 8 characters.",
        ),
        (&[("code", "X1"), ("name", "")], "Name is required."),
        (
            &[("code", "X2"), ("name", "Shade"), ("shade", "101")],
            "Shade is out of the allowed range.",
        ),
        (
            &[("code", "X3"), ("name", "Finish"), ("finish", "satin")],
            "Finish has a value that is not allowed.",
        ),
        (
            &[("code", "X4"), ("name", "Forbidden")],
            "That name is forbidden.",
        ),
    ];
    for (fields, message) in cases {
        let reply = client
            .post("/admin/tests/colors/new", fields, "manager")
            .await;
        assert_eq!(reply.status, Status::OK);
        assert!(reply.body.contains(message), "{message}: {}", reply.body);
    }
    // A reference to a record that does not exist.
    let reply = client
        .post(
            "/admin/tests/colors/new",
            &[("code", "X5"), ("name", "Ghost"), ("family_id", "999")],
            "manager",
        )
        .await;
    assert!(
        reply
            .body
            .contains("Family has a value that is not allowed.")
    );
    assert_eq!(changes().len(), before, "nothing invalid is recorded");
}

async fn the_list_searches_filters_and_pages(client: &Client) {
    for (code, name) in [("BLUE", "Blue"), ("NAVY", "Navy blue"), ("SKY", "Sky blue")] {
        let reply = client
            .post(
                "/admin/tests/colors/new",
                &[("code", code), ("name", name)],
                "manager",
            )
            .await;
        assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
    }
    // Four colors, three per page.
    let first = client.get("/admin/tests/colors?sort=code", "manager").await;
    assert!(first.body.contains("BLUE") && first.body.contains("NAVY"));
    assert!(!first.body.contains("SKY"));
    let second = client
        .get("/admin/tests/colors?sort=code&page=2", "manager")
        .await;
    assert!(second.body.contains("SKY") && !second.body.contains("BLUE"));

    // Search in code or name, case-insensitive; `*` is a wildcard.
    let found = client.get("/admin/tests/colors?q=BLUE", "manager").await;
    assert!(found.body.contains("NAVY") && found.body.contains("SKY"));
    assert!(!found.body.contains(">RED<"));
    let found = client.get("/admin/tests/colors?q=s*", "manager").await;
    assert!(found.body.contains("SKY") && !found.body.contains("NAVY"));
    // `%` is a literal character, not a wildcard.
    let found = client.get("/admin/tests/colors?q=%25", "manager").await;
    assert!(found.body.contains("There are no records."));
}

async fn an_edit_is_recorded_and_stale_edits_are_refused(client: &Client) {
    let id = id_of(client, "/admin/tests/colors", "BLUE").await;
    let edit = format!("/admin/tests/colors/{id}/edit");
    let form = client.get(&edit, "manager").await;
    assert!(
        form.body.contains(r#"name="row_version" value="1""#),
        "{}",
        form.body
    );

    let reply = client
        .post(
            &edit,
            &[
                ("code", "CHANGED"),
                ("name", "Blue sea"),
                ("row_version", "1"),
            ],
            "manager",
        )
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
    // The code does not change; only the name is recorded as changed.
    assert!(changes().contains(&"colors update BLUE name".to_owned()));
    let list = client.get("/admin/tests/colors?q=sea", "manager").await;
    assert!(list.body.contains("BLUE") && !list.body.contains("CHANGED"));

    // Someone else's version: the change is refused and the current values are shown.
    let before = changes().len();
    let reply = client
        .post(&edit, &[("name", "Lost"), ("row_version", "1")], "manager")
        .await;
    assert_eq!(reply.status, Status::OK);
    assert!(reply.body.contains("Someone else has changed this record"));
    assert!(reply.body.contains("Blue sea") && reply.body.contains(r#"value="2""#));
    assert_eq!(changes().len(), before);

    // Saving without changes records nothing.
    let reply = client
        .post(
            &edit,
            &[("name", "Blue sea"), ("row_version", "2")],
            "manager",
        )
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
    assert_eq!(changes().len(), before);
}

async fn deactivation_hides_and_guards_refuse(client: &Client) {
    let id = id_of(client, "/admin/tests/colors", "SKY").await;
    let reply = client
        .htmx_post(&format!("/admin/tests/colors/{id}/deactivate"), "manager")
        .await;
    assert_eq!(reply.status, Status::OK);
    assert!(reply.body.contains("The record has been deactivated."));
    assert!(changes().contains(&"colors deactivate SKY is_active".to_owned()));

    let active = client.get("/admin/tests/colors?q=SKY", "manager").await;
    assert!(active.body.contains("There are no records."));
    let inactive = client
        .get("/admin/tests/colors?q=SKY&status=inactive", "manager")
        .await;
    assert!(inactive.body.contains("SKY") && inactive.body.contains("Reactivate"));

    let reply = client
        .htmx_post(&format!("/admin/tests/colors/{id}/reactivate"), "manager")
        .await;
    assert!(reply.body.contains("The record has been reactivated."));

    // A guard refuses to deactivate it, with its reason.
    *LOCKED_ID.lock().unwrap() = id;
    let before = changes().len();
    let reply = client
        .htmx_post(&format!("/admin/tests/colors/{id}/deactivate"), "manager")
        .await;
    assert!(
        reply.body.contains("The color is locked."),
        "{}",
        reply.body
    );
    assert_eq!(changes().len(), before);
    *LOCKED_ID.lock().unwrap() = -1;

    // Without HTMX, back to the list with the message.
    let reply = client
        .post(
            &format!("/admin/tests/colors/{id}/deactivate"),
            &[],
            "manager",
        )
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER);
}

async fn deletion_is_protected(client: &Client) {
    // A family used by a color cannot be deleted: the foreign key refuses it.
    let warm = id_of(client, "/admin/tests/families", "WARM").await;
    let before = changes().len();
    let reply = client
        .htmx_post(&format!("/admin/tests/families/{warm}/delete"), "manager")
        .await;
    assert!(
        reply.body.contains("The record is in use"),
        "{}",
        reply.body
    );
    assert_eq!(changes().len(), before);

    // One that nobody uses is deleted.
    let cold = id_of(client, "/admin/tests/families", "COLD").await;
    let reply = client
        .htmx_post(&format!("/admin/tests/families/{cold}/delete"), "manager")
        .await;
    assert!(reply.body.contains("The record has been deleted."));
    assert!(changes().contains(&"families delete COLD ".to_owned()));
}

async fn viewers_only_view(client: &Client) {
    let list = client.get("/admin/tests/colors", "viewer").await;
    assert_eq!(list.status, Status::OK);
    assert!(!list.body.contains("/admin/tests/colors/new"));
    assert!(!list.body.contains("Deactivate"));

    let id = id_of(client, "/admin/tests/colors", "RED").await;
    let view = client
        .get(&format!("/admin/tests/colors/{id}/edit"), "viewer")
        .await;
    assert_eq!(view.status, Status::OK);
    assert!(view.body.contains("disabled"));
    assert!(!view.body.contains("row_version"));

    for uri in [
        "/admin/tests/colors/new".to_owned(),
        format!("/admin/tests/colors/{id}/edit"),
        format!("/admin/tests/colors/{id}/deactivate"),
        format!("/admin/tests/colors/{id}/delete"),
    ] {
        let reply = client.post(&uri, &[("name", "x")], "viewer").await;
        assert_eq!(reply.status, Status::FORBIDDEN, "{uri}");
    }
}

async fn the_history_lists_the_changes(client: &Client) {
    let id = id_of(client, "/admin/tests/colors", "BLUE").await;
    let reply = client
        .get(&format!("/admin/tests/colors/{id}/history"), "viewer")
        .await;
    assert_eq!(reply.status, Status::OK);
    assert!(reply.body.contains("Modified") && reply.body.contains("Created"));
    assert!(reply.body.contains("Blue -&gt; Blue sea"), "{}", reply.body);
    let entries: Vec<_> = HISTORY
        .lock()
        .unwrap()
        .iter()
        .filter(|(c, i, _)| c == "colors" && *i == id)
        .map(|(_, _, e)| e.action)
        .collect();
    assert_eq!(entries, [ChangeAction::Create, ChangeAction::Update]);

    // The values are translated into the language of the request, not the default one.
    let red = id_of(client, "/admin/tests/colors", "RED").await;
    let reply = client
        .get(
            &format!("/admin/tests/colors/{red}/history?lang=es-ES"),
            "viewer",
        )
        .await;
    assert!(reply.body.contains("Dark: - -&gt; Sí"), "{}", reply.body);
}

async fn a_csv_is_exported_and_imported(client: &Client) {
    let export = client
        .get("/admin/tests/colors/export.csv?status=all", "viewer")
        .await;
    assert_eq!(export.status, Status::OK);
    assert!(
        export
            .headers
            .get("content-disposition")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.starts_with("attachment; filename=\"colors-"))
    );
    // With the date and the time: `colors-YYYY-MM-DD_HH-MM-SS.csv`.
    let disposition = export.headers["content-disposition"].to_str().unwrap();
    let stamp = &disposition["attachment; filename=\"colors-".len()..disposition.len() - 5];
    assert_eq!(stamp.len(), 19, "{disposition}");
    assert!(
        stamp.chars().enumerate().all(|(i, c)| match i {
            4 | 7 | 13 | 16 => c == '-',
            10 => c == '_',
            _ => c.is_ascii_digit(),
        }),
        "{disposition}"
    );
    let text = export.body.trim_start_matches('\u{FEFF}');
    let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
    assert_eq!(
        lines[0],
        "code,name,weight,is_active,hex,is_dark,shade,finish,family_id,name_i18n:en-GB,name_i18n:es-ES"
    );
    // The reference goes by the code of the family.
    let red = lines.iter().position(|l| l.starts_with("RED,")).unwrap();
    assert!(lines[red].contains(",WARM,"), "{}", lines[red]);

    // The help of the file comes from the definition: its required and key columns, and the limits
    // (the size of the catalog, 64 KiB here, below the one of the server).
    let form = client.get("/admin/tests/colors/import", "manager").await;
    assert!(form.body.contains("Required: Code, Name;"), "{}", form.body);
    assert!(
        form.body.contains("identify each record: Code;"),
        "{}",
        form.body
    );
    assert!(
        form.body.contains("At most 5000 rows and 64 KiB per file."),
        "{}",
        form.body
    );
    let form = client.get("/admin/tests/sizes/import", "manager").await;
    assert!(
        form.body.contains("identify each record: Family, Name;"),
        "{}",
        form.body
    );

    // The export follows the order of the list too, not only its filter.
    let sorted = client
        .get(
            "/admin/tests/colors/export.csv?status=all&sort=code&dir=desc",
            "viewer",
        )
        .await;
    let codes: Vec<&str> = sorted
        .body
        .trim_start_matches('\u{FEFF}')
        .lines()
        .skip(1)
        .map(|l| l.split(',').next().unwrap_or_default())
        .collect();
    let mut descending = codes.clone();
    descending.sort_by(|a, b| b.cmp(a));
    assert!(codes.len() > 1 && codes == descending, "{codes:?}");

    // One change, one new record, the rest unchanged.
    lines[red] = lines[red].replacen("RED,Red,", "RED,Crimson,", 1);
    lines.push("PINK,Pink,7,true,,false,,matte,WARM,,Rosa".to_owned());
    let csv = lines.join("\n");
    let preview = client.upload("/admin/tests/colors/import", &csv).await;
    assert_eq!(preview.status, Status::OK, "{}", preview.body);
    assert!(
        preview.body.contains("1 to create, 1 to change"),
        "{}",
        preview.body
    );
    assert!(preview.body.contains("Will change: Name."));
    let before = changes().len();
    assert!(preview.body.contains("/admin/tests/colors/import/confirm"));
    // Sent again as multipart: as a urlencoded field the CSV would grow up to three times.
    assert!(preview.body.contains("multipart/form-data"));

    // A CSV is read whole in memory, so it cannot exceed `import_max_size` (64 KiB here), in either
    // step.
    let big = format!("{csv}\n{}", "x".repeat(70 * 1024));
    for reply in [
        client.upload("/admin/tests/colors/import", &big).await,
        client
            .confirm("/admin/tests/colors/import/confirm", &big)
            .await,
    ] {
        assert_eq!(reply.status, Status::OK);
        assert!(
            reply.body.contains("The file is too large: at most 64 KiB"),
            "{}",
            reply.body
        );
    }

    // Errors in a row: nothing can be confirmed.
    let wrong = format!("{csv}\nGREY,Grey,0,true,,maybe,,,NOPE,,");
    let reply = client.upload("/admin/tests/colors/import", &wrong).await;
    assert!(
        reply.body.contains("Dark has a value that is not allowed."),
        "{}",
        reply.body
    );
    assert!(
        reply
            .body
            .contains("Family has a value that is not allowed.")
    );
    assert!(!reply.body.contains("import/confirm"));
    let reply = client
        .upload("/admin/tests/colors/import", "code,name,colour\nA,B,C")
        .await;
    assert!(reply.body.contains("Unknown column"), "{}", reply.body);

    // Confirmed: everything in one operation, recorded as an import.
    let reply = client
        .confirm("/admin/tests/colors/import/confirm", &csv)
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
    let recorded = changes()[before..].to_vec();
    assert!(
        recorded.contains(&"colors import RED name".to_owned()),
        "{recorded:?}"
    );
    assert!(
        recorded
            .iter()
            .any(|c| c.starts_with("colors import PINK code,")),
        "{recorded:?}"
    );
    let list = client.get("/admin/tests/colors?q=PINK", "manager").await;
    assert!(list.body.contains("Pink"));

    // Importing again changes nothing.
    let before = changes().len();
    let preview = client.upload("/admin/tests/colors/import", &csv).await;
    assert!(
        preview.body.contains("0 to create, 0 to change"),
        "{}",
        preview.body
    );
    assert_eq!(changes().len(), before);
}

async fn a_catalog_without_code_works_by_its_unique_columns(client: &Client) {
    let warm = id_of(client, "/admin/tests/families", "WARM")
        .await
        .to_string();
    let post = |name: &'static str, inches: &'static str| {
        let warm = warm.clone();
        async move {
            client
                .post(
                    "/admin/tests/sizes/new",
                    &[("family_id", &warm), ("name", name), ("inches", inches)],
                    "manager",
                )
                .await
        }
    };
    // No code: the form has no code field, and the comma is a decimal separator.
    let form = client.get("/admin/tests/sizes/new", "manager").await;
    assert!(!form.body.contains(r#"name="code""#));
    let reply = post("Small", "10,5").await;
    assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
    assert!(changes().contains(&"sizes create Small name,weight,family_id,inches".to_owned()));

    // The combination of family and name cannot repeat.
    let reply = post("Small", "11").await;
    assert!(
        reply
            .body
            .contains("Another record already has the same combination of Family and Name."),
        "{}",
        reply.body
    );
    for (inches, message) in [
        ("abc", "Inches must be a number."),
        ("0,5", "Inches is out of the allowed range."),
    ] {
        let reply = post("Big", inches).await;
        assert!(reply.body.contains(message), "{inches}: {}", reply.body);
    }

    // The list has no code column, shows the decimal, filters by family and names the record with
    // its qualifier.
    let list = client
        .get(&format!("/admin/tests/sizes?f_family_id={warm}"), "manager")
        .await;
    assert!(list.body.contains("10.5"), "{}", list.body);
    assert!(list.body.contains("Small (Warm)"), "{}", list.body);
    assert!(list.body.contains(r#"name="f_family_id""#));
    let other = client
        .get("/admin/tests/sizes?f_family_id=999", "manager")
        .await;
    assert!(other.body.contains("There are no records."));

    // The CSV identifies the record by family (its code) and name.
    let export = client.get("/admin/tests/sizes/export.csv", "manager").await;
    let text = export.body.trim_start_matches('\u{FEFF}');
    assert!(
        text.starts_with("name,weight,is_active,family_id,inches"),
        "{text}"
    );
    assert!(text.contains("Small,0,true,WARM,10.5"), "{text}");
    let csv = "name,family_id,inches\nSmall,WARM,12\nLarge,WARM,30\n";
    let preview = client.upload("/admin/tests/sizes/import", csv).await;
    assert!(
        preview.body.contains("1 to create, 1 to change"),
        "{}",
        preview.body
    );
    let reply = client
        .confirm("/admin/tests/sizes/import/confirm", csv)
        .await;
    assert_eq!(reply.status, Status::SEE_OTHER, "{}", reply.body);
    assert!(changes().contains(&"sizes import Small inches".to_owned()));
}

// A deactivated family is not offered for a new color, but the color that already has it keeps
// it, marked as inactive.
async fn an_inactive_reference_is_kept_but_not_offered(client: &Client) {
    let warm = id_of(client, "/admin/tests/families", "WARM").await;
    let red = id_of(client, "/admin/tests/colors", "RED").await;
    let reply = client
        .htmx_post(
            &format!("/admin/tests/families/{warm}/deactivate"),
            "manager",
        )
        .await;
    assert!(reply.body.contains("The record has been deactivated."));
    let option = format!(r#"<option value="{warm}""#);

    let new = client.get("/admin/tests/colors/new", "manager").await;
    assert!(!new.body.contains(&option), "{}", new.body);

    let edit = client
        .get(&format!("/admin/tests/colors/{red}/edit"), "manager")
        .await;
    assert!(edit.body.contains(&option), "{}", edit.body);
    assert!(edit.body.contains("Warm (inactive)"), "{}", edit.body);
}
