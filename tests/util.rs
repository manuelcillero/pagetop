use pagetop::prelude::*;

use std::{borrow::Cow, fs, io};
use tempfile::TempDir;

async fn setup() {
    Application::new().await;
}

// **< Testing normalize_ascii() >******************************************************************

fn assert_err(input: &str, expected: util::NormalizeAsciiError) {
    let out = util::normalize_ascii(input);
    assert_eq!(
        out,
        Err(expected),
        "Input {:?} expected Err({:?}), got {:?}",
        input,
        expected,
        out
    );
}

fn assert_borrowed(input: &str, expected: &str) {
    let out = util::normalize_ascii(input).expect("Expected Ok(..)");
    assert_eq!(out.as_ref(), expected, "Input {:?}", input);
    assert!(
        matches!(out, Cow::Borrowed(_)),
        "Expected Cow::Borrowed, got {:?} for input {:?}",
        out,
        input
    );
}

fn assert_owned(input: &str, expected: &str) {
    let out = util::normalize_ascii(input).expect("Expected Ok(..)");
    assert_eq!(out.as_ref(), expected, "Input {:?}", input);
    assert!(
        matches!(out, Cow::Owned(_)),
        "Expected Cow::Owned, got {:?} for input {:?}",
        out,
        input
    );
}

#[pagetop::test]
async fn normalize_errors() {
    // Special case: empty string.
    assert_err("", util::NormalizeAsciiError::IsEmpty);

    // Only ASCII separators: nothing is left after trimming.
    for input in [" ", "   ", "\t", "\n", "\r", "\t \n\r  "] {
        assert_err(input, util::NormalizeAsciiError::EmptyAfterTrimming);
    }

    // Any non-ASCII byte must fail, even if the rest could be normalized.
    for input in [
        "©",
        "á",
        "😀",
        "a©b",
        "a b © c",
        "  Foo©BAR  ",
        "\tAáB\n",
        "x y😀",
    ] {
        assert_err(input, util::NormalizeAsciiError::NonAscii);
    }
}

#[pagetop::test]
async fn normalize_borrowed_trim_and_already_normalized() {
    // Trimming only (including trailing separators).
    for (input, expected) in [
        ("  a", "a"),
        ("a  ", "a"),
        (" \t\n a \r ", "a"),
        ("foo\t", "foo"),
        ("foo \t\r\n", "foo"),
        (" \n\tfoo\r", "foo"),
        ("\tfoo", "foo"),
        ("\nfoo", "foo"),
        ("\rfoo", "foo"),
        ("\t\r\nfoo\r\n\t", "foo"),
        ("foo\t\t\t", "foo"),
        ("foo\r\n", "foo"),
        ("foo \r\n\t", "foo"),
    ] {
        assert_borrowed(input, expected);
    }

    // Already normalized (lowercase and a single space between tokens).
    for input in [
        "a",
        "a b",
        "a b c",
        "foo bar baz",
        "btn",
        "btn btn-primary",
        "col-12 col-md-6",
        "username webauthn",
        "off",
        "on",
        "foo-bar",
        "foo_bar",
        "a.b,c",
        "path/to/resource",
        "foo+bar=baz",
        "a-._:/+=",
        "a\x1Bb", // ASCII control byte: preserved as-is.
    ] {
        assert_borrowed(input, input);
    }

    // "Unusual" separator at the end of the string: it is trimmed and a slice is returned.
    for (input, expected) in [
        ("foo bar\t", "foo bar"),
        ("foo bar\r\n", "foo bar"),
        ("foo bar \r\n", "foo bar"),
    ] {
        assert_borrowed(input, expected);
    }
}

#[pagetop::test]
async fn normalize_owned_due_to_uppercase() {
    // Only due to uppercase (and other ASCII that is preserved).
    for (input, expected) in [
        ("A", "a"),
        ("Foo", "foo"),
        ("FOO BAR", "foo bar"),
        ("a B c", "a b c"),
        ("ABC", "abc"),
        ("abcDEF", "abcdef"),
        ("Abc-Def_Ghi", "abc-def_ghi"),
        ("X.Y,Z", "x.y,z"),
        ("Foo-Bar", "foo-bar"),
        ("FOO_BAR", "foo_bar"),
        ("A.B,C", "a.b,c"),
        ("HTTP/2", "http/2"),
        ("ETag:W/\"XYZ\"", "etag:w/\"xyz\""),
        ("Foo+Bar=Baz", "foo+bar=baz"),
        ("A-._:/+=", "a-._:/+="),
        ("A\x1BB", "a\x1bb"), // Only letters get lowercased; the control byte is preserved.
    ] {
        assert_owned(input, expected);
    }
}

#[pagetop::test]
async fn normalize_owned_due_to_internal_whitespace() {
    // Consecutive spaces (must collapse to a single space).
    for (input, expected) in [("a  b", "a b"), ("a   b", "a b")] {
        assert_owned(input, expected);
    }

    // ASCII separators other than ' ' between tokens (tab, newline, CR, CRLF).
    for (input, expected) in [
        ("a\tb", "a b"),
        ("a\nb", "a b"),
        ("a\rb", "a b"),
        ("a\r\nb", "a b"),
        ("foo\tbar", "foo bar"),
        ("foo\nbar", "foo bar"),
        ("foo\rbar", "foo bar"),
        ("foo\r\nbar", "foo bar"),
    ] {
        assert_owned(input, expected);
    }

    // Mixed separators.
    for (input, expected) in [
        ("a \t \n  b", "a b"),
        ("a\t  \n b", "a b"),
        ("foo \tbar", "foo bar"),
        ("foo\t bar", "foo bar"),
        ("foo\t\tbar", "foo bar"),
        ("foo \n\t\r  bar", "foo bar"),
    ] {
        assert_owned(input, expected);
    }

    // The result must never have leading/trailing spaces (after normalizing).
    for (input, expected) in [
        ("  a  b  ", "a b"),
        ("  a\tb  ", "a b"),
        ("  a\nb  ", "a b"),
    ] {
        assert_owned(input, expected);
    }
}

#[pagetop::test]
async fn normalize_owned_due_to_mixed_causes() {
    // Combinations of uppercase and non-normalized separators.
    for (input, expected) in [
        ("  Foo   BAR\tbaz  ", "foo bar baz"),
        ("\nFOO\rbar\tBAZ\n", "foo bar baz"),
        ("FOO\tbar", "foo bar"),
        ("foo\tBAR", "foo bar"),
        ("FOO\tBAR", "foo bar"),
        ("Foo  BAR\tBaz", "foo bar baz"),
        ("x\t y ", "x y"),
        ("x  y\t", "x y"),
    ] {
        assert_owned(input, expected);
    }
}

#[pagetop::test]
async fn normalize_borrowed_vs_owned_edge_cases() {
    // A single token with a trailing separator.
    for (input, expected) in [("x ", "x"), ("x\t", "x"), ("x\n", "x"), ("x\r\n", "x")] {
        assert_borrowed(input, expected);
    }

    // Two tokens with a non-normalized separator.
    for input in ["x  y", "x\t\ty", "x \t y", "x\r\ny"] {
        assert_owned(input, "x y");
    }

    // Two tokens with a clean separator.
    for (input, expected) in [("x y ", "x y"), ("x y\t", "x y"), ("x y\r\n", "x y")] {
        assert_borrowed(input, expected);
    }
}

#[pagetop::test]
async fn normalize_is_idempotent() {
    // Normalization must be idempotent: normalizing the result changes nothing.
    let cases = [
        "a",
        "a b c",
        "foo-bar",
        "foo_bar",
        "a.b,c",
        "  Foo   BAR\tbaz  ",
        "foo\tbar",
        "x y\t",
        "\tfoo\r\n",
        "a\x1Bb",
        "HTTP/2",
    ];

    for &input in &cases {
        // All are ASCII, but this check is kept in case the list is expanded in the future.
        if !input.is_ascii() {
            continue;
        }

        let first = util::normalize_ascii(input).unwrap();
        let second = util::normalize_ascii(first.as_ref()).unwrap();
        assert_eq!(
            first.as_ref(),
            second.as_ref(),
            "Idempotency failed for input {:?}: first={:?} second={:?}",
            input,
            first.as_ref(),
            second.as_ref()
        );
    }
}

// **< Testing resolve_absolute_dir() >*************************************************************

#[cfg(unix)]
mod unix {
    use super::*;

    #[pagetop::test]
    async fn ok_absolute_dir() -> io::Result<()> {
        setup().await;

        // /tmp/<rand>/sub
        let td = TempDir::new()?;
        let sub = td.path().join("sub");
        fs::create_dir(&sub)?;

        let abs = util::resolve_absolute_dir(&sub)?;
        assert_eq!(abs, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn ok_relative_dir_with_manifest() -> io::Result<()> {
        setup().await;

        let td = TempDir::new()?;
        let sub = td.path().join("sub");
        fs::create_dir(&sub)?;

        let res = util::resolve_absolute_dir_with_base("sub", Some(td.path().to_path_buf()));

        assert_eq!(res?, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn error_not_a_directory() -> io::Result<()> {
        setup().await;

        let td = TempDir::new()?;
        let file = td.path().join("foo.txt");
        fs::write(&file, b"data")?;

        let err = util::resolve_absolute_dir(&file).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        Ok(())
    }
}

#[cfg(windows)]
mod windows {
    use super::*;

    #[pagetop::test]
    async fn ok_absolute_dir() -> io::Result<()> {
        setup().await;

        // C:\Users\...\Temp\...
        let td = TempDir::new()?;
        let sub = td.path().join("sub");
        fs::create_dir(&sub)?;

        let abs = util::resolve_absolute_dir(&sub)?;
        assert_eq!(abs, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn ok_relative_dir_with_manifest() -> io::Result<()> {
        setup().await;

        let td = TempDir::new()?;
        let sub = td.path().join("sub");
        fs::create_dir(&sub)?;

        let res = resolve_absolute_dir_with_base("sub", Some(td.path().to_path_buf()));

        assert_eq!(res?, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn error_not_a_directory() -> io::Result<()> {
        setup().await;

        let td = TempDir::new()?;
        let file = td.path().join("foo.txt");
        fs::write(&file, b"data")?;

        let err = util::resolve_absolute_dir(&file).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        Ok(())
    }
}
