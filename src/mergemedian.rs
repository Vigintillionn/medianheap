use std::ops::{Add, Div};
use num::traits::One;

/// MergeMedian is a trait that defines a method to merge two values of the same type into a single value.
/// It's used by the MedianHeap struct to calculate the median of the values in the heap when 2 median candidates are found.
/// 
/// Example:
/// 
/// ```
/// use median_heap::MergeMedian;
/// 
/// struct MyMedian;
/// impl MergeMedian<i32> for MyMedian {
///    fn merge(&self, a: &i32, b: &i32) -> i32 {
///         if a > b {
///             *a
///         } else {
///             *b
///         }
///     }
/// }
/// ```
/// 
/// In this example, MyMedian is a struct that implements the MergeMedian trait for i32 values.
/// The merge method takes two i32 values and returns the larger of the two.
/// 
/// When the MedianHeap struct is created, it's passed an instance of MyMedian as a type parameter.
/// This allows the MedianHeap struct to use the MyMedian struct to calculate the median of the values in the heap when 2 median candidates are found.
/// 
/// ```
/// use median_heap::{MergeMedian, MedianHeap};
/// 
/// struct MyMedian;
/// impl MergeMedian<i32> for MyMedian {
///    fn merge(&self, a: &i32, b: &i32) -> i32 {
///         if a > b {
///             *a
///         } else {
///             *b
///         }
///     }
/// }
/// 
/// let mut heap = MedianHeap::new(MyMedian);
/// heap.push(1);
/// heap.push(2);
/// heap.push(3);
/// heap.push(4);
/// 
/// assert_eq!(3, heap.get_median().unwrap()); // Two median candidates are 2 and 3. MyMedian.merge(2, 3) returns 3.
/// ```
pub trait MergeMedian<T> {
  fn merge(&self, a: &T, b: &T) -> T;
}

/// LeftHandedMedian is a struct that implements the MergeMedian trait.
/// It calculates the median by taking the smaller of the two values.
/// 
/// Example:
/// ```
/// use median_heap::{LeftHandedMedian, MergeMedian};
/// 
/// let left_handed_median = LeftHandedMedian;
/// let a = 1;
/// let b = 2;
/// let median = left_handed_median.merge(&a, &b);
/// assert_eq!(median, 1);
/// ```
#[derive(Clone)]
pub struct LeftHandedMedian;
impl<T: Ord + Add + Copy> MergeMedian<T> for LeftHandedMedian {
  fn merge(&self, a: &T, b: &T) -> T {
      if a < b {
          *a
      } else {
          *b
      }
  }
}

impl Default for LeftHandedMedian {
  fn default() -> Self {
      LeftHandedMedian
  }
}

/// MidpointMedian is a struct that implements the MergeMedian trait.
/// It calculates the median by taking the average of the two values.
/// 
/// Example:
/// ```
/// use median_heap::{MidpointMedian, MergeMedian};
/// 
/// let midpoint_median = MidpointMedian;
/// let a = 2;
/// let b = 4;
/// let median = midpoint_median.merge(&a, &b);
/// assert_eq!(median, 3);
/// ```
#[derive(Clone)]
pub struct MidpointMedian;
impl<T: Div<Output = T> + Add<T, Output = T> + From<i32> + Copy + One> MergeMedian<T> for MidpointMedian {
  fn merge(&self, a: &T, b: &T) -> T {
      (*a + *b) / (T::one() + T::one())
  }
}

impl Default for MidpointMedian {
  fn default() -> Self {
      MidpointMedian
  }
}