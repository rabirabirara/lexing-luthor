
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub struct StateSet<T: Hash + Eq + Ord> {
    inner: HashSet<T>,
}



impl<T: Hash + Eq + Ord> StateSet<T> {
    pub fn new() -> Self {
        Self {
            inner: HashSet::new()
        }
    }
    pub fn insert(&mut self, item: T) {
        self.inner.insert(item);
    }
    pub fn contains(&self, item: &T) -> bool {
        self.inner.contains(item)
    }
}

impl<T: Hash + Eq + Ord> std::string::ToString for StateSet<T> {
    fn to_string(&self) -> String {
        let mut v = Vec::from_iter(self.inner.iter());
        v.sort();
        
    }
}