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
fn user_continues(prompt: &str) -> bool {
    let mut input = String::new();

    println!("{} [y/n] -- press 'Enter' to validate", prompt);

    io::stdin().read_line(&mut input)
        .expect("Line should be readable");

    matches!(input.trim(), "y" | "Y")
}
