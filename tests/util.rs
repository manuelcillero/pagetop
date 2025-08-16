use pagetop::prelude::*;

use std::{env, fs, io};
use tempfile::TempDir;

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
