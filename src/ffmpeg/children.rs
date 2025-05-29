use std::{
    iter::IntoIterator,
    ops::{
        Deref,
        DerefMut,
    },
    process::Child,
    slice::{
        Iter,
        IterMut,
    },
    vec::IntoIter,
};

pub(super) struct Children(Vec<Child>);

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

/*
impl Children {
    pub(super) fn kill_all(self: &Self, sig: Option<c_int>) -> i8 {
        for child in self {
        }
    }
}
*/
