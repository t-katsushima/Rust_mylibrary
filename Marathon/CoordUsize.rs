const H: usize = unimplemented!();
const W: usize = unimplemented!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (usize, usize)) -> Coord {
        Coord { x: p.0, y: p.1 }
    }

    fn from_isize_pair(pos: (isize, isize)) -> Coord {
        Coord {
            x: pos.0 as usize,
            y: pos.1 as usize,
        }
    }

    fn in_field(pos: (isize, isize)) -> bool {
        (0 <= pos.0 && pos.0 < W as isize) && (0 <= pos.1 && pos.1 < H as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn to_isize_pair(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }

    // マンハッタン距離
    fn distance(&self, that: Coord) -> usize {
        let dist_x = max(self.x, that.x) - min(self.x, that.x);
        let dist_y = max(self.y, that.y) - min(self.y, that.y);
        dist_x + dist_y
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let (ix, iy) = self.to_isize_pair();
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&(dx, dy)| (ix + dx, iy + dy))
            .filter(|&p| Coord::in_field(p))
            .map(|p| Coord::from_isize_pair(p))
            .collect()
    }

    // 四則演算
    fn plus(&self, that: &Coord) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Coord) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y][self.x]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y][self.x] = e;
    }
}
