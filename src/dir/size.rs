//! # Size
//!
//! Utilities to mesure the size of the
//! target directory in order to ensure
//! the script can run properly

use std::{
    ffi::CString,
    fs,
    mem::MaybeUninit,
};
use libc;

/// Retrieves the available size (in Gb) on the target device
///
/// To do so, it calls `libc::statvfs` and computes (since we are
/// on macOS), `f_frsize * f_bavail` which gives the number of
/// free blocks available for unpriviledged users times the size
/// (in bytes) of one block
///
/// # Examples
///
/// ```ignore
/// let path = "/home/user/Documents/project";
/// println!("Available size is: {}", size_available(path));
/// ```
///
/// # Panics
///
/// - If path string can't be created
pub(crate) fn size_available(path: &str) -> Result<f64, String> {
    if ! fs::exists(path).map_err(|e| {format!("Got error: '{e}'")})? {
        return Err(format!("Path: '{path}' doesn't exist"));
    }
    let path = CString::new(path.as_bytes())
        .expect("Failure while creating path string");
    let mut buf: MaybeUninit<libc::statvfs>;
    unsafe {
     buf = MaybeUninit::uninit();
     libc::statvfs(path.as_ptr(), buf.as_mut_ptr());
    };
    let buf = unsafe { buf.assume_init() };
    let blocks = buf.f_frsize * buf.f_bavail as u64;
    let size: f64 = blocks as f64/ 1e9;
    Ok(size)
}
