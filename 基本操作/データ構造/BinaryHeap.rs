// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();

heap.push(1);
heap.pop(); // Option<T>

heap.is_empty();
heap.clear();
heap.len();

use std::cmp::Reverse;
heap.push(Reverse(1));

// to Vec
let v = heap.into_vec();
let v = heap.into_sorted_vec(); // 昇順
