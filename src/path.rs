//! Path related types.
//!
//! These aren't used in the API itself, but will be useful to code on both
//! sides of the API, so they live here.

// ============================================================================
// Imports
// ============================================================================

// None

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

/// Represents a (borrowed) path to file.
///
/// Neotron OS uses the following format for file paths:
///
/// `<drive>:/[<directory>/]...<filename>.<extension>`
///
/// Unlike on MS-DOS, the `drive` specifier portion is not limited to a single
/// ASCII letter and can be any UTF-8 string that does not contain `:` or `/`.
///
/// Typically drives will look like `DEV:` or `HD0:`, but that's not enforced
/// here.
///
/// Paths are a sub-set of UTF-8 strings in this API, but be aware that not all
/// filesystems support all Unicode characters. In particular FAT16 and FAT32
/// volumes are likely to be limited to only `A-Z`, `a-z`, `0-9` and
/// `$%-_@~\`!(){}^#&`. This API will expressly disallow UTF-8 codepoints below
/// 32 (i.e. C0 control characters) to avoid confusion, but non-ASCII
/// code-points are accepted.
///
/// Paths are case-preserving but file operations may not be case-sensitive
/// (depending on the filesystem you are accessing). Paths may contain spaces
/// (but your filesystem may not support that).
///  
/// Here are some examples of valid paths:
///
/// ```text
/// # relative to the Current Directory
/// Documents/2023/June/Sales in €.xls
/// # a file on drive HD0
/// HD0:/MYDOCU~1/SALES.TXT
/// # a directory on drive SD0
/// SD0:/MYDOCU~1/
/// # a file on drive SD0, with no file extension
/// SD0:/BOOTLDR
/// ```
///
/// Files and Directories generally have distinct APIs, so a directory without a
/// trailing `/` is likely to be accepted. A file path with a trailing `/` won't
/// be accepted.
pub struct Path<'a>(&'a str);

impl<'a> Path<'a> {
    /// The character that separates one directory name from another directory name.
    pub const PATH_SEP: char = '/';

    /// The character that separates drive specifiers from directories.
    pub const DRIVE_SEP: char = ':';

    /// Create a path from a string.
    ///
    /// If the given string is not a valid path, an `Err` is returned.
    pub fn new(path_str: &'a str) -> Result<Path<'a>, crate::Error> {
        // No empty paths in drive specifier
        if path_str.is_empty() {
            return Err(crate::Error::InvalidPath);
        }

        if let Some((drive_specifier, directory_path)) = path_str.split_once(Self::DRIVE_SEP) {
            if drive_specifier.contains(Self::PATH_SEP) {
                // No slashes in drive specifier
                return Err(crate::Error::InvalidPath);
            }
            if directory_path.contains(Self::DRIVE_SEP) {
                // No colons in directory path
                return Err(crate::Error::InvalidPath);
            }
            if !directory_path.is_empty() && !directory_path.starts_with(Self::PATH_SEP) {
                // No relative paths if drive is specified. An empty path is OK (it means "/")
                return Err(crate::Error::InvalidPath);
            }
        } else if path_str.starts_with(Self::PATH_SEP) {
            // No absolute paths if drive is not specified
            return Err(crate::Error::InvalidPath);
        }
        for ch in path_str.chars() {
            if ch.is_control() {
                // No control characters allowed
                return Err(crate::Error::InvalidPath);
            }
        }
        Ok(Path(path_str))
    }

    /// Is this an absolute path?
    ///
    /// Absolute paths have drive specifiers. Relative paths do not.
    pub fn is_absolute_path(&self) -> bool {
        self.drive_specifier().is_some()
    }

    /// Get the drive specifier for this path.
    ///
    /// * A path like `DS0:/FOO/BAR.TXT` has a drive specifier of `DS0`.
    /// * A path like `BAR.TXT` has no drive specifier.
    pub fn drive_specifier(&self) -> Option<&str> {
        if let Some((drive_specifier, _directory_path)) = self.0.split_once(Self::DRIVE_SEP) {
            Some(drive_specifier)
        } else {
            None
        }
    }

    /// Get the drive path portion.
    ///
    /// That is, everything after the directory specifier.
    pub fn drive_path(&self) -> Option<&str> {
        if let Some((_drive_specifier, drive_path)) = self.0.split_once(Self::DRIVE_SEP) {
            if drive_path.is_empty() {
                // Bare drives are assumed to be at the root
                Some("/")
            } else {
                Some(drive_path)
            }
        } else {
            Some(self.0)
        }
    }

    /// Get the directory portion of this path.
    ///
    /// * A path like `DS0:/FOO/BAR.TXT` has a directory portion of `/FOO`.
    /// * A path like `DS0:/FOO/BAR/` has a directory portion of `/FOO/BAR`.
    /// * A path like `BAR.TXT` has no directory portion.
    pub fn directory(&self) -> Option<&str> {
        let Some(drive_path) = self.drive_path() else {
            return None;
        };
        if let Some((directory, _filename)) = drive_path.rsplit_once(Self::PATH_SEP) {
            if directory.is_empty() {
                // Bare drives are assumed to be at the root
                Some("/")
            } else {
                Some(directory)
            }
        } else {
            Some(drive_path)
        }
    }

    /// Get the filename portion of this path. This filename will include the file extension, if any.
    ///
    /// * A path like `DS0:/FOO/BAR.TXT` has a filename portion of `/BAR.TXT`.
    /// * A path like `DS0:/FOO` has a filename portion of `/FOO`.
    /// * A path like `DS0:/FOO/` has no filename portion (so it's important directories have a trailing `/`)
    pub fn filename(&self) -> Option<&str> {
        let Some(drive_path) = self.drive_path() else {
            return None;
        };
        if let Some((_directory, filename)) = drive_path.rsplit_once(Self::PATH_SEP) {
            if filename.is_empty() {
                None
            } else {
                Some(filename)
            }
        } else {
            Some(drive_path)
        }
    }

    /// Get the filename extension portion of this path.
    ///
    /// A path like `DS0:/FOO/BAR.TXT` has a filename extension portion of `TXT`.
    /// A path like `DS0:/FOO/BAR` has no filename extension portion.
    pub fn extension(&self) -> Option<&str> {
        let Some(filename) = self.filename() else {
            return None;
        };
        if let Some((_basename, extension)) = filename.rsplit_once('.') {
            Some(extension)
        } else {
            None
        }
    }

    /// View this [`Path`] as a string-slice.
    pub fn as_str(&self) -> &str {
        self.0
    }
}

// ============================================================================
// Functions
// ============================================================================

// None

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_path() {
        let path_str = "HD0:/DOCUMENTS/JUNE/SALES.TXT";
        let path = Path::new(path_str).unwrap();
        assert!(path.is_absolute_path());
        assert_eq!(path.drive_specifier(), Some("HD0"));
        assert_eq!(path.drive_path(), Some("/DOCUMENTS/JUNE/SALES.TXT"));
        assert_eq!(path.directory(), Some("/DOCUMENTS/JUNE"));
        assert_eq!(path.filename(), Some("SALES.TXT"));
        assert_eq!(path.extension(), Some("TXT"));
    }

    #[test]
    fn bare_drive() {
        let path_str = "HD0:";
        let path = Path::new(path_str).unwrap();
        assert!(path.is_absolute_path());
        assert_eq!(path.drive_specifier(), Some("HD0"));
        assert_eq!(path.drive_path(), Some("/"));
        assert_eq!(path.directory(), Some("/"));
        assert_eq!(path.filename(), None);
        assert_eq!(path.extension(), None);
    }

    #[test]
    fn relative_path() {
        let path_str = "DOCUMENTS/JUNE/SALES.TXT";
        let path = Path::new(path_str).unwrap();
        assert!(!path.is_absolute_path());
        assert_eq!(path.drive_specifier(), None);
        assert_eq!(path.drive_path(), Some("DOCUMENTS/JUNE/SALES.TXT"));
        assert_eq!(path.directory(), Some("DOCUMENTS/JUNE"));
        assert_eq!(path.filename(), Some("SALES.TXT"));
        assert_eq!(path.extension(), Some("TXT"));
    }

    #[test]
    fn full_dir() {
        let path_str = "HD0:/DOCUMENTS/JUNE/";
        let path = Path::new(path_str).unwrap();
        assert!(path.is_absolute_path());
        assert_eq!(path.drive_specifier(), Some("HD0"));
        assert_eq!(path.drive_path(), Some("/DOCUMENTS/JUNE/"));
        assert_eq!(path.directory(), Some("/DOCUMENTS/JUNE"));
        assert_eq!(path.filename(), None);
        assert_eq!(path.extension(), None);
    }

    #[test]
    fn relative_dir() {
        let path_str = "DOCUMENTS/";
        let path = Path::new(path_str).unwrap();
        assert!(!path.is_absolute_path());
        assert_eq!(path.drive_specifier(), None);
        assert_eq!(path.drive_path(), Some("DOCUMENTS/"));
        assert_eq!(path.directory(), Some("DOCUMENTS"));
        assert_eq!(path.filename(), None);
        assert_eq!(path.extension(), None);
    }
}

// ============================================================================
// End of File
// ============================================================================
