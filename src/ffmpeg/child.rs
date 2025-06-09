use crate::control::StopStatus::{self, Panic};
use std::{
    ops::{Deref, DerefMut},
    process::{self, ExitStatus},
    ffi::CStr,
    io::{self, Error, ErrorKind},
    sync::mpsc::Sender,
};
use libc::{self, c_int, pid_t, SIGKILL};

#[derive(Debug)]
pub(crate) struct Child {
    child: process::Child,
    status: Option<ExitStatus>,
}

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
    pub(crate) fn new(child: process::Child) -> Child {
        Child {
            child,
            status: None,
        }
    }

    pub(super) fn terminate(&mut self, sig: Option<c_int>)
    -> ChildResult<(), io::Error>
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
        ChildResult{id, res}
    }

    pub(super) fn wait(&mut self, kill_result: ChildResult<(), io::Error>)
    -> ChildResult<ExitStatus, io::Error>
    {
        let ChildResult{id, res} = kill_result;
        debug_assert_eq!(self.id(), id, "Child id should be equal to ChildResult id");
        if let Err(_err) = res {
            self.kill().expect("Bro, this guy's a zombie");
        }
        let res: Result<ExitStatus, io::Error> = process::Child::wait(self);
        ChildResult{id, res}
    }

    pub(super) fn status(&self) -> Option<ExitStatus> {
        self.status
    }

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
    pub(super) fn new(id: u32, res: Result<T,E>) -> ChildResult<T,E> {
        ChildResult {
            id,
            res,
        }
    }
}
