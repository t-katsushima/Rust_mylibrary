use rand::{thread_rng, Rng};

let mut rng = thread_rng();

// Generate a random value in the range [`low`, `high`), 
// i.e. inclusive of `low` and exclusive of `high`.
rng.gen_range(1, 101)

// 乱数固定する場合？
let mut rng = rand_pcg::Pcg64Mcg::new(890482);
