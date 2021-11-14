// https://qiita.com/lo48576/items/34887794c146042aebf1

let iter: Iterator<usize> = [0; 100].iter();

iter.sum::<T>;
iter.filter
iter.count()
iter.next()
iter.enumerate() // <(usize, T)>
iter.for_each
iter.fold(init: B, f: F) // F := B -> T -> B
iter.reduce // Iterator<T> -> (T -> T) -> Option<T> ?
