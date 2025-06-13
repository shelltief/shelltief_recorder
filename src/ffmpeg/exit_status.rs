//! # Exit Status
//!
//! An enum that mimics [`std::process::ExitStatus`], in
//! order to use it with unsafe `wait*` functions

/// An enum that holds the different
/// states in which a process can exit
#[derive(Debug)]
pub(super) enum ExitStatus {
    Exited(u32),
    Stopped(u32),
    Signaled(u32),
}
