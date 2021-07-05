// https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

let mut books = BTreeSet::new();

books.insert("Odyssey");
books.remove("Odyssey");

// to Vec
books.into_iter().collect::<Vec<_>>();
