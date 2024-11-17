use pagetop_aliner::{TEMPLATE_BASE_DIR, TEMPLATE_GLOB};

use tera::Tera;

/// Test to ensure Tera can initialize templates from the file system in debug mode.
#[test]
fn aliner_initialization_in_debug_mode() {
    let tera = Tera::new(TEMPLATE_GLOB);
    assert!(tera.is_ok(), "Failed to initialize Tera in debug mode");
}

/// Test to ensure templates embedded in the binary can be properly loaded in release mode.
#[test]
fn aliner_initialization_in_release_mode() {
    for file in TEMPLATE_BASE_DIR.files() {
        let content = file.contents_utf8();
        assert!(
            content.is_some(),
            "File {:?} contains non-UTF-8 content",
            file.path()
        );
    }
}
