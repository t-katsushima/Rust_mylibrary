// 頂点0からスタートして、全ての頂点を1度ずつ巡って頂点0に帰って来る
// O(2^n * n^2)

struct TSP {
    n: usize,
    d: Vec<Vec<isize>>, // 二点間の距離のテーブル
    dp: Vec<Vec<isize>>,
    init_num: isize,
}

impl TSP {
    fn new(n: usize, d: Vec<Vec<isize>>) -> TSP {
        let init_num = 1e16 as isize;

        let mut dp = vec![vec![init_num; n]; 1 << n];
        dp[0][0] = 0;

        TSP { n, d, dp, init_num }
    }

    fn tsp(&mut self, node: usize, gone: usize) {
        if self.dp[gone][node] == self.init_num {
            let pre_bin = gone ^ (1 << node);
            if pre_bin == 0 {
                self.dp[gone][node] = min(self.dp[gone][node], self.dp[0][0] + self.d[0][node]);
            } else {
                for i in 0..self.n {
                    if i != node && gone & (1 << i) > 0 {
                        self.tsp(i, pre_bin);
                        self.dp[gone][node] =
                            min(self.dp[gone][node], self.dp[pre_bin][i] + self.d[i][node])
                    }
                }
            }
        }
    }

    fn solve(&mut self) -> isize {
        self.tsp(0, (1 << self.n) - 1);
        self.dp[(1 << self.n) - 1][0]
    }
}
