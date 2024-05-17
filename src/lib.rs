//! A library to keep track of a running median of a sequence of numbers.

pub mod mergemedian;
pub mod medianheap;
pub use mergemedian::{MergeMedian, LeftHandedMedian, MidpointMedian};
pub use medianheap::MedianHeap;

#[cfg(test)]
mod tests;