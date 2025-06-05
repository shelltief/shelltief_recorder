use std::{
    sync::mpsc::Sender,
    io,
};

#[derive(PartialEq)]
pub(crate) enum StopStatus {
    Exited(u32),
    UserStop,
    Running,
}


pub(super) fn user_control(tx: Sender<StopStatus>){
    user_continues("Press any key to stop recording");
    tx.send(StopStatus::UserStop)
        .expect("If the main thread exited, this thread should have stopped earlier");
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
