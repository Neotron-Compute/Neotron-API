//! Directory related types

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

/// Represents an open directory
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Handle(u8);

impl Handle {
    /// Construct a new `Handle` from an integer.
    ///
    /// Only the OS should call this - applications should not be constructing
    /// their own dir handles! But if you do, you probably can't harm anything
    /// and it's no worse that C just using `int`.
    pub const fn new(value: u8) -> Handle {
        Handle(value)
    }

    /// Get the numeric value of this Directory Handle
    pub const fn value(&self) -> u8 {
        self.0
    }
}

/// Describes an entry in a directory.
///
/// This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The name and extension of the file.
    ///
    /// The name and extension are separated by a single '.'.
    ///
    /// The filename will be in ASCII. Unicode filenames are not supported.
    pub name: [u8; crate::MAX_FILENAME_LEN],
    /// The properties for the file/directory this entry represents.
    pub properties: crate::file::Stat,
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
