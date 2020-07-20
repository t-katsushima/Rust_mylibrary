let mut vec = Vec::new();
let mut vec: Vec<isize> = vec![1, 2, 3];

vec.push(1); // 後ろに足す

vec[1] = 42;

vec.extend([1, 2, 3].iter().copied()); // [1, 2, 3] を後ろに足す

for x in &vec {
    println!("{}", x);
}
