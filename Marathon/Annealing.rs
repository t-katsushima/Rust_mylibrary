// cf. https://img.atcoder.jp/intro-heuristics/editorial.pdf
// 最大化問題の場合
fn annealing() -> Vec<usize> {
    const T0: f64 = 2e3; // 開始温度
    const T1: f64 = 6e2; // 終了温度

    const TL: f64 = 1.95; // 焼きなまし時間(秒)

    let mut rand = rand_pcg::Pcg64Mcg::new(890482);
    let mut state = State::new(&input, t);

    #[allow(non_snake_case)]
    let mut T = T0;
    // 初期値をセット
    let mut best_score = calc_score(res);
    let mut best_out = res.clone(); // res がベクターの場合を例とする

    loop {
        let t = get_time(time) / TL;
        if t >= 1.0 {
            break;
        }
        T = T0.powf(1.0 - t) * T1.powf(t); // T0^(1.0-t) * T1^t

        for _ in 0..100 {
            let old_score = calc_score(res);

            /* 変更処理の実行 */

            let next_score = calc_score(res);

            // スコアが悪化して、かつ `e^(score差 / T)` の確率にヒットしなかったら
            if old_score > next_score
                && !rand.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                /* 変更の巻き戻し */
            }

            if state.score > best_score {
                // ベストスコアの更新
                best_score = state.score;
                best_out = state.t.clone();
            }
        }
    }

    best_out
}
