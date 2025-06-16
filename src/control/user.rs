use crate::ffmpeg::Stream;
use std::{
    sync::mpsc::Sender,
    io,
};

/// Enum of statuses that are to be used between the
/// controlling threads and the main thread
#[derive(PartialEq)]
pub(crate) enum StopStatus {
    Exited(u32),
    UserStop,
    Running,
    Panic(String),
}


/// Receives an input from a user and transfers it through a ['channel']
pub(super) fn user_control(tx: Sender<StopStatus>){
    user_continues("Press any key to stop recording");
    let _ = tx.send(StopStatus::UserStop);
}

/// Prompts for a user choice. For now, user can press
/// `y` or `Y` to accept and any other key to refuse
pub(super) fn user_continues(prompt: &str) -> bool {
    let mut input = String::new();

    println!("{} [y/n] -- press 'Enter' to validate", prompt);

    io::stdin().read_line(&mut input)
        .expect("Line should be readable");

    matches!(input.trim(), "y" | "Y")
}

pub(crate) fn validate_settings(streams: &Vec<Stream>, preview: bool)
-> Result<(), String>
{
    println!("The following streams are about to be recorded");
    for stream in streams {
        stream.display();
    }
    println!("The mode is : {}", if preview { "preview" } else { "record" });
    if ! user_continues("Do you which to continue?") {
        return Err(String::from("Settings weren't validated"));
    }
    Ok(())
}
