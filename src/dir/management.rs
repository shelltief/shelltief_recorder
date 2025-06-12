//! # Management
//!
//! Utilities to create and archive directories
//! and files created while the script runs

use std::{
    fs::{
        self,
        DirBuilder,
    },
    io,
};

/// Archives the current directory under a directory named
/// `N/` where `N` is a base 10 number. `N` is computed by
/// listing the directory and trying parse the directory
/// name into a `u8`. The last directory number is stored
/// and incremented so current can be archived into that
/// directory
///
/// # Examples
///
/// ```ignore
/// let project_path = "/home/user/Documents/project";
/// // The project dir should contain a `current` directory;
/// assert_eq!(archive_current(project_path), Ok(()));
/// ```
///
/// # Errors
///
/// If `current` dir doesn't exist
///
/// # Panics
///
/// - If `project_path` isn't readable and executable
/// - If `dirname` contains non-unicode chars
/// - If `current` dir isn't properly renamed
pub(crate) fn archive_current(project_path: &str) -> Result<(), String>{
    let project_path = if let Some(path) = project_path.strip_suffix("/") {
        path
    } else {
        project_path
    };
    let current_path = project_path.to_owned() + "/" + "current";
    let result = fs::exists(&current_path);
    let result = result.unwrap();
    if ! result {
        return Err(format!("'{current_path}' directory not found"));
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
    Ok(())
}

/// Initializes the project directory within the target
/// directory
///
/// # Examples
///
/// ```ignore
/// let dir_path = "/home/user/Documents/project";
/// let project_name = "my_awesome_project";
/// init_project_dir(dir_path, project_name);
/// ```
///
/// # Errors
///
/// If `dir_path` isn't created beforehand
///
/// # Panics
///
/// - If `project_path` can't be created by the `DirBuilder`
/// - If `project_path` isn't searchable
pub(crate) fn init_project_dir(dir_path: &str, project_name: &str)
-> Result<String, String> {
    let dir_path = if let Some(path) = dir_path.strip_suffix("/") {
        path
    } else {
        dir_path
    };
    let project_name = if let Some(name) = project_name.strip_suffix("/") {
        name
    } else {
        project_name
    };
    let result = fs::exists(dir_path);
    let result = result.unwrap();
    if ! result {
        return Err(format!("'{dir_path}' should exist"));
    }
    let project_path = dir_path.to_string() + "/" + project_name;
    let current_path = project_path.clone() + "/" + "current";
    DirBuilder::new()
        .recursive(true)
        .create(&project_path)
        .expect("Path should be created");
    let exists = fs::exists(&current_path)
        .expect("project directory should be searchable");
    if exists {
        let _ = fs::remove_dir_all(&current_path)
            .map_err(|e| {e.to_string()})?;
    }
    Ok(project_path)
}
