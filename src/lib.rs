#![feature(collections)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::marker::PhantomData;

#[cfg(feature = "revord")]
extern crate revord;

pub trait Stamped<S>
  where S : Ord {

  fn stamp_ref(&self) -> &S;
}

pub struct StampOrdered<S: Stamped<ST>, ST : Ord>(S, PhantomData<ST>);

impl<S, ST> StampOrdered<S, ST>
    where S: Stamped<ST>, ST: Ord{

  pub fn new(stamped:S) -> StampOrdered<S,ST> {
    StampOrdered(stamped, PhantomData)
  }

  pub fn stamped_ref(&self) -> &S {
    let &StampOrdered(ref stamped, _) = self;
    stamped
  }
}

impl<S, ST> Stamped<ST> for StampOrdered<S, ST>
    where S: Stamped<ST>, ST: Ord {
  fn stamp_ref(&self) -> &ST{
    self.stamped_ref().stamp_ref()
  }
}

impl<S, ST> PartialEq for StampOrdered<S, ST>
    where S: Stamped<ST>, ST: Ord {
  fn eq(&self, other:&StampOrdered<S, ST>) -> bool {
    self.stamp_ref() == other.stamp_ref()
  }
}

impl<S, ST> Eq for StampOrdered<S, ST>
    where S: Stamped<ST>, ST: Ord {}

impl<S, ST> PartialOrd for StampOrdered<S, ST>
    where S:Stamped<ST>, ST: Ord {
  fn partial_cmp(&self, other:&StampOrdered<S, ST>) -> Option<Ordering> {
    self.stamp_ref().partial_cmp(other.stamp_ref())
  }
}

impl<S, ST> Ord for StampOrdered<S, ST>
    where S:Stamped<ST>, ST: Ord {
  fn cmp(&self, other:&StampOrdered<S, ST>) -> Ordering {
    self.stamp_ref().cmp(other.stamp_ref())
  }
}

pub trait StampOrderedVec<S: Stamped<ST>, ST: Ord> {
  type _PhantomStampTypeMarker = PhantomData<ST>;
  fn unwrap_ordering(self) -> Vec<S>;
}

impl<S,ST> StampOrderedVec<S, ST> for Vec<StampOrdered<S, ST>>
    where S: Stamped<ST>, ST: Ord {
  type _PhantomStampTypeMarker = PhantomData<ST>;
  fn unwrap_ordering(self) -> Vec<S> {
    self.map_in_place(|sord| sord.0)
  }
}

pub fn wrap_vector_stamp_ordering<ST: Ord, S: Stamped<ST>>(vec: Vec<S>) -> Vec<StampOrdered<S, ST>>{
  vec.map_in_place(|stamped| StampOrdered::new(stamped))
}

trait StampedVec<S: Stamped<ST>, ST: Ord> {
  type _PhantomStampTypeMarker = PhantomData<ST>;
  fn sort_by_stamp(&mut self);
}

pub trait StampedHeap<S, ST> where S: Stamped<ST>, ST: Ord{
  type _PhantomStampTypeMarker = PhantomData<ST>;
  fn push_stamped(&mut self, stamped:S);
  fn pop_stamped(&mut self) -> Option<S>;
  fn peek_stamped(&self) -> Option<&S>;
}

impl<S, ST> StampedHeap<S, ST> for BinaryHeap<StampOrdered<S, ST>>
    where S: Stamped<ST>, ST: Ord {
  type _PhantomStampTypeMarker = PhantomData<ST>;
  fn push_stamped(&mut self, stamped:S) {
    self.push(StampOrdered::new(stamped))
  }

  fn pop_stamped(&mut self) -> Option<S> {
    self.pop().map(|sord| sord.0)
  }

  fn peek_stamped(&self) -> Option<&S> {
    self.peek().map(|sord| &sord.0)
  }
}

pub fn get_stamp_ref<S:Stamped<ST>, ST: Ord>(stamped:&S) -> &ST {
  stamped.stamp_ref()
}

pub fn stamp_ref_cmp<S:Stamped<ST>, ST: Ord>(a:&S,b:&S) -> Ordering {
  a.stamp_ref().cmp(b.stamp_ref())
}

#[cfg(feature = "revord")]
impl<S> Stamped for revord::RevOrd<S> where S:Stamped{
  type Stamp = S::Stamp;
  fn stamp_ref(&self) -> &S::Stamp {
    self.0.stamp_ref()
  }
}

#[cfg(test)]
mod test {
  use super::{Stamped, StampedHeap, StampOrdered, stamp_ref_cmp};
  use std::collections::BinaryHeap;

  impl Stamped<isize> for isize {
    fn stamp_ref(&self) -> &isize {
      self
    }
  }

  #[test]
  fn sort_stamp_vec(){
    let mut stamped_vec = vec![3,0,-10,100];
    stamped_vec.sort_by(stamp_ref_cmp);

    assert_eq!(stamped_vec, vec![-10,0,3,100]);
  }

  #[test]
  fn stamp_heap_test(){
    let mut heap = BinaryHeap::<StampOrdered<isize, isize>>::new();
    heap.push_stamped(12);
    heap.push_stamped(13);
    heap.push_stamped(0);

    assert_eq!(heap.peek_stamped(), Some(&13));
    assert_eq!(heap.pop_stamped(), Some(13));
    assert_eq!(heap.pop_stamped(), Some(12));
    assert_eq!(heap.peek_stamped(), Some(&0));
    assert_eq!(heap.pop_stamped(), Some(0));
    assert_eq!(heap.pop_stamped(), None);
  }
}
