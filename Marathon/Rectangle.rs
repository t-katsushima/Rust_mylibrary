#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub leftup: Coord,
    pub rightdown: Coord,
}
#[allow(dead_code)]
impl Rectangle {
    pub fn new(leftup: Coord, rightdown: Coord) -> Self {
        Rectangle { leftup, rightdown }
    }

    pub fn from_diagonal_point(pos1: Coord, pos2: Coord) -> Self {
        let leftup = Coord::new((pos1.x.min(pos2.x), pos1.y.min(pos2.y)));
        let rightdown = Coord::new((pos1.x.max(pos2.x), pos1.y.max(pos2.y)));
        Self { leftup, rightdown }
    }

    pub fn calc_area(&self) -> isize {
        (self.rightdown.x - self.leftup.x) * (self.rightdown.y - self.leftup.y)
    }

    pub fn in_field(&self) -> bool {
        self.leftup.x >= 0
            && self.leftup.y >= 0
            && self.rightdown.x < SIDE as isize
            && self.rightdown.y < SIDE as isize
    }

    pub fn move_to(&self, vector: &Coord) -> Self {
        Self {
            leftup: self.leftup.plus(vector),
            rightdown: self.rightdown.plus(vector),
        }
    }

    pub fn does_include_point(&self, point: &Coord) -> bool {
        let &Coord { x, y } = point;
        self.leftup.x <= x && x <= self.rightdown.x && self.leftup.y <= y && y <= self.rightdown.y
    }

    /// thatと一部でも重なってるならtrue。ただし、辺が重なってるだけならfalse。
    pub fn does_overlap_rect(&self, that: &Rectangle) -> bool {
        let in_x_overwrapped = self.leftup.x < that.rightdown.x && self.rightdown.x > that.leftup.x;
        let in_y_overwrapped = self.leftup.y < that.rightdown.y && self.rightdown.y > that.leftup.y;
        in_x_overwrapped && in_y_overwrapped
    }
}
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rect{{x: {}-{}, y: {}-{}}}",
            self.leftup.x, self.rightdown.x, self.leftup.y, self.rightdown.y
        )?;
        Ok(())
    }
}
impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rect{{x: {}-{}, y: {}-{}}}",
            self.leftup.x, self.rightdown.x, self.leftup.y, self.rightdown.y
        )?;
        Ok(())
    }
}
