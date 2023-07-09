// O(ElogE)
#[allow(dead_code)]
struct SteinerTreePrim {
    n: usize,                        // 全ノードの数
    graph: Vec<Vec<(usize, usize)>>, // (node, cost) の隣接リスト
    is_terminal: Vec<bool>,          // 必須点ならtrue
}

/*
1. 始点を選ぶ
2. ダイクストラで経路付きのターミナルへのエッジをpqに追加
3. 最小のエッジを取り出す
4. エッジに含まれるすべての点から2をやって、3,4と続ける
 */
#[allow(dead_code)]
impl SteinerTreePrim {
    // 使うエッジの一覧を返す。
    // => Vec<(node_a, node_b)>
    fn solve(&self) -> Vec<(usize, usize)> {
        let mut heapq = BinaryHeap::new();
        let mut done = vec![false; self.n];
        let mut res = vec![];

        let mut init_node = self.n + 1;

        /* init処理 */
        // init_node用に最小番号のターミナルを探す
        for i in 0..self.n {
            if self.is_terminal[i] {
                init_node = i;
                break;
            }
        }
        done[init_node] = true;

        let data = Self::dijkstra(self.n, &self.graph, init_node);
        for i in 0..self.n {
            if self.is_terminal[i] && !done[i] {
                let (cost, path) = data[i].clone();
                heapq.push(Reverse((cost, path)));
            }
        }

        /*  */
        while let Some(Reverse((_cost, path))) = heapq.pop() {
            let to = path[path.len() - 1];
            if done[to] {
                continue;
            }

            done[to] = true;

            // res作り
            for i in 0..path.len() - 1 {
                res.push((path[i], path[i + 1]));
            }
            // heapqにエッジ候補追加
            for i in 1..path.len() {
                let node = path[i];
                let data = Self::dijkstra(self.n, &self.graph, node);
                for i in 0..self.n {
                    if self.is_terminal[i] && !done[i] {
                        let (cost, path) = data[i].clone();
                        heapq.push(Reverse((cost, path)));
                    }
                }
            }
        }

        res
    }

    // パス一覧を返す
    // TODO: perf: Prim法の方で既に通った場所は打ち切ってよさそう
    fn dijkstra(
        n: usize,
        edge: &Vec<Vec<(usize, usize)>>,
        start: usize,
    ) -> Vec<(usize, Vec<usize>)> {
        const MAX: usize = std::usize::MAX;

        let mut dist = vec![(MAX, vec![]); n + 1]; // (cost, path)
        let mut check = vec![false; n + 1];

        dist[start] = (0, vec![start]);

        let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new(); // (cost, node) queue
        heap.push(Reverse((0, start)));

        // 頂点は n 個あるので、n 周する
        for _ in 1..=n {
            // 調べ済みでない頂点のうち、最も近い頂点を now に入れる
            let now_node: usize; // このターンにフォーカスするノード
            loop {
                let Reverse((_, node)) = heap.pop().unwrap(); // 最小コストから貪欲に取っていく
                if !check[node] {
                    now_node = node;
                    check[node] = true;
                    break;
                }
            }

            // その頂点からたどり着ける頂点の情報を更新する
            for &(next_node, cost) in &edge[now_node] {
                let next_dist = dist[now_node].0 + cost;
                // 既存の通路より近いなら更新
                if next_dist < dist[next_node].0 {
                    let mut next_path = dist[now_node].1.clone();
                    next_path.push(next_node);
                    dist[next_node] = (next_dist, next_path);
                    heap.push(Reverse((next_dist, next_node)));
                }
            }
        }

        dist
    }
}
