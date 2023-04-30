use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;

let mut rng = thread_rng();

// Generate a random value in the range [`low`, `high`), 
// i.e. inclusive of `low` and exclusive of `high`.
rng.gen_range(1..101)

// 乱数固定する場合？
let mut rng = rand_pcg::Pcg64Mcg::new(890482);

// vector からの要素の乱択
vec.choose(&mut rng);


// seed指定乱数
let mut rng = rand_pcg::Pcg64Mcg::new(890482);

// cf. https://scrapbox.io/nwtgck/Rustで再現的な乱数を生成する
// (ライブラリ的に)コドゲでも使える
use rand::prelude::StdRng;
let seed: [u8; 32] = [42; 32];
let mut rng: StdRng = rand::SeedableRng::from_seed(seed)
