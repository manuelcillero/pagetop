use std::path::Path;

pub fn except_mdbook_common_resources(p: &Path) -> bool {
    let common_resources: [&str; 28] = [
        "css/chrome.css",
        "css/general.css",
        "css/print.css",
        "css/variables.css",
        "FontAwesome/css/font-awesome.css",
        "FontAwesome/fonts/fontawesome-webfont.eot",
        "FontAwesome/fonts/fontawesome-webfont.svg",
        "FontAwesome/fonts/fontawesome-webfont.ttf",
        "FontAwesome/fonts/fontawesome-webfont.woff",
        "FontAwesome/fonts/fontawesome-webfont.woff2",
        "FontAwesome/fonts/FontAwesome.ttf",
        "fonts/fonts.css",
        "fonts/OPEN-SANS-LICENSE.txt",
        "fonts/open-sans-v17-all-charsets-300.woff2",
        "fonts/open-sans-v17-all-charsets-300italic.woff2",
        "fonts/open-sans-v17-all-charsets-600.woff2",
        "fonts/open-sans-v17-all-charsets-600italic.woff2",
        "fonts/open-sans-v17-all-charsets-700.woff2",
        "fonts/open-sans-v17-all-charsets-700italic.woff2",
        "fonts/open-sans-v17-all-charsets-800.woff2",
        "fonts/open-sans-v17-all-charsets-800italic.woff2",
        "fonts/open-sans-v17-all-charsets-italic.woff2",
        "fonts/open-sans-v17-all-charsets-regular.woff2",
        "fonts/SOURCE-CODE-PRO-LICENSE.txt",
        "fonts/source-code-pro-v11-all-charsets-500.woff2",
        "ayu-highlight.css",
        "highlight.css",
        "tomorrow-night.css",
    ];
    !common_resources.iter().any(|f| p.ends_with(f))
}
