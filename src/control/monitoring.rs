use crate::ffmpeg::{
    self,
    Children,
    MonitorAction,
};
use libc::SIGINT;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
use super::{
    StopStatus,
    user_control,
};

pub(crate) fn control_panel(children: Children) {
    let stopflag: Arc<Mutex<MonitorAction>>
        = Arc::new(Mutex::new(MonitorAction::Continue));
    let monitor_stop = Arc::clone(&stopflag);
    let children: Arc<Mutex<Children>>
        = Arc::new(Mutex::new(children));
    let monitor_children = Arc::clone(&children);
    let (tx, rx) = mpsc::channel::<StopStatus>();
    let tuser = tx.clone();
    let monitor_handle = thread::spawn(move || {
        ffmpeg::monitor(monitor_children, monitor_stop, tx);
    });
    thread::spawn(move || {
        user_control(tuser);
    });
    for message in rx {
        match message {
            StopStatus::Exited(id) => {
                eprintln!("Child: {id} exited early");
                break;
            },
            StopStatus::UserStop => {
                *stopflag.lock().unwrap() = MonitorAction::Stop;
                break;
            },
            StopStatus::Running => {},
            StopStatus::Panic(_) => {
                eprintln!("Monitor thread panicked");
                break;
            },
        }
    }
    match monitor_handle.join() {
        Ok(_) => {},
        Err(e) => eprintln!("Error joining monitoring thread: {:#?}", e),
    };
    let children = Arc::into_inner(children)
        .expect("Since the monitor has been joined, this Arc should be the \
only one alive");
    let mut children = children.into_inner()
        .expect("Value should be retrievable");
    children.cleanup(Some(SIGINT));
}
