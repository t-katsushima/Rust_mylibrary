use rand::prelude::*;

let mut rand = rand_pcg::Pcg64Mcg::new(890482);
rand.gen_range(0, 42)
