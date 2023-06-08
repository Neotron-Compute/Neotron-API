//! Directory related types

/// Represents an open directory
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Handle(u8);

impl Handle {
    /// Construct a new `Handle` from an integer.
    ///
    /// Only the OS should call this.
    ///
    /// # Safety
    ///
    /// The integer given must be a valid index for an open Directory.
    #[cfg(feature = "os")]
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
    /// The name and extension of the file
    pub name: [u8; crate::MAX_FILENAME_LEN],
    /// File properties
    pub properties: crate::file::Stat,
}
