// cf. https://shindannin.hatenadiary.com/entry/20121224/1356364040
// 最大化問題の場合
fn annealing() -> Vec<usize> {
    // 開始温度(スコア差の最大値にすると良さそう。開始直後に35%くらいの確率でこの差量を受け入れる)
    let start_temp: f64 = 2000.0;
    // 終了温度(終盤に悪化遷移を35%程度許容できる値にすると良さそう)
    let end_temp: f64 = 600.0;

    const TL: f64 = 1950.0; // 焼きなまし時間(ミリ秒)

    let mut rng = thread_rng();
    let mut state = State::new(&input, t);

    let mut temp;
    // 初期値をセット
    let mut best_score = calc_score(res);
    let mut best_out = res.clone(); // res がベクターの場合を例とする

    let mut loop_cnt = 0;
    let loop_times = 100;
    loop {
        let spent_time_rate = system_time.elapsed().unwrap().as_millis() as f64 / TL; // (0.0, 1.0)
        if spent_time_rate >= 1.0 {
            break;
        }
        // 温度。段々下がっていく。
        temp = start_temp + (end_temp - start_temp) * spent_time_rate;

        for _ in 0..loop_times {
            let old_score = calc_score(res);

            /* 変更処理の実行 */

            let next_score = calc_score(res);

            // スコアが悪化して、かつ `e^(score差 / T)` の確率にヒットしなかったら
            // `score差` が負の数なのが肝
            if next_score < old_score
                && !rng.gen_bool(f64::exp((next_score - old_score) as f64 / temp))
            {
                /* 変更の巻き戻し */
                // std::mem::swap(a, b);
            }

            if state.score > best_score {
                // ベストスコアの更新
                best_score = state.score;
                best_out = state.t.clone();
            }
        }

        loop_cnt += loop_times;
    }

    eprintln!("loop_cnt: {}", loop_cnt);
    best_out
}
