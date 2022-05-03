// 3桁区切りにして返す
fn pretty_print_num<T: std::string::ToString>(num: T) -> String {
	let mut v = num.to_string().chars().collect::<Vec<_>>();
	let mut cnt = 0;
	for i in (1..v.len()).rev() {
		cnt += 1;
		if cnt % 3 == 0 {
			v.insert(i, ',')
		}
	}
	v.iter().collect()
}
