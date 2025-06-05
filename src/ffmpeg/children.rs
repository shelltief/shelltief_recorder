use libc::{self, c_int, pid_t, SIGKILL};
use std::{
    ffi::CStr,
    io::{self, Error, ErrorKind},
    iter::IntoIterator,
    ops::{
        Deref,
        DerefMut,
    },
    process::{Child, ExitStatus},
    slice::{
        Iter,
        IterMut,
    },
    vec::IntoIter,
};

pub(crate) struct Children(Vec<Child>);

#[must_use = "The `ChildResult` contains a `Result`, which should be checked for
possible failures"]
pub(crate) struct ChildResult<T,E>{
    id: u32,
    res: Result<T,E>
}

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

fn terminate_child(child: &mut Child, sig: Option<c_int>)
-> ChildResult<(), io::Error>
{
    let default_kill: bool = matches!(sig, None | Some(SIGKILL));
    let id: u32 = child.id();
    let res: io::Result<()> = if default_kill {
        child.kill()
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

fn wait_child(child: &mut Child, kill_result: ChildResult<(), io::Error>)
-> ChildResult<ExitStatus, io::Error>
{
    let ChildResult{id, res} = kill_result;
    debug_assert_eq!(child.id(), id, "Child id should be equal to ChildResult id");
    if let Err(_err) = res {
        child.kill().expect("Bro, this guy's a zombie");
    }
    let res: Result<ExitStatus, io::Error> = child.wait();
    ChildResult{id, res}
}

impl Children {
    /// Terminates and waits all children. Last stop before cleanup
    pub(crate) fn cleanup(&mut self, sig: Option<c_int>)
    -> Vec<ChildResult<ExitStatus, io::Error>>
    {
        let mut results: Vec<ChildResult<ExitStatus, io::Error>> = Vec::new();
        for child in self {
            let kill_result = terminate_child(child, sig);
            let res = wait_child(child, kill_result);
            results.push(res);
        }
        results
    }

    pub(super) fn new() -> Self {
        let v: Vec<Child> = Vec::new();
        Children(v)
    }
}
