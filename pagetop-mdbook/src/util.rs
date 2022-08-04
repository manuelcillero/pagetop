use std::path::Path;

pub fn except_mdbook_common_resources(p: &Path) -> bool {
    match p.to_str() {
        Some("ayu-highlight.css") => false,
        Some("highlight.css") => false,
        Some("tomorrow-niht.css") => false,
        _ => {
            if let Some(parent) = p.parent() {
                match parent.to_str() {
                    Some("/css") => false,
                    Some("/FontAwesome") => false,
                    Some("/fonts") => false,
                    _ => true,
                }
            } else {
                true
            }
        }
    }
}
