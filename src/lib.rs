//mod ffmpeg;
//pub use ffmpeg::get_devices;
use std::{
    fs,
    ffi::CString,
    mem::MaybeUninit,
};
use libc;

pub fn run() {
    let test_dir = "/Volumes/T7/code_videos";
    let result = fs::exists(test_dir);
    let result = result.unwrap();
    if ! result {
        panic!("Directory should exist");
    }
    let size = size_available(test_dir);
    println!("Retrieved: {}", size);
    println!("{0:.2}G", size);
}

/// Retrieves the available size (in Gb) on the target device
///
/// To do so, it calls `libc::statvfs` and computes (since we are
/// on macOS), `f_frsize * f_bavail` which gives the number of 
/// free blocks available for unpriviledged users times the size 
/// (in bytes) of one block
fn size_available(path: &str) -> f64 {
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
    size
}
