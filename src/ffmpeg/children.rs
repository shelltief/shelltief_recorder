//! Implementation of the newtype pattern to wrap
//! a vector of `ffmpeg::Child` and add it some
//! functionnalities
use super::{Child, ChildResult, ExitStatus, Signal};
use std::{
    io::{self},
    iter::IntoIterator,
    ops::{
        Deref,
        DerefMut,
    },
    slice::{
        Iter,
        IterMut,
    },
    vec::IntoIter,
};

pub(crate) struct Children(Vec<Child>);

impl Deref for Children {
    type Target = Vec<Child>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Children {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Children {
    fn iter(&self) -> Iter<'_, Child> {
        (**self).iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, Child> {
        (**self).iter_mut()
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = &'a Child;
    type IntoIter = Iter<'a, Child>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Children {
    type Item = &'a mut Child;
    type IntoIter = IterMut<'a, Child>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl IntoIterator for Children {
    type Item = Child;
    type IntoIter = IntoIter<Child>;

    fn into_iter(self) -> Self::IntoIter {
        let Children(childrens) = self;
        childrens.into_iter()
    }
}


impl Children {
    /// Terminates and waits all children. Last stop before cleanup
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::process::Command;
    /// use libc::SIGINT;
    ///
    /// let c1 = Command::new("sleep");
    /// c1.args("2");
    /// let c2 = Command::new("sleep");
    /// c2.args("3");
    /// let children = Children::new();
    /// children.push(c1.spawn().unwrap());
    /// children.push(c2.spawn().unwrap());
    /// children.cleanup(Some(SIGINT));
    /// ```
    ///
    /// # Errors
    ///
    /// If there is an error waiting for the process, an error is reported
    ///
    /// # Panics
    ///
    /// If there is an error in the second firing of the `kill` signal (in
    /// `child.terminate`)
    pub(crate) fn cleanup(&mut self, sig: Option<Signal>)
    -> Vec<ChildResult<ExitStatus, io::Error>>
    {
        let mut results: Vec<ChildResult<ExitStatus, io::Error>> = Vec::new();
        for child in self {
            let res = {
                if let Some(status) = child.status() {
                    ChildResult::new(child.id(), Ok(status))
                } else {
                    child.terminate(sig);
                    child.wait()
                }
            };
            results.push(res);
        }
        results
    }

    /// Wraps a `Vec<std::process::Child>` into a `Children`
    pub(super) fn new() -> Self {
        let v: Vec<Child> = Vec::new();
        Children(v)
    }
}
