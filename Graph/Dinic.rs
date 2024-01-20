// O(|E||V|^2)
pub mod dinic {
    #[derive(Clone)]
    pub struct Edge {
        pub to: usize,  // 行き先
        pub cap: usize, // 容量
        rev_idx: usize, // 逆辺のインデックス. g[to][rev]で逆辺の情報が得られる.
        #[allow(unused)]
        pub is_rev: bool, // 逆辺かどうか
    }
    impl Edge {
        fn new(to: usize, cap: usize, rev_idx: usize, is_rev: bool) -> Edge {
            Edge {
                to,
                cap,
                rev_idx,
                is_rev,
            }
        }
    }

    pub struct G {
        pub g: Vec<Vec<Edge>>, // グラフの隣接リスト表現
    }
    impl G {
        // g: 隣接リスト。(node_id, cap)
        fn new(g0: Vec<Vec<(usize, usize)>>) -> G {
            let mut g = G {
                g: vec![vec![]; g0.len()],
            };
            for from_i in 0..g0.len() {
                for &(to_i, cap) in &g0[from_i] {
                    g.add_edge(from_i, to_i, cap);
                }
            }

            g
        }

        // fromからtoへ向かう容量capの辺をグラフに追加する
        fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
            let to_len = self.g[to].len();
            let from_len = self.g[from].len();
            self.g[from].push(Edge::new(to, cap, to_len, false));
            self.g[to].push(Edge::new(from, 0, from_len, true));
        }

        fn get_edge_num(&self, node: usize) -> usize {
            self.g[node].len()
        }
    }

    pub struct Solver {
        pub n: usize, // ノードの数
        pub g: G,
    }
    impl Solver {
        // g: 隣接リスト。(node_id, cap)
        pub fn new(n: usize, g: Vec<Vec<(usize, usize)>>) -> Solver {
            Solver { n, g: G::new(g) }
        }

        // sからの最短距離をBFSで計算する(capに空きがある辺のみ使用できる)
        // return: sからの距離の配列
        fn bfs(&self, s: usize) -> Vec<usize> {
            let mut level = vec![!0; self.n];
            let mut q = std::collections::VecDeque::new();
            level[s] = 0;
            q.push_back(s);
            while let Some(v) = q.pop_front() {
                for e in &self.g.g[v] {
                    // エッジに容量余りがある、かつ未到達
                    if e.cap > 0 && level[e.to] == !0 {
                        level[e.to] = level[v] + 1;
                        q.push_back(e.to);
                    }
                }
            }

            return level;
        }

        // 増加パスをDFSで探す
        fn dfs(
            &mut self,
            node: usize,           // 現在のノード
            t: usize,              // ゴール
            f: usize,              // 暫定流量
            iter: &mut Vec<usize>, // [node_id] := どのエッジまで調べ終わったか
            level: &Vec<usize>,    // [node_id] := sからの距離.
        ) -> usize {
            // ゴールに到達したら、暫定流量で確定
            if node == t {
                return f;
            }

            // nodeから生えているエッジを順番に見ていく
            for edge_i in iter[node]..self.g.get_edge_num(node) {
                iter[node] = edge_i;

                let e = self.g.g[node][edge_i].clone();
                // エッジに容量余りがある、かつ、最短経路に沿っている
                if e.cap > 0 && level[node] < level[e.to] {
                    // e.to についてdfsを進めて、増加パスになっていたら採用して終了
                    let d = self.dfs(e.to, t, f.min(e.cap), iter, level);
                    if d > 0 {
                        self.g.g[node][edge_i].cap -= d;
                        self.g.g[e.to][e.rev_idx].cap += d;
                        return d;
                    }
                }
            }

            // ゴールで無いノードで、通過可能なエッジがなかった
            return 0;
        }

        // sからtへの最大流を求める
        pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
            let mut flow = 0;
            loop {
                // capに空きがある辺のみ使用した、sからの最短距離一覧
                let level = self.bfs(s);
                // tまでの増加路がなければ終了
                if level[t] == !0 {
                    return flow;
                }

                let mut iter = vec![0; self.n];
                // 今回のbfs結果で増加パスが見つかる限り、dfsを繰り返す
                loop {
                    let f = self.dfs(s, t, usize::MAX, &mut iter, &level);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }
}
