use std::cmp::Ordering;
use crate::float::kdtree::Axis;

pub struct HeapElement<A, T> {
    pub distance: A,
    pub item: T,
}

impl<A: Axis, T> Ord for HeapElement<A, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<A: Axis, T> PartialOrd for HeapElement<A, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<A: Axis, T> Eq for HeapElement<A, T> {}

impl<A: Axis, T> PartialEq for HeapElement<A, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<A: Axis, T> PartialEq<A> for HeapElement<A, T> {
    fn eq(&self, other: &A) -> bool {
        self.distance == *other
    }
}

impl<A: Axis, T> From<HeapElement<A, T>> for (A, T) {
    fn from(elem: HeapElement<A, T>) -> Self {
        (elem.distance, elem.item)
    }
}