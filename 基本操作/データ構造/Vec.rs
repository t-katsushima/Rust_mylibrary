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
