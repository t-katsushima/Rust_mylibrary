use rand::prelude::*;

let mut rand = rand_pcg::Pcg64Mcg::new(890482);

// Generate a random value in the range [`low`, `high`), 
// i.e. inclusive of `low` and exclusive of `high`.
rand.gen_range(0, 42)
