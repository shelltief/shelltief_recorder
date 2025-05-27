use std::{
    fs::{
        self,
        DirBuilder,
    },
    ffi::CString,
    mem::MaybeUninit,
    io,
};
use libc;

/// Archives the current directory under a directory named
/// `N/` where `N` is a base 10 number. `N` is computed by
/// listing the directory and trying parse the directory
/// name into a `u8`. The last directory number is stored
/// and incremented so current can be archived into that
/// directory
pub fn archive_current(project_path: &str) {
    let current_path = project_path.to_owned() + "/" + "current";
    let result = fs::exists(&current_path);
    let result = result.unwrap();
    if ! result {
        return ;
    }
    
    let mut entries = fs::read_dir(project_path)
        .expect("'project_path' should be readable")
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    let mut last_dir: u8 = 0;
    for entry in entries {
        let entry = entry.into_string()
            .expect("dirname should contain only unicode chars");
        let dirname = entry.parse::<u8>();
        if dirname.is_ok() {
            last_dir = dirname.unwrap();
        } else if entry.chars().next().unwrap() > '9'{
            break;
        }
        println!("{:#?}", entry);
    }
    let last_dir = (last_dir + 1).to_string();
    let last_dir = project_path.to_owned() + "/" + &last_dir;
    fs::rename(current_path, last_dir)
        .expect("Dir should be properly renamed");
}

/// Initializes the project directory within the target
/// directory
pub fn init_project_dir(dir_path: &str, project_name: &str) {
    let result = fs::exists(dir_path);
    let result = result.unwrap();
    if ! result {
        panic!("Directory should exist");
    }
    let project_path = dir_path.to_string() + "/" + project_name;
    let current_path = project_path.clone() + "/" + "current";
    DirBuilder::new()
        .recursive(true)
        .create(project_path)
        .expect("Path should be created");
    let mut result = fs::exists(&current_path)
        .expect("project directory should be searchable");
    if ! result {
        return;
    }
    let pids_file = current_path.clone() + "/" + "pids";
    result = fs::exists(&pids_file)
        .expect("'current' directory should be searchable");
    if result {
        panic!("Script is already running or has been abruptly exited");
    }
}

/// Retrieves the available size (in Gb) on the target device
///
/// To do so, it calls `libc::statvfs` and computes (since we are
/// on macOS), `f_frsize * f_bavail` which gives the number of 
/// free blocks available for unpriviledged users times the size 
/// (in bytes) of one block
pub fn size_available(path: &str) -> f64 {
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
