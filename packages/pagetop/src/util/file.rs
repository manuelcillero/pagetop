mod source;
mod toml;

use crate::util::error::*;
use crate::util::source::Source;
use crate::util::value::Value;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use self::source::FileSource;

#[derive(Clone, Debug)]
pub struct File<T>
where
    T: FileSource,
{
    source: T,
    /// A required File will error if it cannot be found.
    required: bool,
}

impl File<source::FileSourceFile> {
    /// Given the basename of a file, will attempt to locate a file by setting its extension to a
    /// registered format.
    pub fn with_name(name: &str) -> Self {
        File {
            source: source::FileSourceFile::new(name.into()),
            required: true,
        }
    }
}

impl<'a> From<&'a Path> for File<source::FileSourceFile> {
    fn from(path: &'a Path) -> Self {
        File {
            source: source::FileSourceFile::new(path.to_path_buf()),
            required: true,
        }
    }
}

impl From<PathBuf> for File<source::FileSourceFile> {
    fn from(path: PathBuf) -> Self {
        File {
            source: source::FileSourceFile::new(path),
            required: true,
        }
    }
}

impl<T: FileSource> File<T> {
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

impl<T: FileSource> Source for File<T>
where
    T: 'static,
    T: Sync + Send,
{
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        // Coerce the file contents to a string.
        let (uri, contents) = match self.source.resolve().map_err(ConfigError::Foreign) {
            Ok((uri, contents)) => (uri, contents),

            Err(error) => {
                if !self.required {
                    return Ok(HashMap::new());
                }

                return Err(error);
            }
        };

        // Parse the string using the given format.
        toml::parse(uri.as_ref(), &contents).map_err(|cause| ConfigError::FileParse { uri, cause })
    }
}
