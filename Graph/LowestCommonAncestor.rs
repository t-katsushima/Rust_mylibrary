// v: ノード数
// ノード番号は 0 ~ v-1
// root: ルートノード
// query: O(log(v))
struct LCA {
    v: usize,
    tree: Vec<Vec<usize>>,
    root: usize,
    max_log_v: usize,
    parent: Vec<Vec<isize>>, // parent[k][v]: vから親を 2^k 回辿って到達する頂点(根を通り過ぎる場合は -1 とする)
    depth: Vec<usize>,       // 根からの深さ
}

impl LCA {
    fn dfs(
        root: usize, // ノード番号
        tree: &Vec<Vec<usize>>,
        parent: &mut Vec<Vec<isize>>,
        depth: &mut Vec<usize>,
    ) {
        let mut stack = vec![];
        stack.push((root, -1, 0));
        while !stack.is_empty() {
            let (i, p, d) = stack.pop().unwrap();
            parent[0][i] = p;
            depth[i] = d;
            for &e in &tree[i] {
                if e != p as usize {
                    stack.push((e, i as isize, d + 1));
                }
            }
        }
    }

    fn new(v: usize, tree: Vec<Vec<usize>>, root: usize) -> LCA {
        let max_log_v = (v as f64).log(2.0).ceil() as usize;
        let mut parent = vec![vec![0; v]; max_log_v];
        let mut depth = vec![0; v];

        // parent(0) と depth を初期化する
        Self::dfs(root, &tree, &mut parent, &mut depth);

        // parent を初期化する
        for k in 0..max_log_v - 1 {
            for i in 0..v {
                if parent[k][i] < 0 {
                    parent[k + 1][i] = -1;
                } else {
                    parent[k + 1][i] = parent[k][parent[k][i] as usize];
                }
            }
        }

        LCA {
            v,
            tree,
            root,
            max_log_v,
            parent,
            depth,
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        // u と v の深さが同じになるまで親を辿る
        if self.depth[u] > self.depth[v] {
            // swap
            u ^= v;
            v ^= u;
            u ^= v;
        }
        for k in 0..self.max_log_v {
            if (self.depth[v] - self.depth[u]) >> k & 1 == 1 {
                v = self.parent[k][v] as usize;
            }
        }

        if u == v {
            return u;
        }

        // 二分探索で LCA を求める
        for k in (0..self.max_log_v).rev() {
            // 親が一致しない最小の深さまで移動する
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u] as usize;
                v = self.parent[k][v] as usize;
            }
        }

        self.parent[0][u] as usize
    }
}
