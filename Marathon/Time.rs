use std::time::SystemTime;

let now = SystemTime::now()
let seconds = now.elapsed().unwrap().as_secs_f64();
