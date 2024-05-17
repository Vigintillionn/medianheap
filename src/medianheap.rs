use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fmt::Debug;
use std::ops::Add;
// use std::vec::IntoIter;

use crate::MergeMedian;

/// MedianHeap is a struct that represents a heap data structure that can calculate the median of the values in the heap in constant time.
/// It uses two binary heaps to store the values in the heap: a max heap and a min heap.
/// The max heap stores the smaller half of the values, and the min heap stores the larger half of the values.
/// 
/// The MedianHeap struct takes two type parameters: T and K.
/// T is the type of the values stored in the heap.
/// K is a type that implements the MergeMedian trait for the type T.
/// 
/// Example:
/// ```
/// use medianheap::{MidpointMedian, MedianHeap};
/// 
/// let mut heap = MedianHeap::new(MidpointMedian);
/// heap.push(2);
/// heap.push(4);
/// heap.push(6);
/// heap.push(8);
/// heap.push(10);
/// 
/// assert_eq!(6, heap.get_median().unwrap()); // The median of the values 2, 4, 6, 8, 10 is 6.
/// ```
pub struct MedianHeap<T, K> {
  median_kind: K,
  max_heap: BinaryHeap<T>,
  min_heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord, K: Default> Default for MedianHeap<T, K> {
  fn default() -> Self {
    MedianHeap {
      median_kind: K::default(),
      max_heap: BinaryHeap::new(),
      min_heap: BinaryHeap::new(),
    }
  }
}

impl<T: Ord, K: MergeMedian<T>> MedianHeap<T, K> {
  /// Creates a new MedianHeap instance with the specified median kind.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MidpointMedian, MedianHeap};
  /// 
  /// let mut heap = MedianHeap::new(MidpointMedian);
  /// heap.push(1);
  /// heap.push(2);
  /// 
  /// assert_eq!(1, heap.get_median().unwrap());
  /// ```
  /// 
  /// In this example, a new MedianHeap instance is created with the MidpointMedian median kind.
  pub fn new(median_kind: K) -> Self {
    MedianHeap {
      median_kind,
      max_heap: BinaryHeap::new(),
      min_heap: BinaryHeap::new(),
    }
  }
}

impl<T: Ord + Add + Copy, K: MergeMedian<T>> MedianHeap<T, K> {
  /// Returns the median of the values in the heap.
  /// If the heap is empty, the method returns None.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// heap.push(1);
  /// heap.push(2);
  /// heap.push(3);
  /// heap.push(4);
  /// 
  /// assert_eq!(2, heap.get_median().unwrap());
  /// ```
  /// 
  /// In this example, the median of the values 1, 2, 3, 4 is 2.
  /// 
  /// # Complexity
  /// O(1)
  pub fn get_median(&self) -> Option<T> {
    if self.max_heap.len() == 0 && self.min_heap.len() == 0 {
      return None
    }

    // If the number of values in the max heap and min heap are equal, two candidates are found.
    // If not then the median is the root of the larger heap.
    if self.max_heap.len() == self.min_heap.len() {
      // Merge the two candidates to get the median.
      let median = self.median_kind.merge(self.max_heap.peek().unwrap(), &self.min_heap.peek().unwrap().0);
      return Some(median)
    } else if self.max_heap.len() > self.min_heap.len() {
      return Some(*self.max_heap.peek().unwrap())
    } else {
      return Some(self.min_heap.peek().unwrap().0)
    }
  }
}

impl<T: Ord + Add + Copy, K: MergeMedian<T>> MedianHeap<T, K> {
  /// Adds a value to the heap.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// heap.push(2);
  /// 
  /// assert_eq!(2, heap.get_median().unwrap());
  /// 
  /// heap.push(1);
  /// 
  /// assert_eq!(1, heap.get_median().unwrap());
  /// ```
  pub fn push(&mut self, value: T) {
    // If the heap is empty, push the value to the max heap.
    if self.is_empty() {
      self.max_heap.push(value);
      return
    }

    // Get the median of the values in the heap.
    let median = self.get_median().unwrap();
    // If the value is less than the median, push it to the max heap.
    // If the value is greater than the median, push it to the min heap.
    if value < median {
      self.max_heap.push(value);
    } else {
      self.min_heap.push(Reverse(value));
    }

    // Balance the heaps.
    // If the difference between the number of values in the max heap and min heap is greater than 1, pop the root of the larger heap and push it to the smaller heap.
    // This ensures that the difference between the number of values in the max heap and min heap is at most 1.
    if self.max_heap.len() > self.min_heap.len() + 1 {
      let value = self.max_heap.pop().unwrap();
      self.min_heap.push(Reverse(value));
    } else if self.min_heap.len() > self.max_heap.len() {
      let value = self.min_heap.pop().unwrap().0;
      self.max_heap.push(value);
    }
  }

  /// Removes and returns the median of the values in the heap.
  /// If the heap is empty, the method returns None.
  /// 
  /// If two median candidates are found, the method pops both and merges them using the median kind to get the median.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// heap.push(1);
  /// heap.push(2);
  /// heap.push(3);
  /// 
  /// assert_eq!(2, heap.pop().unwrap());
  /// assert_eq!(2, heap.len());
  /// 
  /// assert_eq!(1, heap.pop().unwrap());
  /// assert_eq!(0, heap.len());
  /// ```
  /// 
  /// # Complexity
  /// O(1) 
  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      return None
    }

    if self.max_heap.len() == self.min_heap.len() {
      let left = self.max_heap.pop().unwrap();
      let right = self.min_heap.pop().unwrap().0;
      let median = self.median_kind.merge(&left, &right);
      return Some(median)
    } else if self.max_heap.len() > self.min_heap.len() {
      return Some(self.max_heap.pop().unwrap())
    } else {
      return Some(self.min_heap.pop().unwrap().0)
    }
  }

  pub fn delete(&mut self, value: &T) {
    if self.is_empty() {
      return;
    }

    let median = self.get_median().unwrap();

    if *value < median {
      self.max_heap.retain(|x| x != value);
    } else {
      self.min_heap.retain(|x| x.0 != *value);
    }
  }

  /// Returns true if the heap contains the specified value, false otherwise.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// heap.push(1);
  /// heap.push(2);
  /// 
  /// assert_eq!(true, heap.has(1));
  /// assert_eq!(true, heap.has(2));
  /// assert_eq!(false, heap.has(3));
  /// 
  /// heap.push(3);
  /// 
  /// assert_eq!(true, heap.has(3));
  /// ```
  /// 
  /// # Complexity
  /// O(n)
  pub fn has(&self, value: &T) -> bool {
    if self.is_empty() {
      return false
    }

    let median = self.get_median().unwrap();

    if *value == median {
      return true
    }
    else if *value < median {
      // Search in the max heap.
      self.max_heap.iter().any(|x| *x == *value)
    } else {
      // Search in the min heap.
      self.min_heap.iter().any(|x| x.0 == *value)
    }
  }

  pub fn peak_max(&self) -> Option<&T> {
    self.max_heap.peek()
  }

  pub fn peak_min(&self) -> Option<&T> {
    self.min_heap.peek().map(|x| &x.0)
  }
}

impl<T, K> MedianHeap<T, K> {
  /// Returns the number of values in the heap.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// heap.push(1);
  /// heap.push(2);
  /// 
  /// assert_eq!(2, heap.len());
  /// ```
  /// 
  /// # Complexity
  /// O(1)
  pub fn len(&self) -> usize {
    self.max_heap.len() + self.min_heap.len()
  }

  /// Returns true if the heap is empty, false otherwise.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// 
  /// assert_eq!(true, heap.is_empty());
  /// 
  /// heap.push(1);
  /// 
  /// assert_eq!(false, heap.is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Removes all values from the heap.
  /// 
  /// Example:
  /// ```
  /// use medianheap::{MedianHeap, LeftHandedMedian};
  /// 
  /// let mut heap = MedianHeap::new(LeftHandedMedian);
  /// heap.push(1);
  /// heap.push(2);
  /// 
  /// assert_eq!(2, heap.len());
  /// 
  /// heap.clear();
  /// 
  /// assert_eq!(0, heap.len());
  /// ```
  pub fn clear(&mut self) {
    self.max_heap.clear();
    self.min_heap.clear();
  }
}

impl<T: Debug + Copy, K> Debug for MedianHeap<T, K> {
  /// Formats the heap for debugging purposes.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "max_heap: {:?}, min_heap: {:?}", self.max_heap, self.min_heap.iter().map(|x| x.0).collect::<Vec<_>>())
  }
}

// Pretty useless, since there's no order guarantee due to the heaps.
// impl<T: Debug, K> IntoIterator for MedianHeap<T, K> {
//   type Item = T;
//   type IntoIter = IntoIter<T>;

//   fn into_iter(self) -> Self::IntoIter {
//     self.max_heap
//       .into_iter()
//       .chain(
//           self.min_heap.into_iter().map(|x| Reverse(x.0).0)
//       ).collect::<Vec<_>>()
//       .into_iter()
//   }
// }

impl<T: Ord + Add<Output = T> + Copy, K: MergeMedian<T> + Default> FromIterator<T> for MedianHeap<T, K> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut heap = MedianHeap::new(K::default());
    for value in iter {
      heap.push(value);
    }
    heap
  }
}

impl<T: Ord + Clone, K: Clone> Clone for MedianHeap<T, K> {
  fn clone(&self) -> Self {
    MedianHeap {
      median_kind: self.median_kind.clone(),
      max_heap: self.max_heap.clone(),
      min_heap: self.min_heap.clone(),
    }
  }
}