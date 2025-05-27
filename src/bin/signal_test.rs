use std::{
    process::Command,
    io::{
        self,
        Write
    },
    thread,
    time::Duration
};

use libc::{
    SIGINT,
    kill,
    pid_t,
    c_int,
};

pub fn main() {

    let mut test_command: Command = Command::new("bash");
    test_command.arg("experiments/test_script");

    /*
    let output = test_command.output()
        .expect("test_script failed to start");


    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    */

    let mut child = test_command.spawn()
        .expect("test script failed to start");
    let ret: c_int;
    thread::sleep(Duration::from_secs(1));
    unsafe {
        println!("id: {}", child.id());
        ret = kill(child.id() as pid_t, SIGINT);
    }
    let ret = ret as i8;
    let _ = child.wait()
        .expect("Failed to wait on child");
    println!("This is the return value: {}", ret);
}
