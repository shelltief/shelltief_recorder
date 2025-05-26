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
    let _available_space: libc::c_int;
    let path = CString::new(test_dir.as_bytes())
        .expect("Failure while creating path string");
    let mut buf: libc::statvfs;
    unsafe {
     buf = MaybeUninit::uninit().assume_init();
     libc::statvfs(path.as_ptr(), &mut buf as *mut _);
    };
    println!("{}", buf.f_frsize);
}
