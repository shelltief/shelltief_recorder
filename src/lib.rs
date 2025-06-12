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


/*
pub fn run(){
    let devices: Devices = get_devices();
    let stream = Stream::new("FaceTime", Some("MacBook"), "test.mp4");
    let streams: Vec<Stream> = vec![stream];
    let res = ffmpeg::launch(streams, devices);
    let children: Children = match res {
        IResult::Err(err) => {
            println!("Go error: {err}");
            panic!("Couldn't launch recording");
        },
        IResult::Incomplete(mut children, err) => {
            children.cleanup(None);
            println!("Recording was half launched, error is: {err}");
            panic!("Couldn't proceed with full recording");
        }
        IResult::Ok(children) => {
            children
        }
    };
    control_panel(children);
}
*/

pub fn run() -> Result<(), String>{
    let project_path = setup()?;
    let devices = get_devices();
    let face = Stream::new("Facetime", Some("Macbook"), "face.mp4", Some(&project_path));
    let screen = Stream::new("Capture Screen 0", None, "screen.mp4", Some(&project_path));
    let streams: Vec<Stream> = vec![face, screen];
    let res = ffmpeg::launch(streams, devices);
    let children = res.unwrap_with(|mut c| {c.cleanup(None);});
    control_panel(children);
    archive_current(&project_path)
}

// pub fn run() {
//     let mut test_dir = "/Volumes/T7/code_videos/Rushes";
//     if let Some(path) = test_dir.strip_suffix("/") {
//         test_dir = path;
//     }
//     let project_dir = init_project_dir(test_dir, "test");
//     let size = size_available(test_dir);
//     println!("{0:.2}G", size);
//     archive_current(test_dir);
//     if ! user_continues("Would you like to continue?") {
//         println!("User didn't which to continue");
//     } else {
//         println!("User is in!");
//     }
// }
