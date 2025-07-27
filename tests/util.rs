use pagetop::prelude::*;

use std::{fs, io};
use tempfile::TempDir;

#[cfg(unix)]
mod unix {
    use super::*;

    #[pagetop::test]
    async fn ok_absolute_dir() -> io::Result<()> {
        let _app = service::test::init_service(Application::new().test()).await;

        // /tmp/<rand>/sub
        let td = TempDir::new()?;
        let root = td.path();
        let sub = root.join("sub");
        fs::create_dir(&sub)?;

        let abs = util::absolute_dir(root, "sub")?;
        assert_eq!(abs, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn error_not_a_directory() -> io::Result<()> {
        let _app = service::test::init_service(Application::new().test()).await;

        let td = TempDir::new()?;
        let file = td.path().join("foo.txt");
        fs::write(&file, b"data")?;

        let err = util::absolute_dir(td.path(), "foo.txt").unwrap_err();
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
        let root = td.path();
        let sub = root.join("sub");
        fs::create_dir(&sub)?;

        let abs = util::absolute_dir(root, sub.as_path())?;
        assert_eq!(abs, std::fs::canonicalize(&sub)?);
        Ok(())
    }

    #[pagetop::test]
    async fn error_not_a_directory() -> io::Result<()> {
        let _app = service::test::init_service(Application::new().test()).await;

        let td = TempDir::new()?;
        let file = td.path().join("foo.txt");
        fs::write(&file, b"data")?;

        let err = util::absolute_dir(td.path(), "foo.txt").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        Ok(())
    }
}
