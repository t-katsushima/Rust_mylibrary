#[allow(dead_code)]
mod bs {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    // 第一要素を比較対象とする組
    struct ForSort<T> {
        score: isize,
        node: T,
    }
    // ダミー
    impl<T> PartialEq for ForSort<T> {
        fn eq(&self, other: &Self) -> bool {
            self.score == other.score
        }
    }
    impl<T> Eq for ForSort<T> {}

    impl<T> PartialOrd for ForSort<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.score.partial_cmp(&other.score)
        }
    }
    impl<T> Ord for ForSort<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.score.cmp(&other.score)
        }
    }

    pub struct BeamSearchOption {
        pub beam_width: usize,
        pub depth: usize,
    }
    pub trait BeamSearch {
        type State: Clone;

        fn transit(&self, st: &Self::State) -> Vec<Self::State>;
        fn evaluate(&self, st: &Self::State) -> isize;
    }

    pub fn search<A: BeamSearch>(bs: &A, init_st: A::State, opt: &BeamSearchOption) -> A::State {
        let mut pq: BinaryHeap<ForSort<A::State>> = BinaryHeap::new();
        pq.push(ForSort {
            score: bs.evaluate(&init_st),
            node: init_st.clone(),
        });
        for _ in 1..=opt.depth {
            let mut next_pq: BinaryHeap<ForSort<A::State>> = BinaryHeap::new();
            for _ in 0..opt.beam_width {
                if pq.is_empty() {
                    break;
                } else {
                    let st = pq.pop().unwrap().node;
                    for next_st in bs.transit(&st) {
                        next_pq.push(ForSort {
                            score: bs.evaluate(&next_st),
                            node: next_st,
                        })
                    }
                }
            }
            pq = next_pq;
        }
        pq.pop().unwrap().node
    }
}
