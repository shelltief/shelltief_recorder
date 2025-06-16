use crate::{
        defaults::{
            THRESHOLD,
        },
        dir::{
            size_available,
            init_project_dir,
        },
};
use super::user_continues;
use std::{
    process::{Command, Stdio},
};

pub(crate) fn setup(project_path: String, name: String) -> Result<(String, String), String> {
    let projects_path: &str = if let Some(path) = project_path.strip_suffix("/") {
        path
    } else {
        &project_path[..]
    };
    let size = size_available(projects_path)?;
    eprintln!("Size is : {0:.2}G", size);
    if size < THRESHOLD as f64 {
        return Err(format!("Size available is under the threshold: '{THRESHOLD:.2}G'."));
    }
    if ! user_continues("Would you like to continue?") {
        return Err(String::from("User didn't wish to continue"));
    }
    if ffmpeg_running()? {
        return Err(String::from("ffmpeg is running, \
please stop all your ffmpeg processes before attempting to run the script"));
    }
    init_project_dir(projects_path, &name)
        .map_err(|e| format!("Couldn't initialize project dir: '{}'", e))
}

fn ffmpeg_running() -> Result<bool, String> {
    let ffmpeg_running = Command::new("pgrep")
        .arg("ffmpeg")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_e| {String::from("Error in pgrep command, can't check ffmpeg process")})?
        .wait()
        .map_err(|e| {format!("Got error: '{e}'")})?
        .success();
    Ok(ffmpeg_running)
}
