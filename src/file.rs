//! File related types

use bitflags::bitflags;
use neotron_ffi::FfiString;

/// Represents a (borrowed) path to file.
///
/// Neotron OS uses the following format for file paths:
///
/// `<disk>:/[<directory>/]+<filename>.<extension>`
///
/// Unlike on MS-DOS, the `disk` specifier portion is not limited to a single
/// ASCII letter and can be any UTF-8 string that does not contain `:` or `/`.
///
/// Paths are a sub-set of UTF-8 strings in this API, but be aware that not all
/// filesystems support all Unicode characters. In particular FAT16 and FAT32
/// volumes are likely to be limited to only `A-Z`, `a-z`, `0-9` and
/// `$%-_@~\`!(){}^#&`. This API will expressly disallow UTF-8 codepoints below
/// 32 (i.e. C0 control characters) to avoid confusion.
///
/// Paths are case-preserving but may not be case-sensitive. Paths may contain
/// spaces, if your filesystem supports it.
///  
/// Here are some examples of valid paths:
///
/// ```text
/// Documents/2023/June/Sales in €.xls - relative to the Current Directory
/// HD0:/MYDOCU~1/SALES.TXT - a file on drive HD0
/// SD0:/MYDOCU~1/ - a directory on drive SD0
/// SD0:/BOOTLDR - a file on drive SD0, with no file extension
/// CON$: - a special device file
/// SER0$:/bps=9600/parity=N/timeout=100: - a special device file with parameters
/// ```
#[repr(C)]
pub struct Path<'a>(FfiString<'a>);

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
        Ok(Path(FfiString::new(path_str)))
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
        let path_str = self.0.as_str();
        if let Some((drive_specifier, _directory_path)) = path_str.split_once(Self::DRIVE_SEP) {
            Some(drive_specifier)
        } else {
            None
        }
    }

    /// Get the drive path portion.
    ///
    /// That is, everything after the directory specifier.
    pub fn drive_path(&self) -> Option<&str> {
        let path_str = self.0.as_str();
        if let Some((_drive_specifier, drive_path)) = path_str.split_once(Self::DRIVE_SEP) {
            if drive_path.is_empty() {
                Some("/")
            } else {
                Some(drive_path)
            }
        } else {
            Some(path_str)
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
                None
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
}

/// Represents an open file
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle(u8);

impl Handle {
    /// The magic file ID for Standard Input
    const STDIN: u8 = 0;

    /// The magic file ID for Standard Output
    const STDOUT: u8 = 1;

    /// The magic file ID for Standard Error
    const STDERR: u8 = 2;

    /// Construct a new `Handle` from an integer.
    ///
    /// Only the OS should call this - applications should not be constructing
    /// their own file handles! But if you do, you probably can't harm anything
    /// and it's no worse that C just using `int`.
    pub const fn new(value: u8) -> Handle {
        Handle(value)
    }

    /// Create a file handle for Standard Input
    pub const fn new_stdin() -> Handle {
        Handle(Self::STDIN)
    }

    /// Create a file handle for Standard Output
    pub const fn new_stdout() -> Handle {
        Handle(Self::STDOUT)
    }

    /// Create a file handle for Standard Error
    pub const fn new_stderr() -> Handle {
        Handle(Self::STDERR)
    }

    /// Get the numeric value of this File Handle
    pub const fn value(&self) -> u8 {
        self.0
    }
}

/// Describes a file on disk.
///
/// This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stat {
    /// How big is this file
    pub file_size: u64,
    /// When was the file created
    pub ctime: Time,
    /// When was the last modified
    pub mtime: Time,
    /// File attributes (Directory, Volume, etc)
    pub attr: Attributes,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// Describes the attributes of a file.
    pub struct Flags: u8 {
        /// Enable write support for this file.
        const WRITE = 0x01;
        /// Create the file if it doesn't exist.
        const CREATE = 0x02;
        /// Truncate the file to zero length upon opening.
        const TRUNCATE = 0x04;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// The attributes a file on disk can have.alloc
    ///
    /// Based on that supported by the FAT32 file system.
    pub struct Attributes: u8 {
        /// File is read-only
        const READ_ONLY = 0x01;
        /// File should not appear in directory listing
        const HIDDEN = 0x02;
        /// File should not be moved on disk
        const SYSTEM = 0x04;
        /// File is a volume label
        const VOLUME = 0x08;
        /// File is a directory
        const DIRECTORY = 0x10;
        /// File has not been backed up
        const ARCHIVE = 0x20;
        /// File is actually a device
        const DEVICE = 0x40;
    }
}

/// Represents an instant in time, in the local time zone.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Time {
    /// Add 1970 to this file to get the calendar year
    pub year_since_1970: u8,
    /// Add one to this value to get the calendar month
    pub zero_indexed_month: u8,
    /// Add one to this value to get the calendar day
    pub zero_indexed_day: u8,
    /// The number of hours past midnight
    pub hours: u8,
    /// The number of minutes past the hour
    pub minutes: u8,
    /// The number of seconds past the minute
    pub seconds: u8,
}

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
