mod ffmpeg;
use ffmpeg::{
    get_devices,
    Devices,
    Stream,
    IResult,
    Children,
};
use libc;
//mod dir;
//use dir::{
//    init_project_dir,
//    size_available,
//    archive_current,
//};

use std::io;


pub fn run(){
    let devices: Devices = get_devices();
    let stream = Stream::new("FaceTime", Some("Yeti"), "test.mp4");
    let streams: Vec<Stream> = vec![stream];
    let res = ffmpeg::launch(streams, devices);
    let mut children: Children = match res {
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
    user_continues("test");
    children.cleanup(Some(libc::SIGINT));
}

//pub fn run() {
//    let mut test_dir = "/Volumes/T7/code_videos/Rushes";
//    if let Some(path) = test_dir.strip_suffix("/") {
//        test_dir = path;
//    }
//    init_project_dir(test_dir, "test");
//    let size = size_available(test_dir);
//    println!("Retrieved: {}", size);
//    println!("{0:.2}G", size);
//    archive_current(test_dir);
//    if ! user_continues("Would you like to continue?") {
//        println!("User didn't which to continue");
//    } else {
//        println!("User is in!");
//    }
//}

/// Prompts for a user choice. For now, user can press
/// `y` or `Y` to accept and any other key to refuse
fn user_continues(prompt: &str) -> bool {
    let mut input = String::new();

    println!("{} [y/n] -- press 'Enter' to validate", prompt);

    io::stdin().read_line(&mut input)
        .expect("Line should be readable");

    matches!(input.trim(), "y" | "Y")
}

