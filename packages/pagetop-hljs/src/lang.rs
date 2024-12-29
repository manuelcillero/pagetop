use pagetop::prelude::*;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

use std::fmt;

/// Supported coding languages.
///
/// Languages are represented by *PascalCase* enums within the code and are mapped to corresponding
/// [highlight.js](https://highlightjs.org/) language names.
///
/// ```rust
/// use pagetop_hljs::HljsLang;
///
/// assert_eq!(HljsLang::CoffeeScript.to_string(), "coffeescript".to_string());
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HljsLang {
    // Common languages.
    Bash,
    C,
    Cpp,
    Csharp,
    CSS,
    Diff,
    Go,
    GraphQL,
    HTML,
    INI,
    Java,
    JavaScript,
    JSON,
    Kotlin,
    Less,
    Lua,
    Makefile,
    Markdown,
    ObjectiveC,
    Perl,
    PHP,
    PHPTemplate,
    #[default]
    Plaintext,
    Python,
    PythonREPL,
    R,
    Ruby,
    Rust,
    SCSS,
    ShellSession,
    SQL,
    Swift,
    TOML,
    TypeScript,
    VisualBasicNET,
    WebAssembly,
    XML,
    /// Enum variants for languages ranging from `Bash` to `YAML` are all preloaded in the
    /// ***common*** mode. To include additional languages, use the default ***core*** mode.
    ///
    /// See [`config::SETTINGS.hljs.mode`](crate::config::Hljs#structfield.mode).
    YAML,
    // Additional languages.
    ActionScript,
    Ada,
    Apache,
    AppleScript,
    Arduino,
    ARMAssembly,
    AsciiDoc,
    AspectJ,
    AutoHotkey,
    AVRAssembly,
    Awk,
    BASIC,
    Clojure,
    ClojureREPL,
    CMake,
    CoffeeScript,
    Crystal,
    D,
    Dart,
    Delphy,
    Django,
    DNSZone,
    Dockerfile,
    DOS,
    Elixir,
    Elm,
    ERB,
    Erlang,
    ErlangREPL,
    Fortran,
    Fsharp,
    Handlebars,
    Haskell,
    HTTP,
    Julia,
    JuliaREPL,
    LaTeX,
    Lisp,
    LLVMIR,
    Matlab,
    Nginx,
    NodeREPL,
    Ocaml,
    PostgreSQL,
    PowerShell,
    Prolog,
    Properties,
    Scala,
    Scheme,
    Scilab,
    Smalltalk,
    Tcl,
    Twig,
    VBScript,
    X86Asm,
}

static HLJS_LANGS: LazyLock<HashMap<HljsLang, &'static str>> = LazyLock::new(|| {
    use HljsLang::*;
    hm![
        // Common languages.
        Bash           => "bash",
        C              => "c",
        Cpp            => "cpp",
        Csharp         => "csharp",
        CSS            => "css",
        Diff           => "diff",
        Go             => "go",
        GraphQL        => "graphql",
        HTML           => "html,xml",
        INI            => "ini",
        Java           => "java",
        JavaScript     => "javascript",
        JSON           => "json",
        Kotlin         => "kotlin",
        Less           => "less",
        Lua            => "lua",
        Makefile       => "makefile",
        Markdown       => "markdown",
        ObjectiveC     => "objectivec",
        Perl           => "perl",
        PHP            => "php",
        PHPTemplate    => "php-template",
        Plaintext      => "plaintext",
        Python         => "python",
        PythonREPL     => "python-repl",
        R              => "r",
        Ruby           => "ruby",
        Rust           => "rust",
        SCSS           => "scss",
        ShellSession   => "shell",
        SQL            => "sql",
        Swift          => "swift",
        TOML           => "toml,ini",
        TypeScript     => "typescript",
        VisualBasicNET => "vbnet",
        WebAssembly    => "wasm",
        XML            => "xml",
        YAML           => "yaml",
        // Additional languages.
        ActionScript   => "actionscript",
        Ada            => "ada",
        Apache         => "apache",
        AppleScript    => "applescript",
        Arduino        => "arduino",
        ARMAssembly    => "armasm",
        AsciiDoc       => "asciidoc",
        AspectJ        => "aspectj",
        AutoHotkey     => "autohotkey",
        AVRAssembly    => "avrasm",
        Awk            => "awk",
        BASIC          => "basic",
        Clojure        => "clojure",
        ClojureREPL    => "clojure-repl",
        CMake          => "cmake",
        CoffeeScript   => "coffeescript",
        Crystal        => "crystal",
        D              => "d",
        Dart           => "dart",
        Delphy         => "delphy",
        Django         => "django",
        DNSZone        => "dns",
        Dockerfile     => "dockerfile",
        DOS            => "dos",
        Elixir         => "elixir",
        Elm            => "elm",
        ERB            => "erb",
        Erlang         => "erlang",
        ErlangREPL     => "erlang-repl",
        Fortran        => "fortran",
        Fsharp         => "fsharp",
        Handlebars     => "handlebars",
        Haskell        => "haskell",
        HTTP           => "http",
        Julia          => "julia",
        JuliaREPL      => "julia-repl",
        LaTeX          => "latex",
        Lisp           => "lisp",
        LLVMIR         => "llvm",
        Matlab         => "matlab",
        Nginx          => "nginx",
        NodeREPL       => "node-repl",
        Ocaml          => "ocaml",
        PostgreSQL     => "pgsql",
        PowerShell     => "powershell",
        Prolog         => "prolog",
        Properties     => "properties",
        Scala          => "scala",
        Scheme         => "scheme",
        Scilab         => "scilab",
        Smalltalk      => "smalltalk",
        Tcl            => "tcl",
        Twig           => "twig",
        VBScript       => "vbscript",
        X86Asm         => "x86asm",
    ]
});

impl ToString for HljsLang {
    fn to_string(&self) -> String {
        String::from(*HLJS_LANGS.get(self).unwrap())
    }
}

impl FromStr for HljsLang {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HLJS_LANGS
            .iter()
            .find_map(|(&key, &value)| if value == s { Some(key) } else { None })
            .ok_or_else(|| fmt::Error)
    }
}

impl HljsLang {
    pub(crate) fn to_url(language: impl Into<String>) -> String {
        let language = language.into();
        join_string!("/hljs/js/languages/", language, ".min.js")
    }
}
