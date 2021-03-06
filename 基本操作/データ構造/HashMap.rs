// cf. https://qiita.com/garkimasera/items/a6df4d1cd99bc5010a5e#hashmap

let mut a = HashMap::new();

let mut a: HashMap<String, usize> = HashMap::new();
a.insert(String::from("Blue"), 10);
a.contains_key("Blue")

assert_eq!(map["lisp"], 1958); // [ ] でアクセス可能。存在しないキーだとパニック
assert_eq!(map.get("c"), Some(&1972)); // get()を使うと、キーが存在する場合はSomeを、

// 更新
/// キーの値をインクリメント。キーが無ければ0を入れてからインクリメント
let count = map.entry(key).or_insert(0);
*count += 1;

*map.entry(key).or_insert(0) += 1;

// イテレート
for (k, v) in &map {
    println!("{} was born in {}", k, v);
}
