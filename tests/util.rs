use pagetop::prelude::*;

use std::{borrow::Cow, env, fs, io};
use tempfile::TempDir;

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
    // Caso especial: cadena vacía.
    assert_err("", util::NormalizeAsciiError::IsEmpty);

    // Sólo separadores ASCII: tras el recorte no queda nada.
    for input in [" ", "   ", "\t", "\n", "\r", "\t \n\r  "] {
        assert_err(input, util::NormalizeAsciiError::EmptyAfterTrimming);
    }

    // Cualquier byte no-ASCII debe fallar, aunque el resto pueda normalizarse.
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
    // Sólo recorte (incluyendo separadores al final).
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

    // Ya normalizado (minúsculas y un único espacio entre tokens).
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
        "a\x1Bb", // Byte de control ASCII: se conserva tal cual.
    ] {
        assert_borrowed(input, input);
    }

    // Separador "raro" al final de la cadena: se recorta y se devuelve porción.
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
    // Sólo por mayúsculas (y otros ASCII que se preservan).
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
        ("A\x1BB", "a\x1bb"), // Sólo letras en minúsculas; el byte de control se conserva.
    ] {
        assert_owned(input, expected);
    }
}

#[pagetop::test]
async fn normalize_owned_due_to_internal_whitespace() {
    // Espacios consecutivos (deben colapsar a un único espacio).
    for (input, expected) in [("a  b", "a b"), ("a   b", "a b")] {
        assert_owned(input, expected);
    }

    // Separadores ASCII distintos de ' ' entre tokens (tab, newline, CR, CRLF).
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

    // Mezclas de separadores.
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

    // El resultado nunca debe tener espacios al inicio/fin (tras normalizar).
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
    // Combinaciones de mayúsculas y separador no normalizado.
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
    // Un sólo token con separador al final.
    for (input, expected) in [("x ", "x"), ("x\t", "x"), ("x\n", "x"), ("x\r\n", "x")] {
        assert_borrowed(input, expected);
    }

    // Dos tokens con separador no normalizado.
    for input in ["x  y", "x\t\ty", "x \t y", "x\r\ny"] {
        assert_owned(input, "x y");
    }

    // Dos tokens con separación limpia.
    for (input, expected) in [("x y ", "x y"), ("x y\t", "x y"), ("x y\r\n", "x y")] {
        assert_borrowed(input, expected);
    }
}

#[pagetop::test]
async fn normalize_is_idempotent() {
    // La normalización debe ser idempotente: normalizar el resultado no cambia nada.
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
        // Todos son ASCII, pero se deja este control por si se amplía la lista en el futuro.
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
        let _app = service::test::init_service(Application::new().test()).await;

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
        let _app = service::test::init_service(Application::new().test()).await;

        let td = TempDir::new()?;
        let sub = td.path().join("sub");
        fs::create_dir(&sub)?;

        // Fija CARGO_MANIFEST_DIR para que "sub" se resuelva contra td.path()
        let prev_manifest_dir = env::var_os("CARGO_MANIFEST_DIR");
        env::set_var("CARGO_MANIFEST_DIR", td.path());
        let res = util::resolve_absolute_dir("sub");
        // Restaura entorno.
        match prev_manifest_dir {
            Some(v) => env::set_var("CARGO_MANIFEST_DIR", v),
            None => env::remove_var("CARGO_MANIFEST_DIR"),
        }

        assert_eq!(res?, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn error_not_a_directory() -> io::Result<()> {
        let _app = service::test::init_service(Application::new().test()).await;

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
        let _app = service::test::init_service(Application::new().test()).await;

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
        let _app = service::test::init_service(Application::new().test()).await;

        let td = TempDir::new()?;
        let sub = td.path().join("sub");
        fs::create_dir(&sub)?;

        // Fija CARGO_MANIFEST_DIR para que "sub" se resuelva contra td.path()
        let prev_manifest_dir = env::var_os("CARGO_MANIFEST_DIR");
        env::set_var("CARGO_MANIFEST_DIR", td.path());
        let res = util::resolve_absolute_dir("sub");
        // Restaura entorno.
        match prev_manifest_dir {
            Some(v) => env::set_var("CARGO_MANIFEST_DIR", v),
            None => env::remove_var("CARGO_MANIFEST_DIR"),
        }

        assert_eq!(res?, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn error_not_a_directory() -> io::Result<()> {
        let _app = service::test::init_service(Application::new().test()).await;

        let td = TempDir::new()?;
        let file = td.path().join("foo.txt");
        fs::write(&file, b"data")?;

        let err = util::resolve_absolute_dir(&file).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        Ok(())
    }
}
