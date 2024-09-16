println!("{:?}", vec![1,2,3]); // ベクターのプリント

// float のソート
let mut v: [f32; 5] = [5.0, 4.0, 1.0, 3.0, 2.0];
v.sort_by(|a, b| a.partial_cmp(b).unwrap());
assert!(v == [1.0, 2.0, 3.0, 4.0, 5.0]);
