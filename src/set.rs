use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

struct Set<T: Eq + Hash>(HashSet<T>);

/**
 * Constructor from a Vec<T> where T implement Eq + Hash
 * @constructor
 */
impl<T: Eq + Hash> From<Vec<T>> for Set<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut new_hashset: HashSet<T> = HashSet::new();

        for i in vec {
            new_hashset.insert(i);
        }

        Self(new_hashset)
    }
}

/**
 * Constructor from a Set<T> to a Vec<T>
 * @constructor
 */
impl<T: Eq + Hash> From<Set<T>> for Vec<T> {
    fn from(set: Set<T>) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        for i in set.0 {
            result.push(i);
        }
        result
    }
}

impl<T: Eq + Hash> Set<T> {
    pub fn singleton(elt: T) -> Set<T> {
        let mut new_hashset: HashSet<T> = HashSet::new();
        new_hashset.insert(elt);
        Self(new_hashset)
    }
}
#[test]
fn check_singleton() {
    let a = Set::singleton(1);
    assert_eq!(Vec::from(a), vec![1]);
}

impl<T: Eq + Hash> Set<T> {
    /**
     * Check if an element is a member of a set
     */
    pub fn elem(&self, element: &T) -> bool {
        self.0.contains(element)
    }
}

#[test]
fn check_elem() {
    let a = Set::singleton(1);
    assert_eq!(a.elem(&1), true);
    assert_eq!(a.elem(&2), false);
}

impl<T: Eq + Hash> Set<T> {
    /**
     * Check if a set is empty
     */
    pub fn empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Eq + Hash> Set<T> {
    /// An iterator that filters the elements of `iter` with `predicate`.
    ///
    /// This `struct` is created by the [`filter`] method on [`Iterator`]. See its
    /// documentation for more.
    ///
    /// [`filter`]: trait.Iterator.html#method.filter
    /// [`Iterator`]: trait.Iterator.html
    pub fn every<P: FnMut(&T) -> bool>(&self, predicate: P) -> bool {
        todo!()
    }
}

/*
struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    fn new(v: Vec<T>) -> Result<Self, Vec<T>> {
        if v.is_empty() {
            Err(v)
        } else {
            Ok(NonEmptyVec(v))
        }
    }
}
*/
