// cf. https://qiita.com/Morifolium/items/6c8f0a188af2f9620db2
struct TopologicalSort {
    n: usize, // ノード数
    graph: Vec<Vec<usize>>,
}

impl TopologicalSort {
    fn run(&self) -> Vec<usize> {
        let mut degree = vec![0; self.n];
        for i in 0..self.n {
            for &j in &self.graph[i] {
                degree[j] += 1;
            }
        }

        let mut stack = vec![];
        for i in 0..self.n {
            if degree[i] == 0 {
                stack.push(i);
            }
        }

        let mut res = Vec::with_capacity(self.n);

        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            for &j in &self.graph[i] {
                degree[j] -= 1;
                if degree[j] == 0 {
                    stack.push(j);
                }
            }

            res.push(i);
        }

        res
    }
}
