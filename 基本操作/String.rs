let s = "hoge";

s.chars(); // => Chars(['h', 'o', 'g', 'e'])

// to int
"42".to_digit(10).unwrap();

// reverse
s.chars().rev().collect::<String>()

(57 as u8) as char // to char

format!("{}-{}", "hoge", "fuga") // "hoge-fuga"


let mut t = "fuga";
t.push_str("piyo");
t.push('p');
