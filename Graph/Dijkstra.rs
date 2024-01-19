// n: 頂点数(1-based index), edge: (ノード, コスト) の隣接リスト
// return: from から to への最短距離
fn dijkstra(n: usize, edge: &Vec<Vec<(usize, usize)>>, from: usize, to: usize) -> usize {
    const MAX: usize = std::usize::MAX;

    let mut dist = vec![MAX; n + 1];
    let mut check = vec![false; n + 1];

    dist[from] = 0;

    let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new(); // (cost, node) queue
    heap.push(Reverse((0, from)));

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
            let next_dist = dist[now_node] + cost;
            // 既存の通路より近いなら更新
            if next_dist < dist[next_node] {
                dist[next_node] = next_dist;
                heap.push(Reverse((next_dist, next_node)));
            }
        }
    }

    dist[to]
}
