let s = "hoge";

s.chars(); // => Chars(['h', 'o', 'g', 'e'])

// to int
"42".to_digit(10).unwrap();

// reverse
s.chars().rev().collect::<String>()

(57 as u8) as char // to char

format!("{}-{}", "hoge", "fuga") // "hoge-fuga"

// 文字列の拡張
let mut t = "fuga";
t.push_str("piyo");
t.push('p');

// 文字列Vecの文字列への変換
vec!["Hello", "World"].join(" ")

let v = vec!['a', 'b', 'c', 'd'];
let s: String = v.into_iter().collect();
