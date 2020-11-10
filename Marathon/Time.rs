use std::time::SystemTime;

let start_time = SystemTime::now();

// 副作用挟む操作っぽい
let ms: u128 = start_time.elapsed().unwrap().as_millis();
let seconds = start_time.elapsed().unwrap().as_secs_f64();
