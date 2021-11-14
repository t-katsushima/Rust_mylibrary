let mut vec = Vec::new();
let mut vec: Vec<isize> = vec![1, 2, 3];

vec.push(1); // 後ろに足す

vec[1] = 42;

vec.extend([1, 2, 3].iter().copied()); // [1, 2, 3] を後ろに足す
vec.sort();

for x in &vec {
    println!("{}", x);
}

// remove (O(N))
fn remove_item<T: Eq>(v: &mut Vec<T>, e: &T) {
    let index = v.iter().position(|elem| *elem == *e).unwrap();
    v.remove(index);
}
//remove
let mut vec = vec![1, 2, 3, 4];
vec.retain(|&x| x % 2 == 0);
assert_eq!(vec, [2, 4]);

// 分割
let mut cpus = vec!["sh", "x86", "arm", "mips"];
let mut later = cpus.split_off(2);
println!("cpus = {:?}, later = {:?}", cpus, later);
//cpus = ["sh", "x86"], later = ["arm", "mips"]

// 結合
cpus.append(&mut later);
println!("cpus = {:?}, later = {:?}", cpus, later);
//cpus = ["sh", "x86", "arm", "mips"], later = []

// メモリ割り当て数を指定して空ベクター作成（上手く設定するとメモリ再割り当てにコストを割かずに済む）
Vec::with_capacity(100)

// スライス作成
v[..]
v[1..3]
