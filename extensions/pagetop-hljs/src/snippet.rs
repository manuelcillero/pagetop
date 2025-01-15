//! Add a new component to put code snippets on web pages.

use pagetop::prelude::*;

use crate::context::HljsContext;
use crate::lang::HljsLang;

#[derive(AutoDefault)]
/// Component to put code snippets on web pages.
pub struct HljsSnippet {
    language: HljsLang,
    snippet: String,
}

impl ComponentTrait for HljsSnippet {
    fn new() -> Self {
        HljsSnippet::default()
    }

    fn setup_before_prepare(&mut self, cx: &mut Context) {
        cx.add_hljs_language(self.language());
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            pre {
                code class=(join_string!("language-", self.language().to_string())) {
                    (self.snippet())
                }
            }
        })
    }
}

impl HljsSnippet {
    pub fn with(language: HljsLang, code: impl Into<String>) -> Self {
        HljsSnippet::new()
            .with_language(language)
            .with_snippet(code)
    }

    // Hljs BUILDER.

    #[fn_builder]
    pub fn with_language(mut self, language: HljsLang) -> Self {
        self.language = language;
        self
    }

    #[fn_builder]
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = snippet.into().trim().to_string();
        self
    }

    // Hljs GETTERS.

    pub fn language(&self) -> &HljsLang {
        &self.language
    }

    pub fn snippet(&self) -> &String {
        &self.snippet
    }
}
