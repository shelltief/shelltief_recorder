//! # Child
//!
//! A module to overwrite the default Child structure that
//! `std::process` provides
use crate::control::StopStatus::{self, Panic};
use std::{
    ops::{Deref, DerefMut},
    process::{self, ExitStatus},
    ffi::CStr,
    io::{self, Error, ErrorKind},
    sync::mpsc::Sender,
};
use libc::{self, c_int, pid_t, SIGKILL};

/// A Child structure that holds the child exit status if
/// the child already exited and none otherwise.
#[derive(Debug)]
pub(crate) struct Child {
    child: process::Child,
    status: Option<ExitStatus>,
}

/// A result that also holds a child id to identify the exited child
#[must_use = "The `ChildResult` contains a `Result`, which should be checked for
possible failures"]
pub(crate) struct ChildResult<T,E>{
    id: u32,
    res: Result<T,E>
}

impl Deref for Child {
    type Target = process::Child;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl DerefMut for Child {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

impl Child {
    /// Creates a new Child from a `std::process::Child`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::Command;
    /// let mut command = Command::new("ls");
    /// let child = command.spawn()
    ///                 .expect("ls command failed");
    /// let mut child = Child::new(child);
    /// ```
    pub(crate) fn new(child: process::Child) -> Child {
        Child {
            child,
            status: None,
        }
    }

    /// terminates a Child by sending it the `sig` Signal or
    /// SIGKILL if `sig` is none
    ///
    /// # Panics
    ///
    /// If the value returned from `strerror` is not valid `UTF-8` data
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::Command;
    /// use libc::SIGINT;
    ///
    /// let command = Command::new("sleep");
    /// command.arg("2");
    /// let child = Child::new(command.spawn().expect("sleep command failed"));
    /// child.terminate(Some(SIGINT));
    /// ```
    pub(super) fn terminate(&mut self, sig: Option<c_int>)
    {
        let default_kill: bool = matches!(sig, None | Some(SIGKILL));
        let id: u32 = self.id();
        let res: io::Result<()> = if default_kill {
            self.kill()
        } else {
            let sig = sig.unwrap();
            let ret: c_int = unsafe {
                libc::kill(id as pid_t, sig)
            };
            match ret {
                0 => Ok(()),
                -1 => {
                    let error = unsafe {
                    let err: *mut c_int = libc::__error();
                    let slice = CStr::from_ptr(libc::strerror(*err));
                    slice.to_str()
                        .expect("Return value from strerror should be UTF-8")
                    };
                    Err(Error::new(ErrorKind::Other, error))
                },
                _ => unreachable!("`kill` should only return 0 or -1"),
            }
        };
        if let Err(_) = res {
            self.kill().expect("Bro, this guy won't budge");
        }
    }

    /// Waits for a child and returns a `ChildResult`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::Command;
    ///
    /// let command = Command::new("ls");
    /// let child = Child::new(command.spawn().expect("ls command failed"));
    /// let res = child.wait();
    /// match res {
    ///     Ok(status) => println!("Child: {} exited with status {}", res.id, status),
    ///     Err(e) => eprintln!("Child: {} exited with error {}", res.id, e),
    /// };
    /// ```
    pub(super) fn wait(&mut self)
    -> ChildResult<ExitStatus, io::Error>
    {
        let res: Result<ExitStatus, io::Error> = process::Child::wait(self);
        ChildResult{id: self.id(), res}
    }

    /// Returns the `Child` exit status in `Some(status)` if it already
    /// exited and was waited for and `None` otherwise
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::Command;
    ///
    /// let command = Command::new("ls");
    /// let child = Child::new(command.spawn().expect("ls command failed"));
    /// match child.status() {
    ///     Ok(status) => println!("ls command exited with status: {status}"),
    ///     None => {
    ///         println!("Child hasn't been waited for, waiting...");
    ///         let _ = child.wait();
    ///     }
    /// }
    /// ```
    pub(super) fn status(&self) -> Option<ExitStatus> {
        self.status
    }

    /// This function was originally meant to be used in a monitor thread
    /// in order to check if the `Child` that was tracked is still running
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::Command;
    ///
    /// let command = Command::new("sleep");
    /// command.arg("2");
    /// let child = Child::new(command.spawn().expect("ls command failed"));
    /// if child.is_running() {
    ///     println!("The child : {} is still running", child.id());
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is an error in the `try_wait` function
    pub(super) fn is_running(&mut self, tx: Option<&Sender<StopStatus>>) -> bool {
        match self.try_wait() {
            Ok(Some(status)) => {
                self.status = Some(status);
                false
            },
            Ok(None) => true,
            Err(e) => {
                let error = format!("Error: {e} in try_wait function");
                if let Some(tx) = tx {
                    let _ = tx.send(Panic(error));
                }
                panic!("Error in try_wait function");
            },
        }
    }
}

impl<T,E> ChildResult<T,E> {
    /// Creates a new `ChildResult` from a `Result`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::{self, Command};
    ///
    /// let command = Command::new("ls");
    /// let mut child = Child::new(command.spawn().expect("ls command failed"));
    /// let res = process::Child::wait(&mut child);
    /// let child_res = ChildResult::new(id: child.id(), res);
    /// ```
    pub(super) fn new(id: u32, res: Result<T,E>) -> ChildResult<T,E> {
        ChildResult {
            id,
            res,
        }
    }
}
