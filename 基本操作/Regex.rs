// cf. https://qiita.com/scivola/items/60141f262caa53983c3a

use regex::Regex;

// Regex作成
let re = Regex::new(r"\d").unwrap();

// 置換
re.replace_all("MZ-80K2E", "*")
