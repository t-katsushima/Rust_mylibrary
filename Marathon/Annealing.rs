// cf. https://img.atcoder.jp/intro-heuristics/editorial.pdf
fn annealing() -> Vec<usize> {
    const T0: f64 = 2e3; // 開始温度
    const T1: f64 = 6e2; // 終了温度

    const TL: f64 = 1.95; // 焼きなまし時間(秒)

    let mut rand = rand_pcg::Pcg64Mcg::new(890482);
    let mut state = State::new(&input, t);

    let mut cnt = 0;
    #[allow(non_snake_case)]
    let mut T = T0;
    let mut best = calc_score(res);
    let mut best_out = res.clone(); // res がベクターの場合を例とする

    loop {
        cnt += 1;
        if cnt % 100 == 0 {
            let t = get_time(time) / TL;
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t); // T0^(1.0-t) * T1^t
        }

        let old_score = calc_score(res);
        //
        // 変更処理の実行
        //
        let next_score = calc_score(res);
        if old_score > next_score && !rand.gen_bool(f64::exp((state.score - old_score) as f64 / T))
        // e^(score差 / T)
        {
            // 変更の巻き戻し
        }

        if state.score > best {
            // ベストスコアの更新
            best = state.score;
            best_out = state.t.clone();
        }
    }

    best_out
}
