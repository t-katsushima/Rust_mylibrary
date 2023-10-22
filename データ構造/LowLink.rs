// 連結無向単純グラフが対象
// articulation, bridge を用意する。
// LowLink::new が O(V)
// cf. https://ei1333.github.io/luzhiled/snippets/graph/lowlink.html
struct LowLink {
    used: Vec<bool>,                  // dfs用のメモ関数
    root_node: usize,                 // dfs木の根
    ord: Vec<usize>,                  // dfsで訪れた順番
    low: Vec<usize>, // 頂点idxからDFS木の葉方向の辺を0回以上, 後退辺を1回以下通って到達可能な頂点のordの最小値
    pub articulations: Vec<usize>, // 関節点一覧。
    pub bridges: Vec<(usize, usize)>, // 橋の一覧。ノード番号が小さい方から大きい方への一方向のみ載る。
}
impl LowLink {
    // graph: 隣接リスト(0-based index)
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut lowlink = LowLink {
            used: vec![false; n],
            root_node: 0,
            ord: vec![1e15 as usize; n],
            low: vec![1e15 as usize; n],
            articulations: vec![],
            bridges: vec![],
        };

        let mut k = 0;
        lowlink.dfs(lowlink.root_node, &mut k, None, &graph);

        lowlink
    }

    /* 補助関数 */
    fn dfs(
        &mut self,
        node: usize,
        k: &mut usize, // 何番目に訪れたか
        parent_node: Option<usize>,
        graph: &Vec<Vec<usize>>,
    ) {
        self.used[node] = true;

        *k += 1;
        self.ord[node] = *k;
        self.low[node] = self.ord[node];

        let mut child_cnt = 0; // ルートノードの間接点検査用。子の連結成分の数。ルートのみ子が２個以上だと関節点。
        let mut is_articulation = false;
        for &to in &graph[node] {
            if !self.used[to] {
                child_cnt += 1;

                self.dfs(to, k, Some(node), &graph);

                // 子ノードの最小low値を受け取る
                self.low[node] = min(self.low[node], self.low[to]);

                if node != self.root_node {
                    is_articulation |= self.ord[node] <= self.low[to];
                }

                // 橋の検出
                if self.ord[node] < self.low[to] {
                    let edge = (min(node, to), max(node, to));
                    self.bridges.push(edge);
                }
            }
            // toが親ノードでは無い行ったことのあるノードだったら(後退辺だったら)
            else if parent_node.filter(|par_node| to != *par_node).is_some() {
                self.low[node] = min(self.low[node], self.ord[to]);
            }
        }

        if node == self.root_node {
            is_articulation = child_cnt >= 2;
        }

        if is_articulation {
            self.articulations.push(node);
        }
    }
}
