#[allow(unused)]
pub mod MCTS {
    use std::fmt::Debug;
    use std::time::SystemTime;

    #[derive(Debug, Clone)]
    pub enum WinningStatus {
        Win,
        Lose,
        Draw,
    }
    impl WinningStatus {
        pub fn to_f64(&self) -> f64 {
            use WinningStatus::*;
            match self {
                Win => 1.0,
                Draw => 0.5,
                Lose => 0.0,
            }
        }
    }

    pub trait State {
        type Action: Clone + Debug;

        fn legal_actions(&self) -> Vec<Self::Action>;

        fn is_done(&self) -> bool;

        fn get_winning_status(&self) -> WinningStatus;

        // 指定したactionでゲームを1ターン進め、次のプレイヤー視点の盤面にする
        fn advance(&mut self, action: &Self::Action);

        fn playout(self) -> f64;
    }

    const C: f64 = 1.0; // UCB1の計算に使う定数
    const EXPAND_THRESHOLD: usize = 10; // ノードを展開する閾値

    #[derive(Debug, Clone)]
    pub struct Node<S: State + Clone> {
        state: S, // 盤面の状態
        w: f64,   // 累計価値
        pub child_nodes: Vec<Self>,
        pub n: usize, // 試行回数
    }
    impl<S: State + Clone> Node<S> {
        fn new(state: S) -> Self {
            Self {
                state,
                w: 0.0,
                n: 0,
                child_nodes: vec![],
            }
        }

        pub fn evaluate(&mut self) -> f64 {
            // ゲーム終了時
            if self.state.is_done() {
                // 勝敗に応じた評価を累計価値に足し、累計価値を返す。
                // TODO: サンプル実装なので、問題に応じて置き換える
                let value = match self.state.get_winning_status() {
                    WinningStatus::Win => 1.0,
                    WinningStatus::Lose => 0.0,
                    WinningStatus::Draw => 0.5,
                };

                self.w = value;
                self.n += 1;

                return value;
            }

            // 子ノードが存在しない時
            // プレイアウト結果を累計価値に足し、累計価値を返す。試行回数が閾値を超えたら子ノードを展開する。
            if self.child_nodes.is_empty() {
                let value = self.state.clone().playout();

                self.w += value;
                self.n += 1;

                if self.n == EXPAND_THRESHOLD {
                    self.expand();
                }

                return value;
            }
            // 子ノードが存在する時
            // 子ノードの評価を累計価値に足し、累計価値を返す。
            else {
                // 二人対戦ゲームの場合、プレイヤー視点が逆のため、以下のように評価値を反転する
                // TODO: 一人ゲームの場合反転しないように書き換える
                let value = 1.0 - self.next_childnode().evaluate();

                self.w += value;
                self.n += 1;

                return value;
            }
        }

        // ノードを展開する
        fn expand(&mut self) {
            let legal_actions = self.state.legal_actions();
            self.child_nodes.clear(); // TODO: 要らない気がする
            let parent = self.clone();
            for action in legal_actions {
                let mut next_node = parent.clone();
                next_node.state.advance(&action);
                self.child_nodes.push(next_node);
            }
        }

        // どのノードを評価するか選択する
        fn next_childnode(&self) -> Self {
            // 未試行の子ノードがあったら再優先で実行
            for child_node in &self.child_nodes {
                if child_node.n == 0 {
                    return child_node.clone();
                }
            }

            let mut t = 0.0;
            for child_node in &self.child_nodes {
                t += child_node.n as f64;
            }

            let mut best_value = std::f64::MIN;
            let mut best_action_index = !0;
            for i in 0..self.child_nodes.len() {
                let child_node = &self.child_nodes[i];
                // TODO: 一人ゲームの場合反転しないように書き換える
                let ucb1_value = 1.0 - child_node.w / child_node.n as f64
                    + C as f64 * (2.0 * t.ln() / child_node.n as f64).sqrt();
                if ucb1_value > best_value {
                    best_action_index = i;
                    best_value = ucb1_value;
                }
            }

            self.child_nodes[best_action_index].clone()
        }
    }

    // プレイアウト数を指定してMCTSで行動を決定する
    pub fn mcts_action<S: State + Clone>(
        state: &S,
        end_time: u128,
        system_time: &SystemTime,
    ) -> S::Action {
        let mut root_node = Node::new(state.clone());

        // 所定回数プレイアウトを実行
        root_node.expand();
        let mut playout_num = 0;
        while system_time.elapsed().unwrap().as_millis() < end_time {
            playout_num += 1;
            root_node.evaluate();
        }
        eprintln!("playout num: {}", playout_num);

        // 一番良さそうな手(viz.試行された手)を選ぶ
        let legal_actions = state.legal_actions();
        let mut best_action_searched_number: isize = -1;
        let mut best_action_index: usize = !0;
        assert!(legal_actions.len() == root_node.child_nodes.len());
        for i in 0..legal_actions.len() {
            let n = root_node.child_nodes[i].n;
            if n as isize > best_action_searched_number {
                best_action_index = i;
                best_action_searched_number = n as isize;
            }
        }

        legal_actions[best_action_index].clone()
    }
}
