use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::io::{self, Read};
use std::iter::Iterator;
use std::path::{Path, PathBuf};

/// Describes where the file is sourced.
pub trait FileSource: Debug + Clone {
    fn resolve(&self) -> Result<(Option<String>, String), Box<dyn Error + Send + Sync>>;
}

/// Describes a file sourced from a file.
#[derive(Clone, Debug)]
pub struct FileSourceFile {
    /// Path of configuration file.
    name: PathBuf,
}

impl FileSourceFile {
    pub fn new(name: PathBuf) -> FileSourceFile {
        FileSourceFile { name }
    }

    fn find_file(&self) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        // First check for an _exact_ match.
        let mut filename = env::current_dir()?.as_path().join(self.name.clone());
        if filename.is_file() {
            if ["toml"].contains(
                &filename
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            ) {
                return Ok(filename);
            }

            Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "configuration file \"{}\" is not of a registered file format",
                    filename.to_string_lossy()
                ),
            )))
        } else {
            filename.set_extension("toml");

            if filename.is_file() {
                return Ok(filename);
            }

            Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "configuration file \"{}\" not found",
                    self.name.to_string_lossy()
                ),
            )))
        }
    }
}

impl FileSource for FileSourceFile {
    fn resolve(&self) -> Result<(Option<String>, String), Box<dyn Error + Send + Sync>> {
        // Find file.
        let filename = self.find_file()?;

        // Attempt to use a relative path for the URI.
        let base = env::current_dir()?;
        let uri = match path_relative_from(&filename, &base) {
            Some(value) => value,
            None => filename.clone(),
        };

        // Read contents from file.
        let mut file = fs::File::open(filename)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        Ok((Some(uri.to_string_lossy().into_owned()), text))
    }
}

// TODO: This should probably be a crate.
// https://github.com/rust-lang/rust/blob/master/src/librustc_trans/back/rpath.rs#L128
fn path_relative_from(path: &Path, base: &Path) -> Option<PathBuf> {
    use std::path::Component;

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(Component::CurDir)) => comps.push(a),
                (Some(_), Some(Component::ParentDir)) => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
