#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vector {
    pub fn new(p: (f64, f64)) -> Self {
        Self { x: p.0, y: p.1 }
    }
    pub fn from_usize_pair(p: (usize, usize)) -> Self {
        Self {
            x: p.0 as f64,
            y: p.1 as f64,
        }
    }
    pub fn from_isize_pair(p: (isize, isize)) -> Self {
        Self {
            x: p.0 as f64,
            y: p.1 as f64,
        }
    }

    // 始点、終点からベクトルを作る
    pub fn from_s_g(s: &Coord, g: &Coord) -> Self {
        Self::new((g.x - s.x, g.y - s.y))
    }

    // ペアへの変換
    pub fn to_pair(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    // ユークリッド距離
    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    // 長さを1に正規化
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            return Self::new((0.0, 0.0));
        }
        Self::new((self.x / len, self.y / len))
    }

    // angle度だけ回転させる
    pub fn rotate(&self, angle: f64) -> Self {
        let rad = angle * PI / 180.0;
        let nx = (self.x * rad.cos()) - (self.y * rad.sin());
        let ny = (self.x * rad.sin()) + (self.y * rad.cos());
        Self::new((nx, ny))
    }

    // 角度を度数で返す
    pub fn getAngle(&self) -> f64 {
        self.y.atan2(self.x) * 180.0 / PI
    }

    // 算術演算
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    // 四則演算
    pub fn plus(&self, that: &Self) -> Self {
        Self::new((self.x + that.x, self.y + that.y))
    }
    pub fn minus(&self, that: &Self) -> Self {
        Self::new((self.x - that.x, self.y - that.y))
    }
}

// println!("{}") での表示内容
impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}
// println!("{:?}") での表示内容
impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}
