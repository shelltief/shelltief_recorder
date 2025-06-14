//! # Exit Status
//!
//! An enum that mimics [`std::process::ExitStatus`], in
//! order to use it with unsafe `wait*` functions

use super::Signal;
use std::{
    convert::From,
    process,
};

/// An enum that holds the different
/// states in which a process can exit
#[derive(Clone, Copy, Debug)]
pub(crate) enum ExitStatus {
    Exited(i32),
    Signaled(Option<Signal>),
    Stopped(Option<Signal>),
}

impl From<process::ExitStatus> for ExitStatus {
    fn from(value: process::ExitStatus) -> Self {
        match value.code() {
            Some(code) => ExitStatus::Exited(code),
            None => ExitStatus::Signaled(None),
        }
    }
}
