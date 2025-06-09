use crate::control::StopStatus::{self,*};
use std::{
        process::{
            Child,
            Command,
        },
        sync::{
            Arc,
            Mutex,
            mpsc::Sender,
        },
        thread,
        time,
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
        let mut children = children.lock().unwrap();
        for child in &mut *children {
            println!("{:#?}", child);
            if ! child.is_running(Some(&tx)) {
                tx.send(Exited(child.id()))
                    .expect("Main thread receiving end is closed");
                return;
            }
        }
        if tx.send(Running).is_err() {
            break;
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn is_running(child: &Child) -> bool {
    let mut check = Command::new("ps");
    check.arg("-p")
        .arg(format!("{}", child.id()));
    /*check.stdout(Stdio::null())
        .stderr(Stdio::null());*/
    match check.status() {
        Ok(st) => {match st.code() {
            Some(0) => true,
            _ => false,
        }},
        _ => false,
    }
}
