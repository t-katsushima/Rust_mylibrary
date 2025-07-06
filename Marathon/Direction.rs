#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[allow(dead_code)]
impl Direction {
    pub fn list() -> Vec<Self> {
        use Direction::*;
        vec![Left, Right, Up, Down]
    }

    pub fn to_delta(&self) -> Coord {
        match *self {
            Self::Left => Coord::new((-1, 0)),
            Self::Right => Coord::new((1, 0)),
            Self::Up => Coord::new((0, -1)),
            Self::Down => Coord::new((0, 1)),
        }
    }

    pub fn reverse(&self) -> Self {
        match *self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
