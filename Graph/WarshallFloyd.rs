// 未テスト

#[allow(dead_code)]
struct WarshallFloyd {
    n: usize,              // ノード数
    d: Vec<Vec<isize>>,    // 最短コストの隣接行列
    not_applicable: isize, // 経路が存在しないことを表現する値
}
impl WarshallFloyd {
    fn new(
        n: usize,
        graph: &Vec<Vec<(usize, isize)>>, // 隣接リスト. (ノード番号, 移動コスト)
    ) -> Self {
        let not_applicable = std::isize::MAX / 2;
        let mut d = vec![vec![not_applicable; n]; n];

        // 初期化
        for a in 0..n {
            for &(b, cost) in &graph[a] {
                d[a][b] = cost;
            }
        }
        for i in 0..n {
            d[i][i] = 0;
        }

        // warshall_floyd
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
                }
            }
        }

        Self {
            n,
            d,
            not_applicable,
        }
    }

    fn get(&self, a: usize, b: usize) -> Option<isize> {
        let cost = self.d[a][b];
        if cost == self.not_applicable {
            None
        } else {
            Some(cost)
        }
    }
}
