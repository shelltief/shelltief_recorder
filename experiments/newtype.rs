use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    vec::IntoIter
};

#[derive(Debug)]
struct Test(Vec<i8>);

impl Deref for Test{
    type Target = Vec<i8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Test {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Test{
    pub fn new(v: Vec<i8>) -> Test {
        Test(v)
    }
    pub fn iter(&self) -> Iter<'_, i8> {
        (**self).iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, i8>{
        (**self).iter_mut()
    }
}

impl<'a> IntoIterator for &'a Test {
    type Item = &'a i8;
    type IntoIter = Iter<'a, i8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Test {
    type Item = &'a mut i8;
    type IntoIter = IterMut<'a, i8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> IntoIterator for Test {
    type Item = i8;
    type IntoIter = IntoIter<i8>;

    fn into_iter(self) -> Self::IntoIter {
        let Test(vec) = self;
        vec.into_iter()
    }
}

pub fn main() {
    let v: Vec<i8> = vec![1, 2, 3];
    let t: Test = Test::new(v);
    println!("{:#?}", t);
    for i in &t {
        println!("i: {i}");
    }
}
