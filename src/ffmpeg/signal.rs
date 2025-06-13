//! # Signal
//!
//! A signal structure to ensure the user signal is
//! always valid

use libc::c_int;
use std::ops::Deref;

#[derive(PartialEq, Copy, Clone)]
pub(crate) struct Signal(c_int);

impl Deref for Signal {
    type Target = c_int;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Signal {
    /// Creates a new signal, ensuring that it has
    /// a valid signal number
    ///
    /// # Examples
    ///
    /// ```
    /// use libc::SIGINT;
    ///
    /// let sig = Signal::new(SIGINT);
    /// ```
    ///
    /// ```should_panic
    /// let sig = Signal::new(-1);
    /// ```
    ///
    /// # Panics
    ///
    /// - If `sig` is not a valid signal number
    /// (see `man 7 signal`)
    pub(crate) fn new(sig: c_int) -> Signal {
        match sig {
            0..=31 => Signal(sig),
            _ => panic!("Invalid signal number provided"),
        }
    }
}
