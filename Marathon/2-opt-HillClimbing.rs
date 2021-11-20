// 2-opt 山登り

use std::time::SystemTime;

#[allow(dead_code)]
struct Yamanobori {
    path: Vec<usize>, // ノード番号が入る。ノード番号は、tableのindexとも対応する。
    score: usize,
    table: Vec<Vec<usize>>, // [aノード番号][bノード番号] := a-b 間の距離
}
#[allow(dead_code)]
impl Yamanobori {
    fn new(start_path: Vec<usize>, table: Vec<Vec<usize>>) -> Yamanobori {
        let mut score = 0;
        let path_length = start_path.len();

        // 初期スコアの作成
        for i in 0..path_length - 1 {
            score += table[start_path[i]][start_path[i + 1]];
        }

        Yamanobori {
            path: start_path,
            score,
            table,
        }
    }

    // [li, ri] を反転
    fn range_reverse(&mut self, li: usize, ri: usize) {
        let diff = (ri - li) + 1;
        for i in 0..diff / 2 {
            self.path.swap(li + i, ri - i);
        }
    }

    fn access_table_by_path_id(&self, i1: usize, i2: usize) -> usize {
        self.table[self.path[i1]][self.path[i2]]
    }

    // end_time: main関数の開始後からの時間を指す
    // 始点終点は固定される
    fn run(
        &mut self,
        during_time: u128, // 焼きなまし実行時間(ミリ秒)
    ) {
        let system_time = SystemTime::now();
        let start_time = system_time.elapsed().unwrap().as_millis();

        let mut rand = rand_pcg::Pcg64Mcg::new(890482);
        let path_length = self.path.len();

        while system_time.elapsed().unwrap().as_millis() - start_time < during_time {
            for _ in 0..1000 {
                let mut lci = rand.gen_range(0, path_length); // left cut i
                let mut rci = rand.gen_range(0, path_length); // right cut i
                if lci > rci {
                    // swap
                    lci ^= rci;
                    rci ^= lci;
                    lci ^= rci;
                }

                if lci == rci || (lci == 0 || rci == path_length - 1) {
                    continue;
                }

                let pre = self.access_table_by_path_id(lci - 1, lci)
                    + self.access_table_by_path_id(rci, rci + 1);
                let next = self.access_table_by_path_id(lci - 1, rci)
                    + self.access_table_by_path_id(lci, rci + 1);

                if next < pre {
                    self.score += next;
                    self.score -= pre;

                    self.range_reverse(lci, rci);
                }
            }
        }
    }
}
