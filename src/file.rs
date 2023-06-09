//! File related types

// ============================================================================
// Imports
// ============================================================================

use bitflags::bitflags;

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

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

// ============================================================================
// Functions
// ============================================================================

// None

// ============================================================================
// Tests
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================
