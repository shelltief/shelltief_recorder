mod control;
use control::{
    control_panel,
    setup,
};
mod defaults;
mod dir;
use dir::archive_current;
mod ffmpeg;
use ffmpeg::{
    get_devices,
    Stream,
};


pub fn run() -> Result<(), String>{
    let project_path = setup()?;
    println!("Project_path is : '{project_path}'");
    let devices = get_devices();
    let face = Stream::new("FaceTime", Some("MacBook"), "face.mp4", Some(&project_path));
    let screen = Stream::new("Capture screen 0", None, "screen.mp4", Some(&project_path));
    let streams: Vec<Stream> = vec![face, screen];
    let res = ffmpeg::launch(streams, devices);
    let children = res.unwrap_with(|mut c| {c.cleanup(None);});
    control_panel(children);
    archive_current(&project_path)
}
