// cf. https://qiita.com/4hiziri/items/9a0ba5788b9e010fd389

use std::fs;
use std::io::Write;

// write
let string = "Hello, file io!";
let mut f = fs::File::create("test.txt").unwrap(); // open file, you can write to file.
// "create" open as write only mode.
f.write_all(string.as_bytes()).unwrap(); // byte-only
