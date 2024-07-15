/*
※蟻本を参考に書いてみたが、うまく動かせなかった。
ac-library-rs に入ってるので、そっち使う https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/mincostflow.rs
使用例: https://atcoder.jp/contests/practice2/submissions/55637661
*/

// 最小費用流問題を解くアルゴリズム
// O(F|E|log|V|) または O(F|V|^2)
// cost は非負である必要がある？

/// Solver::new がエントリーポイント
pub mod mcf {
    #[derive(Clone)]
    pub struct Edge {
        pub to: usize,   // 行き先
        pub cap: usize,  // 容量
        pub cost: isize, // コスト
        rev_idx: usize,  // 逆辺のインデックス. g[to][rev]で逆辺の情報が得られる.
    }
    impl Edge {
        fn new(to: usize, cap: usize, cost: isize, rev_idx: usize) -> Self {
            Self {
                to,
                cap,
                cost,
                rev_idx,
            }
        }
    }

    pub struct G {
        pub g: Vec<Vec<Edge>>, // グラフの隣接リスト表現
    }
    impl G {
        /// g: 隣接リスト。(node_id, cap, cost)
        fn new(g0: Vec<Vec<(usize, usize, isize)>>) -> G {
            let mut g = G {
                g: vec![vec![]; g0.len()],
            };
            for from_i in 0..g0.len() {
                for &(to_i, cap, cost) in &g0[from_i] {
                    g.add_edge(from_i, to_i, cap, cost);
                }
            }

            g
        }

        /// fromからtoへ向かう、容量cap,コストcost の辺をグラフに追加する
        fn add_edge(&mut self, from: usize, to: usize, cap: usize, cost: isize) {
            let to_len = self.g[to].len();
            let from_len = self.g[from].len();
            self.g[from].push(Edge::new(to, cap, cost, to_len));
            self.g[to].push(Edge::new(from, 0, cost, from_len));
        }
    }

    pub struct Solver {
        pub n: usize, // ノードの数
        pub g: G,
    }
    impl Solver {
        /// g: 隣接リスト。(node_id, cap, cost)
        pub fn new(n: usize, g: Vec<Vec<(usize, usize, isize)>>) -> Solver {
            Solver { n, g: G::new(g) }
        }

        /// sからtへの流量fの最小費用流を求める
        /// 流せない場合はNoneを返す
        pub fn min_cost_flow(&mut self, s: usize, t: usize, mut f: usize) -> Option<usize> {
            let mut h: Vec<isize> = vec![0; self.n]; // ポテンシャル
            let mut dist: Vec<isize> = vec![0_isize; self.n]; // 最短距離
            let mut prev_v = vec![0; self.n]; // 直前の頂点
            let mut prev_e = vec![0; self.n]; // 直前の辺

            let mut res = 0;

            while f > 0 {
                // ダイクストラ法を用いてhを更新する
                let mut que = std::collections::BinaryHeap::new();
                for i in 0..self.n {
                    dist[i] = isize::MAX;
                }
                dist[s] = 0;
                que.push((0, s));
                while let Some(p) = que.pop() {
                    let v = p.1;
                    if dist[v] < p.0 {
                        continue;
                    }

                    for i in 0..self.g.g[v].len() {
                        let e = &self.g.g[v][i];
                        let b1 = e.cap > 0;
                        let b2 = if h[v] == isize::MAX {
                            true
                        } else if h[e.to] == isize::MAX {
                            false
                        } else {
                            dist[e.to] > dist[v] + e.cost + h[v] - h[e.to]
                        };
                        if b1 && b2 {
                            dist[e.to] = dist[v] + e.cost + h[v] - h[e.to];
                            prev_v[e.to] = v;
                            prev_e[e.to] = i;
                            que.push((dist[e.to], e.to));
                        }
                    }
                    if dist[t] == isize::MAX {
                        // これ以上流せない
                        return None;
                    }

                    for v in 0..self.n {
                        if h[v] == isize::MAX || dist[v] == isize::MAX {
                            h[v] = isize::MAX;
                        } else {
                            h[v] += dist[v];
                        }
                    }

                    // s-t間最短路に沿って目一杯流す
                    let mut d = f;
                    {
                        let mut v = t;
                        while v != s {
                            d = d.min(self.g.g[prev_v[v]][prev_e[v]].cap);
                            v = prev_v[v];
                        }
                    }
                    f -= d;
                    res += d * h[t] as usize;
                    {
                        let mut v = t;
                        while v != s {
                            let e = &mut self.g.g[prev_v[v]][prev_e[v]];
                            e.cap -= d;

                            let rev_idx = e.rev_idx;
                            // eのポインタを解放
                            let _ = e;

                            self.g.g[v][rev_idx].cap += d;
                            v = prev_v[v];
                        }
                    }
                }
            }

            Some(res)
        }
    }
}
