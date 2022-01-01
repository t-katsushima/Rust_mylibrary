// f64 sort
// cf. https://uma0317.github.io/rust-cookbook-ja/algorithms/sorting.html#floatのベクタをソートする


// f64 heap
// cf. https://stackoverflow.com/a/39950148
#[derive(PartialEq)]
struct MinNonNan(f64);

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

heapq.push(MinNonNan(1.8));

