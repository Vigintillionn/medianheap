use medianheap::{MedianHeap, MidpointMedian};

fn main() {
  let mut heap = MedianHeap::new(MidpointMedian);
  heap.push(1);
  heap.push(2);
  heap.push(3);
  heap.push(4);
  heap.push(6);
  heap.push(7);
  heap.push(8);
  heap.push(9);
  heap.push(0);

  println!("{:?}", heap);
  println!("{:?}", heap.get_median());
}