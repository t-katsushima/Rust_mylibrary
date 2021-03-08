use std::time::SystemTime;

let system_time = SystemTime::now();

// 副作用挟む操作っぽい
let now_ms: u128 = system_time.elapsed().unwrap().as_millis();
let now_seconds = system_time.elapsed().unwrap().as_secs_f64();
