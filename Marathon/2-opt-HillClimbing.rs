// 2-opt 山登り

use std::time::SystemTime;

#[allow(dead_code)]
struct Yamanobori {
    /// 暫定のパス。ノード番号が入る。ノード番号は、tableのindexとも対応する。
    path: Vec<usize>,
    /// ノード間の距離の２次元テーブル。
    table: Vec<Vec<usize>>, // [ノード番号a][ノード番号b] := a-b 間の距離
    /// pathの距離の合算値
    score: usize,
}
#[allow(dead_code)]
impl Yamanobori {
    /// start_path: 初期値
    /// table: ノード間の距離の２次元テーブル
    fn new(start_path: Vec<usize>, table: Vec<Vec<usize>>) -> Yamanobori {
        let mut score = 0;
        let path_length = start_path.len();

        // 初期スコアの作成
        for i in 0..path_length - 1 {
            score += table[start_path[i]][start_path[i + 1]];
        }

        Yamanobori {
            path: start_path,
            table,
            score,
        }
    }

    /// pathの[li, ri] の区間を反転
    fn range_reverse(&mut self, li: usize, ri: usize) {
        let diff = (ri - li) + 1;
        for i in 0..diff / 2 {
            self.path.swap(li + i, ri - i);
        }
    }

    /// i1, i2 のノード間の距離を返す。
    fn access_table_by_path_id(&self, i1: usize, i2: usize) -> usize {
        self.table[self.path[i1]][self.path[i2]]
    }

    /// 2-opt山登りの実行。
    /// 始点終点は固定される。
    fn run(
        &mut self,
        during_time: u128, // 焼きなまし実行時間(ミリ秒)
        rng: &mut StdRng,
    ) {
        let system_time = SystemTime::now();
        let start_time = system_time.elapsed().unwrap().as_millis();

        let path_length = self.path.len();

        while system_time.elapsed().unwrap().as_millis() - start_time < during_time {
            for _ in 0..1000 {
                // lciとlci-1, rciとrci+1の間を切る
                let mut lci = rng.gen_range(1..path_length - 1); // left cut i
                let mut rci = rng.gen_range(1..path_length - 1); // right cut i
                if lci > rci {
                    std::mem::swap(&mut lci, &mut rci);
                }

                // 同一点だったらやり直し
                if lci == rci {
                    continue;
                }

                // 差分箇所のスコア計算
                let pre = self.access_table_by_path_id(lci - 1, lci)
                    + self.access_table_by_path_id(rci, rci + 1);
                let next = self.access_table_by_path_id(lci - 1, rci)
                    + self.access_table_by_path_id(lci, rci + 1);

                // ベストの更新
                if next < pre {
                    self.score += next;
                    self.score -= pre;

                    self.range_reverse(lci, rci);
                }
            }
        }
    }
}
