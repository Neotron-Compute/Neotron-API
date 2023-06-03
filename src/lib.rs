//! The Neotron API
//!
//! Defines the API supplied to applications that run on Neotron OS. This API is
//! provided in the form of a rust `struct Api`, where every field is a function
//! pointer.

#![no_std]

// ============================================================================
// Imports
// ============================================================================

pub mod dir;
pub mod file;

pub use neotron_ffi::{FfiBuffer, FfiByteSlice, FfiString};

// ============================================================================
// Constants
// ============================================================================

/// Maximum length of a filename (with no directory components), including the
/// extension.
pub const MAX_FILENAME_LEN: usize = 11;

// ============================================================================
// Types
// ============================================================================

/// The result type for any SDK API function.
///
/// Like a [`neotron_ffi::FfiResult`] but the error type is [`Error`].
pub type Result<T> = neotron_ffi::FfiResult<T, Error>;

/// The syscalls provided by the Neotron OS to a Neotron Application.
#[repr(C)]
pub struct Api {
    /// Open a file, given a path as UTF-8 string.
    ///
    /// If the file does not exist, or is already open, it returns an error.
    ///
    /// Path may be relative to current directory, or it may be an absolute
    /// path.
    ///
    /// # Limitations
    ///
    /// * You cannot open a file if it is currently open.
    /// * Paths must confirm to the rules for the filesystem for the given drive.
    /// * Relative paths are taken relative to the current directory (see `Api::chdir`).
    pub open: extern "C" fn(path: file::Path, flags: file::Flags) -> Result<file::Handle>,
    /// Close a previously opened file.
    ///
    /// Closing a file is important, as only this action will cause the
    /// directory entry for the file to be updated. Crashing the system without
    /// closing a file may cause the directory entry to be incorrect, and you
    /// may need to run `CHKDSK` (or similar) on your disk to fix it.
    pub close: extern "C" fn(fd: file::Handle) -> Result<()>,
    /// Write to an open file handle, blocking until everything is written.
    ///
    /// Some files do not support writing and will produce an error. You will
    /// also get an error if you run out of disk space.
    ///
    /// The `buffer` is only borrowed for the duration of the function call and
    /// is then forgotten.
    pub write: extern "C" fn(fd: file::Handle, buffer: FfiByteSlice) -> Result<()>,
    /// Read from an open file, returning how much was actually read.
    ///
    /// You might get less data than you asked for. If you do an `Api::read` and
    /// you are already at the end of the file you will get
    /// `Err(Error::EndOfFile)`.
    ///
    /// Data is stored to the given `buffer. The `buffer` is only borrowed for
    /// the duration of the function call and is then forgotten.
    pub read: extern "C" fn(fd: file::Handle, buffer: FfiBuffer) -> Result<usize>,
    /// Move the file offset (for the given file handle) to the given position.
    ///
    /// Some files do not support seeking and will produce an error.
    pub seek_set: extern "C" fn(fd: file::Handle, position: u64) -> Result<()>,
    /// Move the file offset (for the given file handle) relative to the current position
    ///
    /// Some files do not support seeking and will produce an error.
    pub seek_cur: extern "C" fn(fd: file::Handle, offset: i64) -> Result<()>,
    /// Move the file offset (for the given file handle) to the end of the file
    ///
    /// Some files do not support seeking and will produce an error.
    pub seek_end: extern "C" fn(fd: file::Handle) -> Result<()>,
    /// Rename a file.
    ///
    /// # Limitations
    ///
    /// * You cannot rename a file if it is currently open.
    /// * You cannot rename a file where the `old_path` and the `new_path` are
    /// not on the same drive.
    /// * Paths must confirm to the rules for the filesystem for the given drive.
    pub rename: extern "C" fn(old_path: file::Path, new_path: file::Path) -> Result<()>,
    /// Perform a special I/O control operation.
    pub ioctl: extern "C" fn(fd: file::Handle, command: u64, value: u64) -> Result<u64>,
    /// Open a directory, given a path as a UTF-8 string.
    pub opendir: extern "C" fn(path: file::Path) -> Result<dir::Handle>,
    /// Close a previously opened directory.
    pub closedir: extern "C" fn(dir: dir::Handle) -> Result<()>,
    /// Read from an open directory
    pub readdir: extern "C" fn(dir: dir::Handle) -> Result<dir::Entry>,
    /// Get information about a file.
    pub stat: extern "C" fn(path: file::Path) -> Result<file::Stat>,
    /// Get information about an open file.
    pub fstat: extern "C" fn(fd: file::Handle) -> Result<file::Stat>,
    /// Delete a file or directory.
    ///
    /// # Limitations
    ///
    /// * You cannot delete a file if it is currently open.
    /// * You cannot delete a root directory.
    /// * You cannot delete a directory that has any files in it.
    pub delete: extern "C" fn(path: file::Path) -> Result<()>,
    /// Change the current directory.
    ///
    /// Relative file paths (e.g. passed to `Api::open`) are taken to be relative to the current directory.
    ///
    /// Unlike on MS-DOS, there is only one current directory for the whole
    /// system, not one per drive.
    pub chdir: extern "C" fn(path: file::Path) -> Result<()>,
    /// Change the current directory to the given open directory.
    ///
    /// Unlike on MS-DOS, there is only one current directory for the whole
    /// system, not one per drive.
    pub dchdir: extern "C" fn(dir: dir::Handle) -> Result<()>,
    /// Get the current directory.
    ///
    /// The current directory is stored as UTF-8 into the given buffer. The
    /// function returns the number of bytes written to the buffer, or an error.
    /// If the function did not return an error, the buffer can be assumed to
    /// contain a valid file path.
    pub pwd: extern "C" fn(path: FfiBuffer) -> Result<usize>,
    /// Allocate some memory.
    ///
    /// * `size` - the number of bytes required
    /// * `alignment` - the returned address will have this alignment, or
    ///   better. For example, pass `4` if you are allocating an array of `u32`.
    pub malloc: extern "C" fn(size: usize, alignment: usize) -> Result<*mut core::ffi::c_void>,
    /// Free some previously allocated memory.
    ///
    /// You must pass the same `size` and `alignment` values that you passed to `malloc`.
    pub free: extern "C" fn(ptr: *mut core::ffi::c_void, size: usize, alignment: usize),
}

/// Describes how something has failed
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// The given file path was not found
    FileNotFound,
    /// Tried to write to a read-only file
    FileReadOnly,
    /// Reached the end of the file
    EndOfFile,
    /// The API has not been implemented
    Unimplemented,
    /// An invalid argument was given to the API
    InvalidArg,
    /// A bad file handle was given to the API
    BadFileHandle,
    /// An device-specific error occurred. Look at the BIOS source for more details.
    DeviceSpecific,
    /// The OS does not have enough memory
    OutOfMemory,
    /// The given path was invalid
    InvalidPath,
}

// ============================================================================
// End of File
// ============================================================================
