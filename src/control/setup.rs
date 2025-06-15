use crate::{
        defaults::{
            PROJECTS_PATH,
            THRESHOLD,
        },
        dir::{
            size_available,
            init_project_dir,
        },
};
use super::user_continues;
use std::process::{Command, Stdio};

pub(crate) fn setup() -> Result<(String, String), String> {
    let projects_path: &str = if let Some(path) = PROJECTS_PATH.strip_suffix("/") {
        path
    } else {
        PROJECTS_PATH
    };
    let size = size_available(projects_path)?;
    eprintln!("Size is : {0:.2}G", size);
    if size < THRESHOLD as f64 {
        return Err(format!("Size is under the threshold: '{THRESHOLD}'."));
    }
    if ! user_continues("Would you like to continue?") {
        return Err(String::from("User didn't wish to continue"));
    }
    if ffmpeg_running()? {
        return Err(String::from("ffmpeg is running, \
please stop all your ffmpeg processes before attempting to run the script"));
    }
    init_project_dir(projects_path, "name")
        .map_err(|e| format!("Couldn't initialize project dir: '{}'", e))
}

fn ffmpeg_running() -> Result<bool, String> {
    let ffmpeg_running = Command::new("pgrep")
        .arg("ffmpeg")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| {String::from("Error in pgrep command, can't check ffmpeg process")})?
        .wait()
        .map_err(|e| {format!("Got error: '{e}'")})?
        .success();
    Ok(ffmpeg_running)
}
