use crate::control::StopStatus::{self,*};
use std::{
        process::{
            Child,
            Command,
            Stdio,
        },
        sync::{
            Arc,
            Mutex,
            mpsc::Sender,
        },
};
use super::Children;

#[derive(PartialEq)]
pub(crate) enum MonitorAction{
    Stop,
    Continue,
}


pub(crate) fn monitor(
    children: Arc<Mutex<Children>>,
    stopflag: Arc<Mutex<MonitorAction>>,
    tx: Sender<StopStatus>) {
    loop {
        if *stopflag.lock().unwrap() == MonitorAction::Stop {
            return;
        }
        let children = children.lock().unwrap();
        for child in &*children {
            if ! is_running(&child) {
                tx.send(Exited(child.id()));
                return;
            }
        }
        tx.send(Running)
            .expect("If main thread has been stopped, monitor should have been\
killed beforehand");
    }
}

fn is_running(child: &Child) -> bool {
    let mut check = Command::new("ps");
    check.arg("-p")
        .arg(format!("{}", child.id()));
    check.stdout(Stdio::null())
        .stderr(Stdio::null());
    match check.status() {
        Ok(st) => match st.code() {
            Some(0) => true,
            _ => false,
        },
        _ => false,
    }
}
