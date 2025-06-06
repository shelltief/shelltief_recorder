use std::process::{Command, Stdio};

fn main() {
    let mut pgrep = Command::new("pgrep");
    pgrep.arg("[pP]ython");
    pgrep.stdout(Stdio::null())
        .stderr(Stdio::null());
    match pgrep.status() {
        Ok(st) => {
            match st.code() {
                Some(0) => println!("Successful termination"),
                Some(code) => println!("Exited with error: {code}"),
                None => println!("Got killed by signal"),
            };
        },
        Err(st) => println!("Failed: {st}"),
    };
}
