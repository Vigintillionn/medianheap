use super::*;

#[test]
fn test_left_handed_median() {
    let left_handed_median = LeftHandedMedian;
    let a = 1;
    let b = 2;
    let median = left_handed_median.merge(&a, &b);
    assert_eq!(median, 1);
}

#[test]
fn test_right_handed_median() {
    let right_handed_median = MidpointMedian;
    let a = 2;
    let b = 4;
    let median = right_handed_median.merge(&a, &b);
    assert_eq!(median, 3);
}

#[test]
fn test_median_heap() {
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

    assert_eq!(9, heap.len());
    assert_eq!(4, heap.get_median().unwrap());
}

#[test]
fn test_median_heap_clear() {
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

    assert_eq!(9, heap.len());
    heap.clear();
    assert_eq!(0, heap.len());
}

#[test]
fn test_median_heap_is_empty() {
    let mut heap = MedianHeap::new(MidpointMedian);
    assert_eq!(true, heap.is_empty());
    heap.push(1);
    assert_eq!(false, heap.is_empty());
}

#[test]
fn test_median_heap_get_median() {
    let mut heap = MedianHeap::new(MidpointMedian);
    heap.push(1);
    heap.push(2);
    heap.push(3);
    heap.push(4);
    assert_eq!(2, heap.get_median().unwrap());
}

#[test]
fn test_median_heap_push() {
    let mut heap = MedianHeap::new(MidpointMedian);
    heap.push(1);
    heap.push(2);
    heap.push(3);
    heap.push(4);

    assert_eq!(2, heap.get_median().unwrap());

    heap.push(5);
    assert_eq!(3, heap.get_median().unwrap());

    heap.push(6);
    heap.push(7);
    assert_eq!(4, heap.get_median().unwrap());
}

#[test]
fn test_median_heap_pop() {
    let mut heap = MedianHeap::new(MidpointMedian);
    heap.push(1);
    heap.push(2);
    heap.push(3);
    heap.push(4);
    heap.push(5);
    heap.push(6);
    heap.push(7);
    heap.push(7);
    heap.push(7);

    assert_eq!(5, heap.get_median().unwrap());
    assert_eq!(9, heap.len());

    assert_eq!(5, heap.pop().unwrap());
    assert_eq!(8, heap.len());

    assert_eq!(5, heap.pop().unwrap());
    assert_eq!(6, heap.len());

    assert_eq!(5, heap.pop().unwrap());
    assert_eq!(4, heap.len());

    assert_eq!(4, heap.pop().unwrap());
    assert_eq!(2, heap.len());

    assert_eq!(4, heap.pop().unwrap());
    assert_eq!(0, heap.len());

    assert_eq!(None, heap.pop());
}

#[test]
fn test_median_heap_has() {
  let mut heap = MedianHeap::new(MidpointMedian);
  heap.push(1);
  heap.push(2);
  heap.push(3);
  heap.push(4);
  heap.push(5);
  heap.push(6);
  heap.push(7);
  heap.push(7);
  heap.push(7);

  assert_eq!(true, heap.has(&1));
  assert_eq!(true, heap.has(&2));
  assert_eq!(true, heap.has(&3));
  assert_eq!(true, heap.has(&4));
  assert_eq!(true, heap.has(&5));
  assert_eq!(true, heap.has(&6));
  assert_eq!(true, heap.has(&7));
  assert_eq!(false, heap.has(&8));
  assert_eq!(false, heap.has(&9));
}

#[test]
fn test_median_heap_fromiter() {
  let iter = vec![1, 2, 3, 4, 5, 6, 7, 7, 7].into_iter();
  let heap: MedianHeap<i32, MidpointMedian> = MedianHeap::from_iter(iter);

  assert_eq!(5, heap.get_median().unwrap());
  assert_eq!(9, heap.len());
}

#[test]
fn test_median_heap_clone() {
  let mut heap = MedianHeap::new(MidpointMedian);
  heap.push(1);
  heap.push(2);
  heap.push(3);
  heap.push(4);
  heap.push(5);
  heap.push(6);
  heap.push(7);
  heap.push(7);
  heap.push(7);

  let cloned_heap = heap.clone();
  assert_eq!(5, cloned_heap.get_median().unwrap());
  assert_eq!(9, cloned_heap.len());
}