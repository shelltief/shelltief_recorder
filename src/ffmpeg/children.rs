use super::{Child, ChildResult};
use libc::c_int;
use std::{
    io::{self},
    iter::IntoIterator,
    ops::{
        Deref,
        DerefMut,
    },
    process::ExitStatus,
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
    pub(crate) fn cleanup(&mut self, sig: Option<c_int>)
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

    pub(super) fn new() -> Self {
        let v: Vec<Child> = Vec::new();
        Children(v)
    }
}
