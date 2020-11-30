// 1-based-index
struct UnionFind {
    uni: Vec<isize>, // 根であれば *そのグループの要素数(負)* が、子であれば親の番号が入る。
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            uni: vec![-1; n + 1],
        }
    }

    // 頂点 v の所属するグループを調べる
    fn root(&mut self, v: usize) -> usize {
        if self.uni[v] < 0 {
            v
        } else {
            self.uni[v] = self.root(self.uni[v] as usize) as isize;
            self.uni[v] as usize
        }
    }

    // 頂点 a と頂点 b を繋ぐ。元々同じグループのとき　false を返す
    fn connect(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.root(a) as usize;
        let mut root_b = self.root(b) as usize;

        if root_a == root_b {
            return false;
        }

        // a 側が大きいグループになるようにスワップ
        if self.uni[root_a] > self.uni[root_b] {
            root_a ^= root_b;
            root_b ^= root_a;
            root_a ^= root_b;
        }

        // root_a と root_b を結合し、root_b の親を root_a とする
        self.uni[root_a] += self.uni[root_b];
        self.uni[root_b] = root_a as isize;

        return true;
    }

    // 頂点 a, b が同じグループであるかを調べる
    fn is_connect(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    // 頂点 v を含むグループの頂点数を調べる
    fn size(&mut self, v: usize) -> usize {
        let root = self.root(v);
        self.uni[root].abs() as usize
    }
}
