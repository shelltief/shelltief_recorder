//mod ffmpeg;
//pub use ffmpeg::get_devices;
//mod dir;
//use dir::{
//    init_project_dir,
//    size_available,
//    archive_current,
//};

use std::io;



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

