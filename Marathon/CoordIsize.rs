const H: usize = unimplemented!();
const W: usize = unimplemented!();

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[allow(dead_code)]
impl Coord {
    pub fn new(p: (isize, isize)) -> Self {
        Self { x: p.0, y: p.1 }
    }
    pub fn from_usize_pair(p: (usize, usize)) -> Self {
        Self {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    pub fn in_field(&self) -> bool {
        (0 <= self.x && self.x < W as isize) && (0 <= self.y && self.y < H as isize)
    }

    // ペアへの変換
    pub fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    pub fn to_usize_pair(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    /// マンハッタン距離
    pub fn distance(&self, that: &Self) -> isize {
        (self.x - that.x).abs() + (self.y - that.y).abs()
    }

    pub fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
            .collect()
    }

    pub fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // 四則演算
    pub fn plus(&self, that: &Self) -> Self {
        Self::new((self.x + that.x, self.y + that.y))
    }
    pub fn minus(&self, that: &Self) -> Self {
        Self::new((self.x - that.x, self.y - that.y))
    }

    // getter/setter
    pub fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }
    pub fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }
}
// println!("{}") での表示内容
impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}
// println!("{:?}") での表示内容
impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}
