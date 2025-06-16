mod control;
use control::{
    control_panel,
    parse_command_line,
    parse_options,
    setup,
    validate_settings,
};
mod defaults;
mod dir;
use dir::archive_current;
mod ffmpeg;
use ffmpeg::{
    get_devices,
};


#[cfg(target_os = "macos")]
pub fn run() -> Result<(), String> {
    let command_line = parse_command_line()?;
    let (mut streams, project_name, project_path, preview) = parse_options(command_line)?;
    let (project_path, current_path) = setup(project_path, project_name)?;
    let devices = get_devices();
    for stream in &mut streams {
        stream.set_path(&current_path);
    }
    let _ = validate_settings(&streams, preview)?;
    let res = ffmpeg::launch(streams, devices, preview);
    let children = res.unwrap_with(|mut c| {c.cleanup(None);});
    control_panel(children);
    if ! preview {
        let archive = archive_current(&project_path)?;
        println!("Session archived at : {archive}");
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn run() -> Result<(), String> {
    Err(String::from("This tool only runs on MacOS"))
}
