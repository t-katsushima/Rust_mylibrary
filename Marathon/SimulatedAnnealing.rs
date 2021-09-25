// cf. https://github.com/tanakh/simulated-annealing/blob/master/src/lib.rs
// １スレッドへの固定と、コードコメントを書き足した。
// ステップ数じゃなくて実行時間で終了を制御するように

mod sa {
    use rand::prelude::*;

    #[derive(Clone)]
    pub struct AnnealingOptions {
        pub steps: usize,    // time_limit に変更 // 焼きなまし実行時間
        pub limit_temp: f64, // 最低温度？
        pub restart: usize,
        pub silent: bool, // デバッグログの表示有無
    }

    pub trait Annealer {
        type State: Clone;
        // 近傍への移動の表現
        type Move;

        fn init_state(&self, rng: &mut impl Rng) -> Self::State;
        fn start_temp(&self, init_score: f64) -> f64;

        fn is_done(&self, _score: f64) -> bool {
            false
        }

        fn eval(&self, state: &Self::State) -> f64;

        fn neighbour(&self, state: &Self::State, rng: &mut impl Rng) -> Self::Move;

        // 近傍への移動の適用とアンドゥ
        fn apply(&self, state: &mut Self::State, mov: &Self::Move);
        fn unapply(&self, state: &mut Self::State, mov: &Self::Move);

        fn apply_and_eval(
            &self,
            state: &mut Self::State,
            mov: &Self::Move,
            _prev_score: f64,
        ) -> f64 {
            self.apply(state, mov);
            self.eval(state)
        }
    }

    pub fn annealing<A: Annealer>(
        annealer: &A,
        opt: &AnnealingOptions,
        seed: u64,
    ) -> (f64, <A as Annealer>::State) {
        let mut rng = SmallRng::seed_from_u64(seed);

        let mut state = annealer.init_state(&mut rng);
        let mut cur_score = annealer.eval(&state);
        let mut best_score = cur_score;
        let mut best_ans = state.clone();

        macro_rules! progress {
            ($($arg:expr),*) => {
                if !opt.silent {
                    eprintln!($($arg),*);
                }
            };
        }

        progress!("Initial score: {}", cur_score);

        let mut restart_cnt = 0;

        let t_max = annealer.start_temp(cur_score);
        let t_min = opt.limit_temp;

        let step = opt.steps as f64;
        // 減衰
        let decay = ((t_min / t_max).ln() / step).exp();

        progress!("Temperature decay: {}", decay);

        let mut temp = t_max;
        loop {
            if temp < t_min {
                restart_cnt += 1;
                if restart_cnt >= opt.restart {
                    break;
                }
                progress!("Restarting... {}/{}", restart_cnt, opt.restart);
                temp = t_max;
            }

            let mov = annealer.neighbour(&state, &mut rng);
            let new_score = annealer.apply_and_eval(&mut state, &mov, cur_score);

            if new_score <= cur_score
                || rng.gen::<f64>() <= ((cur_score - new_score) as f64 / temp).exp()
            {
                cur_score = new_score;
                if cur_score < best_score {
                    if best_score - cur_score > 1e-6 {
                        progress!("Best: score = {:.3}, temp = {:.9}", cur_score, temp);
                    }
                    best_score = cur_score;
                    best_ans = state.clone();
                }
                if annealer.is_done(cur_score) {
                    break;
                }
            } else {
                annealer.unapply(&mut state, &mov);
            }

            temp *= decay;
        }
        (best_score, best_ans)
    }
}
