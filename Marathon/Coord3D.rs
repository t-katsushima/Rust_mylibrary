#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}
#[allow(dead_code)]
impl Coord {
    pub fn new(p: (isize, isize, isize)) -> Self {
        Coord {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
    pub fn from_usize_triple(p: (usize, usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
            z: p.2 as isize,
        }
    }

    pub fn in_field(&self, d: usize) -> bool {
        (0 <= self.x && self.x < d as isize)
            && (0 <= self.y && self.y < d as isize)
            && (0 <= self.z && self.z < d as isize)
    }

    // ペアへの変換
    pub fn to_triple(&self) -> (isize, isize, isize) {
        (self.x, self.y, self.z)
    }
    pub fn to_usize_triple(&self) -> (usize, usize, usize) {
        (self.x as usize, self.y as usize, self.z as usize)
    }

    // マンハッタン距離
    pub fn distance(&self, that: &Self) -> isize {
        (self.x - that.x).abs() + (self.y - that.y).abs() + (self.z - that.z).abs()
    }

    // 隣接6方向の内、フィールドに収まるものを返す
    pub fn mk_6dir(&self, d: usize) -> Vec<Self> {
        let delta = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field(d))
            .collect()
    }

    // // 四則演算
    pub fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y, self.z + that.z))
    }
    pub fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y, self.z - that.z))
    }

    pub fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<Vec<T>>>) -> &'a T {
        &mat[self.x as usize][self.y as usize][self.z as usize]
    }

    pub fn set_matrix<T>(&self, mat: &mut Vec<Vec<Vec<T>>>, e: T) {
        mat[self.x as usize][self.y as usize][self.z as usize] = e;
    }
}

// println!("{}") での表示内容
impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)?;
        Ok(())
    }
}
// println!("{:?}") での表示内容
impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)?;
        Ok(())
    }
}
