// 最大二部マッチング
// O(|V||E|)
// これとは別に、Hopcroft–Karp という特定条件下で O(|E|√|V|) で動くアルゴリズムもあるらしい
// 動作確認は、多分大丈夫、くらい
#[allow(dead_code)]
struct BipartiteMatching {
    pub n: usize,             // 頂点数
    pub g: Vec<Vec<usize>>,   // 隣接リスト
    pub match_to: Vec<usize>, // [node_id] := node_idとマッチングしている頂点のid
}
#[allow(dead_code)]
impl BipartiteMatching {
    pub fn new(n: usize, g: Vec<Vec<usize>>) -> Self {
        Self {
            n,
            g,
            match_to: vec![!0; n],
        }
    }

    // 増加パスをDFSで探す
    fn dfs(&mut self, v: usize, used: &mut Vec<bool>) -> bool {
        used[v] = true;

        for u in self.g[v].clone().into_iter() {
            let w = self.match_to[u];
            // vと隣接するノードの内、未マッチか、マッチしててもマッチ相手に別のマッチング先があるなら
            if w == !0 || (!used[w] && self.dfs(w, used)) {
                self.match_to[v] = u;
                self.match_to[u] = v;
                return true;
            }
        }

        false
    }

    // 二部グラフの最大マッチングを求める
    // => マッチングの個数を返す
    pub fn run(&mut self) -> usize {
        let mut res = 0;
        for v in 0..self.n {
            if self.match_to[v] == !0 {
                let mut used = vec![false; self.n];
                if self.dfs(v, &mut used) {
                    res += 1;
                }
            }
        }
        res
    }
}
