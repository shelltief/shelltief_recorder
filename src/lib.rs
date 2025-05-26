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
    let mut buf: MaybeUninit<libc::statvfs>;
    unsafe {
     buf = MaybeUninit::uninit();
     libc::statvfs(path.as_ptr(), buf.as_mut_ptr());
    };
    let buf = unsafe { buf.assume_init() };
    let blocks = buf.f_frsize * buf.f_bavail as u64;
    let size: u64 = blocks;
    let size_str = match size {
        0..1000 => format!("{}b", size),
        1000..1000000 => format!("{0:.2}k", size as f32 / 1e3),
        1000000..1000000000 => format!("{0:.2}M", size as f32 / 1e6),
        _ => format!("{0:.2}G", size as f64 / 1e9),
    };
    let size: f64 = size as f64/ 1e9;
    println!("Size is: {size_str}");
    println!("Size var: {size}");
}
