# MedianHeap

MedianHeap is a Rust crate that provides a data structure for calculating the median of a collection of values in constant time. It utilizes two binary heaps to store values efficiently and offers flexibility in calculating the median by allowing different strategies through user-defined traits.

# Installation

Idk, I might publish this but for now just clone the repo :shrug:.

# Usage

To use `MedianHeap`, add this to your code:

```rs
use median_heap::{MedianHeap, MergeMedian};

struct MyMedian;
impl MergeMedian<i32> for MyMedian {
  fn merge(&self, a: &i32, b: &i32) -> i32 {
    if a > b {
      *a
    } else {
      *b
    }
  }
}

fn main() {
  let mut heap = MedianHeap::new(MyMedian);
  heap.push(1);
  heap.push(2);
  heap.push(3);
  heap.push(4);

  assert_eq!(3, heap.get_median().unwrap()); // Two median candidates are 2 and 3. MyMedian.merge(2, 3) returns 3.
}
```

# Examples

If I don't feel lazy I might add some examples. It's a really simple library though. I have faith in you that you'll get it to work <3.

# TODO

Here are some things I - might - still implement.
- Search/Get from the heap
- Pop/Delete from the heap
- Add some more utility

# License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/Vigintillionn/medianheap/blob/main/LICENSE) file for details.