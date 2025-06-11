//! # Monitor
//!
//! A module to monitor the `ffmpeg` processes
use crate::control::StopStatus::{self,*};
use std::{
        sync::{
            Arc,
            Mutex,
            mpsc::Sender,
        },
        thread,
        time,
};
use super::Children;

/// An enum for the main thread to interact
/// with the monitor thread
#[derive(PartialEq)]
pub(crate) enum MonitorAction{
    Stop,
    Continue,
}


/// A function that monitors an array of `Children` to check
/// whether or not they are still running
///
/// # Examples
///
/// ```ignore
/// use std::thread;
/// use std::sync::{Arc, Mutex, mpsc};
/// use std::Command;
/// use MonitorAction::{Stop, Continue};
/// use crate::control::StopStatus;
///
/// let command = Command::new("sleep");
/// command.args("3");
/// let children = Children::new();
/// children.push(Child::new(command.spawn().unwrap()));
/// let children = Arc::new(Mutex::new(children));
/// let (tx, rx) = channel::<StopStatus>();
/// let stopflag = Arc::new(Mutex::new(Continue));
/// thread::spawn(move || {
///     monitor(children, stopflag, tx);
/// });
/// for msg in rx {
///     match msg {
///         StopStatus::Exited(id) => {
///             println!("Child {id} exited");
///             break;
///         },
///         _ => println!("Continuing...");
///     }
/// }
/// ```
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
