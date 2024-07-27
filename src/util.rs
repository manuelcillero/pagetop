//! Functions and macro helpers.

use crate::trace;

use std::io;
use std::path::PathBuf;

pub enum TypeInfo {
    FullName,
    ShortName,
    NameFrom(isize),
    NameTo(isize),
    PartialName(isize, isize),
}

impl TypeInfo {
    pub fn of<T: ?Sized>(&self) -> &'static str {
        let type_name = std::any::type_name::<T>();
        match self {
            TypeInfo::FullName => type_name,
            TypeInfo::ShortName => Self::partial(type_name, -1, None),
            TypeInfo::NameFrom(start) => Self::partial(type_name, *start, None),
            TypeInfo::NameTo(end) => Self::partial(type_name, 0, Some(*end)),
            TypeInfo::PartialName(start, end) => Self::partial(type_name, *start, Some(*end)),
        }
    }

    fn partial(type_name: &'static str, start: isize, end: Option<isize>) -> &'static str {
        let maxlen = type_name.len();
        let mut segments = Vec::new();
        let mut segment_start = 0; // Start position of the current segment.
        let mut angle_brackets = 0; // Counter for tracking '<' and '>'.
        let mut previous_char = '\0'; // Initializes to a null character, no previous character.

        for (idx, c) in type_name.char_indices() {
            match c {
                ':' if angle_brackets == 0 => {
                    if previous_char == ':' {
                        if segment_start < idx - 1 {
                            segments.push((segment_start, idx - 1)); // Do not include last '::'.
                        }
                        segment_start = idx + 1; // Next segment starts after '::'.
                    }
                }
                '<' => angle_brackets += 1,
                '>' => angle_brackets -= 1,
                _ => {}
            }
            previous_char = c;
        }

        // Include the last segment if there's any.
        if segment_start < maxlen {
            segments.push((segment_start, maxlen));
        }

        // Calculates the start position.
        let start_pos = segments
            .get(if start >= 0 {
                start as usize
            } else {
                segments.len() - start.unsigned_abs()
            })
            .map_or(0, |&(s, _)| s);

        // Calculates the end position.
        let end_pos = segments
            .get(if let Some(end) = end {
                if end >= 0 {
                    end as usize
                } else {
                    segments.len() - end.unsigned_abs()
                }
            } else {
                segments.len() - 1
            })
            .map_or(maxlen, |&(_, e)| e);

        // Returns the partial string based on the calculated positions.
        &type_name[start_pos..end_pos]
    }
}

// *************************************************************************************************
// FUNCTIONS HELPERS.
// *************************************************************************************************

/// Calculates the absolute directory given a root path and a relative path.
///
/// # Arguments
///
/// * `root_path` - A string slice that holds the root path.
/// * `relative_path` - A string slice that holds the relative path.
///
/// # Returns
///
/// * `Ok` - If the operation is successful, returns the absolute directory as a `String`.
/// * `Err` - If an I/O error occurs, returns an `io::Error`.
///
/// # Errors
///
/// This function will return an error if:
/// - The root path or relative path are invalid.
/// - There is an issue with file system operations, such as reading the directory.
///
/// # Examples
///
/// ```
/// let root = "/home/user";
/// let relative = "documents";
/// let abs_dir = absolute_dir(root, relative).unwrap();
/// println!("{}", abs_dir);
/// ```
pub fn absolute_dir(
    root_path: impl Into<String>,
    relative_path: impl Into<String>,
) -> Result<String, io::Error> {
    let root_path = PathBuf::from(root_path.into());
    let full_path = root_path.join(relative_path.into());
    let absolute_dir = full_path.to_string_lossy().into();

    if !full_path.is_absolute() {
        let message = format!("Path \"{absolute_dir}\" is not absolute");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    if !full_path.exists() {
        let message = format!("Path \"{absolute_dir}\" does not exist");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::NotFound, message));
    }

    if !full_path.is_dir() {
        let message = format!("Path \"{absolute_dir}\" is not a directory");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    Ok(absolute_dir)
}

// *************************************************************************************************
// MACRO HELPERS.
// *************************************************************************************************

#[macro_export]
/// Macro para construir grupos de pares clave-valor.
///
/// ```rust#ignore
/// let args = kv![
///     "userName" => "Roberto",
///     "photoCount" => 3,
///     "userGender" => "male",
/// ];
/// ```
macro_rules! kv {
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key.into(), $value.into());
        )*
        a
    }};
}
